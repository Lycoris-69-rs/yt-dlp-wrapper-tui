pub mod general;
pub mod paths;
pub mod queue_table;
pub mod status_bar;

pub use general::{ADVANCED_OPTION_COUNT, AdvancedOptions, GeneralConfig};
pub use paths::PathsConfig;
pub use queue_table::{ALL_COLUMNS, Alignment, QueueTableConfig};
pub use status_bar::StatusBarConfig;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub paths: PathsConfig,
    #[serde(default)]
    pub status_bar: StatusBarConfig,
    #[serde(default)]
    pub queue_table: QueueTableConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            paths: PathsConfig::default(),
            status_bar: StatusBarConfig::default(),
            queue_table: QueueTableConfig::default(),
        }
    }
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("yt-dlp-tui").join("config.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };
        match std::fs::read_to_string(&path) {
            Ok(s) => {
                let mut config: Self = toml::from_str(&s).unwrap_or_default();
                config.paths.sort();
                config
            }
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let Some(path) = Self::config_path() else {
            anyhow::bail!("Cannot determine config path");
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let s = toml::to_string_pretty(self)?;
        std::fs::write(&path, s)?;
        Ok(())
    }

    pub fn add_favorite(&mut self, path: &str) {
        if !path.is_empty() && !self.paths.favorites.iter().any(|p| p == path) {
            self.paths.favorites.push(path.to_string());
            self.paths.sort();
        }
    }

    pub fn remove_favorite(&mut self, path: &str) {
        self.paths.favorites.retain(|p| p != path);
    }
}
