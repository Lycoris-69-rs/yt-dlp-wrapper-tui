pub mod form;
pub mod help;
pub mod keys;
pub mod logs;
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

/// A single selectable row inside the "Select path" picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathPickerEntry {
    Pinned { fav_idx: usize },
    Path { fav_idx: usize },
    AddCurrent,
    AddNew,
    Delete,
    Pin,
}

impl PathPickerEntry {
    pub fn favorite_idx(self) -> Option<usize> {
        match self {
            PathPickerEntry::Pinned { fav_idx } | PathPickerEntry::Path { fav_idx } => Some(fav_idx),
            PathPickerEntry::AddCurrent
            | PathPickerEntry::AddNew
            | PathPickerEntry::Delete
            | PathPickerEntry::Pin => None,
        }
    }
}

pub struct App {
    pub queue: Vec<QueueItem>,
    pub selected: usize,
    pub focus: Focus,
    pub marquee_offset: usize,
    pub anim_frame: usize,
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

    pub show_logs: bool,
    pub logs_scroll: u16,
    pub logs_follow: bool,

    pub settings_index: usize,
    pub popup: Popup,
    pub path_picker_index: usize,
    pub path_picker_last_path: Option<String>,
    pub path_picker_from_settings: bool,
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
    pub active_id: Option<usize>,
    pub active_gen: Option<u64>,
    pub next_gen: u64,
}

