//! Document session management, dirty-state tracking, and file I/O operations for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::document::RuqaDocument;
use ruqa_core::error::DocumentError;
use std::path::{Path, PathBuf};
use tracing::info;

impl WhiteboardApp {
    /// Returns the dynamically computed OS window title reflecting document name and dirty state.
    #[must_use]
    pub fn window_title(&self) -> String {
        let name = self
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");
        let dirty_suffix = if self.is_dirty { "*" } else { "" };
        format!("Ruqa - {name}{dirty_suffix}")
    }

    /// Updates the OS window title to reflect current file path and dirty state.
    pub fn update_window_title(&self) {
        if let Some(w) = &self.window {
            w.set_title(&self.window_title());
        }
    }

    /// Marks the active document as having unsaved modifications and updates window title.
    pub fn mark_dirty(&mut self) {
        self.last_dirty_time = Some(std::time::Instant::now());
        if !self.is_dirty {
            self.is_dirty = true;
            self.update_window_title();
        }
    }

    /// Marks the active document as saved and updates window title.
    pub fn mark_clean(&mut self) {
        self.last_dirty_time = None;
        if self.is_dirty {
            self.is_dirty = false;
            self.update_window_title();
        }
    }

    /// Returns the next available untitled file path that does not yet exist on disk.
    ///
    /// If `"untitled.ruqa"` does not exist in `dir`, returns that path.
    /// Otherwise, checks `"untitled-2.ruqa"`, `"untitled-3.ruqa"`, etc. until an unused path is found.
    #[must_use]
    pub fn next_untitled_path(dir: &Path) -> PathBuf {
        let base_name = dir.join("untitled.ruqa");
        if !base_name.exists() {
            return base_name;
        }

        let mut index = 2;
        loop {
            let candidate = dir.join(format!("untitled-{index}.ruqa"));
            if !candidate.exists() {
                return candidate;
            }
            index += 1;
        }
    }

    /// Saves the active document to its existing file path, or chooses the next available
    /// non-conflicting untitled name in the Study Vault or working directory if untitled.
    pub fn save(&mut self) -> Result<PathBuf, DocumentError> {
        let path = match self.file_path.clone() {
            Some(p) => p,
            None => {
                let target_dir = if self.vault.root.is_dir() {
                    &self.vault.root
                } else {
                    Path::new(".")
                };
                Self::next_untitled_path(target_dir)
            }
        };
        self.save_as(&path)?;
        Ok(path)
    }

    /// Saves the active document to the specified file path and resets the dirty flag.
    pub fn save_as(&mut self, path: impl AsRef<Path>) -> Result<(), DocumentError> {
        let path = path.as_ref();
        let doc = RuqaDocument::new(&self.scene, self.viewport);
        doc.save_to_file(path)?;
        self.file_path = Some(path.to_path_buf());
        self.is_dirty = false;
        self.last_dirty_time = None;
        self.update_window_title();
        self.record_recent_file(path);
        self.refresh_vault_entries();
        info!(path = ?path, "saved whiteboard document");
        Ok(())
    }

    /// Triggers an atomic autosave to disk if the document is dirty and has an assigned file path.
    ///
    /// Returns `Ok(true)` if autosave was performed, or `Ok(false)` if clean or untitled.
    pub fn autosave(&mut self) -> Result<bool, DocumentError> {
        if self.is_dirty {
            if let Some(path) = self.file_path.clone() {
                let doc = RuqaDocument::new(&self.scene, self.viewport);
                doc.save_to_file(&path)?;
                self.is_dirty = false;
                self.last_dirty_time = None;
                self.update_window_title();
                info!(path = ?path, "atomic autosave succeeded");
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Checks whether the autosave inactivity timeout has elapsed and triggers autosave if needed.
    ///
    /// Returns `Ok(true)` if an autosave occurred, or `Ok(false)` otherwise.
    pub fn maybe_autosave(&mut self, now: std::time::Instant) -> Result<bool, DocumentError> {
        if self.is_dirty && self.file_path.is_some() {
            if let Some(dirty_time) = self.last_dirty_time {
                if now.duration_since(dirty_time) >= self.autosave_interval {
                    return self.autosave();
                }
            }
        }
        Ok(false)
    }

    /// Computes the next scheduled deadline when an autosave should fire, if currently dirty and titled.
    #[must_use]
    pub fn next_autosave_deadline(&self) -> Option<std::time::Instant> {
        if self.is_dirty && self.file_path.is_some() {
            self.last_dirty_time.map(|t| t + self.autosave_interval)
        } else {
            None
        }
    }

    /// Opens a document file from disk, replacing the current scene and viewport.
    pub fn open_file(&mut self, path: impl AsRef<Path>) -> Result<(), DocumentError> {
        let path = path.as_ref();
        let doc = RuqaDocument::load_from_file(path)?;
        let (scene, viewport) = doc.into_scene_and_viewport();

        self.scene = scene;
        self.viewport = viewport;
        self.selection.clear();
        self.history.clear();
        self.file_path = Some(path.to_path_buf());
        self.is_dirty = false;
        self.last_dirty_time = None;
        self.update_window_title();
        self.record_recent_file(path);
        self.request_redraw();
        info!(path = ?path, "opened whiteboard document");
        Ok(())
    }

    /// Resets the canvas to a fresh untitled document.
    pub fn new_document(&mut self) {
        self.scene = ruqa_core::scene::Scene::new();
        self.viewport = ruqa_core::geometry::Viewport::default();
        self.selection.clear();
        self.history.clear();
        self.file_path = None;
        self.is_dirty = false;
        self.last_dirty_time = None;
        self.update_window_title();
        self.request_redraw();
    }
}
