//! Recent whiteboard document tracking, LRU management, and JSON configuration persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Maximum number of recent file paths retained in configuration.
pub const MAX_RECENT_FILES: usize = 10;

/// Default configuration directory name under user config root.
pub const CONFIG_DIR_NAME: &str = "ruqa";

/// Default recent files configuration filename.
pub const CONFIG_FILE_NAME: &str = "recent_files.json";

/// Entry recording a recently opened or saved whiteboard document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentFileEntry {
    /// Absolute or canonical path to the `.ruqa` file.
    pub path: PathBuf,
    /// Unix timestamp in seconds when the file was last opened or saved.
    pub last_modified: u64,
}

impl RecentFileEntry {
    /// Creates a new recent file record with current system timestamp.
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            path,
            last_modified: timestamp,
        }
    }

    /// Returns the file name of the document, or `"Untitled"` if unavailable.
    #[must_use]
    pub fn display_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string()
    }
}

/// Collection of recent documents managed in Least-Recently-Used (LRU) order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentFiles {
    /// Ordered list of recent files (most recent first, capped at [`MAX_RECENT_FILES`]).
    pub entries: Vec<RecentFileEntry>,
}

impl RecentFiles {
    /// Creates an empty recent files list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a file path, moving it to the front of the list and truncating to [`MAX_RECENT_FILES`].
    pub fn add(&mut self, path: impl AsRef<Path>) {
        let path_buf = path.as_ref().to_path_buf();
        // Remove any existing entry matching this path
        self.entries.retain(|e| e.path != path_buf);

        // Prepend new entry to front
        self.entries.insert(0, RecentFileEntry::new(path_buf));

        // Truncate list to maximum capacity
        if self.entries.len() > MAX_RECENT_FILES {
            self.entries.truncate(MAX_RECENT_FILES);
        }
    }

    /// Removes a file path from the recent list (e.g. if deleted or nonexistent).
    pub fn remove(&mut self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        let initial_len = self.entries.len();
        self.entries.retain(|e| e.path != path);
        self.entries.len() != initial_len
    }

    /// Clears all recent file entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns `true` if no recent files are recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of recent file entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Resolves the default user configuration path for `recent_files.json`.
    ///
    /// Respects `XDG_CONFIG_HOME` on Unix/Linux, `%APPDATA%` on Windows,
    /// and `~/Library/Application Support` on macOS.
    #[must_use]
    pub fn default_config_path() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            std::env::var_os("APPDATA").map(|appdata| {
                PathBuf::from(appdata)
                    .join(CONFIG_DIR_NAME)
                    .join(CONFIG_FILE_NAME)
            })
        }

        #[cfg(target_os = "macos")]
        {
            std::env::var_os("HOME").map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join(CONFIG_DIR_NAME)
                    .join(CONFIG_FILE_NAME)
            })
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
                Some(
                    PathBuf::from(xdg)
                        .join(CONFIG_DIR_NAME)
                        .join(CONFIG_FILE_NAME),
                )
            } else {
                std::env::var_os("HOME").map(|home| {
                    PathBuf::from(home)
                        .join(".config")
                        .join(CONFIG_DIR_NAME)
                        .join(CONFIG_FILE_NAME)
                })
            }
        }
    }

    /// Loads recent files from the specified JSON file path.
    ///
    /// # Errors
    /// Returns [`std::io::Error`] if file reading or JSON deserialization fails.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Saves recent files to the specified JSON path atomically.
    ///
    /// # Errors
    /// Returns [`std::io::Error`] if directory creation or file writing fails.
    pub fn save_to_path(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    }

    /// Loads recent files from the standard system configuration path, or returns default on missing file.
    #[must_use]
    pub fn load() -> Self {
        Self::default_config_path()
            .and_then(|p| Self::load_from_path(p).ok())
            .unwrap_or_default()
    }

    /// Saves recent files to the standard system configuration path.
    ///
    /// # Errors
    /// Returns [`std::io::Error`] if standard config path cannot be resolved or write fails.
    pub fn save(&self) -> Result<(), std::io::Error> {
        if let Some(path) = Self::default_config_path() {
            self.save_to_path(path)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "could not resolve user configuration directory",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_recent_file_deduplicates_and_prepends() {
        let mut list = RecentFiles::new();
        list.add(PathBuf::from("/notes/lecture1.ruqa"));
        list.add(PathBuf::from("/notes/lecture2.ruqa"));
        assert_eq!(list.len(), 2);
        assert_eq!(list.entries[0].path, PathBuf::from("/notes/lecture2.ruqa"));

        // Re-adding lecture1 moves it to the front
        list.add(PathBuf::from("/notes/lecture1.ruqa"));
        assert_eq!(list.len(), 2);
        assert_eq!(list.entries[0].path, PathBuf::from("/notes/lecture1.ruqa"));
        assert_eq!(list.entries[1].path, PathBuf::from("/notes/lecture2.ruqa"));
    }

    #[test]
    fn test_truncation_to_max_recent_files() {
        let mut list = RecentFiles::new();
        for i in 0..15 {
            list.add(PathBuf::from(format!("/notes/board_{i}.ruqa")));
        }
        assert_eq!(list.len(), MAX_RECENT_FILES);
        assert_eq!(list.entries[0].path, PathBuf::from("/notes/board_14.ruqa"));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!("ruqa_test_{}", std::process::id()));
        let file = dir.join("recent.json");

        let mut original = RecentFiles::new();
        original.add(PathBuf::from("/study/discrete_math.ruqa"));
        original.add(PathBuf::from("/study/os_kernels.ruqa"));

        original.save_to_path(&file).expect("save succeeds");
        let loaded = RecentFiles::load_from_path(&file).expect("load succeeds");

        assert_eq!(original.len(), loaded.len());
        assert_eq!(original.entries[0].path, loaded.entries[0].path);
        assert_eq!(original.entries[1].path, loaded.entries[1].path);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_display_name_helper() {
        let entry = RecentFileEntry::new(PathBuf::from("/vault/Algorithms.ruqa"));
        assert_eq!(entry.display_name(), "Algorithms.ruqa");
    }
}
