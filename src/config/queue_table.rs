use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const ALL_COLUMNS: [&str; 10] = [
    "num", "added", "name", "url", "mode", "quality", "status", "progress", "percent", "speed",
];

pub const WEIGHT_MIN: u16 = 1;
pub const WEIGHT_MAX: u16 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Alignment {
    pub const ALL: [Alignment; 3] = [Alignment::Left, Alignment::Center, Alignment::Right];

    pub fn label(&self) -> &'static str {
        match self {
            Alignment::Left => "Left",
            Alignment::Center => "Center",
            Alignment::Right => "Right",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Alignment::Left => Alignment::Center,
            Alignment::Center => Alignment::Right,
            Alignment::Right => Alignment::Left,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Alignment::Left => Alignment::Right,
            Alignment::Center => Alignment::Left,
            Alignment::Right => Alignment::Center,
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueTableConfig {
    #[serde(default = "default_queue_columns")]
    pub columns: Vec<String>,
    #[serde(default = "default_queue_widths")]
    pub widths: HashMap<String, u16>,
    #[serde(default)]
    pub auto_width: HashMap<String, bool>,
    #[serde(default = "default_weights")]
    pub weights: HashMap<String, u16>,
    #[serde(default = "default_alignments")]
    pub alignment: HashMap<String, Alignment>,
}

impl Default for QueueTableConfig {
    fn default() -> Self {
        Self {
            columns: default_queue_columns(),
            widths: default_queue_widths(),
            auto_width: default_auto_width(),
            weights: default_weights(),
            alignment: default_alignments(),
        }
    }
}

fn default_auto_width() -> HashMap<String, bool> {
    let mut m = HashMap::new();
    for col in ALL_COLUMNS {
        if col != "num" {
            m.insert(col.to_string(), true);
        }
    }
    m
}

impl QueueTableConfig {
    pub fn width_of(&self, col: &str) -> u16 {
        self.widths.get(col).copied().unwrap_or_else(|| match col {
            "num" => 1,
            "added" => 10,
            "name" => 10,
            "url" => 10,
            "mode" => 10,
            "quality" => 10,
            "status" => 10,
            "progress" => 10,
            "percent" => 10,
            "speed" => 10,
            _ => 10,
        })
    }

    pub fn weight_of(&self, col: &str) -> u16 {
        self.weights.get(col).copied().unwrap_or_else(|| match col {
            "num" => 1,
            "added" => 5,
            "name" => 5,
            "url" => 5,
            "mode" => 5,
            "quality" => 5,
            "status" => 5,
            "progress" => 5,
            "percent" => 5,
            "speed" => 5,
            _ => 5,
        })
    }

    pub fn set_weight(&mut self, col: &str, w: u16) {
        self.weights
            .insert(col.to_string(), w.clamp(WEIGHT_MIN, WEIGHT_MAX));
    }

    pub fn bump_weight(&mut self, col: &str, delta: i16) -> u16 {
        let cur = self.weight_of(col) as i32;
        let next = (cur + delta as i32).clamp(WEIGHT_MIN as i32, WEIGHT_MAX as i32) as u16;
        self.set_weight(col, next);
        next
    }

    pub fn set_width(&mut self, col: &str, w: u16) {
        self.widths.insert(col.to_string(), w.max(1));
    }

    pub fn is_auto(&self, col: &str) -> bool {
        self.auto_width.get(col).copied().unwrap_or(false)
    }

    pub fn set_auto(&mut self, col: &str, auto: bool) {
        if auto {
            self.auto_width.insert(col.to_string(), true);
        } else {
            self.auto_width.remove(col);
        }
    }

    pub fn toggle_auto(&mut self, col: &str) {
        let next = !self.is_auto(col);
        self.set_auto(col, next);
    }

    pub fn move_column_left(&mut self, idx: usize) {
        if idx > 0 && idx < self.columns.len() {
            self.columns.swap(idx, idx - 1);
        }
    }

    pub fn move_column_right(&mut self, idx: usize) {
        if idx + 1 < self.columns.len() {
            self.columns.swap(idx, idx + 1);
        }
    }

    pub fn add_column(&mut self, col: &str) {
        if !self.columns.iter().any(|c| c == col) {
            self.columns.push(col.to_string());
        }
    }


    pub fn alignment_of(&self, col: &str) -> Alignment {
        self.alignment.get(col).copied().unwrap_or_default()
    }

    pub fn set_alignment(&mut self, col: &str, a: Alignment) {
        self.alignment.insert(col.to_string(), a);
    }

    pub fn cycle_alignment_next(&mut self, col: &str) -> Alignment {
        let a = self.alignment_of(col).next();
        self.set_alignment(col, a);
        a
    }

    pub fn cycle_alignment_prev(&mut self, col: &str) -> Alignment {
        let a = self.alignment_of(col).prev();
        self.set_alignment(col, a);
        a
    }
}

fn default_queue_columns() -> Vec<String> {
    ["num", "name", "url", "progress", "percent", "status"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn default_queue_widths() -> HashMap<String, u16> {
    let mut m = HashMap::new();
    m.insert("num".into(), 1);
    m.insert("added".into(), 10);
    m.insert("name".into(), 10);
    m.insert("url".into(), 10);
    m.insert("mode".into(), 10);
    m.insert("quality".into(), 10);
    m.insert("status".into(), 10);
    m.insert("progress".into(), 10);
    m.insert("percent".into(), 10);
    m.insert("speed".into(), 10);
    m
}

fn default_weights() -> HashMap<String, u16> {
    let mut m = HashMap::new();
    m.insert("num".into(), 1);
    m.insert("added".into(), 5);
    m.insert("name".into(), 5);
    m.insert("url".into(), 5);
    m.insert("mode".into(), 5);
    m.insert("quality".into(), 5);
    m.insert("status".into(), 5);
    m.insert("progress".into(), 5);
    m.insert("percent".into(), 5);
    m.insert("speed".into(), 5);
    m
}

fn default_alignments() -> HashMap<String, Alignment> {
    let mut m = HashMap::new();
    m.insert("num".into(), Alignment::Right);
    m.insert("added".into(), Alignment::Left);
    m.insert("name".into(), Alignment::Left);
    m.insert("url".into(), Alignment::Left);
    m.insert("mode".into(), Alignment::Left);
    m.insert("quality".into(), Alignment::Left);
    m.insert("status".into(), Alignment::Left);
    m.insert("progress".into(), Alignment::Left);
    m.insert("percent".into(), Alignment::Left);
    m.insert("speed".into(), Alignment::Left);
    m
}
