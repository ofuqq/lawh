//! Study Vault drawer sidebar actions, board navigation, and interaction for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::geometry::Point;
use ruqa_render::vault_layout::{VaultAction, VaultLayout};
use std::path::{Path, PathBuf};
use tracing::warn;

impl WhiteboardApp {
    /// Toggles the visibility of the Study Vault drawer sidebar and refreshes file entries.
    pub fn toggle_vault_sidebar(&mut self) {
        self.show_vault_sidebar = !self.show_vault_sidebar;
        if self.show_vault_sidebar {
            self.refresh_vault_entries();
        }
        self.request_redraw();
    }

    /// Sets a new root directory for the Study Vault.
    pub fn set_vault_root(&mut self, root: impl Into<PathBuf>) {
        self.vault.root = root.into();
        self.refresh_vault_entries();
        self.request_redraw();
    }

    /// Refreshes the list of visible vault entries from disk.
    pub fn refresh_vault_entries(&mut self) {
        match self.vault.list_entries() {
            Ok(entries) => {
                self.vault_entries = entries;
            }
            Err(e) => {
                warn!(?e, path = ?self.vault.root, "could not read Study Vault directory");
                self.vault_entries.clear();
            }
        }
    }

    /// Computes the screen layout for the Study Vault drawer sidebar.
    #[must_use]
    pub fn vault_layout(&self) -> VaultLayout {
        let size = self.logical_size();
        VaultLayout::new(size.height, &self.vault_entries)
    }

    /// Handles pointer down events while the Study Vault drawer sidebar is visible.
    ///
    /// Returns `true` if the event was consumed by the sidebar.
    pub fn handle_vault_pointer_down(&mut self, screen_pt: Point) -> bool {
        if !self.show_vault_sidebar {
            return false;
        }

        let layout = self.vault_layout();
        match layout.action_at(screen_pt) {
            Some(VaultAction::ToggleFolder(path)) => {
                self.vault.toggle_folder(&path);
                self.refresh_vault_entries();
                self.request_redraw();
                true
            }
            Some(VaultAction::OpenBoard(path)) => {
                self.open_vault_board(&path);
                true
            }
            Some(VaultAction::CollapseSidebar) => {
                self.show_vault_sidebar = false;
                self.request_redraw();
                true
            }
            Some(VaultAction::NewBoard) => {
                self.new_document();
                self.request_redraw();
                true
            }
            None => {
                // Click was inside drawer bounds (e.g. background padding): consume it
                layout.contains(screen_pt)
            }
        }
    }

    /// Opens a board from the Study Vault, saving any unsaved changes if needed.
    pub fn open_vault_board(&mut self, path: &Path) {
        // If current document has unsaved modifications and an assigned path, autosave first
        if self.is_dirty && self.file_path.is_some() {
            let _ = self.save();
        }

        if let Err(err) = self.open_file(path) {
            warn!(?err, path = ?path, "failed to open vault board");
        } else {
            self.record_recent_file(path);
            self.request_redraw();
        }
    }

    /// Updates hovered vault entry state during pointer motion.
    pub fn handle_vault_cursor_moved(&mut self, screen_pt: Point) {
        if !self.show_vault_sidebar {
            return;
        }

        let layout = self.vault_layout();
        let new_hover = layout
            .rows
            .iter()
            .find(|row| row.row_bounds.contains(screen_pt))
            .map(|row| row.item.path.clone());

        if self.hovered_vault_item != new_hover {
            self.hovered_vault_item = new_hover;
            self.request_redraw();
        }
    }
}
