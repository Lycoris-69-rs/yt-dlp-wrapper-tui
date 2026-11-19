use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    #[serde(default)]
    pub favorites: Vec<String>,
    #[serde(default)]
    pub pinned: Vec<String>,
}

impl PathsConfig {
    pub fn with_defaults() -> Self {
        let mut favorites = Vec::new();
        if let Some(dl) = dirs::download_dir() {
            favorites.push(dl.to_string_lossy().to_string());
        }
        let mut cfg = Self {
            favorites,
            pinned: Vec::new(),
        };
        cfg.sort();
        cfg
    }

    pub fn sort(&mut self) {
        self.favorites.sort_by_cached_key(|p| p.to_lowercase());
        self.pinned.sort_by_cached_key(|p| p.to_lowercase());
    }

    pub fn is_pinned(&self, path: &str) -> bool {
        self.pinned.iter().any(|p| p == path)
    }

    pub fn pin(&mut self, path: &str) {
        if !path.is_empty() && !self.is_pinned(path) {
            self.pinned.push(path.to_string());
            self.pinned.sort_by_cached_key(|p| p.to_lowercase());
        }
    }

    pub fn unpin(&mut self, path: &str) {
        self.pinned.retain(|p| p != path);
    }

    /// Toggles the pin state for `path`. Returns `true` when the path is now
    /// pinned and `false` when it has been unpinned.
    pub fn toggle_pin(&mut self, path: &str) -> bool {
        if self.is_pinned(path) {
            self.unpin(path);
            false
        } else {
            self.pin(path);
            true
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}
