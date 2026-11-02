//! Recent files tracking, modal HUD interaction, and document loading for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::error::DocumentError;
use ruqa_core::geometry::Point;
use ruqa_render::recent_files_layout::{RecentFilesAction, RecentFilesLayout};
use std::path::Path;

impl WhiteboardApp {
    /// Loads recent files from the standard system configuration path and displays welcome HUD if not empty.
    pub fn load_recent_files(&mut self) {
        self.recent_files = ruqa_core::recent_files::RecentFiles::load();
        self.show_welcome_hud = !self.recent_files.is_empty();
    }

    /// Computes the screen layout for the Recent Files / Welcome HUD modal overlay.
    #[must_use]
    pub fn recent_files_layout(&self) -> RecentFilesLayout {
        let size = self.logical_size();
        RecentFilesLayout::new(size.width, size.height, &self.recent_files.entries)
    }

    /// Records an opened or saved file path in recent files history and persists it to disk.
    pub fn record_recent_file(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        self.recent_files.add(path);
        if let Err(e) = self.recent_files.save() {
            tracing::warn!(?e, "failed to persist recent files configuration");
        }
    }

    /// Opens a recent document from disk, adds it to recent files, and hides the welcome HUD.
    pub fn open_recent_file(&mut self, path: impl AsRef<Path>) -> Result<(), DocumentError> {
        let path = path.as_ref();
        self.open_file(path)?;
        self.record_recent_file(path);
        self.show_welcome_hud = false;
        self.request_redraw();
        Ok(())
    }

    /// Handles pointer down events while the Recent Files HUD is visible.
    ///
    /// Returns `true` if the pointer event was consumed by the HUD overlay.
    pub fn handle_recent_files_pointer_down(&mut self, screen_pt: Point) -> bool {
        if !self.show_welcome_hud {
            return false;
        }

        let layout = self.recent_files_layout();
        match layout.action_at(screen_pt) {
            Some(RecentFilesAction::OpenFile(path)) => {
                if let Err(err) = self.open_recent_file(&path) {
                    tracing::warn!(?err, path = ?path, "could not open recent file; removing from list");
                    self.recent_files.remove(&path);
                    let _ = self.recent_files.save();
                }
                true
            }
            Some(RecentFilesAction::Dismiss) => {
                self.show_welcome_hud = false;
                self.request_redraw();
                true
            }
            Some(RecentFilesAction::NewBoard) => {
                self.new_document();
                self.show_welcome_hud = false;
                self.request_redraw();
                true
            }
            None => {
                // Clicked inside modal dialog padding: consume click to isolate canvas
                layout.contains(screen_pt)
            }
        }
    }

    /// Updates hovered recent file item state during cursor movement.
    pub fn handle_recent_files_cursor_moved(&mut self, screen_pt: Point) {
        if !self.show_welcome_hud {
            return;
        }

        let layout = self.recent_files_layout();
        let new_hover = layout
            .item_bounds
            .iter()
            .find(|(_, bounds)| bounds.contains(screen_pt))
            .map(|(path, _)| path.clone());

        if self.hovered_recent_file != new_hover {
            self.hovered_recent_file = new_hover;
            self.request_redraw();
        }
    }
}
