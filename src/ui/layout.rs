//! Main TUI layout.

use crate::app::{App, AppMode};
use crate::config::Theme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::dialogs::confirmation;
use super::help::help_overlay;
use super::monitor_grid::monitor_grid;
use super::preview::status_line;
use super::settings_panel::settings_panel;

/// Main layout: header, content (grid | settings), footer.
pub fn draw(f: &mut Frame, app: &App, theme: &Theme) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(area);
    draw_header(f, chunks[0], theme);
    draw_content(f, chunks[1], app, theme);
    draw_footer(f, chunks[2], app, theme);

    if app.mode == AppMode::Help {
        let help_area = centered_rect(60, 70, area);
        help_overlay(f, help_area, theme);
    }
    if let AppMode::Confirm { action, message } = &app.mode {
        let dialog_area = centered_rect(50, 30, area);
        confirmation(f, dialog_area, action, message, theme);
    }
}

fn draw_header(f: &mut Frame, area: Rect, theme: &Theme) {
    let line = Line::from(vec![
        Span::styled(" hypr-monitor-tui ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::raw(" v0.1.0 "),
        Span::styled(" [?) Help ", Style::default().fg(theme.fg_dim)),
    ]);
    let p = Paragraph::new(line);
    f.render_widget(p, area);
}

fn draw_content(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);
    monitor_grid(
        f,
        chunks[0],
        &app.monitors,
        app.selected_monitor,
        theme,
    );
    let monitor = app.selected();
    settings_panel(
        f,
        chunks[1],
        monitor,
        theme,
        None,
    );
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let status = app.status_message.as_deref();
    let err = app.error_message.as_deref();
    if status.is_some() || err.is_some() {
        status_line(f, area, status, err, theme);
    } else {
        let line = Line::from(Span::styled(
            "[↑↓] Select  [Enter] Edit  [m] Move  [a] Apply  [?] Help  [q] Quit",
            Style::default().fg(theme.fg_dim),
        ));
        let p = Paragraph::new(line);
        f.render_widget(p, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_w = area.width * percent_x / 100;
    let popup_h = area.height * percent_y / 100;
    let x = area.x + (area.width.saturating_sub(popup_w)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_h)) / 2;
    Rect::new(x, y, popup_w, popup_h)
}
