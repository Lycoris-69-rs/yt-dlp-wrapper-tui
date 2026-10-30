use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{App, Focus, Popup};

impl App {
    pub(crate) async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return Ok(());
        }

        if key.code == KeyCode::F(1) {
            if self.show_help {
                self.show_help = false;
            } else {
                self.open_help();
            }
            return Ok(());
        }
        if key.code == KeyCode::F(2) {
            if matches!(self.popup, Popup::Settings) {
                self.popup = Popup::None;
            } else {
                self.open_settings();
            }
            return Ok(());
        }

        if self.show_help {
            self.handle_help_key(key);
            return Ok(());
        }

        match self.popup {
            Popup::PathPicker => return self.handle_path_picker_key(key).await,
            Popup::PathInput => return self.handle_path_input_key(key).await,
            Popup::PathDelete => return self.handle_path_delete_key(key).await,
            Popup::Settings => return self.handle_settings_key(key).await,
            Popup::SettingsDefaults => return self.handle_settings_defaults_key(key).await,
            Popup::SettingsStatusBar => return self.handle_settings_status_bar_key(key).await,
            Popup::SettingsQueue => return self.handle_settings_queue_key(key).await,
            Popup::SettingsPathPicker => return self.handle_settings_path_picker_key(key).await,
            Popup::None => {}
        }

        if key.code == KeyCode::Tab {
            self.focus = match self.focus {
                Focus::Queue => Focus::Form,
                Focus::Form => Focus::Queue,
            };
            self.sync_url_editing();
            return Ok(());
        }

        if self.editing_url {
            return self.handle_url_editing(key).await;
        }

        if self.focus == Focus::Queue && key.code == KeyCode::Char('q') {
            self.should_quit = true;
            return Ok(());
        }

        match self.focus {
            Focus::Queue => self.handle_queue_key(key).await?,
            Focus::Form => self.handle_form_key(key).await?,
        }
        Ok(())
    }
}
