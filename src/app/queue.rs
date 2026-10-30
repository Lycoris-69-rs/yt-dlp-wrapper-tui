use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use super::App;
use crate::queue::ItemState;

impl App {
    pub(crate) async fn handle_queue_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if !self.queue.is_empty() && self.selected + 1 < self.queue.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Char('d') | KeyCode::Delete => {
                if !self.queue.is_empty() && self.selected < self.queue.len() {
                    let is_downloading = matches!(
                        self.queue[self.selected].state,
                        ItemState::Downloading { .. }
                    );
                    self.queue.remove(self.selected);
                    if self.selected >= self.queue.len() && !self.queue.is_empty() {
                        self.selected = self.queue.len() - 1;
                    }
                    if is_downloading {
                        if let Some(token) = self.active_token.take() {
                            token.cancel();
                        }
                        self.status = "⏹ Download cancelled".into();
                    }
                    self.start_next();
                }
            }
            KeyCode::Char('r') => {
                if let Some(item) = self.queue.get_mut(self.selected) {
                    if matches!(item.state, ItemState::Failed(_)) {
                        item.state = ItemState::Pending;
                        self.status = "Item marked for retry".into();
                    }
                }
                self.start_next();
            }
            KeyCode::Char('s') => {
                self.queue_running = !self.queue_running;
                if self.queue_running {
                    self.status = "▶ Queue resumed".into();
                    self.start_next();
                } else {
                    self.status = "⏸ Queue paused".into();
                }
            }
            _ => {}
        }
        Ok(())
    }
}
