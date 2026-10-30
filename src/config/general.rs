use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_path")]
    pub default_path: String,
    #[serde(default = "default_quality")]
    pub default_quality: String,
    #[serde(default = "default_mode_str")]
    pub default_mode: String,
    #[serde(default = "default_false")]
    pub default_playlist: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_path: default_path(),
            default_quality: default_quality(),
            default_mode: default_mode_str(),
            default_playlist: false,
        }
    }
}

fn default_path() -> String {
    dirs::download_dir()
        .or_else(dirs::home_dir)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".into())
}
fn default_quality() -> String {
    "Best".into()
}
fn default_mode_str() -> String {
    "Video".into()
}
fn default_false() -> bool {
    false
}
