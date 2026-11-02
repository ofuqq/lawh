//! Window application harness implementing `winit::application::ApplicationHandler`.
//!
//! This module owns the [`WhiteboardApp`] state machine that responds to winit
//! events. Frame presentation is delegated to [`Presenter`], which owns the
//! softbuffer surface and the tiny-skia pixmap.

use crate::event_dispatch::{PointerState, ViewportCamera};
use crate::presenter::Presenter;
use std::sync::Arc;
use tracing::{debug, info, warn};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

/// Default logical window width in CSS pixels.
const DEFAULT_WIDTH: u32 = 1280;
/// Default logical window height in CSS pixels.
const DEFAULT_HEIGHT: u32 = 800;
/// Minimum logical window width in CSS pixels.
const MIN_WIDTH: u32 = 640;
/// Minimum logical window height in CSS pixels.
const MIN_HEIGHT: u32 = 480;

/// Desktop whiteboard application — the top-level winit [`ApplicationHandler`].
///
/// Owns window state, the frame presenter, pointer state, and viewport camera.
/// For headless/smoke-test runs see the `--smoke-test` flag in `main.rs`.
pub struct WhiteboardApp {
    /// Window handle; set in [`resumed`](ApplicationHandler::resumed).
    pub window: Option<Arc<Window>>,
    /// Frame presenter (softbuffer surface + tiny-skia pixmap).
    pub presenter: Option<Presenter>,
    /// Mouse pointer tracking (zero-allocation).
    pub pointer: PointerState,
    /// Viewport camera (pan + zoom).
    pub camera: ViewportCamera,
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
            camera: ViewportCamera::new(),
            physical_size: PhysicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            scale_factor: 1.0,
            frames_presented: 0,
            motion_events_count: 0,
        }
    }
}

impl ApplicationHandler for WhiteboardApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let mut attrs = WindowAttributes::default()
            .with_title("Ruqa")
            .with_inner_size(LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT))
            .with_min_inner_size(LogicalSize::new(MIN_WIDTH, MIN_HEIGHT));

        if let Some(monitor) = event_loop
            .primary_monitor()
            .or_else(|| event_loop.available_monitors().next())
        {
            let monitor_size = monitor.size();
            let scale = monitor.scale_factor();
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let win_w = (f64::from(DEFAULT_WIDTH) * scale).round() as u32;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let win_h = (f64::from(DEFAULT_HEIGHT) * scale).round() as u32;
            let x = monitor_size.width.saturating_sub(win_w) / 2;
            let y = monitor_size.height.saturating_sub(win_h) / 2;
            attrs = attrs.with_position(PhysicalPosition::new(x, y));
        }

        match event_loop.create_window(attrs) {
            Ok(w) => {
                let window = Arc::new(w);
                self.scale_factor = window.scale_factor();
                self.physical_size = window.inner_size();

                match Presenter::new(window.clone()) {
                    Some(presenter) => {
                        info!(
                            scale_factor = self.scale_factor,
                            width = self.physical_size.width,
                            height = self.physical_size.height,
                            "window and presenter ready"
                        );
                        self.presenter = Some(presenter);
                        self.window = Some(window);
                    }
                    None => {
                        warn!("softbuffer presenter unavailable; window will not render");
                        self.window = Some(window);
                    }
                }
            }
            Err(e) => {
                warn!(?e, "failed to create window");
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            // ── Lifecycle ────────────────────────────────────────────────────
            WindowEvent::CloseRequested => {
                info!(frames = self.frames_presented, "close requested; exiting");
                event_loop.exit();
            }

            // ── Resize & HiDPI ───────────────────────────────────────────────
            WindowEvent::Resized(new_size) => {
                debug!(width = new_size.width, height = new_size.height, "resized");
                self.physical_size = new_size;
                if let Some(p) = &mut self.presenter {
                    if let Err(e) = p.resize(new_size.width, new_size.height) {
                        warn!(?e, "presenter resize failed");
                    }
                }
                self.request_redraw();
            }

            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                debug!(scale_factor, "scale factor changed");
                self.scale_factor = scale_factor;
                if let Some(w) = &self.window {
                    let new_size = w.inner_size();
                    if new_size != self.physical_size {
                        self.physical_size = new_size;
                        if let Some(p) = &mut self.presenter {
                            if let Err(e) = p.resize(new_size.width, new_size.height) {
                                warn!(?e, "presenter resize on scale factor change failed");
                            }
                        }
                    }
                }
                self.request_redraw();
            }

            // ── Pointer ──────────────────────────────────────────────────────
            WindowEvent::CursorMoved { position, .. } => {
                self.motion_events_count += 1;
                let logical_pos = position.to_logical::<f64>(self.scale_factor);
                if let Some((dx, dy)) = self.pointer.update_cursor(logical_pos.x, logical_pos.y) {
                    if self.pointer.is_panning() {
                        self.camera.pan(dx, dy);
                    }
                }
                self.request_redraw();
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.pointer.left_pressed = pressed,
                    MouseButton::Middle => self.pointer.middle_pressed = pressed,
                    _ => {}
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let factor = scroll_zoom_factor(delta);
                if let Some((cx, cy)) = self.pointer.cursor_pos {
                    self.camera.zoom_at(cx, cy, factor);
                }
                self.request_redraw();
            }

            // ── Rendering ────────────────────────────────────────────────────
            WindowEvent::RedrawRequested => {
                if let Some(presenter) = &mut self.presenter {
                    presenter.clear_dark();
                    if let Err(e) = presenter.present() {
                        warn!(?e, "frame present failed");
                    } else {
                        self.frames_presented += 1;
                    }
                }
            }

            _ => {}
        }
    }
}

