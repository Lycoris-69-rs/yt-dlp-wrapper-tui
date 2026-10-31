use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
};

use crate::app::App;

use super::common::{centered_rect, popup_block};

pub fn draw_dependencies(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(64, 60, area);
    f.render_widget(Clear, popup);

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            " Required tools are missing",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if app.missing_deps.is_empty() {
        lines.push(Line::from(Span::styled(
            " Checking dependencies…",
            Style::default().fg(Color::DarkGray),
        )));
    }

    for dep in &app.missing_deps {
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {} ", dep.name()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                dep.description().to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
        lines.push(Line::from(Span::styled(
            format!("   {} · {}", dep.action(), dep.download_url()),
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(""));
    }

    if app.deps_downloading {
        lines.push(Line::from(Span::styled(
            " Downloading… please wait",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            " Enter — download · Esc — continue anyway",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let p = Paragraph::new(lines)
        .block(popup_block(" Dependencies "))
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}
