//! Config generation tests.

use hypr_monitor_tui::hyprland::{Monitor, Position, Resolution, Transform};
use hypr_monitor_tui::hyprland::generate_config;

fn make_monitor(name: &str, x: i32, y: i32, w: u32, h: u32, hz: f32, scale: f32) -> Monitor {
    let res = Resolution { width: w, height: h };
    Monitor {
        name: name.to_string(),
        description: name.to_string(),
        position: Position { x, y },
        resolution: res.clone(),
        available_resolutions: vec![res],
        refresh_rate: hz,
        available_refresh_rates: vec![hz],
        scale,
        transform: Transform::Normal,
        enabled: true,
        primary: name == "DP-1",
    }
}

#[test]
fn test_generate_config_single_monitor() {
    let monitors = vec![make_monitor("DP-1", 0, 0, 2560, 1440, 144.0, 1.0)];
    let out = generate_config(&monitors);
    assert!(out.contains("monitor=DP-1"));
    assert!(out.contains("2560x1440"));
    assert!(out.contains("144.00"));
    assert!(out.contains("0x0"));
    assert!(out.contains(",1"));
}

#[test]
fn test_generate_config_two_monitors() {
    let monitors = vec![
        make_monitor("DP-1", 0, 0, 2560, 1440, 144.0, 1.0),
        make_monitor("HDMI-A-1", 2560, 0, 1920, 1080, 60.0, 1.0),
    ];
    let out = generate_config(&monitors);
    assert!(out.contains("DP-1"));
    assert!(out.contains("HDMI-A-1"));
    assert!(out.contains("2560x0"));
}

#[test]
fn test_generate_config_disabled_omitted() {
    let mut m = make_monitor("DP-1", 0, 0, 1920, 1080, 60.0, 1.0);
    m.enabled = false;
    let monitors = vec![m];
    let out = generate_config(&monitors);
    assert!(!out.contains("monitor=DP-1"));
}
