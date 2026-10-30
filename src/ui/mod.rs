pub mod common;
pub mod form;
pub mod help;
pub mod path_picker;
pub mod queue_table;
pub mod settings;
pub mod status_bar;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, Popup};

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let sb_enabled = app.config.status_bar.enabled;

    let constraints = if sb_enabled {
        vec![
            Constraint::Min(5),
            Constraint::Length(9),
            Constraint::Length(1),
        ]
    } else {
        vec![Constraint::Min(5), Constraint::Length(9)]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(size);

    queue_table::draw_queue(f, app, chunks[0]);
    form::draw_form(f, app, chunks[1]);
    if sb_enabled {
        status_bar::draw_status(f, app, chunks[2]);
    }

    if app.show_help {
        help::draw_help(f, app, size);
    }

    match app.popup {
        Popup::PathPicker => path_picker::draw_path_picker(f, app, size),
        Popup::PathInput => path_picker::draw_path_input(f, app, size),
        Popup::PathDelete => path_picker::draw_path_delete(f, app, size),
        Popup::Settings => settings::draw_settings(f, app, size),
        Popup::SettingsDefaults => settings::draw_settings_defaults(f, app, size),
        Popup::SettingsAdvancedInput => settings::draw_settings_advanced_input(f, app, size),
        Popup::SettingsStatusBar => settings::draw_settings_status_bar(f, app, size),
        Popup::SettingsQueue => settings::draw_settings_queue(f, app, size),
        Popup::SettingsPathPicker => settings::draw_settings_path_picker(f, app, size),
        Popup::None => {}
    }
}
