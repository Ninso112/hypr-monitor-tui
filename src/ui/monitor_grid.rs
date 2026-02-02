//! Visual monitor grid for placement.

use crate::config::Theme;
use crate::hyprland::Monitor;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

/// Minimum size for a monitor box so positions stay visible.
const MIN_BOX_W: u16 = 12;
const MIN_BOX_H: u16 = 4;

/// Scale factor and bounds for fitting all monitors in the grid area.
fn scale_and_bounds(monitors: &[Monitor], inner: Rect) -> (f32, i32, i32, i32, i32) {
    if monitors.is_empty() {
        return (1.0, 0, 0, 0, 0);
    }
    let (min_x, max_x, min_y, max_y) = monitors.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), m| {
            let w = m.resolution.width as i32;
            let h = m.resolution.height as i32;
            let x2 = m.position.x + w;
            let y2 = m.position.y + h;
            (
                min_x.min(m.position.x),
                max_x.max(x2),
                min_y.min(m.position.y),
                max_y.max(y2),
            )
        },
    );
    let total_w = (max_x - min_x).max(1);
    let total_h = (max_y - min_y).max(1);
    let area_w = inner.width as i32;
    let area_h = inner.height as i32;
    let scale_x = area_w as f32 / total_w as f32;
    let scale_y = area_h as f32 / total_h as f32;
    let scale = scale_x.min(scale_y).clamp(0.01, 10.0);
    (scale, min_x, min_y, max_x, max_y)
}

/// Draw the monitor grid with monitors at their relative positions.
pub fn monitor_grid(
    f: &mut Frame,
    area: Rect,
    monitors: &[Monitor],
    selected: usize,
    theme: &Theme,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" Monitor Grid (Positions) ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    if monitors.is_empty() {
        let p = Paragraph::new("No monitors detected")
            .style(Style::default().fg(theme.fg_dim));
        f.render_widget(p, inner);
        return;
    }

    let (scale, min_x, min_y, _, _) = scale_and_bounds(monitors, inner);

    for (i, m) in monitors.iter().enumerate() {
        let rel_x = (m.position.x - min_x) as f32 * scale;
        let rel_y = (m.position.y - min_y) as f32 * scale;
        let w = (m.resolution.width as f32 * scale).max(MIN_BOX_W as f32) as u16;
        let h = (m.resolution.height as f32 * scale).max(MIN_BOX_H as f32) as u16;
        let x = inner.x + rel_x as u16;
        let y = inner.y + rel_y as u16;
        let box_w = w.min(inner.width.saturating_sub(rel_x as u16));
        let box_h = h.min(inner.height.saturating_sub(rel_y as u16));
        if box_w == 0 || box_h == 0 {
            continue;
        }
        let box_area = Rect::new(x, y, box_w, box_h);
        let border_style = if i == selected {
            Style::default().fg(theme.border_active)
        } else {
            Style::default().fg(theme.border)
        };
        let title = if m.primary {
            format!(" {} ★", m.name)
        } else {
            format!(" {} ", m.name)
        };
        let b = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(title)
            .style(Style::default().bg(theme.selection));
        let inner_box = b.inner(box_area);
        f.render_widget(b, box_area);
        let lines = vec![
            Line::from(format!("{}", m.resolution)),
            Line::from(format!("{} Hz", m.refresh_rate as u32)),
        ];
        let p = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.fg));
        f.render_widget(p, inner_box);
    }
}
