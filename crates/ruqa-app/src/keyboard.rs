//! Keyboard input processing and shortcut dispatch for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

impl WhiteboardApp {
    /// Handles key state changes directly by logical key and element state.
    pub fn handle_key(&mut self, key: &Key, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        if matches!(key, Key::Named(NamedKey::Space)) {
            if self.is_editing_text {
                self.pointer.space_held = false;
            } else {
                self.pointer.space_held = pressed;
            }
        }
        if matches!(key, Key::Named(NamedKey::Shift)) {
            self.pointer.shift_held = pressed;
        }
        if matches!(key, Key::Named(NamedKey::Control | NamedKey::Super)) {
            self.pointer.ctrl_held = pressed;
        }

        if pressed {
            if self.show_quit_dialog && self.handle_quit_dialog_key(key) {
                return;
            }
            if self.is_editing_text {
                match key {
                    Key::Named(NamedKey::Escape) => {
                        self.commit_text_edit();
                        self.request_redraw();
                        return;
                    }
                    Key::Named(NamedKey::Enter) => {
                        if self.pointer.shift_held {
                            self.text_edit_buffer.push('\n');
                        } else {
                            self.commit_text_edit();
                        }
                        self.request_redraw();
                        return;
                    }
                    Key::Named(NamedKey::Backspace) => {
                        self.text_edit_buffer.pop();
                        self.request_redraw();
                        return;
                    }
                    Key::Named(NamedKey::Space) => {
                        self.text_edit_buffer.push(' ');
                        self.request_redraw();
                        return;
                    }
                    Key::Character(c) => {
                        self.text_edit_buffer.push_str(c);
                        self.request_redraw();
                        return;
                    }
                    _ => return,
                }
            }
            if matches!(key, Key::Named(NamedKey::Escape)) {
                if self.show_welcome_hud {
                    self.show_welcome_hud = false;
                    self.request_redraw();
                    return;
                }
                if self.show_vault_sidebar {
                    self.show_vault_sidebar = false;
                    self.request_redraw();
                    return;
                }
            }
            if matches!(key, Key::Named(NamedKey::Enter)) && self.show_welcome_hud {
                if let Some(path) = self.hovered_recent_file.clone() {
                    let _ = self.open_recent_file(&path);
                } else {
                    self.show_welcome_hud = false;
                    self.request_redraw();
                }
                return;
            }
            if matches!(key, Key::Named(NamedKey::Delete | NamedKey::Backspace)) {
                self.delete_selected();
            } else if self.pointer.ctrl_held {
                match key {
                    Key::Character(c) if c.eq_ignore_ascii_case("b") => {
                        self.toggle_vault_sidebar();
                    }
                    Key::Character(c) if c.eq_ignore_ascii_case("o") => {
                        self.show_welcome_hud = !self.show_welcome_hud;
                        self.request_redraw();
                    }
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
                    Key::Character(c) if c.eq_ignore_ascii_case("e") => {
                        let _ = self.export_high_res_png_default();
                    }
                    Key::Character(c)
                        if c.eq_ignore_ascii_case("t") || c.eq_ignore_ascii_case("i") =>
                    {
                        self.toggle_theme();
                    }
                    Key::Character(c)
                        if c.eq_ignore_ascii_case("q") || c.eq_ignore_ascii_case("w") =>
                    {
                        self.request_close();
                    }
                    Key::Character(c) if c == "'" || c == "\"" => {
                        self.toggle_grid();
                    }
                    _ => {}
                }
            } else if !self.is_editing_text {
                if let Key::Character(c) = key {
                    if let Some(first_char) = c.chars().next() {
                        if let Some(tool) = ruqa_core::tool::Tool::from_shortcut(first_char) {
                            self.set_active_tool(tool);
                        }
                    }
                }
            }
        }
    }

    /// Handles keyboard events (e.g. Spacebar held for panning, Shift for multi-selection, Ctrl+Z/Y for undo/redo).
    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        self.handle_key(&event.logical_key, event.state);
    }
}
