//! Custom widgets (MonitorBox, ScaleSlider, etc.).

use crate::config::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

/// Draw a monitor box (name, resolution, Hz, primary star).
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn monitor_box(
    f: &mut Frame,
    area: Rect,
    name: &str,
    resolution: &str,
    hz: f32,
    primary: bool,
    selected: bool,
    theme: &Theme,
) {
    let border_style = if selected {
        Style::default().fg(theme.border_active)
    } else {
        Style::default().fg(theme.border)
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!(" {} ", name));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let primary_mark = if primary { " ★" } else { "" };
    let text = vec![
        Line::from(Span::raw(format!("{} {}", resolution, primary_mark))),
        Line::from(Span::raw(format!("{}Hz", hz as u32))),
    ];
    let p = Paragraph::new(text).style(Style::default().fg(theme.fg));
    f.render_widget(p, inner);
}

/// Draw a scale slider (0.5 to 3.0).
pub fn scale_slider(
    f: &mut Frame,
    area: Rect,
    value: f32,
    selected: bool,
    theme: &Theme,
) {
    let ratio = ((value - 0.5) / 2.5).clamp(0.0, 1.0);
    let style = if selected {
        Style::default().fg(theme.accent)
    } else {
        Style::default().fg(theme.border)
    };
    let gauge = Gauge::default()
        .gauge_style(style)
        .ratio(ratio as f64)
        .label(format!("{:.2}", value));
    f.render_widget(gauge, area);
}

/// Draw a list of options (e.g. resolutions).
#[allow(dead_code)]
pub fn option_list(
    f: &mut Frame,
    area: Rect,
    title: &str,
    items: &[String],
    selected_index: Option<usize>,
    theme: &Theme,
) {
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if selected_index == Some(i) {
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };
            ListItem::new(s.as_str()).style(style)
        })
        .collect();
    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(title),
    );
    f.render_widget(list, area);
}

/// Draw a labeled value row.
#[allow(dead_code)]
pub fn labeled_value(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    selected: bool,
    theme: &Theme,
) {
    let style = if selected {
        Style::default().fg(theme.accent)
    } else {
        Style::default().fg(theme.fg)
    };
    let line = Line::from(vec![
        Span::styled(label, Style::default().fg(theme.fg_dim)),
        Span::raw(" "),
        Span::styled(value, style),
    ]);
    let p = Paragraph::new(line).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}