impl WhiteboardApp {
    /// Asks the window to schedule a redraw, if the window exists.
    fn request_redraw(&self) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}

/// Maps a mouse-scroll delta to a zoom factor.
fn scroll_zoom_factor(delta: MouseScrollDelta) -> f64 {
    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            if y > 0.0 {
                1.15
            } else if y < 0.0 {
                1.0 / 1.15
            } else {
                1.0
            }
        }
        MouseScrollDelta::PixelDelta(pos) => {
            if pos.y > 0.0 {
                1.05
            } else if pos.y < 0.0 {
                1.0 / 1.05
            } else {
                1.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::MouseScrollDelta;

    #[test]
    fn scroll_zoom_factor_line_delta_up() {
        let f = scroll_zoom_factor(MouseScrollDelta::LineDelta(0.0, 1.0));
        assert!((f - 1.15).abs() < 1e-9);
    }

    #[test]
    fn scroll_zoom_factor_line_delta_down() {
        let f = scroll_zoom_factor(MouseScrollDelta::LineDelta(0.0, -1.0));
        assert!((f - 1.0 / 1.15).abs() < 1e-9);
    }

    #[test]
    fn scroll_zoom_factor_zero_is_identity() {
        let f = scroll_zoom_factor(MouseScrollDelta::LineDelta(0.0, 0.0));
        assert_eq!(f, 1.0);
    }

    #[test]
    fn scroll_zoom_factor_pixel_delta() {
        let f_up = scroll_zoom_factor(MouseScrollDelta::PixelDelta(
            winit::dpi::PhysicalPosition::new(0.0, 15.0),
        ));
        assert!((f_up - 1.05).abs() < 1e-9);

        let f_down = scroll_zoom_factor(MouseScrollDelta::PixelDelta(
            winit::dpi::PhysicalPosition::new(0.0, -15.0),
        ));
        assert!((f_down - 1.0 / 1.05).abs() < 1e-9);

        let f_zero = scroll_zoom_factor(MouseScrollDelta::PixelDelta(
            winit::dpi::PhysicalPosition::new(0.0, 0.0),
        ));
        assert_eq!(f_zero, 1.0);
    }

    #[test]
    fn whiteboard_app_initial_state() {
        let app = WhiteboardApp::new();
        assert!(app.window.is_none());
        assert!(app.presenter.is_none());
        assert_eq!(app.frames_presented, 0);
        assert_eq!(app.scale_factor, 1.0);
        assert_eq!(app.physical_size.width, DEFAULT_WIDTH);
        assert_eq!(app.physical_size.height, DEFAULT_HEIGHT);
    }

    #[test]
    fn window_dimensions_constraints() {
        const { assert!(DEFAULT_WIDTH >= MIN_WIDTH) };
        const { assert!(DEFAULT_HEIGHT >= MIN_HEIGHT) };
        assert_eq!(DEFAULT_WIDTH, 1280);
        assert_eq!(DEFAULT_HEIGHT, 800);
        assert_eq!(MIN_WIDTH, 640);
        assert_eq!(MIN_HEIGHT, 480);
    }
}
