//! Pointer and mouse input dispatch implementation for [`WhiteboardApp`].

use crate::event_dispatch::scroll_zoom_factor;
use crate::window_app::WhiteboardApp;
use ruqa_core::geometry::Point;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};

impl WhiteboardApp {
    /// Handles mouse motion in physical window coordinates.
    pub fn handle_cursor_moved(&mut self, physical_x: f64, physical_y: f64) -> bool {
        self.motion_events_count += 1;
        let logical_x = physical_x / self.scale_factor;
        let logical_y = physical_y / self.scale_factor;
        #[allow(clippy::cast_possible_truncation)]
        let screen_pt = Point::new(logical_x as f32, logical_y as f32);

        let prev_quit = self.hovered_quit_action;
        let prev_recent = self.hovered_recent_file.clone();
        let prev_vault = self.hovered_vault_item.clone();
        let prev_tool = self.hovered_tool;

        self.handle_quit_dialog_cursor_moved(screen_pt);
        self.handle_recent_files_cursor_moved(screen_pt);
        self.handle_vault_cursor_moved(screen_pt);
        self.handle_toolbar_cursor_moved(screen_pt);

        let hover_changed = self.hovered_quit_action != prev_quit
            || self.hovered_recent_file != prev_recent
            || self.hovered_vault_item != prev_vault
            || self.hovered_tool != prev_tool;

        // Unconditionally update cursor position so pointer tracking is never stale or None
        let delta = self.pointer.update_cursor(logical_x, logical_y);

        if self.show_quit_dialog
            || self.show_welcome_hud
            || (self.show_vault_sidebar && self.vault_layout().contains(screen_pt))
            || self.toolbar_pressed
            || self.style_panel_pressed
        {
            return hover_changed;
        }

        let mut canvas_changed = false;
        if let Some((dx, dy)) = delta {
            if self.pointer.is_panning() {
                self.pointer.panned_during_press = true;
                #[allow(clippy::cast_possible_truncation)]
                let world_dx = -(dx as f32) / self.viewport.zoom;
                #[allow(clippy::cast_possible_truncation)]
                let world_dy = -(dy as f32) / self.viewport.zoom;
                self.viewport.pan_by(Point::new(world_dx, world_dy));
                canvas_changed = true;
            } else if self.pointer.left_pressed {
                #[allow(clippy::cast_possible_truncation)]
                let world_pt = self.viewport.screen_to_world(
                    Point::new(logical_x as f32, logical_y as f32),
                    self.logical_size(),
                );
                if self.active_tool == ruqa_core::tool::Tool::Eraser {
                    self.handle_eraser_pointer_move(world_pt);
                } else {
                    self.selection.handle_pointer_move(
                        &mut self.scene,
                        world_pt,
                        self.pointer.shift_held,
                    );
                }
                canvas_changed = true;
            }
        }
        hover_changed || canvas_changed
    }

    /// Handles mouse button input state changes.
    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let pressed = state == ElementState::Pressed;
        match button {
            MouseButton::Left => {
                self.pointer.left_pressed = pressed;
                if pressed {
                    self.pointer.panned_during_press = false;
                    let Some((cx, cy)) = self.pointer.cursor_pos else {
                        return;
                    };
                    #[allow(clippy::cast_possible_truncation)]
                    let screen_pt = Point::new(cx as f32, cy as f32);

                    // 1. Modals have top priority
                    if self.show_quit_dialog && self.handle_quit_dialog_pointer_down(screen_pt) {
                        return;
                    }
                    if self.show_welcome_hud && self.handle_recent_files_pointer_down(screen_pt) {
                        return;
                    }

                    // 2. Toolbar HUD (top center)
                    if !self.pointer.is_panning() && self.handle_toolbar_pointer_down(screen_pt) {
                        return;
                    }

                    // 3. Drawer Sidebar (Study Vault - rendered in front of left HUD)
                    if self.show_vault_sidebar && self.handle_vault_pointer_down(screen_pt) {
                        return;
                    }

                    // 4. Style Panel HUD (underneath sidebar when sidebar is open)
                    if !self.pointer.is_panning()
                        && self.show_style_panel
                        && self.handle_style_panel_pointer_down(screen_pt)
                    {
                        return;
                    }

                    // 5. Canvas interactions
                    if !self.pointer.is_panning() {
                        let logical_size = self.logical_size();
                        let world_pt = self.viewport.screen_to_world(screen_pt, logical_size);
                        if self.active_tool == ruqa_core::tool::Tool::Eraser {
                            self.handle_eraser_pointer_down(world_pt);
                        } else if self.active_tool == ruqa_core::tool::Tool::Text {
                            self.handle_text_tool_pointer_down(world_pt);
                        } else {
                            if self.is_editing_text {
                                self.commit_text_edit();
                            }
                            self.selection.handle_pointer_down(
                                &self.scene,
                                world_pt,
                                screen_pt,
                                &self.viewport,
                                logical_size,
                                self.pointer.shift_held,
                                ruqa_core::hit_test::DEFAULT_HIT_TOLERANCE,
                            );
                        }
                    }
                } else {
                    let mut consumed = self.handle_toolbar_pointer_up();
                    consumed |= self.handle_style_panel_pointer_up();
                    if consumed {
                        return;
                    }
                    if self.active_tool == ruqa_core::tool::Tool::Eraser {
                        self.handle_eraser_pointer_up();
                        return;
                    }
                    if !self.pointer.panned_during_press {
                        if let Some(cmd) = self
                            .selection
                            .handle_pointer_up(&self.scene, self.pointer.shift_held)
                        {
                            self.history.push(cmd);
                            self.mark_dirty();
                        }
                        self.sync_style_from_selection();
                    }
                }
            }
            MouseButton::Middle => {
                self.pointer.middle_pressed = pressed;
                if pressed {
                    self.pointer.panned_during_press = false;
                }
            }
            _ => {}
        }
    }

    /// Handles mouse wheel scroll events for anchored zooming.
    pub fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        let factor = scroll_zoom_factor(delta);
        if let Some((cx, cy)) = self.pointer.cursor_pos {
            let logical_size = self.logical_size();
            #[allow(clippy::cast_possible_truncation)]
            self.viewport.zoom_at(
                Point::new(cx as f32, cy as f32),
                factor as f32,
                logical_size,
            );
        }
    }

    /// Handles window focus changes (clearing held inputs on focus loss).
    pub fn handle_focused(&mut self, focused: bool) {
        if !focused {
            self.pointer.reset_inputs();
            self.selection.active_drag = None;
            self.eraser.reset();
        }
    }
}
