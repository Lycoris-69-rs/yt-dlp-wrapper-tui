use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum DownloadEvent {
    Started(usize),
    Progress {
        id: usize,
        percent: f32,
        speed: String,
    },
    Log {
        id: usize,
        line: String,
    },
    Finished {
        id: usize,
        success: bool,
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum MetaEvent {
    Title { id: usize, title: String },
    TitleFailed { id: usize },
}

#[derive(Debug)]
pub enum AppEvent {
    Key(KeyEvent),
    Download(DownloadEvent),
    Meta(MetaEvent),
    Tick,
}

pub struct EventHandler {
    _handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tx: mpsc::UnboundedSender<AppEvent>) -> Self {
        let handle = thread::spawn(move || loop {
            if tx.is_closed() {
                break;
            }
            match event::poll(Duration::from_millis(100)) {
                Ok(true) => match event::read() {
                    Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                        if tx.send(AppEvent::Key(key)).is_err() {
                            break;
                        }
                    }
                    Ok(Event::Resize(_, _)) => {
                        let _ = tx.send(AppEvent::Tick);
                    }
                    _ => {}
                },
                Ok(false) => {}
                Err(_) => break,
            }
        });
        Self { _handle: handle }
    }
}
