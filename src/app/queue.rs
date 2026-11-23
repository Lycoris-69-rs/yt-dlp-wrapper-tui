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
                    let id = self.queue[self.selected].id;
                    let is_downloading = matches!(
                        self.queue[self.selected].state,
                        ItemState::Downloading { .. }
                    );
                    if is_downloading && self.active_id == Some(id) {
                        if let Some(token) = self.active_token.take() {
                            token.cancel();
                        }
                        self.active_id = None;
                        self.active_gen = None;
                    }
                    self.queue.remove(self.selected);
                    if self.selected >= self.queue.len() && !self.queue.is_empty() {
                        self.selected = self.queue.len() - 1;
                    }
                    if is_downloading {
                        self.status = "⏹ Download cancelled".into();
                    }
                    self.start_next();
                }
            }
            KeyCode::Char('r') => {
                if self.selected < self.queue.len() {
                    let sel = self.selected;
                    let sel_downloading =
                        matches!(self.queue[sel].state, ItemState::Downloading { .. });
                    let active_other = self.queue.iter().enumerate().any(|(i, item)| {
                        i != sel && matches!(item.state, ItemState::Downloading { .. })
                    });
                    if active_other {
                        self.status = "⏳ Another download is running".into();
                    } else {
                        if sel_downloading {
                            self.cancel_active();
                        }
                        self.queue[sel].state = ItemState::Pending;
                        self.queue_running = true;
                        self.start_item(sel);
                        self.status = "↻ Retry".into();
                    }
                }
            }
            KeyCode::Char('p') => {
                if self.queue_running {
                    self.cancel_active();
                    self.queue_running = false;
                    self.status = "⏸ Paused".into();
                } else {
                    self.queue_running = true;
                    self.status = "▶ Resumed".into();
                    self.start_next();
                }
            }
            KeyCode::Char('s') => {
                if self.selected < self.queue.len() {
                    let sel = self.selected;
                    match self.queue[sel].state {
                        ItemState::Downloading { .. } => {
                            // Skip: pause the current item and start the next pending one.
                            self.cancel_active();
                            let next = self
                                .queue
                                .iter()
                                .enumerate()
                                .find(|(i, item)| {
                                    *i != sel && matches!(item.state, ItemState::Pending)
                                })
                                .map(|(i, _)| i);
                            if let Some(idx) = next {
                                self.queue_running = true;
                                self.start_item(idx);
                                self.status = "⏭ Skipped, next started".into();
                            } else {
                                self.status = "⏭ Skipped (no pending items)".into();
                            }
                        }
                        ItemState::Pending => {
                            // Start the selected pending item, stopping the
                            // previously running download.
                            self.cancel_active();
                            self.queue_running = true;
                            self.start_item(sel);
                            self.status = "⏩ Started selected item".into();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
