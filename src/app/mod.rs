pub mod form;
pub mod help;
pub mod keys;
pub mod queue;
pub mod settings;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::config::Config;
use crate::event::{AppEvent, DownloadEvent, MetaEvent};
use crate::queue::{ItemState, QueueItem};

pub use form::{Form, FormField};
pub use settings::{DefaultsTab, Popup, QueueTab};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Queue,
    Form,
}

pub struct App {
    pub queue: Vec<QueueItem>,
    pub selected: usize,
    pub focus: Focus,
    pub form: Form,
    pub config: Config,
    pub status: String,
    pub should_quit: bool,
    pub queue_running: bool,
    pub next_id: usize,
    pub event_tx: mpsc::UnboundedSender<AppEvent>,
    pub logs: Vec<(usize, String)>,

    pub show_help: bool,
    pub help_scroll: u16,

    pub settings_index: usize,
    pub popup: Popup,
    pub path_picker_index: usize,
    pub path_input: String,
    pub path_input_cursor: usize,
    pub status_bar_index: usize,
    pub defaults_index: usize,
    pub defaults_tab: DefaultsTab,
    pub advanced_input: String,
    pub advanced_input_cursor: usize,
    pub advanced_edit_index: usize,
    pub queue_tab: QueueTab,
    pub queue_table_index: usize,

    pub editing_url: bool,
    pub active_token: Option<CancellationToken>,
}

impl App {
    pub fn new(event_tx: mpsc::UnboundedSender<AppEvent>) -> Self {
        let config = Config::load();
        let form = Form::new(&config);
        let mut app = Self {
            queue: Vec::new(),
            selected: 0,
            focus: Focus::Form,
            form,
            config,
            status: "Ready".into(),
            should_quit: false,
            queue_running: false,
            next_id: 1,
            event_tx,
            logs: Vec::new(),
            show_help: false,
            help_scroll: 0,
            settings_index: 0,
            popup: Popup::None,
            path_picker_index: 0,
            path_input: String::new(),
            path_input_cursor: 0,
            status_bar_index: 0,
            defaults_index: 0,
            defaults_tab: DefaultsTab::General,
            advanced_input: String::new(),
            advanced_input_cursor: 0,
            advanced_edit_index: 0,
            queue_tab: QueueTab::Columns,
            queue_table_index: 0,
            editing_url: false,
            active_token: None,
        };
        app.sync_url_editing();
        app
    }

    pub async fn handle_event(&mut self, ev: AppEvent) -> anyhow::Result<()> {
        match ev {
            AppEvent::Key(key) => self.handle_key(key).await?,
            AppEvent::Download(dl) => self.handle_download_event(dl).await,
            AppEvent::Meta(meta) => self.handle_meta_event(meta),
            AppEvent::Tick => {}
        }
        Ok(())
    }

    pub async fn on_tick(&mut self) {}

    pub(crate) fn sync_url_editing(&mut self) {
        self.editing_url = self.focus == Focus::Form && self.form.field == FormField::Url;
        if self.editing_url {
            self.form.url_cursor = self.form.url.chars().count();
        }
    }

    pub(crate) fn open_help(&mut self) {
        self.show_help = true;
        self.help_scroll = 0;
        self.popup = Popup::None;
    }

    pub(crate) fn open_settings(&mut self) {
        self.popup = Popup::Settings;
        self.settings_index = 0;
        self.show_help = false;
    }

    pub(crate) fn submit_form(&mut self) {
        if self.form.url.trim().is_empty() {
            self.status = "❌ URL is empty".into();
            return;
        }
        let id = self.next_id;
        let url = self.form.url.trim().to_string();
        let item = QueueItem {
            id,
            url: url.clone(),
            title: None,
            mode: self.form.mode,
            quality: self.form.quality,
            path: self.form.path.clone(),
            is_playlist: self.form.is_playlist,
            advanced: self.config.general.advanced.clone(),
            state: ItemState::Pending,
            added_at: std::time::SystemTime::now(),
        };
        self.next_id += 1;
        self.queue.push(item);
        self.config.add_favorite(&self.form.path);
        let _ = self.config.save();

        let tx = self.event_tx.clone();
        tokio::spawn(async move {
            crate::downloader::fetch_title(url, id, tx).await;
        });

        self.form.reset_after_submit(&self.config);
        self.editing_url = false;
        self.sync_url_editing();

        if self.queue_running {
            self.status = "✅ Added to queue".into();
            self.start_next();
        } else {
            self.queue_running = true;
            self.status = "▶ Queue started".into();
            self.start_next();
        }
    }

    pub(crate) fn start_next(&mut self) {
        if !self.queue_running {
            return;
        }
        if self
            .queue
            .iter()
            .any(|i| matches!(i.state, ItemState::Downloading { .. }))
        {
            return;
        }
        let Some(idx) = self
            .queue
            .iter()
            .position(|i| matches!(i.state, ItemState::Pending))
        else {
            return;
        };

        self.queue[idx].state = ItemState::Downloading {
            percent: 0.0,
            speed: "—".into(),
        };
        let item = self.queue[idx].clone();
        let tx = self.event_tx.clone();
        let token = CancellationToken::new();
        self.active_token = Some(token.clone());

        tokio::spawn(async move {
            crate::downloader::run_download(item, tx, token).await;
        });
    }

    pub(crate) async fn handle_download_event(&mut self, ev: DownloadEvent) {
        match ev {
            DownloadEvent::Started(id) => self.status = format!("Downloading #{}", id),
            DownloadEvent::Progress { id, percent, speed } => {
                if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                    item.state = ItemState::Downloading { percent, speed };
                }
            }
            DownloadEvent::Log { id, line } => {
                self.logs.push((id, line));
                if self.logs.len() > 1000 {
                    self.logs.remove(0);
                }
            }
            DownloadEvent::Finished {
                id,
                success,
                message,
            } => {
                if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                    item.state = if success {
                        ItemState::Done
                    } else {
                        ItemState::Failed(message)
                    };
                }
                self.active_token = None;
                self.start_next();
            }
        }
    }

    pub(crate) fn handle_meta_event(&mut self, ev: MetaEvent) {
        match ev {
            MetaEvent::Title { id, title } => {
                if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                    item.title = Some(title);
                }
            }
            MetaEvent::TitleFailed { id } => {
                if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                    if item.title.is_none() {
                        item.title = Some("—".into());
                    }
                }
            }
        }
    }
}
