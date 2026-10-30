use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
};

use crate::app::{App, DefaultsTab, QueueTab};
use crate::config::{ADVANCED_OPTION_COUNT, ALL_COLUMNS, AdvancedOptions, Alignment};

use super::common::{centered_rect, col_label, menu_rect, popup_block, truncate};

fn on_off(b: bool) -> &'static str {
    if b { "ON " } else { "OFF" }
}

pub fn draw_settings(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = menu_rect(area);
    f.render_widget(Clear, popup);

    let mut lines: Vec<Line> = vec![Line::from(Span::styled(
        " Categories",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];

    let categories = [
        ("Defaults", "General · Advanced (proxy, cookies, etc.)"),
        ("Status Bar", "Show/hide bottom bar and its sections"),
        ("Queue", "Columns order · widths · alignment"),
    ];

    for (i, (name, hint)) in categories.iter().enumerate() {
        let selected = i == app.settings_index;
        let style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let text = format!(
            " {} {}  —  {}",
            if selected { "▶" } else { " " },
            name,
            hint
        );
        lines.push(Line::from(Span::styled(text, style)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Note: changes saved to config.toml immediately.",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).block(popup_block(" Settings "));
    f.render_widget(p, popup);
}

pub fn draw_settings_defaults(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = menu_rect(area);
    f.render_widget(Clear, popup);

    let mut tabs: Vec<Span> = Vec::new();
    let all_tabs = [DefaultsTab::General, DefaultsTab::Advanced];
    for (i, t) in all_tabs.iter().enumerate() {
        let active = *t == app.defaults_tab;
        let style = if active {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        if i > 0 {
            tabs.push(Span::raw("  "));
        }
        tabs.push(Span::styled(format!(" {} ", t.label()), style));
    }

    let mut lines: Vec<Line> = vec![Line::from(tabs), Line::from("")];

    match app.defaults_tab {
        DefaultsTab::General => {
            lines.push(Line::from(Span::styled(
                " New download defaults",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));

            let items = [
                format!(
                    " Default Type:        [{}]",
                    app.config.general.default_mode
                ),
                format!(
                    " Default Quality:     [{}]",
                    app.config.general.default_quality
                ),
                format!(
                    " Default Playlist:    [{}]",
                    on_off(app.config.general.default_playlist)
                ),
                format!(
                    " Default Path:        [{}]",
                    app.config.general.default_path
                ),
            ];

            for (i, item) in items.iter().enumerate() {
                if i == app.defaults_index {
                    lines.push(Line::from(Span::styled(
                        format!(" ▶{}", item),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));
                } else {
                    lines.push(Line::from(format!("  {}", item)));
                }
            }
        }
        DefaultsTab::Advanced => {
            lines.push(Line::from(Span::styled(
                " Yt-dlp options for new downloads",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));

            for i in 0..ADVANCED_OPTION_COUNT {
                let value = app.config.general.advanced.value(i);
                let shown = if value.is_empty() {
                    "—".to_string()
                } else {
                    value
                };
                let text = truncate(
                    &format!("{:<14} [{}]", AdvancedOptions::label(i), shown),
                    56,
                );
                if i == app.defaults_index {
                    lines.push(Line::from(Span::styled(
                        format!(" ▶ {}", text),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )));
                } else {
                    lines.push(Line::from(format!("   {}", text)));
                }
            }
        }
    }

    lines.push(Line::from(""));
    let note = match app.defaults_tab {
        DefaultsTab::General => " Note: settings apply to new downloads.",
        DefaultsTab::Advanced => " Note: empty text = option off; extra args split by spaces.",
    };
    lines.push(Line::from(Span::styled(
        note,
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).block(popup_block(" Defaults "));
    f.render_widget(p, popup);
}

pub fn draw_settings_advanced_input(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = centered_rect(60, 20, area);
    f.render_widget(Clear, popup);

    let byte_idx = app.advanced_input_cursor.min(app.advanced_input.len());
    let (before, after) = app.advanced_input.split_at(byte_idx);
    let display = format!("{}█{}", before, after);
    let title = format!(" {} ", AdvancedOptions::label(app.advanced_edit_index));

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Value: "),
            Span::styled(display, Style::default().fg(Color::Black).bg(Color::Cyan)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(popup_block(&title))
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}

pub fn draw_settings_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = menu_rect(area);
    f.render_widget(Clear, popup);

    let sb = &app.config.status_bar;

    let mut lines = vec![Line::from(Span::styled(
        " Visibility",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];

    let items = [
        format!(" Show status bar:         [{}]", on_off(sb.enabled)),
        format!(" Show hints (F1/F2/Tab):  [{}]", on_off(sb.show_hints)),
        format!(
            " Show queue stats:        [{}]",
            on_off(sb.show_queue_stats)
        ),
        format!(" Show active downloads:   [{}]", on_off(sb.show_active)),
        format!(
            " Show status message:     [{}]",
            on_off(sb.show_status_message)
        ),
    ];

    for (i, item) in items.iter().enumerate() {
        if i == app.status_bar_index {
            lines.push(Line::from(Span::styled(
                format!(" ▶{}", item),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(format!("  {}", item)));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Note: changes saved to config.toml immediately.",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).block(popup_block(" Status Bar "));
    f.render_widget(p, popup);
}

pub fn draw_settings_queue(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = menu_rect(area);
    f.render_widget(Clear, popup);

    let cfg = &app.config.queue_table;

    let mut tabs: Vec<Span> = Vec::new();
    let all_tabs = [QueueTab::Columns, QueueTab::Widths, QueueTab::Style];
    for (i, t) in all_tabs.iter().enumerate() {
        let active = *t == app.queue_tab;
        let style = if active {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        if i > 0 {
            tabs.push(Span::raw("  "));
        }
        tabs.push(Span::styled(format!(" {} ", t.label()), style));
    }

    let mut lines: Vec<Line> = vec![Line::from(tabs), Line::from("")];

    match app.queue_tab {
        QueueTab::Columns => {
            let vis = &cfg.columns;
            let hidden: Vec<&str> = ALL_COLUMNS
                .iter()
                .copied()
                .filter(|c| !vis.iter().any(|x| x == c))
                .collect();
            let vis_len = vis.len();

            lines.push(Line::from(Span::styled(
                " Visible",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            for (i, col) in vis.iter().enumerate() {
                let selected = i == app.queue_table_index;
                let marker = if selected { "▶" } else { " " };
                let text = format!(" {} {} ", marker, col_label(col));
                let style = if selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                lines.push(Line::from(Span::styled(text, style)));
            }

            if !hidden.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    " Hidden",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                for (j, col) in hidden.iter().enumerate() {
                    let i = vis_len + j;
                    let selected = i == app.queue_table_index;
                    let marker = if selected { "▶" } else { " " };
                    let text = format!(" {} {} ", marker, col_label(col));
                    let style = if selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    lines.push(Line::from(Span::styled(text, style)));
                }
            }
        }
        QueueTab::Widths => {
            let hidden = cfg.hidden_columns();
            let vis_len = cfg.columns.len();

            lines.push(Line::from(Span::styled(
                " Visible",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            for (i, col) in cfg.columns.iter().enumerate() {
                let selected = i == app.queue_table_index;
                let marker = if selected { "▶" } else { " " };

                let (label_txt, is_auto) = if cfg.is_auto(col) {
                    (format!("[AUTO w={:>2}]", cfg.weight_of(col)), true)
                } else {
                    (format!("[{:>7}]", cfg.width_of(col)), false)
                };

                let text = format!(" {} {:<10} {} ", marker, col_label(col), label_txt);
                let base = if is_auto {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                let style = if selected {
                    base.bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    base
                };
                lines.push(Line::from(Span::styled(text, style)));
            }

            if !hidden.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    " Hidden",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                for (j, col) in hidden.iter().enumerate() {
                    let i = vis_len + j;
                    let selected = i == app.queue_table_index;
                    let marker = if selected { "▶" } else { " " };

                    let label_txt = if cfg.is_auto(col) {
                        format!("[AUTO w={:>2}]", cfg.weight_of(col))
                    } else {
                        format!("[{:>7}]", cfg.width_of(col))
                    };

                    let text = format!(" {} {:<10} {} ", marker, col_label(col), label_txt);
                    let style = if selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    lines.push(Line::from(Span::styled(text, style)));
                }
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                " AUTO columns share free space by weight (1..20).",
                Style::default().fg(Color::DarkGray),
            )));
        }
        QueueTab::Style => {
            let hidden = cfg.hidden_columns();
            let vis_len = cfg.columns.len();

            lines.push(Line::from(Span::styled(
                " Visible",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            for (i, col) in cfg.columns.iter().enumerate() {
                let selected = i == app.queue_table_index;
                let marker = if selected { "▶" } else { " " };

                let align = cfg.alignment_of(col);
                let icon = match align {
                    Alignment::Left => "◀",
                    Alignment::Center => "◆",
                    Alignment::Right => "▶",
                };

                let text = format!(
                    " {} {:<10} [{}] {} ",
                    marker,
                    col_label(col),
                    align.label(),
                    icon,
                );
                let style = if selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                lines.push(Line::from(Span::styled(text, style)));
            }

            if !hidden.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    " Hidden",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                for (j, col) in hidden.iter().enumerate() {
                    let i = vis_len + j;
                    let selected = i == app.queue_table_index;
                    let marker = if selected { "▶" } else { " " };

                    let align = cfg.alignment_of(col);
                    let icon = match align {
                        Alignment::Left => "◀",
                        Alignment::Center => "◆",
                        Alignment::Right => "▶",
                    };

                    let text = format!(
                        " {} {:<10} [{}] {} ",
                        marker,
                        col_label(col),
                        align.label(),
                        icon,
                    );
                    let style = if selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    lines.push(Line::from(Span::styled(text, style)));
                }
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                " Affects header & cell rendering of the queue table.",
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Note: changes saved to config.toml immediately.",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).block(popup_block(" Queue "));
    f.render_widget(p, popup);
}

pub fn draw_settings_path_picker(f: &mut Frame, app: &mut App, area: Rect) {
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
        let marker = if &app.config.general.default_path == path {
            "●"
        } else {
            " "
        };
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
        format!(" + Set current form path: {} ", app.form.path),
        style,
    )));

    let p = Paragraph::new(lines)
        .block(popup_block(" Select Default Path "))
        .wrap(Wrap { trim: false });
    f.render_widget(p, popup);
}
