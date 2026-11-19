use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, PathPickerEntry};

use super::common::{centered_rect, popup_block};

fn section_header(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn label_style(highlighted: bool) -> Style {
    if highlighted {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn path_suffix(
    path: &str,
    form_path: &str,
    default_path: &str,
    highlighted: bool,
) -> Vec<Span<'static>> {
    let is_selected = path == form_path;
    let is_default = path == default_path;
    let text = match (is_selected, is_default) {
        (true, true) => "Selected by default",
        (true, false) => "Selected",
        (false, true) => "Default",
        (false, false) => return Vec::new(),
    };
    vec![Span::styled(text, label_style(highlighted))]
}

fn path_row(app: &App, fav_idx: usize, highlighted: bool) -> Line<'static> {
    let path = &app.config.paths.favorites[fav_idx];
    let marker = if highlighted { "▶" } else { " " };
    let base = if highlighted {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let mut spans = vec![Span::styled(format!(" {marker} {path}"), base)];
    let suffix = path_suffix(path, &app.form.path, &app.config.general.default_path, highlighted);
    if !suffix.is_empty() {
        spans.push(Span::styled("  ", base));
        spans.extend(suffix);
    }
    Line::from(spans)
}

fn action_row(text: String, color: Color, highlighted: bool) -> Line<'static> {
    let style = if highlighted {
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color)
    };
    Line::from(Span::styled(text, style))
}

pub fn draw_path_picker(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);

    let entries = app.path_picker_entries();
    let pinned_count = entries
        .iter()
        .filter(|e| matches!(e, PathPickerEntry::Pinned { .. }))
        .count();
    let path_count = entries
        .iter()
        .filter(|e| matches!(e, PathPickerEntry::Path { .. }))
        .count();
    let actions_start = pinned_count + path_count;

    let mut lines: Vec<Line> = Vec::new();
    for (i, entry) in entries.iter().enumerate() {
        if i == 0 {
            lines.push(section_header("Pinned paths:"));
        }
        if i == pinned_count {
            if i != 0 {
                lines.push(Line::from(""));
            }
            lines.push(section_header("Paths:"));
        }
        if i == actions_start {
            if i != 0 {
                lines.push(Line::from(""));
            }
            lines.push(section_header("Actions:"));
        }

        let highlighted = i == app.path_picker_index;
        let line = match entry {
            PathPickerEntry::Pinned { fav_idx } | PathPickerEntry::Path { fav_idx } => {
                path_row(app, *fav_idx, highlighted)
            }
            PathPickerEntry::AddCurrent => action_row(
                format!(" + Add current path: {} ", app.form.path),
                Color::Green,
                highlighted,
            ),
            PathPickerEntry::AddNew => {
                action_row(" + Add new path… ".to_string(), Color::Green, highlighted)
            }
            PathPickerEntry::Delete => {
                action_row(" − Select and delete path… ".to_string(), Color::Red, highlighted)
            }
            PathPickerEntry::Pin => {
                action_row(" Pin path ".to_string(), Color::Yellow, highlighted)
            }
        };
        lines.push(line);
    }

    let p = Paragraph::new(lines)
        .block(popup_block(" Select Path "))
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}

pub fn draw_path_pin(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);

    let entries = app.path_pin_entries();
    let pinned_count = entries
        .iter()
        .filter(|e| matches!(e, PathPickerEntry::Pinned { .. }))
        .count();

    let mut lines: Vec<Line> = Vec::new();
    if entries.is_empty() {
        lines.push(section_header("Pinned paths:"));
        lines.push(Line::from(""));
        lines.push(section_header("Paths:"));
        lines.push(Line::from(Span::styled(
            "  No saved paths.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, entry) in entries.iter().enumerate() {
            if i == 0 {
                lines.push(section_header("Pinned paths:"));
            }
            if i == pinned_count {
                if i != 0 {
                    lines.push(Line::from(""));
                }
                lines.push(section_header("Paths:"));
            }
            let highlighted = i == app.path_picker_index;
            if let Some(fav_idx) = entry.favorite_idx() {
                lines.push(path_row(app, fav_idx, highlighted));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Enter — move path between categories · Esc — back",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines)
        .block(popup_block(" Pin Paths "))
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
