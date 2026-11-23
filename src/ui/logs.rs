use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

use crate::app::App;

use super::common::{menu_rect, popup_block};

pub fn draw_logs(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = menu_rect(area);
    f.render_widget(Clear, popup);

    let title = format!("Logs ({} lines)", app.logs.len());

    let mut lines: Vec<Line> = Vec::new();
    if app.logs.is_empty() {
        lines.push(Line::from(Span::styled(
            " No log output yet.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (id, text) in &app.logs {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("#{:<4} ", id),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw(text.clone()),
            ]));
        }
    }

    let visible = popup.height.saturating_sub(2).max(1) as usize;
    let max_scroll = lines.len().saturating_sub(visible) as u16;
    if app.logs_follow {
        app.logs_scroll = max_scroll;
    } else {
        app.logs_scroll = app.logs_scroll.min(max_scroll);
    }

    let p = Paragraph::new(lines)
        .block(popup_block(&title))
        .scroll((app.logs_scroll, 0));
    f.render_widget(p, popup);
}
