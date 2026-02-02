//! Settings panel (resolution, Hz, scale).

use crate::config::Theme;
use crate::hyprland::Monitor;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use super::widgets::scale_slider;

/// Draw the settings panel for the selected monitor.
pub fn settings_panel(
    f: &mut Frame,
    area: Rect,
    monitor: Option<&Monitor>,
    theme: &Theme,
    _edit_field: Option<crate::app::EditField>,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" Settings ");
    let inner = block.inner(area);
    f.render_widget(block, area);
    let Some(m) = monitor else {
        let p = Paragraph::new("No monitor selected").style(Style::default().fg(theme.fg_dim));
        f.render_widget(p, inner);
        return;
    };
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(inner);
    // Preview: resolution with refresh rate below
    let preview_lines = vec![
        Line::from(vec![
            Span::styled("Resolution ", Style::default().fg(theme.fg_dim)),
            Span::styled(m.resolution.to_string(), Style::default().fg(theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("Refresh Rate ", Style::default().fg(theme.fg_dim)),
            Span::styled(
                format!("{:.0} Hz", m.refresh_rate),
                Style::default().fg(theme.fg),
            ),
        ]),
    ];
    let preview = Paragraph::new(preview_lines);
    f.render_widget(preview, chunks[0]);
    let scale_line = Line::from(vec![
        Span::styled("Scale ", Style::default().fg(theme.fg_dim)),
        Span::styled(format!("{:.2}", m.scale), Style::default().fg(theme.fg)),
    ]);
    f.render_widget(Paragraph::new(scale_line), chunks[1]);
    scale_slider(f, chunks[2], m.scale, false, theme);
    let transform_line = Line::from(vec![
        Span::styled("Transform ", Style::default().fg(theme.fg_dim)),
        Span::styled(
            format!("{}", m.transform),
            Style::default().fg(theme.fg),
        ),
    ]);
    f.render_widget(Paragraph::new(transform_line), chunks[3]);
    let enabled = if m.enabled { "Yes" } else { "No" };
    let primary = if m.primary { "Yes" } else { "No" };
    let opts_line = Line::from(vec![
        Span::styled("Enabled: ", Style::default().fg(theme.fg_dim)),
        Span::styled(enabled, Style::default().fg(theme.fg)),
        Span::raw("  "),
        Span::styled("Primary: ", Style::default().fg(theme.fg_dim)),
        Span::styled(primary, Style::default().fg(theme.fg)),
    ]);
    f.render_widget(Paragraph::new(opts_line), chunks[4]);
}
