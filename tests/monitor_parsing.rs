//! Monitor data parsing tests.

use hypr_monitor_tui::hyprland::{Resolution, Transform};

#[test]
fn test_resolution_to_string() {
    let r = Resolution {
        width: 2560,
        height: 1440,
    };
    assert_eq!(r.to_string(), "2560x1440");
}

#[test]
fn test_transform_cycle() {
    let t = Transform::Normal;
    let t1 = t.next();
    let t2 = t1.next();
    assert_ne!(t, t1);
    assert_ne!(t1, t2);
}

#[test]
fn test_transform_hyprland_str() {
    assert_eq!(Transform::Normal.to_hyprland_str(), "0");
    assert_eq!(Transform::Rotate90.to_hyprland_str(), "1");
}

#[test]
fn test_transform_from_hyprland_str() {
    assert!(matches!(Transform::from_hyprland_str("0"), Transform::Normal));
    assert!(matches!(Transform::from_hyprland_str("1"), Transform::Rotate90));
}
