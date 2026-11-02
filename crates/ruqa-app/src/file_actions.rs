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
        if !self.is_dirty {
            self.is_dirty = true;
            self.update_window_title();
        }
    }

    /// Marks the active document as saved and updates window title.
    pub fn mark_clean(&mut self) {
        if self.is_dirty {
            self.is_dirty = false;
            self.update_window_title();
        }
    }

    /// Saves the active document to its existing file path, or defaults to `"untitled.ruqa"` if untitled.
    pub fn save(&mut self) -> Result<PathBuf, DocumentError> {
        let path = self
            .file_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("untitled.ruqa"));
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
        self.update_window_title();
        info!(path = ?path, "saved whiteboard document");
        Ok(())
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
        self.update_window_title();
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
        self.update_window_title();
        self.request_redraw();
    }
}
