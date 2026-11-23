use crossterm::event::{KeyCode, KeyEvent};

use super::App;

impl App {
    pub(crate) fn handle_logs_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.show_logs = false,
            KeyCode::Up => {
                self.logs_scroll = self.logs_scroll.saturating_sub(1);
                self.logs_follow = false;
            }
            KeyCode::Down => {
                self.logs_scroll = self.logs_scroll.saturating_add(1);
                self.logs_follow = false;
            }
            KeyCode::PageUp => {
                self.logs_scroll = self.logs_scroll.saturating_sub(10);
                self.logs_follow = false;
            }
            KeyCode::PageDown => {
                self.logs_scroll = self.logs_scroll.saturating_add(10);
                self.logs_follow = false;
            }
            KeyCode::Home => {
                self.logs_scroll = 0;
                self.logs_follow = false;
            }
            KeyCode::End => self.logs_follow = true,
            _ => {}
        }
    }
}
