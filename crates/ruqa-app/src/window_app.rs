//! Window application harness implementing `winit::application::ApplicationHandler`.

use crate::event_dispatch::{PointerState, ViewportCamera};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use tracing::{debug, info, warn};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

/// Desktop window application state.
pub struct WhiteboardApp {
    /// Window handle wrapped in Arc for softbuffer sharing.
    pub window: Option<Arc<Window>>,
    /// Softbuffer graphics context.
    pub context: Option<Context<Arc<Window>>>,
    /// Softbuffer presentation surface.
    pub surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    /// Mouse pointer tracking state.
    pub pointer: PointerState,
    /// Infinite canvas viewport camera.
    pub camera: ViewportCamera,
    /// Current window physical width and height.
    pub size: PhysicalSize<u32>,
    /// Total pointer motion events received.
    pub motion_events_count: u64,
    /// Total frames presented to the window.
    pub frames_presented: u64,
}

impl Default for WhiteboardApp {
    fn default() -> Self {
        Self::new()
    }
}

impl WhiteboardApp {
    /// Creates a new whiteboard application with initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            window: None,
            context: None,
            surface: None,
            pointer: PointerState::new(),
            camera: ViewportCamera::new(),
            size: PhysicalSize::new(1280, 720),
            motion_events_count: 0,
            frames_presented: 0,
        }
    }
}

impl ApplicationHandler for WhiteboardApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title("Ruqa Whiteboard (Spike 3)")
            .with_inner_size(self.size);

        if let Ok(w) = event_loop.create_window(attrs) {
            let window = Arc::new(w);
            if let Ok(context) = Context::new(window.clone()) {
                if let Ok(surface) = Surface::new(&context, window.clone()) {
                    info!(
                        width = self.size.width,
                        height = self.size.height,
                        "window created"
                    );
                    self.context = Some(context);
                    self.surface = Some(surface);
                    self.window = Some(window);
                } else {
                    warn!("failed to create softbuffer surface");
                }
            } else {
                warn!("failed to create softbuffer context");
            }
        } else {
            warn!("failed to create winit window");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!(
                    frames = self.frames_presented,
                    motion_events = self.motion_events_count,
                    "window close requested; exiting"
                );
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                debug!(
                    width = new_size.width,
                    height = new_size.height,
                    "window resized"
                );
                self.size = new_size;
                if let (Some(surface), Some(w), Some(h)) = (
                    &mut self.surface,
                    NonZeroU32::new(new_size.width),
                    NonZeroU32::new(new_size.height),
                ) {
                    let _ = surface.resize(w, h);
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.motion_events_count += 1;
                if let Some((dx, dy)) = self.pointer.update_cursor(position.x, position.y) {
                    if self.pointer.is_panning() {
                        self.camera.pan(dx, dy);
                    }
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.pointer.left_pressed = pressed;
                    }
                    MouseButton::Middle => {
                        self.pointer.middle_pressed = pressed;
                    }
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let factor = match delta {
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
                };

                if let Some((cx, cy)) = self.pointer.cursor_pos {
                    self.camera.zoom_at(cx, cy, factor);
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface {
                    if let Ok(mut buffer) = surface.buffer_mut() {
                        // Fill dark matte background #121212
                        buffer.fill(0x00121212);
                        let _ = buffer.present();
                        self.frames_presented += 1;
                    }
                }
            }
            _ => {}
        }
    }
}
