//! hypr-monitor-tui - TUI for Hyprland multi-monitor configuration.

#![deny(clippy::unwrap_used)]

use anyhow::{Context, Result};
use clap::Parser;
use hypr_monitor_tui::app::{App, AppMode, ConfirmAction, EditField};
use hypr_monitor_tui::config;
use hypr_monitor_tui::events;
use hypr_monitor_tui::hyprland;
use hypr_monitor_tui::ui;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .map(|p| p.join("hypr-monitor-tui").join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("config.toml"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(log_level.parse().unwrap_or(tracing::Level::INFO.into())),
        )
        .init();

    if cli.apply.is_some() && !cli.dry_run {
        return apply_profile(&cli);
    }
    if cli.export {
        return export_config(&cli);
    }
    if cli.list_profiles {
        return list_profiles(&cli);
    }
    run_tui(&cli)
}

fn apply_profile(cli: &Cli) -> Result<()> {
    if !hyprland::HyprlandClient::is_available() {
        anyhow::bail!(
            "Hyprland is not running or not reachable. Run hypr-monitor-tui inside a Hyprland session."
        );
    }
    let profile_name = cli.apply.as_deref().context("--apply requires profile name")?;
    let config_dir = cli
        .config
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let profile_path = config_dir.join("profiles").join(format!("{}.toml", profile_name));
    let profile = config::load_profile(&profile_path)
        .with_context(|| format!("Failed to load profile: {}", profile_name))?;
    let client = hyprland::HyprlandClient::new();
    let current = client.get_monitors().context("Failed to get current monitors")?;
    let monitors = profile.to_monitors(&current);
    if cli.dry_run {
        println!("Would apply {} monitors from profile {}", monitors.len(), profile_name);
        for m in &monitors {
            println!("  {}: {} @ {}Hz, {}x{}, scale {}", m.name, m.resolution, m.refresh_rate, m.position.x, m.position.y, m.scale);
        }
        return Ok(());
    }
    client.apply_all(&monitors).context("Failed to apply configuration")?;
    println!("Applied profile: {}", profile_name);
    Ok(())
}

fn export_config(_cli: &Cli) -> Result<()> {
    if !hyprland::HyprlandClient::is_available() {
        anyhow::bail!(
            "Hyprland is not running or not reachable. Run hypr-monitor-tui inside a Hyprland session."
        );
    }
    let client = hyprland::HyprlandClient::new();
    let monitors = client.get_monitors().context("Failed to get monitors")?;
    let out = hyprland::generate_config(&monitors);
    print!("{}", out);
    Ok(())
}

fn list_profiles(cli: &Cli) -> Result<()> {
    let config_dir = cli.config.parent().unwrap_or_else(|| std::path::Path::new("."));
    let profiles_dir = config_dir.join("profiles");
    let names = config::list_profiles(&profiles_dir);
    for n in &names {
        println!("{}", n);
    }
    Ok(())
}

