use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Wrap},
};

use crate::app::App;

use super::common::{menu_rect, popup_block};

pub fn draw_help(f: &mut Frame, app: &mut App, area: Rect) {
    let popup = menu_rect(area);
    f.render_widget(Clear, popup);

    let mut lines: Vec<Line> = Vec::new();

    let sections: &[(&str, &[&str])] = &[
        (
            " General",
            &[
                " F1            Toggle this help",
                " F2            Toggle settings",
                " F3            Toggle download logs",
                " Esc           Close menu",
                " Ctrl+C        Quit",
                " Tab           Switch focus (Queue / Form)",
            ],
        ),
        (
            " Form navigation",
            &[
                " ↑ / ↓        Prev / next field",
                " ← / →        Prev / next field",
                " Enter        Activate / edit URL / submit",
                " Space        Quick toggle (Type / Playlist / Quality)",
                " Shift+Enter  Cycle quality backward",
                " Ctrl+S       Submit form",
            ],
        ),
        (
            " URL editing",
            &[
                " Type         Enter",
                " ← / →        Move cursor inside URL",
                " Home / End   Jump to start / end",
                " Backspace    Delete previous char",
                " Delete       Delete char at cursor",
                " Esc          Exit editing / unfocus",
            ],
        ),
        (
            " Queue",
            &[
                " ↑ / ↓        Select an item",
                " d / Delete   Remove item (cancels if active)",
                " r            Retry the selected download",
                " p            Pause / resume the queue",
                " s            Running item: skip · pending item: start now",
            ],
        ),
        (
            " Queue editor",
            &[
                " Tab          Switch tab (General / Columns / Widths / Style)",
                " ↑ / ↓        Select column",
                " ← / →        Move col / width in chars",
                " Enter/Space  Hide column (Columns tab)",
                " A/Enter      Toggle AUTO width (Widths tab)",
                " +/−          Width in weight (Widths tab)",
                " Shift+←/→    Width ±5 chars",
            ],
        ),
        (
            " Settings",
            &[
                " ↑ / ↓        Navigate items",
                " Enter        Open category / toggle",
                " Categories:  Defaults · Status Bar · Queue",
                " Esc          Back",
            ],
        ),
        (
            " Defaults — General tab",
            &[
                " Tab          Switch tab (General / Advanced)",
                " ↑ / ↓        Select item",
                " Enter        Toggle type / quality / playlist / path",
            ],
        ),
        (
            " Defaults — Advanced tab",
            &[
                " Tab          Switch tab (General / Advanced)",
                " ↑ / ↓        Select option",
                " Enter/Space  Toggle geo-bypass / edit text option",
                " Options:     Proxy · Geo bypass · Cookies · User agent · Rate limit · Extra args",
                " Text fields: Enter — save · Esc — cancel",
            ],
        ),
        (
            " Queue settings — General tab",
            &[
                " ↑ / ↓        Select setting",
                " ← / →        Decrease / increase value",
                " Scroll offset — marquee chars per tick (1..20)",
                " Side padding  — spaces on each side (0..10)",
            ],
        ),
        (
            " Queue settings — Columns tab",
            &[
                " Tab          Switch tab (General / Columns / Widths / Style)",
                " ↑ / ↓        Select column",
                " ← / →        Move column left / right",
                " Enter/Space  Hide column",
            ],
        ),
        (
            " Queue settings — Widths tab",
            &[
                " ↑ / ↓        Select column",
                " ← / →        Width in chars (switches to fixed)",
                " Shift+←/→    Width ±5 chars",
                " + / =        Increase weight (switches to AUTO)",
                " - / _        Decrease weight (switches to AUTO)",
                " Shift +/−    Weight ±5",
                " A / Enter    Toggle AUTO",
            ],
        ),
        (
            " Queue settings — Style tab",
            &[
                " ↑ / ↓        Select column",
                " ←            Align left",
                " Enter/Space  Align center",
                " →            Align right",
            ],
        ),
        (
            " Path picker",
            &[
                " Enter        Select path",
                " P            Pin / unpin selected path",
                " Pinned paths / Paths / Actions",
                " Pin path     Open pin manager",
                " + Add current path to favorites",
                " + Add new path…",
                " − Select and delete path…",
            ],
        ),
        (
            " Pin manager",
            &[
                " ↑ / ↓        Select path",
                " Enter        Move path between categories",
                " P            Pin / unpin selected path",
                " Esc          Back to path picker",
            ],
        ),
        (
            " Logs",
            &[
                " F3           Toggle download logs",
                " ↑ / ↓        Scroll",
                " PageUp/PageDown  Scroll by 10 lines",
                " Home / End   First line / follow latest",
                " Esc, q       Close",
            ],
        ),
    ];

    for (header, items) in sections {
        lines.push(Line::from(Span::styled(
            *header,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for item in *items {
            lines.push(Line::from(format!("  {}", item)));
        }
        lines.push(Line::from(""));
    }

    let p = Paragraph::new(lines)
        .block(popup_block(" Help "))
        .wrap(Wrap { trim: false })
        .scroll((app.help_scroll, 0));
    f.render_widget(p, popup);
}
