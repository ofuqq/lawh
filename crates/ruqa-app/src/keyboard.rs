//! Keyboard input processing and shortcut dispatch for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

impl WhiteboardApp {
    /// Handles key state changes directly by logical key and element state.
    pub fn handle_key(&mut self, key: &Key, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        if matches!(key, Key::Named(NamedKey::Space)) {
            self.pointer.space_held = pressed;
        }
        if matches!(key, Key::Named(NamedKey::Shift)) {
            self.pointer.shift_held = pressed;
        }
        if matches!(key, Key::Named(NamedKey::Control | NamedKey::Super)) {
            self.pointer.ctrl_held = pressed;
        }

        if pressed {
            if matches!(key, Key::Named(NamedKey::Delete | NamedKey::Backspace)) {
                self.delete_selected();
            } else if self.pointer.ctrl_held {
                match key {
                    Key::Character(c) if c.eq_ignore_ascii_case("z") => {
                        let changed = if self.pointer.shift_held {
                            self.history.redo(&mut self.scene)
                        } else {
                            self.history.undo(&mut self.scene)
                        };
                        if changed {
                            self.mark_dirty();
                            self.request_redraw();
                        }
                    }
                    Key::Character(c) if c.eq_ignore_ascii_case("y") => {
                        let changed = self.history.redo(&mut self.scene);
                        if changed {
                            self.mark_dirty();
                            self.request_redraw();
                        }
                    }
                    Key::Character(c) if c.eq_ignore_ascii_case("d") => {
                        self.duplicate_selected();
                    }
                    Key::Character(c) if c.eq_ignore_ascii_case("c") => {
                        self.copy_selected();
                    }
                    Key::Character(c) if c.eq_ignore_ascii_case("v") => {
                        self.paste_clipboard();
                    }
                    Key::Character(c) if c.eq_ignore_ascii_case("s") => {
                        let _ = self.save();
                    }
                    _ => {}
                }
            }
        }
    }

    /// Handles keyboard events (e.g. Spacebar held for panning, Shift for multi-selection, Ctrl+Z/Y for undo/redo).
    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        self.handle_key(&event.logical_key, event.state);
    }
}
