use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Focus, FormField};
use crate::queue::DownloadMode;

pub fn draw_form(f: &mut Frame, app: &mut App, area: Rect) {
    let border_style = if app.focus == Focus::Form {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let title = if app.focus == Focus::Form {
        " New Download "
    } else {
        " New Download "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let form = &app.form;
    let focused = app.focus == Focus::Form;

    let url_focused = focused && form.field == FormField::Url;
    let url_style = if url_focused {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default()
    };

    let mut spans = vec![Span::raw("URL:      ")];
    if url_focused && app.editing_url {
        let byte_idx = form.char_to_byte(form.url_cursor);
        let (before, after) = form.url.split_at(byte_idx);
        spans.push(Span::styled(before.to_string(), url_style));
        if let Some(next_ch) = after.chars().next() {
            let next_len = next_ch.len_utf8();
            let (cur, rest) = after.split_at(next_len);
            spans.push(Span::styled(
                cur.to_string(),
                Style::default().fg(Color::Cyan).bg(Color::Black),
            ));
            spans.push(Span::styled(rest.to_string(), url_style));
        }
    } else {
        spans.push(Span::styled(form.url.clone(), url_style));
    }
    spans.push(Span::styled(" ", url_style));

    f.render_widget(Paragraph::new(Line::from(spans)), chunks[0]);

    let type_focused = focused && form.field == FormField::Mode;
    let type_value = match form.mode {
        DownloadMode::Video => " Video ",
        DownloadMode::Audio => " Audio ",
    };
    let type_style = if type_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Type:     "),
            Span::styled(format!("< {} >", type_value.trim()), type_style),
        ])),
        chunks[1],
    );

    let pl_focused = focused && form.field == FormField::Playlist;
    let pl_value = if form.is_playlist {
        " Download entire playlist "
    } else {
        " Download only this "
    };
    let pl_style = if pl_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Playlist: "),
            Span::styled(format!("<{}>", pl_value), pl_style),
        ])),
        chunks[2],
    );

    let quality_focused = focused && form.field == FormField::Quality;
    let q_style = if quality_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Quality:  "),
            Span::styled(format!("< {} >", form.quality.label()), q_style),
        ])),
        chunks[3],
    );

    let path_focused = focused && form.field == FormField::Path;
    let p_style = if path_focused {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default()
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("Path:     "),
            Span::styled(format!(" {} ", form.path), p_style),
        ])),
        chunks[4],
    );

    let submit_focused = focused && form.field == FormField::Submit;
    let btn_style = if submit_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("          "),
            Span::styled(" [✓ Add to Queue (Enter)] ", btn_style),
        ])),
        chunks[6],
    );
}
