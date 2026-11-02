//! Local Study Vault folder scanning, hierarchy modeling, and board discovery.

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

/// Maximum directory nesting depth to prevent infinite recursion or symlink loops.
pub const MAX_VAULT_DEPTH: usize = 6;

/// Standard file extension for Ruqa whiteboard documents.
pub const RUQA_EXTENSION: &str = "ruqa";

/// Flattened item representation for linear UI rendering and hit-testing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatVaultItem {
    /// Absolute path to the file or directory.
    pub path: PathBuf,
    /// Display name (with `.ruqa` extension stripped for boards).
    pub name: String,
    /// Whether this entry is a subfolder or a board file.
    pub is_folder: bool,
    /// Visual nesting indentation level (0 for top-level root children).
    pub depth: usize,
    /// For folders: whether child entries are currently expanded.
    pub is_expanded: bool,
}

/// Study Vault folder manager tracking vault root and expanded folders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudyVault {
    /// Root directory path of the study vault.
    pub root: PathBuf,
    /// Set of expanded directory paths.
    pub expanded_dirs: HashSet<PathBuf>,
}

impl StudyVault {
    /// Creates a new Study Vault rooted at `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            expanded_dirs: HashSet::new(),
        }
    }

    /// Resolves default vault directory (falls back to current working directory).
    #[must_use]
    pub fn default_vault_dir() -> PathBuf {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }

    /// Toggles the expanded/collapsed state of a folder path within the vault.
    pub fn toggle_folder(&mut self, folder: &Path) {
        let folder_buf = folder.to_path_buf();
        if !self.expanded_dirs.remove(&folder_buf) {
            self.expanded_dirs.insert(folder_buf);
        }
    }

    /// Expands the specified folder path.
    pub fn expand_folder(&mut self, folder: &Path) {
        self.expanded_dirs.insert(folder.to_path_buf());
    }

    /// Collapses the specified folder path.
    pub fn collapse_folder(&mut self, folder: &Path) {
        self.expanded_dirs.remove(folder);
    }

    /// Returns display name of the vault root directory.
    #[must_use]
    pub fn vault_name(&self) -> String {
        self.root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Study Vault")
            .to_string()
    }

    /// Scans the vault directory tree and returns a flattened list of visible entries.
    ///
    /// # Errors
    /// Returns [`std::io::Error`] if reading the root directory fails.
    pub fn list_entries(&self) -> io::Result<Vec<FlatVaultItem>> {
        let mut entries = Vec::new();
        if self.root.is_dir() {
            Self::scan_recursive(&self.root, 0, &self.expanded_dirs, &mut entries)?;
        }
        Ok(entries)
    }

    fn scan_recursive(
        dir: &Path,
        depth: usize,
        expanded_dirs: &HashSet<PathBuf>,
        out: &mut Vec<FlatVaultItem>,
    ) -> io::Result<()> {
        if depth >= MAX_VAULT_DEPTH {
            return Ok(());
        }

        let mut folders = Vec::new();
        let mut boards = Vec::new();

        let read_dir = fs::read_dir(dir)?;
        for entry_res in read_dir {
            let entry = match entry_res {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Ignore hidden files and directories (starting with '.')
            if file_name.starts_with('.') {
                continue;
            }

            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };

            if file_type.is_dir() {
                folders.push((file_name, path));
            } else if file_type.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case(RUQA_EXTENSION) {
                        let stem = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or(&file_name)
                            .to_string();
                        boards.push((stem, path));
                    }
                }
            }
        }

        // Sort folders alphabetically (case-insensitive)
        folders.sort_by_key(|a| a.0.to_lowercase());
        // Sort boards alphabetically (case-insensitive)
        boards.sort_by_key(|a| a.0.to_lowercase());

        // Add subfolders, and recursively scan if expanded
        for (name, path) in folders {
            let is_expanded = expanded_dirs.contains(&path);
            out.push(FlatVaultItem {
                path: path.clone(),
                name,
                is_folder: true,
                depth,
                is_expanded,
            });

            if is_expanded {
                let _ = Self::scan_recursive(&path, depth + 1, expanded_dirs, out);
            }
        }

        // Add board files
        for (name, path) in boards {
            out.push(FlatVaultItem {
                path,
                name,
                is_folder: false,
                depth,
                is_expanded: false,
            });
        }

        Ok(())
    }
}

/// Helper to get file modification time as seconds since UNIX epoch.
#[must_use]
pub fn file_modified_timestamp(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_scanning_and_expansion() {
        let tmp = std::env::temp_dir().join(format!("ruqa_vault_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("Math/Calculus")).unwrap();
        fs::create_dir_all(tmp.join("CS")).unwrap();

        fs::write(tmp.join("Math/Calculus/Derivatives.ruqa"), "{}").unwrap();
        fs::write(tmp.join("CS/Algorithms.ruqa"), "{}").unwrap();
        fs::write(tmp.join("Welcome.ruqa"), "{}").unwrap();
        fs::write(tmp.join("notes.txt"), "hello").unwrap(); // Non-ruqa file ignored
        fs::write(tmp.join(".hidden.ruqa"), "{}").unwrap(); // Hidden file ignored

        let mut vault = StudyVault::new(&tmp);

        // Initially collapsed: top level only
        let entries = vault.list_entries().unwrap();
        assert_eq!(entries.len(), 3); // CS folder, Math folder, Welcome board
        assert_eq!(entries[0].name, "CS");
        assert!(entries[0].is_folder);
        assert!(!entries[0].is_expanded);

        assert_eq!(entries[1].name, "Math");
        assert!(entries[1].is_folder);

        assert_eq!(entries[2].name, "Welcome");
        assert!(!entries[2].is_folder);
        assert_eq!(entries[2].depth, 0);

        // Expand CS folder
        vault.expand_folder(&tmp.join("CS"));
        let entries = vault.list_entries().unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].name, "CS");
        assert!(entries[0].is_expanded);
        assert_eq!(entries[1].name, "Algorithms");
        assert_eq!(entries[1].depth, 1);
        assert!(!entries[1].is_folder);

        // Toggle Math folder
        vault.toggle_folder(&tmp.join("Math"));
        let entries = vault.list_entries().unwrap();
        assert_eq!(entries.len(), 5);
        assert_eq!(entries[2].name, "Math");
        assert_eq!(entries[3].name, "Calculus");
        assert_eq!(entries[3].depth, 1);

        // Collapse CS
        vault.collapse_folder(&tmp.join("CS"));
        let entries = vault.list_entries().unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].name, "CS");
        assert!(!entries[0].is_expanded);

        let _ = fs::remove_dir_all(&tmp);
    }
}
