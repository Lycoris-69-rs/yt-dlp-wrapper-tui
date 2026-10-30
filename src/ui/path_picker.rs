use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

use super::common::{centered_rect, popup_block};

pub fn draw_path_picker(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);
    let favs = &app.config.paths.favorites;

    let mut lines: Vec<Line> = Vec::new();
    for (i, path) in favs.iter().enumerate() {
        let selected = i == app.path_picker_index;
        let style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let marker = if &app.form.path == path { "●" } else { " " };
        lines.push(Line::from(Span::styled(
            format!(" {} {} ", marker, path),
            style,
        )));
    }

    let idx_current = favs.len();
    let selected = app.path_picker_index == idx_current;
    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    lines.push(Line::from(Span::styled(
        format!(" + Add current path: {} ", app.form.path),
        style,
    )));

    let idx_new = favs.len() + 1;
    let selected = app.path_picker_index == idx_new;
    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    lines.push(Line::from(Span::styled(
        " + Add new path… ".to_string(),
        style,
    )));

    let idx_delete = favs.len() + 2;
    let selected = app.path_picker_index == idx_delete;
    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };
    lines.push(Line::from(Span::styled(
        " - Select and delete path… ".to_string(),
        style,
    )));

    let p = Paragraph::new(lines)
        .block(popup_block(" Select Path "))
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}

pub fn draw_path_input(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(60, 20, area);
    f.render_widget(Clear, popup);
    let byte_idx = app.path_input_cursor.min(app.path_input.len());
    let (before, after) = app.path_input.split_at(byte_idx);
    let display = format!("{}█{}", before, after);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Path: "),
            Span::styled(display, Style::default().fg(Color::Black).bg(Color::Cyan)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(popup_block(" New Path "))
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}

pub fn draw_path_delete(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        " Select path to DELETE:",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for (i, path) in app.config.paths.favorites.iter().enumerate() {
        let selected = i == app.path_picker_index;
        let style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        lines.push(Line::from(Span::styled(format!(" {} ", path), style)));
    }

    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(Span::styled(
                    " Delete Path ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}
