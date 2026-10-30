use crossterm::event::{KeyCode, KeyEvent};

use super::App;

impl App {
    pub(crate) fn handle_help_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.show_help = false,
            KeyCode::Up => self.help_scroll = self.help_scroll.saturating_sub(1),
            KeyCode::Down => self.help_scroll += 1,
            KeyCode::PageUp => self.help_scroll = self.help_scroll.saturating_sub(10),
            KeyCode::PageDown => self.help_scroll += 10,
            KeyCode::Home => self.help_scroll = 0,
            _ => {}
        }
    }
}
