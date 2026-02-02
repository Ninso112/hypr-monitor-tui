//! Live configuration preview (status line).

use crate::config::Theme;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::layout::Rect;

/// Draw status / preview line.
pub fn status_line(f: &mut Frame, area: Rect, status: Option<&str>, error: Option<&str>, theme: &Theme) {
    let style = if error.is_some() {
        Style::default().fg(theme.error)
    } else {
        Style::default().fg(theme.fg_dim)
    };
    let text = error
        .or(status)
        .map(Line::from)
        .unwrap_or_else(|| Line::from(""));
    let p = Paragraph::new(text).style(style);
    f.render_widget(p, area);
}
