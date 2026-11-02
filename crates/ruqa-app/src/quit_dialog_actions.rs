//! Actions, keyboard shortcuts, and state handling for the unsaved changes quit confirmation prompt.

use crate::window_app::WhiteboardApp;
use ruqa_core::geometry::Point;
use ruqa_render::quit_dialog_layout::{QuitDialogAction, QuitDialogLayout};
use tracing::{info, warn};
use winit::keyboard::{Key, NamedKey};

impl WhiteboardApp {
    /// Computes the layout geometry for the quit confirmation dialog.
    #[must_use]
    pub fn quit_dialog_layout(&self) -> QuitDialogLayout {
        let size = self.logical_size();
        QuitDialogLayout::new(size.width, size.height)
    }

    /// Handles close requests (e.g. from the OS window close button or Ctrl+Q/Ctrl+W).
    ///
    /// If unsaved modifications exist ([`is_dirty`](WhiteboardApp::is_dirty)),
    /// displays the quit confirmation dialog. If clean, signals the event loop to exit immediately.
    pub fn request_close(&mut self) {
        if self.is_dirty {
            info!("close requested with unsaved changes; prompting confirmation dialog");
            self.show_quit_dialog = true;
            self.request_redraw();
        } else {
            info!(
                frames = self.frames_presented,
                "close requested with clean document; exiting"
            );
            self.should_exit = true;
        }
    }

    /// Handles pointer motion over the quit confirmation dialog.
    pub fn handle_quit_dialog_cursor_moved(&mut self, screen_pt: Point) {
        if !self.show_quit_dialog {
            return;
        }
        let layout = self.quit_dialog_layout();
        let hit = layout.hit_test(screen_pt);
        if self.hovered_quit_action != hit {
            self.hovered_quit_action = hit;
            self.request_redraw();
        }
    }

    /// Handles pointer click down over the quit confirmation dialog.
    ///
    /// Returns `true` if the event was consumed by the dialog (preventing canvas interaction).
    pub fn handle_quit_dialog_pointer_down(&mut self, screen_pt: Point) -> bool {
        if !self.show_quit_dialog {
            return false;
        }

        let layout = self.quit_dialog_layout();
        if let Some(action) = layout.hit_test(screen_pt) {
            match action {
                QuitDialogAction::Save => {
                    self.confirm_quit_save();
                }
                QuitDialogAction::DontSave => {
                    self.confirm_quit_dont_save();
                }
                QuitDialogAction::Cancel => {
                    self.cancel_quit_dialog();
                }
            }
            return true;
        }

        // Click inside modal dialog body (outside buttons) is absorbed
        layout.contains(screen_pt)
    }

    /// Handles keyboard shortcuts while the quit confirmation dialog is active.
    ///
    /// - `Escape` -> Cancel and return to whiteboard
    /// - `Enter` / `Return` -> Save changes and exit
    /// - `D` / `d` -> Discard changes and exit
    ///
    /// Returns `true` if the key was consumed by the quit confirmation dialog.
    pub fn handle_quit_dialog_key(&mut self, key: &Key) -> bool {
        if !self.show_quit_dialog {
            return false;
        }

        match key {
            Key::Named(NamedKey::Escape) => {
                self.cancel_quit_dialog();
                true
            }
            Key::Named(NamedKey::Enter) => {
                self.confirm_quit_save();
                true
            }
            Key::Character(c) if c.eq_ignore_ascii_case("d") => {
                self.confirm_quit_dont_save();
                true
            }
            _ => true, // Absorb all other keystrokes while modal dialog is active
        }
    }

    /// Saves the active document and marks the application for immediate exit.
    pub fn confirm_quit_save(&mut self) {
        info!("saving document before quitting");
        match self.save() {
            Ok(path) => {
                info!(path = ?path, "saved document successfully; quitting application");
                self.show_quit_dialog = false;
                self.should_exit = true;
            }
            Err(err) => {
                warn!(?err, "failed to save document during quit confirmation");
            }
        }
    }

    /// Discards all unsaved modifications and exits the application immediately.
    pub fn confirm_quit_dont_save(&mut self) {
        info!("discarding unsaved changes; quitting application");
        self.show_quit_dialog = false;
        self.should_exit = true;
    }

    /// Cancels the quit confirmation dialog and returns to editing.
    pub fn cancel_quit_dialog(&mut self) {
        info!("cancelled quit confirmation dialog; returning to whiteboard");
        self.show_quit_dialog = false;
        self.should_exit = false;
        self.request_redraw();
    }
}
