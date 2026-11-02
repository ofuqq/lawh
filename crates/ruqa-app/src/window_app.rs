//! Window application harness implementing `winit::application::ApplicationHandler`.
//!
//! This module owns the [`WhiteboardApp`] state machine that responds to winit
//! events. Frame presentation is delegated to [`Presenter`], which owns the
//! softbuffer surface and the tiny-skia pixmap.

use crate::event_dispatch::{scroll_zoom_factor, PointerState};
use crate::presenter::Presenter;
use crate::selection::SelectionManager;
use ruqa_core::geometry::{Point, Size, Viewport};
use ruqa_core::scene::Scene;
use std::sync::Arc;
use tiny_skia::Transform;
use tracing::warn;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::window::Window;

/// Default logical window width in CSS pixels.
pub const DEFAULT_WIDTH: u32 = 1280;
/// Default logical window height in CSS pixels.
pub const DEFAULT_HEIGHT: u32 = 800;
/// Minimum logical window width in CSS pixels.
pub const MIN_WIDTH: u32 = 640;
/// Minimum logical window height in CSS pixels.
pub const MIN_HEIGHT: u32 = 480;
/// Default inactivity duration before autosave triggers for dirty documents.
pub const DEFAULT_AUTOSAVE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);

/// Desktop whiteboard application — the top-level winit [`ApplicationHandler`].
///
/// Owns window state, the frame presenter, pointer state, and the canvas viewport.
pub struct WhiteboardApp {
    /// Window handle; set in [`resumed`](ApplicationHandler::resumed).
    pub window: Option<Arc<Window>>,
    /// Frame presenter (softbuffer surface + tiny-skia pixmap).
    pub presenter: Option<Presenter>,
    /// Mouse pointer tracking (zero-allocation).
    pub pointer: PointerState,
    /// Infinite canvas viewport camera.
    pub viewport: Viewport,
    /// Active scene document graph.
    pub scene: Scene,
    /// Selection tool controller and state.
    pub selection: SelectionManager,
    /// Internal clipboard of copied elements.
    pub clipboard: Vec<ruqa_core::element::Element>,
    /// Undo/redo history stack using the command pattern.
    pub history: ruqa_core::history::HistoryStack,
    /// Active document file path on disk, or `None` if untitled.
    pub file_path: Option<std::path::PathBuf>,
    /// Whether the canvas has unsaved modifications.
    pub is_dirty: bool,
    /// Inactivity interval after which an autosave triggers.
    pub autosave_interval: std::time::Duration,
    /// Timestamp of the most recent modification while dirty.
    pub last_dirty_time: Option<std::time::Instant>,
    /// Active tool mode selected in the toolbar HUD.
    pub active_tool: ruqa_core::tool::Tool,
    /// Currently hovered toolbar button, if cursor is over the toolbar.
    pub hovered_tool: Option<ruqa_core::tool::Tool>,
    /// Whether the pointer was pressed inside the toolbar HUD.
    pub toolbar_pressed: bool,
    /// Current physical window size.
    pub physical_size: PhysicalSize<u32>,
    /// HiDPI scale factor (e.g. 1.0 on 96 DPI, 2.0 on Retina).
    pub scale_factor: f64,
    /// Total frames presented.
    pub frames_presented: u64,
    /// Total pointer-motion events received.
    pub motion_events_count: u64,
}

impl Default for WhiteboardApp {
    fn default() -> Self {
        Self::new()
    }
}

impl WhiteboardApp {
    /// Creates initial application state (no window yet).
    #[must_use]
    pub fn new() -> Self {
        Self {
            window: None,
            presenter: None,
            pointer: PointerState::new(),
            viewport: Viewport::default(),
            scene: Scene::new(),
            selection: SelectionManager::new(),
            clipboard: Vec::new(),
            history: ruqa_core::history::HistoryStack::new(),
            file_path: None,
            is_dirty: false,
            autosave_interval: DEFAULT_AUTOSAVE_INTERVAL,
            last_dirty_time: None,
            active_tool: ruqa_core::tool::Tool::default(),
            hovered_tool: None,
            toolbar_pressed: false,
            physical_size: PhysicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            scale_factor: 1.0,
            frames_presented: 0,
            motion_events_count: 0,
        }
    }

    /// Returns the current logical window size in pixels.
    #[must_use]
    pub fn logical_size(&self) -> Size {
        #[allow(clippy::cast_possible_truncation)]
        Size::new(
            (f64::from(self.physical_size.width) / self.scale_factor) as f32,
            (f64::from(self.physical_size.height) / self.scale_factor) as f32,
        )
    }

