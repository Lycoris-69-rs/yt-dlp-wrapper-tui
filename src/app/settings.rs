use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::App;
use crate::queue::Quality;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Popup {
    None,
    PathPicker,
    PathInput,
    PathDelete,
    Settings,
    SettingsDefaults,
    SettingsStatusBar,
    SettingsQueue,
    SettingsPathPicker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueTab {
    Columns,
    Widths,
    Style,
}

impl QueueTab {
    const ALL: [QueueTab; 3] = [QueueTab::Columns, QueueTab::Widths, QueueTab::Style];
    pub(crate) fn next(self) -> Self {
        let i = Self::ALL.iter().position(|&t| t == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    pub(crate) fn prev(self) -> Self {
        let i = Self::ALL.iter().position(|&t| t == self).unwrap_or(0);
        Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()]
    }
    pub fn label(&self) -> &'static str {
        match self {
            QueueTab::Columns => "Columns",
            QueueTab::Widths => "Widths",
            QueueTab::Style => "Style",
        }
    }
}

pub const SETTINGS_CATEGORIES: [&str; 3] = ["Defaults", "Status Bar", "Queue"];

impl App {
    pub(crate) async fn handle_settings_key(&mut self, key: KeyEvent) -> Result<()> {
        const LAST_IDX: usize = SETTINGS_CATEGORIES.len() - 1;
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::None,
            KeyCode::Up => {
                if self.settings_index > 0 {
                    self.settings_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.settings_index < LAST_IDX {
                    self.settings_index += 1;
                }
            }
            KeyCode::Enter => match self.settings_index {
                0 => {
                    self.popup = Popup::SettingsDefaults;
                    self.defaults_index = 0;
                }
                1 => {
                    self.popup = Popup::SettingsStatusBar;
                    self.status_bar_index = 0;
                }
                2 => {
                    self.popup = Popup::SettingsQueue;
                    self.queue_tab = QueueTab::Columns;
                    self.queue_table_index = 0;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_settings_defaults_key(&mut self, key: KeyEvent) -> Result<()> {
        const LAST_IDX: usize = 3;
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::Settings,
            KeyCode::Up => {
                if self.defaults_index > 0 {
                    self.defaults_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.defaults_index < LAST_IDX {
                    self.defaults_index += 1;
                }
            }
            KeyCode::Enter => match self.defaults_index {
                0 => {
                    self.config.general.default_mode =
                        if self.config.general.default_mode == "Video" {
                            "Audio".into()
                        } else {
                            "Video".into()
                        };
                    let _ = self.config.save();
                }
                1 => {
                    let q = Quality::from_label(&self.config.general.default_quality);
                    self.config.general.default_quality = q.next().label().to_string();
                    let _ = self.config.save();
                }
                2 => {
                    self.config.general.default_playlist = !self.config.general.default_playlist;
                    let _ = self.config.save();
                }
                3 => {
                    self.popup = Popup::SettingsPathPicker;
                    self.path_picker_index = self
                        .config
                        .paths
                        .favorites
                        .iter()
                        .position(|p| p == &self.config.general.default_path)
                        .unwrap_or(0);
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_settings_status_bar_key(&mut self, key: KeyEvent) -> Result<()> {
        const LAST_IDX: usize = 4;
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::Settings,
            KeyCode::Up => {
                if self.status_bar_index > 0 {
                    self.status_bar_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.status_bar_index < LAST_IDX {
                    self.status_bar_index += 1;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let sb = &mut self.config.status_bar;
                match self.status_bar_index {
                    0 => sb.enabled = !sb.enabled,
                    1 => sb.show_hints = !sb.show_hints,
                    2 => sb.show_queue_stats = !sb.show_queue_stats,
                    3 => sb.show_active = !sb.show_active,
                    4 => sb.show_status_message = !sb.show_status_message,
                    _ => {}
                }
                let _ = self.config.save();
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_settings_queue_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.code == KeyCode::Tab {
            self.queue_tab = self.queue_tab.next();
            self.queue_table_index = 0;
            return Ok(());
        }
        if key.code == KeyCode::BackTab {
            self.queue_tab = self.queue_tab.prev();
            self.queue_table_index = 0;
            return Ok(());
        }

        match self.queue_tab {
            QueueTab::Columns => {
                use crate::config::ALL_COLUMNS;
                let vis = self.config.queue_table.columns.clone();
                let hidden_cols: Vec<&'static str> = ALL_COLUMNS
                    .iter()
                    .copied()
                    .filter(|c| !vis.iter().any(|x| x == c))
                    .collect();
                let vis_len = vis.len();
                let total = vis_len + hidden_cols.len();

                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::Settings,
                    KeyCode::Up => {
                        if self.queue_table_index > 0 {
                            self.queue_table_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.queue_table_index + 1 < total {
                            self.queue_table_index += 1;
                        }
                    }
                    KeyCode::Left => {
                        if self.queue_table_index < vis_len && self.queue_table_index > 0 {
                            self.config
                                .queue_table
                                .move_column_left(self.queue_table_index);
                            self.queue_table_index -= 1;
                            let _ = self.config.save();
                        }
                    }
                    KeyCode::Right => {
                        if self.queue_table_index + 1 < vis_len {
                            self.config
                                .queue_table
                                .move_column_right(self.queue_table_index);
                            self.queue_table_index += 1;
                            let _ = self.config.save();
                        }
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if self.queue_table_index < vis_len {
                            if self.config.queue_table.columns.len() > 1 {
                                self.config
                                    .queue_table
                                    .columns
                                    .remove(self.queue_table_index);
                                if total > 0 && self.queue_table_index >= total {
                                    self.queue_table_index = total - 1;
                                }
                                let _ = self.config.save();
                            }
                        } else {
                            let h_idx = self.queue_table_index - vis_len;
                            if let Some(col) = hidden_cols.get(h_idx) {
                                self.config.queue_table.add_column(col);
                                if total > 0 && self.queue_table_index >= total {
                                    self.queue_table_index = total - 1;
                                }
                                let _ = self.config.save();
                            }
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
            QueueTab::Widths => {
                let cols_len = self.config.queue_table.columns.len();
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::Settings,
                    KeyCode::Up => {
                        if self.queue_table_index > 0 {
                            self.queue_table_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.queue_table_index + 1 < cols_len {
                            self.queue_table_index += 1;
                        }
                    }
                    KeyCode::Left => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            if self.config.queue_table.is_auto(&col) {
                                self.config.queue_table.set_auto(&col, false);
                            } else {
                                let w = self.config.queue_table.width_of(&col);
                                let step = if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    5
                                } else {
                                    1
                                };
                                self.config
                                    .queue_table
                                    .set_width(&col, w.saturating_sub(step).max(1));
                            }
                            let _ = self.config.save();
                        }
                    }
                    KeyCode::Right => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            if self.config.queue_table.is_auto(&col) {
                                self.config.queue_table.set_auto(&col, false);
                            } else {
                                let w = self.config.queue_table.width_of(&col);
                                let step = if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    5
                                } else {
                                    1
                                };
                                self.config.queue_table.set_width(&col, (w + step).min(120));
                            }
                            let _ = self.config.save();
                        }
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            if self.config.queue_table.is_auto(&col) {
                                let step: i16 = if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    5
                                } else {
                                    1
                                };
                                let w = self.config.queue_table.bump_weight(&col, step);
                                let _ = self.config.save();
                                self.status = format!("Weight {}: {}", col, w);
                            } else {
                                self.status =
                                    "Weight only editable for AUTO columns (press A)".into();
                            }
                        }
                    }
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            if self.config.queue_table.is_auto(&col) {
                                let step: i16 = if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    -5
                                } else {
                                    -1
                                };
                                let w = self.config.queue_table.bump_weight(&col, step);
                                let _ = self.config.save();
                                self.status = format!("Weight {}: {}", col, w);
                            } else {
                                self.status =
                                    "Weight only editable for AUTO columns (press A)".into();
                            }
                        }
                    }
                    KeyCode::Char('a')
                    | KeyCode::Char('A')
                    | KeyCode::Enter
                    | KeyCode::Char(' ') => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            self.config.queue_table.toggle_auto(&col);
                            let _ = self.config.save();
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
            QueueTab::Style => {
                let cols_len = self.config.queue_table.columns.len();
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::Settings,
                    KeyCode::Up => {
                        if self.queue_table_index > 0 {
                            self.queue_table_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.queue_table_index + 1 < cols_len {
                            self.queue_table_index += 1;
                        }
                    }
                    KeyCode::Right | KeyCode::Char(' ') | KeyCode::Enter => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            let a = self.config.queue_table.cycle_alignment_next(&col);
                            let _ = self.config.save();
                            self.status = format!("{} alignment: {}", col, a.label());
                        }
                    }
                    KeyCode::Left => {
                        if self.queue_table_index < cols_len {
                            let col =
                                self.config.queue_table.columns[self.queue_table_index].clone();
                            let a = self.config.queue_table.cycle_alignment_prev(&col);
                            let _ = self.config.save();
                            self.status = format!("{} alignment: {}", col, a.label());
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    }

    pub(crate) async fn handle_settings_path_picker_key(&mut self, key: KeyEvent) -> Result<()> {
        let favs_len = self.config.paths.favorites.len();
        let total = favs_len + 1;

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::SettingsDefaults,
            KeyCode::Up => {
                if self.path_picker_index > 0 {
                    self.path_picker_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.path_picker_index + 1 < total {
                    self.path_picker_index += 1;
                }
            }
            KeyCode::Enter => {
                if self.path_picker_index < favs_len {
                    self.config.general.default_path =
                        self.config.paths.favorites[self.path_picker_index].clone();
                    let _ = self.config.save();
                    self.popup = Popup::SettingsDefaults;
                    self.status = format!("✅ Default path: {}", self.config.general.default_path);
                } else if self.path_picker_index == favs_len {
                    let current = self.form.path.clone();
                    self.config.add_favorite(&current);
                    let _ = self.config.save();
                    self.config.general.default_path = current;
                    self.popup = Popup::SettingsDefaults;
                    self.status = "✅ Default path set & added to favorites".into();
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_path_picker_key(&mut self, key: KeyEvent) -> Result<()> {
        let favs_len = self.config.paths.favorites.len();
        let total = favs_len + 3;
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.popup = Popup::None,
            KeyCode::Up => {
                if self.path_picker_index > 0 {
                    self.path_picker_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.path_picker_index + 1 < total {
                    self.path_picker_index += 1;
                }
            }
            KeyCode::Enter => {
                if self.path_picker_index < favs_len {
                    self.form.path = self.config.paths.favorites[self.path_picker_index].clone();
                    self.popup = Popup::None;
                    self.status = format!("✅ Path: {}", self.form.path);
                } else if self.path_picker_index == favs_len {
                    let current = self.form.path.clone();
                    self.config.add_favorite(&current);
                    let _ = self.config.save();
                    self.status = format!("✅ Added to favorites: {}", current);
                    self.path_picker_index = self
                        .config
                        .paths
                        .favorites
                        .iter()
                        .position(|p| p == &current)
                        .unwrap_or(0);
                } else if self.path_picker_index == favs_len + 1 {
                    self.popup = Popup::PathInput;
                    self.path_input.clear();
                    self.path_input_cursor = 0;
                } else if self.config.paths.favorites.is_empty() {
                    self.status = "❌ No favorites to delete".into();
                } else {
                    self.popup = Popup::PathDelete;
                    self.path_picker_index = 0;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_path_input_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.popup = Popup::None,
            KeyCode::Enter => {
                let path = self.path_input.trim().to_string();
                if path.is_empty() {
                    self.status = "❌ Path is empty".into();
                    return Ok(());
                }
                self.config.add_favorite(&path);
                let _ = self.config.save();
                self.form.path = path.clone();
                self.popup = Popup::None;
                self.status = format!("✅ Path: {}", path);
            }
            KeyCode::Char(c) => {
                let byte_idx = self.path_input_cursor.min(self.path_input.len());
                self.path_input.insert(byte_idx, c);
                self.path_input_cursor += c.len_utf8();
            }
            KeyCode::Backspace => {
                if self.path_input_cursor > 0 {
                    let mut new_cursor = self.path_input_cursor - 1;
                    while !self.path_input.is_char_boundary(new_cursor) {
                        new_cursor -= 1;
                    }
                    self.path_input
                        .replace_range(new_cursor..self.path_input_cursor, "");
                    self.path_input_cursor = new_cursor;
                }
            }
            KeyCode::Left => {
                if self.path_input_cursor > 0 {
                    let mut c = self.path_input_cursor - 1;
                    while !self.path_input.is_char_boundary(c) {
                        c -= 1;
                    }
                    self.path_input_cursor = c;
                }
            }
            KeyCode::Right => {
                if self.path_input_cursor < self.path_input.len() {
                    let mut c = self.path_input_cursor + 1;
                    while c < self.path_input.len() && !self.path_input.is_char_boundary(c) {
                        c += 1;
                    }
                    self.path_input_cursor = c;
                }
            }
            KeyCode::Home => self.path_input_cursor = 0,
            KeyCode::End => self.path_input_cursor = self.path_input.len(),
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_path_delete_key(&mut self, key: KeyEvent) -> Result<()> {
        let favs_len = self.config.paths.favorites.len();
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.popup = Popup::PathPicker;
                self.path_picker_index = self.config.paths.favorites.len() + 2;
            }
            KeyCode::Up => {
                if self.path_picker_index > 0 {
                    self.path_picker_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.path_picker_index + 1 < favs_len {
                    self.path_picker_index += 1;
                }
            }
            KeyCode::Enter => {
                if self.path_picker_index < self.config.paths.favorites.len() {
                    let removed = self.config.paths.favorites[self.path_picker_index].clone();
                    self.config.remove_favorite(&removed);
                    let _ = self.config.save();
                    self.status = format!("🗑 Deleted: {}", removed);
                    if self.config.paths.favorites.is_empty() {
                        self.popup = Popup::PathPicker;
                        self.path_picker_index = 0;
                    } else if self.path_picker_index >= self.config.paths.favorites.len() {
                        self.path_picker_index = self.config.paths.favorites.len() - 1;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