fn run_tui(cli: &Cli) -> Result<()> {
    let client = hyprland::HyprlandClient::new();
    let monitors = if hyprland::HyprlandClient::is_available() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let c = hyprland::HyprlandClient::new();
            c.get_monitors()
        })) {
            Ok(Ok(m)) => m,
            Ok(Err(e)) => {
                tracing::warn!("Could not get monitors: {}", e);
                Vec::new()
            }
            Err(_) => {
                tracing::warn!("Hyprland socket not reachable, starting with empty monitor list");
                Vec::new()
            }
        }
    } else {
        tracing::warn!("Hyprland not available, starting with empty monitor list");
        Vec::new()
    };

    let mut theme = config::Theme::default();
    if cli.config.exists() {
        if let Ok(s) = std::fs::read_to_string(&cli.config) {
            if let Ok(cfg) = toml::from_str::<config::AppConfig>(&s) {
                cfg.theme.apply_to(&mut theme);
            }
        }
    }

    let mut app = App::new(monitors, cli.config.clone(), theme);
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    let res = run_loop(&mut terminal, &mut app, &client);

    disable_raw_mode().context("Failed to disable raw mode")?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    if let Err(e) = res {
        tracing::error!("{:?}", e);
        return Err(e);
    }
    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    client: &hyprland::HyprlandClient,
) -> Result<()> {
    let mut event_handler = events::EventHandler::new(Duration::from_millis(100));
    loop {
        terminal.draw(|f| ui::draw(f, app, &app.theme))?;
        let ev = event_handler.recv_event().map_err(anyhow::Error::msg)?;
        match ev {
            events::AppEvent::Key(key) => {
                if handle_key(app, client, key) {
                    break;
                }
            }
            events::AppEvent::Resize(_, _) => {}
            events::AppEvent::Tick => {
                app.clear_messages();
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_key(
    app: &mut App,
    client: &hyprland::HyprlandClient,
    key: KeyEvent,
) -> bool {
    let code = key.code;
    let mods = key.modifiers;

    match &app.mode {
        AppMode::Confirm { action, .. } => {
            if code == KeyCode::Char('y') || code == KeyCode::Char('Y') {
                confirm_yes(app, client, action.clone());
                app.mode = AppMode::Normal;
            } else if code == KeyCode::Char('n') || code == KeyCode::Char('N') || code == KeyCode::Esc {
                app.mode = AppMode::Normal;
            }
            return false;
        }
        AppMode::Help => {
            if code == KeyCode::Char('?') || code == KeyCode::Esc {
                app.mode = AppMode::Normal;
            }
            return false;
        }
        AppMode::ProfileSelect => {
            if code == KeyCode::Esc {
                app.mode = AppMode::Normal;
            }
            return false;
        }
        _ => {}
    }

    if code == KeyCode::Char('q') && mods != KeyModifiers::CONTROL {
        if app.unsaved_changes {
            app.mode = AppMode::Confirm {
                action: ConfirmAction::Quit,
                message: "Unsaved changes. Quit anyway?".to_string(),
            };
        } else {
            return true;
        }
        return false;
    }

    match code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.selected_monitor = app.selected_monitor.saturating_sub(1);
            if !app.monitors.is_empty() && app.selected_monitor >= app.monitors.len() {
                app.selected_monitor = app.monitors.len() - 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.selected_monitor + 1 < app.monitors.len() {
                app.selected_monitor += 1;
            }
        }
        KeyCode::Tab => {
            app.focus_settings = !app.focus_settings;
        }
        KeyCode::BackTab => {
            app.focus_settings = !app.focus_settings;
        }
        KeyCode::Enter | KeyCode::Char('e') => {
            app.mode = AppMode::Editing {
                field: EditField::Resolution,
            };
        }
        KeyCode::Char('m') => {
            app.mode = AppMode::Moving;
        }
        KeyCode::Char(' ') => {
            let idx = app.selected_monitor;
            if idx < app.monitors.len() {
                for mon in app.monitors.iter_mut() {
                    mon.primary = false;
                }
                app.monitors[idx].primary = true;
                app.unsaved_changes = true;
            }
        }
        KeyCode::Char('d') => {
            if let Some(m) = app.selected_mut() {
                m.enabled = !m.enabled;
                app.unsaved_changes = true;
            }
        }
        KeyCode::Char('r') => {
            if let Some(m) = app.selected_mut() {
                m.transform = m.transform.next();
                app.unsaved_changes = true;
            }
        }
        KeyCode::Char('a') => {
            app.push_history();
            if let Err(e) = client.apply_all(&app.monitors) {
                app.set_error(e.to_string());
            } else {
                app.set_status("Applied.".to_string());
                app.unsaved_changes = false;
            }
        }
        KeyCode::Char('s') => {
            let path = app
                .config_path
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("profiles");
            let _ = std::fs::create_dir_all(&path);
            let profile = config::Profile::from_monitors(
                "default".to_string(),
                None,
                &app.monitors,
            );
            let p = path.join("default.toml");
            if let Err(e) = config::save_profile(&p, &profile) {
                app.set_error(e.to_string());
            } else {
                app.set_status("Saved to default profile.".to_string());
                app.unsaved_changes = false;
            }
        }
        KeyCode::Char('x') => {
            let out = hyprland::generate_config(&app.monitors);
            if let Err(e) = std::io::Write::write_all(&mut std::io::stdout(), out.as_bytes()) {
                app.set_error(e.to_string());
            } else {
                app.set_status("Exported to stdout.".to_string());
            }
        }
        KeyCode::Char('p') => {
            app.mode = AppMode::ProfileSelect;
        }
        KeyCode::Char('u') => {
            app.undo();
        }
        KeyCode::Char('R') => {
            app.mode = AppMode::Confirm {
                action: ConfirmAction::Reset,
                message: "Reset to current Hyprland config?".to_string(),
            };
        }
        KeyCode::Char('?') => {
            app.mode = AppMode::Help;
        }
        _ => {}
    }
    false
}

fn confirm_yes(app: &mut App, client: &hyprland::HyprlandClient, action: ConfirmAction) {
    match action {
        ConfirmAction::Quit => {
            std::process::exit(0);
        }
        ConfirmAction::Apply => {
            let _ = client.apply_all(&app.monitors);
            app.unsaved_changes = false;
        }
        ConfirmAction::Save => {}
        ConfirmAction::Reset => {
            if let Ok(monitors) = client.get_monitors() {
                app.push_history();
                app.monitors = monitors;
                app.selected_monitor = app.selected_monitor.min(app.monitors.len().saturating_sub(1));
                app.unsaved_changes = false;
            }
        }
        ConfirmAction::DeleteProfile(_) => {}
    }
}

#[derive(clap::Parser, Debug)]
#[command(name = "hypr-monitor-tui", version, about = "TUI for Hyprland multi-monitor configuration")]
struct Cli {
    #[arg(short, long, default_value_os_t = default_config_path())]
    config: PathBuf,
    #[arg(short, long)]
    apply: Option<String>,
    #[arg(short, long)]
    export: bool,
    #[arg(short = 'l', long)]
    list_profiles: bool,
    #[arg(short, long)]
    dry_run: bool,
    #[arg(short, long)]
    verbose: bool,
}
