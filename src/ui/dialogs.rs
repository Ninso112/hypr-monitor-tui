//! Confirmation dialogs and popups.

use crate::app::ConfirmAction;
use crate::config::Theme;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

/// Draw a confirmation dialog.
pub fn confirmation(
    f: &mut Frame,
    area: Rect,
    action: &ConfirmAction,
    message: &str,
    theme: &Theme,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent))
        .title(" Confirm ");
    let inner = block.inner(area);
    f.render_widget(block, area);
    let action_str = match action {
        ConfirmAction::Quit => "Quit",
        ConfirmAction::Apply => "Apply changes",
        ConfirmAction::Save => "Save to file",
        ConfirmAction::Reset => "Reset to current config",
        ConfirmAction::DeleteProfile(_) => "Delete profile",
    };
    let text = vec![
        Line::from(Span::styled(
            action_str,
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::raw(message)),
        Line::from(""),
        Line::from(Span::styled(
            "[y] Yes  [n] No",
            Style::default().fg(theme.fg_dim),
        )),
    ];
    let p = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(p, inner);
}

/// Draw a simple message popup.
#[allow(dead_code)]
pub fn message(f: &mut Frame, area: Rect, title: &str, body: &str, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(format!(" {} ", title));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let p = Paragraph::new(body)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(theme.fg));
    f.render_widget(p, inner);
}
