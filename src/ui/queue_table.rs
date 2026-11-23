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

use super::common::{col_label, compute_column_widths, marquee, truncate};

const SEP: &str = " ";

pub fn draw_queue(f: &mut Frame, app: &mut App, area: Rect) {
    let cfg = &app.config.queue_table;
    let n = cfg.columns.len();
    if n == 0 {
        return;
    }

    const BORDERS: u16 = 2;
    let pad = cfg.side_padding;
    let available = area.width.saturating_sub(BORDERS + pad.saturating_mul(2));

    let widths_u16 = compute_column_widths(cfg, available);

    let mut header_cells: Vec<ratatui::widgets::Cell> = Vec::new();
    if pad > 0 {
        header_cells.push(ratatui::widgets::Cell::from(""));
    }
    header_cells.extend(
        cfg.columns
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
            }),
    );
    if pad > 0 {
        header_cells.push(ratatui::widgets::Cell::from(""));
    }
    let header = Row::new(header_cells).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .queue
        .iter()
        .map(|item| {
            let mut cells: Vec<ratatui::widgets::Cell> = Vec::new();
            if pad > 0 {
                cells.push(ratatui::widgets::Cell::from(""));
            }
            cells.extend(
                cfg.columns
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
                            "name" => match &item.title {
                                Some(title) => display_name(title),
                                None => spinner_char(app.anim_frame).to_string(),
                            },
                            "url" => item.url.clone(),
                            "path" => item.path.clone(),
                            "mode" => item.mode.label().to_string(),
                            "quality" => match item.mode {
                                DownloadMode::Video => item.quality.label().to_string(),
                                DownloadMode::Audio => "—".to_string(),
                            },
                            "status" => item.state.label(),
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
                                ItemState::Downloading { speed, .. } => speed.clone(),
                                ItemState::Pending => String::new(),
                                _ => "-".to_string(),
                            },
                            _ => String::new(),
                        };

                        // Scroll text horizontally when it is wider than its column.
                        let shown = if scrolls(col) && raw.width() > content_w {
                            marquee(&raw, content_w, app.marquee_offset)
                        } else {
                            truncate(&raw, content_w)
                        };

                        let mut text = pad_align(&shown, content_w, cfg.alignment_of(col));
                        if j + 1 < n {
                            text.push_str(SEP);
                        }
                        let style = Style::default().fg(column_color(col));
                        ratatui::widgets::Cell::from(text).style(style)
                    }),
            );
            if pad > 0 {
                cells.push(ratatui::widgets::Cell::from(""));
            }
            Row::new(cells)
        })
        .collect();

    let mut widths: Vec<Constraint> = Vec::new();
    if pad > 0 {
        widths.push(Constraint::Length(pad.saturating_sub(1)));
    }
    widths.extend(widths_u16.iter().map(|&w| Constraint::Length(w)));
    if pad > 0 {
        widths.push(Constraint::Length(pad.saturating_sub(1)));
    }

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

/// Text columns whose content should scroll when wider than the cell.
fn scrolls(col: &str) -> bool {
    matches!(col, "name" | "url" | "path" | "status" | "speed")
}

/// Frame of the spinner shown in the Name column while the title is fetched.
fn spinner_char(frame: usize) -> char {
    const FRAMES: [char; 4] = ['-', '/', '|', '\\'];
    FRAMES[frame % FRAMES.len()]
}

fn is_dash(c: char) -> bool {
    matches!(c, '-' | '–' | '—' | '−')
}

/// Splits a title like `Artist - Album - Track` into its non-empty parts.
/// Supports regular, en, em and minus dashes surrounded by whitespace.
fn name_parts(title: &str) -> Vec<String> {
    let chars: Vec<char> = title.chars().collect();
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && is_dash(chars[j]) {
                let mut k = j;
                while k < chars.len() && is_dash(chars[k]) {
                    k += 1;
                }
                if k < chars.len() && chars[k].is_whitespace() {
                    let part = current.trim();
                    if !part.is_empty() {
                        parts.push(part.to_string());
                    }
                    current.clear();
                    i = k;
                    continue;
                }
            }
        }
        current.push(chars[i]);
        i += 1;
    }
    let part = current.trim();
    if !part.is_empty() {
        parts.push(part.to_string());
    }
    parts
}

/// Shows the track name: the last part of a title split by dashes.
fn display_name(title: &str) -> String {
    let parts = name_parts(title);
    if parts.is_empty() {
        title.to_string()
    } else {
        parts[parts.len() - 1].clone()
    }
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
