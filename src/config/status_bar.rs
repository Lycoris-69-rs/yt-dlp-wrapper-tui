use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub show_hints: bool,
    #[serde(default = "default_true")]
    pub show_queue_stats: bool,
    #[serde(default = "default_true")]
    pub show_active: bool,
    #[serde(default = "default_true")]
    pub show_status_message: bool,
}

impl Default for StatusBarConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_hints: true,
            show_queue_stats: true,
            show_active: true,
            show_status_message: true,
        }
    }
}

fn default_true() -> bool {
    true
}
