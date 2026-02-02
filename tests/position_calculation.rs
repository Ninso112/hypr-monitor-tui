//! Position/snap calculation tests.

#[test]
fn test_position_bounds() {
    let positions = [(0, 0), (2560, 0), (0, 1440)];
    let min_x = positions.iter().map(|p| p.0).min().unwrap();
    let max_x = positions.iter().map(|p| p.0).max().unwrap();
    let min_y = positions.iter().map(|p| p.1).min().unwrap();
    let max_y = positions.iter().map(|p| p.1).max().unwrap();
    assert_eq!(min_x, 0);
    assert_eq!(max_x, 2560);
    assert_eq!(min_y, 0);
    assert_eq!(max_y, 1440);
}

#[test]
fn test_scale_fit() {
    let total_w = 2560 + 1920;
    let total_h = 1440;
    let area_w = 80f32;
    let area_h = 24f32;
    let scale_x = (area_w - 4.0) / total_w as f32;
    let scale_y = (area_h - 2.0) / total_h as f32;
    let scale = scale_x.min(scale_y).min(1.0).max(0.1);
    assert!(scale > 0.0);
    assert!(scale <= 1.0);
}
