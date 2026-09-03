use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoryEntry {
    pub id: u64,
    pub when: String,
    pub repo: String,
    pub text: String,
    pub year: i32,
    pub start_week: usize,
    pub commits: usize,
    pub before_head: String,
    pub after_head: String,
}

pub struct HistoryLog {
    path: PathBuf,
    entries: Vec<HistoryEntry>,
}

impl HistoryLog {
    pub fn default_path() -> PathBuf {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(std::env::temp_dir()))
            .join("gitpixel");
        dir.join("history.json")
    }

    pub fn open() -> Self {
        Self::open_at(Self::default_path())
    }

    pub fn open_at(path: PathBuf) -> Self {
        let entries: Vec<HistoryEntry> = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        HistoryLog { path, entries }
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn record(&mut self, entry: HistoryEntry) -> std::io::Result<()> {
        self.entries.push(entry);
        self.save()
    }

    pub fn remove_id(&mut self, id: u64) -> Option<HistoryEntry> {
        let idx = self.entries.iter().position(|e| e.id == id)?;
        let entry = self.entries.remove(idx);
        self.save().ok()?;
        Some(entry)
    }

    pub fn last(&self) -> Option<&HistoryEntry> {
        self.entries.last()
    }

    fn save(&self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.entries)?;
        fs::write(&self.path, json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn entry(id: u64) -> HistoryEntry {
        HistoryEntry {
            id,
            when: "now".into(),
            repo: "/tmp/repo".into(),
            text: "HI".into(),
            year: 2026,
            start_week: 10,
            commits: 4,
            before_head: "ab".into(),
            after_head: "cd".into(),
        }
    }

    #[test]
    fn records_and_persists_entries() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("history.json");

        let mut log = HistoryLog::open_at(path.clone());
        log.record(entry(1)).unwrap();
        log.record(entry(2)).unwrap();
        assert_eq!(log.entries().len(), 2);

        // Reopen and confirm persistence.
        let log2 = HistoryLog::open_at(path);
        assert_eq!(log2.entries().len(), 2);
        assert_eq!(log2.entries()[1].id, 2);
    }

    #[test]
    fn remove_id_removes_specific_entry() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("h.json");
        let mut log = HistoryLog::open_at(path.clone());
        log.record(entry(1)).unwrap();
        log.record(entry(2)).unwrap();
        let removed = log.remove_id(1).unwrap();
        assert_eq!(removed.id, 1);
        assert_eq!(log.entries().len(), 1);
        assert_eq!(log.entries()[0].id, 2);
    }
}
