use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    #[serde(default)]
    pub favorites: Vec<String>,
}

impl PathsConfig {
    pub fn with_defaults() -> Self {
        let mut favorites = Vec::new();
        if let Some(dl) = dirs::download_dir() {
            favorites.push(dl.to_string_lossy().to_string());
        }
        let mut cfg = Self { favorites };
        cfg.sort();
        cfg
    }

    pub fn sort(&mut self) {
        self.favorites.sort_by_cached_key(|p| p.to_lowercase());
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}