impl App {
    pub fn new(event_tx: mpsc::UnboundedSender<AppEvent>) -> Self {
        let config = Config::load();
        let form = Form::new(&config);
        let mut app = Self {
            queue: Vec::new(),
            selected: 0,
            focus: Focus::Form,
            marquee_offset: 0,
            anim_frame: 0,
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
            show_logs: false,
            logs_scroll: 0,
            logs_follow: true,
            settings_index: 0,
            popup: Popup::None,
            path_picker_index: 0,
            path_picker_last_path: None,
            path_picker_from_settings: false,
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
            active_id: None,
            active_gen: None,
            next_gen: 1,
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

    pub async fn on_tick(&mut self) {
        // Advance the horizontal marquee used for cells wider than their column.
        let step = self.config.queue_table.scroll_offset.max(1) as usize;
        self.marquee_offset = self.marquee_offset.wrapping_add(step);
        // Advance the "fetching title" spinner.
        self.anim_frame = self.anim_frame.wrapping_add(1);
    }

    pub(crate) fn sync_url_editing(&mut self) {
        self.editing_url = self.focus == Focus::Form && self.form.field == FormField::Url;
        if self.editing_url {
            self.form.url_cursor = self.form.url.chars().count();
        }
    }

    /// Ordered, selectable rows of the "Select path" picker: pinned favorites
    /// first, then the remaining paths, then the action buttons.
    pub(crate) fn path_picker_entries(&self) -> Vec<PathPickerEntry> {
        let favs = &self.config.paths.favorites;
        let mut entries = Vec::new();
        for (i, p) in favs.iter().enumerate() {
            if self.config.paths.is_pinned(p) {
                entries.push(PathPickerEntry::Pinned { fav_idx: i });
            }
        }
        for (i, p) in favs.iter().enumerate() {
            if !self.config.paths.is_pinned(p) {
                entries.push(PathPickerEntry::Path { fav_idx: i });
            }
        }
        entries.push(PathPickerEntry::Pin);
        entries.push(PathPickerEntry::AddCurrent);
        entries.push(PathPickerEntry::AddNew);
        entries.push(PathPickerEntry::Delete);
        entries
    }

    /// Path-only rows for the pin manager (no action buttons).
    pub(crate) fn path_pin_entries(&self) -> Vec<PathPickerEntry> {
        self.path_picker_entries()
            .into_iter()
            .filter(|e| e.favorite_idx().is_some())
            .collect()
    }

    /// Flat entry index for a favorite path (pinned or not).
    pub(crate) fn path_picker_find_favorite(&self, path: &str) -> Option<usize> {
        self.path_picker_entries().iter().position(|e| match e {
            PathPickerEntry::Pinned { fav_idx } | PathPickerEntry::Path { fav_idx } => self
                .config
                .paths
                .favorites
                .get(*fav_idx)
                .is_some_and(|p| p == path),
            _ => false,
        })
    }

    /// Flat entry index for a specific action button.
    pub(crate) fn path_picker_action_index(&self, action: PathPickerEntry) -> Option<usize> {
        self.path_picker_entries()
            .iter()
            .position(|e| *e == action)
    }

    /// The path the pin toggle should act on: the currently highlighted path,
    /// or, when an action row is highlighted, the last visited path.
    pub(crate) fn path_picker_current_path(&self) -> Option<String> {
        let entries = self.path_picker_entries();
        if let Some(entry) = entries.get(self.path_picker_index) {
            if let Some(fav_idx) = entry.favorite_idx() {
                return self.config.paths.favorites.get(fav_idx).cloned();
            }
        }
        self.path_picker_last_path.clone()
    }

    pub(crate) fn path_picker_track_path(&mut self) {
        let entries = self.path_picker_entries();
        if let Some(entry) = entries.get(self.path_picker_index) {
            if let Some(fav_idx) = entry.favorite_idx() {
                self.path_picker_last_path = self.config.paths.favorites.get(fav_idx).cloned();
            }
        }
    }

    pub(crate) fn path_picker_toggle_pin(&mut self) {
        let Some(target) = self.path_picker_current_path() else {
            self.status = "❌ Nothing to pin".into();
            return;
        };
        let pinned = self.config.paths.toggle_pin(&target);
        let _ = self.config.save();
        self.status = if pinned {
            format!("📌 Pinned: {target}")
        } else {
            format!("📌 Unpinned: {target}")
        };
        if let Some(idx) = self.path_picker_find_favorite(&target) {
            self.path_picker_index = idx;
            self.path_picker_last_path = Some(target);
        }
    }

    pub(crate) fn open_help(&mut self) {
        self.show_help = true;
        self.help_scroll = 0;
        self.show_logs = false;
        self.popup = Popup::None;
    }

    pub(crate) fn open_logs(&mut self) {
        self.show_logs = true;
        self.show_help = false;
        self.logs_scroll = 0;
        self.logs_follow = true;
        self.popup = Popup::None;
    }

    pub(crate) fn open_settings(&mut self) {
        self.popup = Popup::Settings;
        self.settings_index = 0;
        self.show_help = false;
        self.show_logs = false;
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
        if self.active_id.is_some() {
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
        self.start_item(idx);
    }

    pub(crate) fn start_item(&mut self, idx: usize) {
        let Some(item) = self.queue.get_mut(idx) else {
            return;
        };
        item.state = ItemState::Downloading {
            percent: 0.0,
            speed: "—".into(),
        };
        let id = item.id;
        let item = item.clone();
        let tx = self.event_tx.clone();
        let token = CancellationToken::new();
        let run_id = self.next_gen;
        self.next_gen = self.next_gen.wrapping_add(1);
        self.active_token = Some(token.clone());
        self.active_id = Some(id);
        self.active_gen = Some(run_id);

        tokio::spawn(async move {
            crate::downloader::run_download(item, tx, token, run_id).await;
        });
    }

    /// Cancels the currently running download and puts its item back to
    /// `Pending`. Returns the id of the cancelled item, if any.
    pub(crate) fn cancel_active(&mut self) -> Option<usize> {
        let id = self.active_id.take().or_else(|| {
            self.queue
                .iter()
                .find(|i| matches!(i.state, ItemState::Downloading { .. }))
                .map(|i| i.id)
        });
        if let Some(token) = self.active_token.take() {
            token.cancel();
        }
        self.active_gen = None;
        if let Some(id) = id {
            if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                item.state = ItemState::Pending;
            }
        }
        id
    }

    pub(crate) async fn handle_download_event(&mut self, ev: DownloadEvent) {
        match ev {
            DownloadEvent::Started { id, run_id } => {
                if self.active_gen == Some(run_id) && self.active_id == Some(id) {
                    self.status = format!("Downloading #{}", id);
                }
            }
            DownloadEvent::Progress {
                id,
                run_id,
                percent,
                speed,
            } => {
                if self.active_gen == Some(run_id) && self.active_id == Some(id) {
                    if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                        item.state = ItemState::Downloading { percent, speed };
                    }
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
                run_id,
                success,
                message,
            } => {
                // Ignore the tail of a download that was cancelled (paused,
                // skipped or deleted) — a newer item may already be running.
                if self.active_gen != Some(run_id) {
                    return;
                }
                self.active_gen = None;
                self.active_id = None;
                self.active_token = None;
                if let Some(item) = self.queue.iter_mut().find(|i| i.id == id) {
                    item.state = if success {
                        ItemState::Done
                    } else {
                        ItemState::Failed(message)
                    };
                }
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
