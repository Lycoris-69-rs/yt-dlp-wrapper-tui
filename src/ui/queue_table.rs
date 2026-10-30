use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, Focus};
use crate::config::Alignment;
use crate::queue::{DownloadMode, ItemState};

use super::common::{col_label, compute_column_widths, truncate};

const SEP: &str = " ";

pub fn draw_queue(f: &mut Frame, app: &mut App, area: Rect) {
    let cfg = &app.config.queue_table;
    let n = cfg.columns.len();
    if n == 0 {
        return;
    }

    const BORDERS: u16 = 2;
    let available = area.width.saturating_sub(BORDERS);

    let widths_u16 = compute_column_widths(cfg, available);

    let header_cells: Vec<ratatui::widgets::Cell> = cfg
        .columns
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let real_w = widths_u16.get(i).copied().unwrap_or(1);
            let content_w = if i + 1 < n {
                real_w.saturating_sub(1) as usize
            } else {
                real_w as usize
            };
            let mut t = col_label(c).to_string();
            t = pad_align(&t, content_w, cfg.alignment_of(c));
            if i + 1 < n {
                t.push_str(SEP);
            }
            ratatui::widgets::Cell::from(t)
        })
        .collect();
    let header = Row::new(header_cells).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .queue
        .iter()
        .map(|item| {
            let cells: Vec<ratatui::widgets::Cell> = cfg
                .columns
                .iter()
                .enumerate()
                .map(|(j, col)| {
                    let real_w = widths_u16.get(j).copied().unwrap_or(1);
                    let content_w = if j + 1 < n {
                        real_w.saturating_sub(1) as usize
                    } else {
                        real_w as usize
                    };

                    let raw = match col.as_str() {
                        "num" => format!("{}", item.id),
                        "added" => item.added_at_str(),
                        "name" => truncate(&item.display_name(), content_w),
                        "url" => truncate(&item.url, content_w),
                        "mode" => item.mode.label().to_string(),
                        "quality" => match item.mode {
                            DownloadMode::Video => item.quality.label().to_string(),
                            DownloadMode::Audio => "—".to_string(),
                        },
                        "status" => truncate(&item.state.label(), content_w),
                        "progress" => match &item.state {
                            ItemState::Downloading { percent, .. } => {
                                render_progress_bar(*percent, content_w)
                            }
                            ItemState::Done => render_progress_bar(100.0, content_w),
                            _ => String::new(),
                        },
                        "percent" => match &item.state {
                            ItemState::Downloading { percent, .. } => {
                                format!("{:.1}%", percent)
                            }
                            ItemState::Done => "100.0%".to_string(),
                            _ => String::new(),
                        },
                        "speed" => match &item.state {
                            ItemState::Downloading { speed, .. } => truncate(speed, content_w),
                            _ => String::new(),
                        },
                        _ => String::new(),
                    };

                    let mut text = pad_align(&raw, content_w, cfg.alignment_of(col));
                    if j + 1 < n {
                        text.push_str(SEP);
                    }
                    let style = Style::default().fg(column_color(col));
                    ratatui::widgets::Cell::from(text).style(style)
                })
                .collect();
            Row::new(cells)
        })
        .collect();

    let widths: Vec<Constraint> = widths_u16.iter().map(|&w| Constraint::Length(w)).collect();

    let border_style = if app.focus == Focus::Queue {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let title = if app.focus == Focus::Queue {
        " Queue "
    } else {
        " Queue "
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        .row_highlight_style(if app.focus == Focus::Queue {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().bg(Color::Rgb(40, 40, 40))
        });

    let mut state = TableState::default();
    if !app.queue.is_empty() {
        state.select(Some(app.selected.min(app.queue.len().saturating_sub(1))));
    }
    f.render_stateful_widget(table, area, &mut state);
}

fn column_color(_col: &str) -> Color {
    Color::White
}

fn pad_align(s: &str, width: usize, align: Alignment) -> String {
    let w = s.width();
    if w >= width {
        return s.to_string();
    }
    let pad = width - w;
    match align {
        Alignment::Left => {
            let mut out = String::with_capacity(s.len() + pad);
            out.push_str(s);
            out.push_str(&" ".repeat(pad));
            out
        }
        Alignment::Right => {
            let mut out = String::with_capacity(s.len() + pad);
            out.push_str(&" ".repeat(pad));
            out.push_str(s);
            out
        }
        Alignment::Center => {
            let left = pad / 2;
            let right = pad - left;
            let mut out = String::with_capacity(s.len() + pad);
            out.push_str(&" ".repeat(left));
            out.push_str(s);
            out.push_str(&" ".repeat(right));
            out
        }
    }
}

fn render_progress_bar(percent: f32, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let pct = percent.clamp(0.0, 100.0);
    let filled = ((pct / 100.0) * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
