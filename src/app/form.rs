use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

use super::{App, Focus, Popup};
use crate::config::Config;
use crate::queue::{DownloadMode, Quality};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormField {
    Url,
    Mode,
    Playlist,
    Quality,
    Path,
    Submit,
}

impl FormField {
    const ALL: [FormField; 6] = [
        FormField::Url,
        FormField::Mode,
        FormField::Playlist,
        FormField::Quality,
        FormField::Path,
        FormField::Submit,
    ];
    pub(crate) fn next(self) -> Self {
        let i = Self::ALL.iter().position(|&f| f == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    pub(crate) fn prev(self) -> Self {
        let i = Self::ALL.iter().position(|&f| f == self).unwrap_or(0);
        Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug)]
pub struct Form {
    pub url: String,
    pub url_cursor: usize,
    pub mode: DownloadMode,
    pub quality: Quality,
    pub path: String,
    pub is_playlist: bool,
    pub field: FormField,
}

impl Form {
    pub fn new(config: &Config) -> Self {
        Self {
            url: String::new(),
            url_cursor: 0,
            mode: if config.general.default_mode == "Audio" {
                DownloadMode::Audio
            } else {
                DownloadMode::Video
            },
            quality: Quality::from_label(&config.general.default_quality),
            path: config.general.default_path.clone(),
            is_playlist: config.general.default_playlist,
            field: FormField::Url,
        }
    }

    pub fn reset_after_submit(&mut self, _config: &Config) {
        self.url.clear();
        self.url_cursor = 0;
        self.field = FormField::Url;
    }

    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.url
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.url.len())
    }

    pub(crate) fn url_insert(&mut self, c: char) {
        let byte_idx = self.char_to_byte(self.url_cursor);
        self.url.insert(byte_idx, c);
        self.url_cursor += 1;
    }

    pub(crate) fn url_backspace(&mut self) {
        if self.url_cursor == 0 {
            return;
        }
        let start = self.char_to_byte(self.url_cursor - 1);
        let end = self.char_to_byte(self.url_cursor);
        self.url.replace_range(start..end, "");
        self.url_cursor -= 1;
    }

    pub(crate) fn url_delete(&mut self) {
        if self.url_cursor >= self.url.chars().count() {
            return;
        }
        let start = self.char_to_byte(self.url_cursor);
        let end = self.char_to_byte(self.url_cursor + 1);
        self.url.replace_range(start..end, "");
    }
}

impl App {
    pub(crate) async fn handle_form_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Left => {
                self.form.field = self.form.field.prev();
                self.sync_url_editing();
            }
            KeyCode::Down | KeyCode::Right => {
                self.form.field = self.form.field.next();
                self.sync_url_editing();
            }
            KeyCode::Enter => match self.form.field {
                FormField::Url => {
                    self.form.field = self.form.field.next();
                    self.sync_url_editing();
                }
                FormField::Mode => {
                    self.form.mode = match self.form.mode {
                        DownloadMode::Video => DownloadMode::Audio,
                        DownloadMode::Audio => DownloadMode::Video,
                    };
                    self.status = format!("Type: {}", self.form.mode.label());
                }
                FormField::Playlist => {
                    self.form.is_playlist = !self.form.is_playlist;
                    self.status = if self.form.is_playlist {
                        "Playlist: download entire playlist".into()
                    } else {
                        "Playlist: download only this".into()
                    };
                }
                FormField::Quality => {
                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                        self.form.quality = self.form.quality.prev();
                    } else {
                        self.form.quality = self.form.quality.next();
                    }
                    self.status = format!("Quality: {}", self.form.quality.label());
                }
                FormField::Path => {
                    self.popup = Popup::PathPicker;
                    self.path_picker_index = self
                        .config
                        .paths
                        .favorites
                        .iter()
                        .position(|p| p == &self.form.path)
                        .unwrap_or(0);
                }
                FormField::Submit => self.submit_form(),
            },
            KeyCode::Char(' ') => match self.form.field {
                FormField::Mode => {
                    self.form.mode = match self.form.mode {
                        DownloadMode::Video => DownloadMode::Audio,
                        DownloadMode::Audio => DownloadMode::Video,
                    };
                }
                FormField::Playlist => self.form.is_playlist = !self.form.is_playlist,
                FormField::Quality => self.form.quality = self.form.quality.next(),
                _ => {}
            },
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.submit_form()
            }
            KeyCode::Esc => {
                self.editing_url = false;
                self.focus = Focus::Queue;
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn handle_url_editing(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up => {
                self.form.field = self.form.field.prev();
                self.sync_url_editing();
            }
            KeyCode::Down => {
                self.form.field = self.form.field.next();
                self.sync_url_editing();
            }
            KeyCode::Enter => {
                self.form.field = self.form.field.next();
                self.sync_url_editing();
            }
            KeyCode::Esc => {
                self.editing_url = false;
                self.focus = Focus::Queue;
            }
            KeyCode::Char(c) => self.form.url_insert(c),
            KeyCode::Backspace => self.form.url_backspace(),
            KeyCode::Delete => self.form.url_delete(),
            KeyCode::Left => {
                if self.form.url_cursor > 0 {
                    self.form.url_cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.form.url_cursor < self.form.url.chars().count() {
                    self.form.url_cursor += 1;
                }
            }
            KeyCode::Home => self.form.url_cursor = 0,
            KeyCode::End => self.form.url_cursor = self.form.url.chars().count(),
            _ => {}
        }
        Ok(())
    }
}
