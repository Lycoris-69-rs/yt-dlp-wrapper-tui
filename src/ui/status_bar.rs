use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;
use crate::queue::ItemState;

const SEP: &str = "│";
const HINT_BG: Color = Color::Yellow;

pub fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    if !app.config.status_bar.enabled {
        return;
    }

    let sb = &app.config.status_bar;
    let hint_style = Style::default().fg(Color::Black).bg(HINT_BG);
    let sep = format!(" {} ", SEP);

    let mut spans: Vec<Span> = Vec::new();

    if sb.show_hints {
        spans.push(Span::styled(" F1 ", hint_style));
        spans.push(Span::raw(" help "));
        spans.push(Span::styled(" F2 ", hint_style));
        spans.push(Span::raw(" settings "));
        spans.push(Span::styled(" Tab ", hint_style));
        spans.push(Span::raw(" focus "));
    }

    if (sb.show_queue_stats || sb.show_active || sb.show_status_message) && !spans.is_empty() {
        spans.push(Span::raw(sep.clone()));
    }

    if sb.show_queue_stats {
        let total = app.queue.len();
        let done = app
            .queue
            .iter()
            .filter(|i| matches!(i.state, ItemState::Done))
            .count();
        spans.push(Span::raw(format!("Queue: {}/{}", done, total)));
    }

    if sb.show_active {
        let active = app
            .queue
            .iter()
            .filter(|i| matches!(i.state, ItemState::Downloading { .. }))
            .count();
        if sb.show_queue_stats {
            spans.push(Span::raw(sep.clone()));
        }
        spans.push(Span::raw(format!("Active: {}", active)));
    }

    if sb.show_status_message {
        if sb.show_queue_stats || sb.show_active {
            spans.push(Span::raw(sep.clone()));
        }
        spans.push(Span::styled(&app.status, Style::default().fg(Color::White)));
    }

    f.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(Color::Rgb(20, 20, 20))),
        area,
    );
}
