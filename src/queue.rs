use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::AdvancedOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadMode {
    Video,
    Audio,
}

impl DownloadMode {
    pub fn label(&self) -> &'static str {
        match self {
            DownloadMode::Video => "Video",
            DownloadMode::Audio => "Audio",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quality {
    Best,
    P1080,
    P720,
    P480,
    P360,
}

impl Quality {
    pub const ALL: [Quality; 5] = [
        Quality::Best,
        Quality::P1080,
        Quality::P720,
        Quality::P480,
        Quality::P360,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Quality::Best => "Best",
            Quality::P1080 => "1080p",
            Quality::P720 => "720p",
            Quality::P480 => "480p",
            Quality::P360 => "360p",
        }
    }

    pub fn format_str(&self) -> &'static str {
        match self {
            Quality::Best => "bestvideo+bestaudio/best",
            Quality::P1080 => "bestvideo[height<=1080]+bestaudio/best[height<=1080]",
            Quality::P720 => "bestvideo[height<=720]+bestaudio/best[height<=720]",
            Quality::P480 => "bestvideo[height<=480]+bestaudio/best[height<=480]",
            Quality::P360 => "bestvideo[height<=360]+bestaudio/best[height<=360]",
        }
    }

    pub fn from_label(s: &str) -> Quality {
        match s {
            "1080p" => Quality::P1080,
            "720p" => Quality::P720,
            "480p" => Quality::P480,
            "360p" => Quality::P360,
            _ => Quality::Best,
        }
    }

    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|q| q == self).unwrap_or(0)
    }

    pub fn next(&self) -> Quality {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    pub fn prev(&self) -> Quality {
        Self::ALL[(self.index() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone)]
pub enum ItemState {
    Pending,
    Downloading { percent: f32, speed: String },
    Done,
    Failed(String),
}

impl ItemState {
    pub fn label(&self) -> String {
        match self {
            ItemState::Pending => "⏸ Pending".into(),
            ItemState::Downloading { .. } => "⏳ Downloading".into(),
            ItemState::Done => "✅ Done".into(),
            ItemState::Failed(e) => format!("❌ {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id: usize,
    pub url: String,
    pub title: Option<String>,
    pub mode: DownloadMode,
    pub quality: Quality,
    pub path: String,
    pub is_playlist: bool,
    pub advanced: AdvancedOptions,
    pub state: ItemState,
    pub added_at: SystemTime,
}

impl QueueItem {
    pub fn added_at_str(&self) -> String {
        let duration = self.added_at.duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = duration.as_secs() as i64;
        let nanos = duration.subsec_nanos();
        match Local.timestamp_opt(secs, nanos) {
            chrono::LocalResult::Single(dt) => dt.format("%H:%M:%S").to_string(),
            _ => "--:--:--".to_string(),
        }
    }

    pub fn display_name(&self) -> String {
        self.title.clone().unwrap_or_else(|| "…".to_string())
    }
}
