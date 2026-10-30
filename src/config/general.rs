use serde::{Deserialize, Serialize};

pub const ADVANCED_OPTION_COUNT: usize = 6;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdvancedOptions {
    #[serde(default)]
    pub proxy: String,
    #[serde(default)]
    pub geo_bypass: bool,
    #[serde(default)]
    pub cookies: String,
    #[serde(default)]
    pub user_agent: String,
    #[serde(default)]
    pub rate_limit: String,
    #[serde(default)]
    pub extra_args: String,
}

impl AdvancedOptions {
    pub fn label(index: usize) -> &'static str {
        match index {
            0 => "Proxy URL",
            1 => "Geo bypass",
            2 => "Cookies file",
            3 => "User agent",
            4 => "Rate limit",
            5 => "Extra args",
            _ => "",
        }
    }

    pub fn is_toggle(index: usize) -> bool {
        index == 1
    }

    pub fn value(&self, index: usize) -> String {
        match index {
            0 => self.proxy.clone(),
            1 => {
                if self.geo_bypass {
                    "ON".into()
                } else {
                    "OFF".into()
                }
            }
            2 => self.cookies.clone(),
            3 => self.user_agent.clone(),
            4 => self.rate_limit.clone(),
            5 => self.extra_args.clone(),
            _ => String::new(),
        }
    }

    pub fn text(&self, index: usize) -> &str {
        match index {
            0 => &self.proxy,
            2 => &self.cookies,
            3 => &self.user_agent,
            4 => &self.rate_limit,
            5 => &self.extra_args,
            _ => "",
        }
    }

    pub fn set_text(&mut self, index: usize, value: String) {
        match index {
            0 => self.proxy = value,
            2 => self.cookies = value,
            3 => self.user_agent = value,
            4 => self.rate_limit = value,
            5 => self.extra_args = value,
            _ => {}
        }
    }

    pub fn toggle(&mut self, index: usize) {
        if index == 1 {
            self.geo_bypass = !self.geo_bypass;
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if !self.proxy.trim().is_empty() {
            args.push("--proxy".into());
            args.push(self.proxy.trim().into());
        }
        if self.geo_bypass {
            args.push("--geo-bypass".into());
        }
        if !self.cookies.trim().is_empty() {
            args.push("--cookies".into());
            args.push(self.cookies.trim().into());
        }
        if !self.user_agent.trim().is_empty() {
            args.push("--user-agent".into());
            args.push(self.user_agent.trim().into());
        }
        if !self.rate_limit.trim().is_empty() {
            args.push("--limit-rate".into());
            args.push(self.rate_limit.trim().into());
        }
        args.extend(self.extra_args.split_whitespace().map(|s| s.to_string()));
        args
    }
}

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
    #[serde(default)]
    pub advanced: AdvancedOptions,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_path: default_path(),
            default_quality: default_quality(),
            default_mode: default_mode_str(),
            default_playlist: false,
            advanced: AdvancedOptions::default(),
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
