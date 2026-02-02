//! Help overlay.

use crate::config::Theme;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

const HELP_LINES: &[(&str, &str)] = &[
    ("↑/k", "Previous monitor"),
    ("↓/j", "Next monitor"),
    ("Tab", "Switch panel"),
    ("Enter/e", "Edit monitor"),
    ("m", "Move mode"),
    ("Space", "Toggle primary"),
    ("d", "Enable/disable"),
    ("r", "Cycle rotation"),
    ("a", "Apply (live)"),
    ("s", "Save to config"),
    ("x", "Export snippet"),
    ("p", "Profiles"),
    ("u", "Undo"),
    ("R", "Reset"),
    ("?", "This help"),
    ("q", "Quit"),
];

/// Draw help overlay.
pub fn help_overlay(f: &mut Frame, area: Rect, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .title(" Help ");
    let inner = block.inner(area);
    f.render_widget(block, area);
    let lines: Vec<Line> = HELP_LINES
        .iter()
        .map(|(k, v)| {
            Line::from(vec![
                Span::styled(
                    format!("{:12}", k),
                    Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
                ),
                Span::raw(*v),
            ])
        })
        .chain([Line::from(""), Line::from(Span::styled(
            "Press ? to close",
            Style::default().fg(theme.fg_dim),
        ))])
        .collect();
    let p = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(p, inner);
}
