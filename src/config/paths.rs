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
        Self { favorites }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}
