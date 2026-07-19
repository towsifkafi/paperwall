use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use colored::*;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Local>,
    pub source: String,
    pub image_url: String,
    pub local_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    const MAX_ENTRIES: usize = 50;

    fn get_history_path() -> Option<PathBuf> {
        if let Some(data_dir) = dirs::data_dir() {
            let paperwall_dir = data_dir.join("paperwall");
            if !paperwall_dir.exists() {
                let _ = fs::create_dir_all(&paperwall_dir);
            }
            Some(paperwall_dir.join("history.json"))
        } else {
            None
        }
    }

    pub fn load() -> Self {
        if let Some(path) = Self::get_history_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(history) = serde_json::from_str::<History>(&content) {
                    return history;
                }
            }
        }
        History::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::get_history_path() {
            if let Ok(content) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, content);
            }
        }
    }

    pub fn add(source: &str, image_url: &str, local_path: Option<PathBuf>) {
        let mut history = Self::load();
        
        let entry = HistoryEntry {
            timestamp: Local::now(),
            source: source.to_string(),
            image_url: image_url.to_string(),
            local_path,
        };

        history.entries.push(entry);

        // Enforce limit
        if history.entries.len() > Self::MAX_ENTRIES {
            let excess = history.entries.len() - Self::MAX_ENTRIES;
            history.entries.drain(0..excess);
        }

        history.save();
    }

    pub fn list() {
        let history = Self::load();
        if history.entries.is_empty() {
            info!("{}", "History is empty.".bright_white());
            return;
        }

        info!("{}", "Wallpaper History:".bright_white().bold());
        for (i, entry) in history.entries.iter().rev().enumerate() {
            let idx = i + 1;
            let time = entry.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().bright_cyan();
            let source = entry.source.bright_green();
            let url = entry.image_url.bright_black();
            println!("  {}. [{}] [{}] {}", idx.to_string().bright_yellow(), time, source, url);
        }
    }

    pub fn clear() {
        if let Some(path) = Self::get_history_path() {
            match fs::remove_file(&path) {
                Ok(_) => success!("{}", "History cleared successfully.".bright_green()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    info!("{}", "History is already empty.".bright_white());
                }
                Err(_) => error!("{}", "Failed to clear history.".bright_red()),
            }
        }
    }

    pub fn get(index: usize) -> Option<HistoryEntry> {
        let history = Self::load();
        if index == 0 || history.entries.is_empty() {
            return None;
        }
        
        // Index 1 means "go back 1 step", which is the 2nd most recent entry (len - 2)
        let reverse_idx = history.entries.len().checked_sub(index + 1)?;
        history.entries.get(reverse_idx).cloned()
    }
}
