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
            " Queue editor",
            &[
                " Tab          Switch tab (Columns / Widths)",
                " ↑ / ↓        Select column",
                " ← / →        Move col / adjust width",
                " Enter/Space  Hide column (Columns tab)",
                " A/Enter      Toggle AUTO width (Widths tab)",
                " Shift+←/→    Width ±5",
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
            " Queue settings — Columns tab",
            &[
                " Tab          Switch tab (Columns / Widths / Style)",
                " ↑ / ↓        Select column",
                " ← / →        Move column left / right",
                " Enter/Space  Hide column",
            ],
        ),
        (
            " Queue settings — Widths tab",
            &[
                " ↑ / ↓        Select column",
                " ← / →        Adjust fixed width (or drop AUTO)",
                " + / =        Increase weight  (AUTO, +1)",
                " - / _        Decrease weight  (AUTO, −1)",
                " Shift +/−    Weight ±5        (AUTO)",
                " A / Enter    Toggle AUTO",
            ],
        ),
        (
            " Queue settings — Style tab",
            &[
                " ↑ / ↓        Select column",
                " ← / →        Cycle alignment backward / forward",
                " Enter/Space  Cycle alignment forward",
                " Values:      Left · Center · Right",
            ],
        ),
        (
            " Path picker",
            &[
                " Enter        Select path",
                " + Add current path to favorites",
                " + Add new path…",
                " - Select and delete path…",
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
