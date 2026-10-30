mod app;
mod config;
mod downloader;
mod event;
mod queue;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::app::App;
use crate::event::{AppEvent, EventHandler};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<AppEvent>();

    let _event_handler = EventHandler::new(event_tx.clone());

    let mut app = App::new(event_tx);

    let mut tick = tokio::time::interval(Duration::from_millis(250));

    let res: Result<()> = loop {
        if let Err(e) = terminal.draw(|f| ui::draw(f, &mut app)) {
            break Err(e.into());
        }

        tokio::select! {
            ev = event_rx.recv() => {
                if let Some(ev) = ev {
                    if let Err(e) = app.handle_event(ev).await {
                        break Err(e);
                    }
                }
            }
            _ = tick.tick() => {
                app.on_tick().await;
            }
        }

        if app.should_quit {
            break Ok(());
        }
    };

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}
