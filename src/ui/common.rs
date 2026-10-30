use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn popup_block(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
}

pub const MENU_W: u16 = 62;
pub const MENU_H: u16 = 72;

pub fn menu_rect(area: Rect) -> Rect {
    centered_rect(MENU_W, MENU_H, area)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn col_label(col: &str) -> &'static str {
    match col {
        "num" => "#",
        "added" => "Added",
        "name" => "Name",
        "url" => "URL",
        "mode" => "Mode",
        "quality" => "Quality",
        "status" => "Status",
        "progress" => "Progress",
        "percent" => "%",
        "speed" => "Speed",
        _ => "?",
    }
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.width() <= max {
        return s.to_string();
    }
    if max <= 3 {
        return s.chars().take(max).collect();
    }
    let budget = max - 3;
    let mut width = 0;
    let mut out = String::new();
    for ch in s.chars() {
        let cw = ch.width().unwrap_or(0);
        if width + cw > budget {
            break;
        }
        width += cw;
        out.push(ch);
    }
    out.push_str("...");
    out
}

pub fn distribute(total: u16, weights: &[u16]) -> Vec<u16> {
    let n = weights.len();
    if n == 0 {
        return Vec::new();
    }
    let total = total as u32;
    let sum_w: u32 = weights.iter().map(|&w| w.max(1) as u32).sum();

    if total < n as u32 {
        return vec![1; n];
    }

    if sum_w == 0 {
        let each = (total / n as u32) as u16;
        return vec![each.max(1); n];
    }

    let mut out: Vec<u16> = weights
        .iter()
        .map(|&w| {
            let w = w.max(1) as u32;
            ((total * w) / sum_w).max(1) as u16
        })
        .collect();

    let mut sum: i64 = out.iter().map(|&x| x as i64).sum();
    let target = total as i64;
    let mut i = 0usize;
    while sum != target && i < n * 8 {
        let idx = i % n;
        if sum < target {
            out[idx] += 1;
            sum += 1;
        } else if out[idx] > 1 {
            out[idx] -= 1;
            sum -= 1;
        }
        i += 1;
    }
    out
}

pub fn compute_column_widths(cfg: &crate::config::QueueTableConfig, available: u16) -> Vec<u16> {
    let cols = &cfg.columns;
    let n = cols.len();
    if n == 0 {
        return Vec::new();
    }

    let sep_total = (n.saturating_sub(1)) as u16;
    let usable = available.saturating_sub(sep_total);

    let mut widths: Vec<u16> = vec![0; n];
    let mut auto_indices: Vec<usize> = Vec::new();
    let mut fixed_sum: u32 = 0;
    for (i, c) in cols.iter().enumerate() {
        if cfg.is_auto(c) {
            auto_indices.push(i);
        } else {
            let w = cfg.width_of(c).max(1);
            widths[i] = w;
            fixed_sum += w as u32;
        }
    }

    if !auto_indices.is_empty() {
        let remaining = (usable as u32).saturating_sub(fixed_sum);
        let min_total = auto_indices.len() as u32;
        let to_distribute = remaining.max(min_total).min(u16::MAX as u32) as u16;

        let weights: Vec<u16> = auto_indices
            .iter()
            .map(|&i| cfg.weight_of(&cols[i]))
            .collect();
        let parts = distribute(to_distribute, &weights);
        for (k, &i) in auto_indices.iter().enumerate() {
            widths[i] = parts.get(k).copied().unwrap_or(1).max(1);
        }
    }

    let total: u32 = widths.iter().map(|&w| w as u32).sum();
    if total > usable as u32 && usable > 0 {
        let scale = usable as f32 / total as f32;
        for w in widths.iter_mut() {
            *w = ((*w as f32 * scale).floor() as u16).max(1);
        }
    }

    widths
}