    /// Handles mouse motion in physical window coordinates.
    pub fn handle_cursor_moved(&mut self, physical_x: f64, physical_y: f64) {
        self.motion_events_count += 1;
        let logical_x = physical_x / self.scale_factor;
        let logical_y = physical_y / self.scale_factor;
        #[allow(clippy::cast_possible_truncation)]
        let screen_pt = Point::new(logical_x as f32, logical_y as f32);
        self.handle_toolbar_cursor_moved(screen_pt);

        if self.toolbar_pressed {
            return;
        }

        if let Some((dx, dy)) = self.pointer.update_cursor(logical_x, logical_y) {
            if self.pointer.is_panning() {
                self.pointer.panned_during_press = true;
                #[allow(clippy::cast_possible_truncation)]
                let world_dx = -(dx as f32) / self.viewport.zoom;
                #[allow(clippy::cast_possible_truncation)]
                let world_dy = -(dy as f32) / self.viewport.zoom;
                self.viewport.pan_by(Point::new(world_dx, world_dy));
            } else if self.pointer.left_pressed {
                #[allow(clippy::cast_possible_truncation)]
                let world_pt = self.viewport.screen_to_world(
                    Point::new(logical_x as f32, logical_y as f32),
                    self.logical_size(),
                );
                self.selection.handle_pointer_move(
                    &mut self.scene,
                    world_pt,
                    self.pointer.shift_held,
                );
            }
        }
    }

    /// Handles mouse button input state changes.
    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let pressed = state == ElementState::Pressed;
        match button {
            MouseButton::Left => {
                self.pointer.left_pressed = pressed;
                if pressed {
                    self.pointer.panned_during_press = false;
                    if let Some((cx, cy)) = self.pointer.cursor_pos {
                        #[allow(clippy::cast_possible_truncation)]
                        let screen_pt = Point::new(cx as f32, cy as f32);
                        if self.handle_toolbar_pointer_down(screen_pt) {
                            return;
                        }
                    }
                    if !self.pointer.is_panning() {
                        if let Some((cx, cy)) = self.pointer.cursor_pos {
                            #[allow(clippy::cast_possible_truncation)]
                            let screen_pt = Point::new(cx as f32, cy as f32);
                            let logical_size = self.logical_size();
                            let world_pt = self.viewport.screen_to_world(screen_pt, logical_size);
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
                    if self.handle_toolbar_pointer_up() {
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
        }
    }

    /// Asks the window to schedule a redraw, if the window exists.
    pub fn request_redraw(&self) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    /// Renders the scene and interactive overlays into the presenter surface.
    pub fn render_frame(&mut self) {
        let logical_size = self.logical_size();
        let center = logical_size.center();
        let zoom = self.viewport.zoom;
        let pan = self.viewport.pan;
        let active_marquee = self.selection.active_marquee();
        let selection_bounds = self.scene.selection_bounds(self.selection.selected_ids());
        let toolbar_layout = ruqa_render::toolbar_render::ToolbarLayout::new(logical_size.width);

        let Some(presenter) = &mut self.presenter else {
            return;
        };

        presenter.clear_dark();
        ruqa_render::shape_render::render_scene_viewport(
            &self.scene,
            &self.viewport,
            logical_size,
            &mut presenter.pixmap_mut().as_mut(),
        );

        if let Some(bounds) = selection_bounds {
            ruqa_render::selection_render::render_selection(
                bounds,
                &self.viewport,
                logical_size,
                &mut presenter.pixmap_mut().as_mut(),
                &ruqa_render::selection_render::SelectionStyle::default(),
            );
        }

        if let Some(marquee) = active_marquee {
            let transform = Transform::from_row(
                zoom,
                0.0,
                0.0,
                zoom,
                center.x - pan.x * zoom,
                center.y - pan.y * zoom,
            );
            ruqa_render::shape_render::render_marquee(
                marquee.rect(),
                &mut presenter.pixmap_mut().as_mut(),
                transform,
            );
        }

        ruqa_render::toolbar_render::render_toolbar(
            &mut presenter.pixmap_mut().as_mut(),
            &toolbar_layout,
            self.active_tool,
            self.hovered_tool,
            self.scale_factor,
            &ruqa_render::toolbar_render::ToolbarStyle::default(),
        );

        if let Err(e) = presenter.present() {
            warn!(?e, "frame present failed");
        } else {
            self.frames_presented += 1;
        }
    }
}
