//! Implementation of the `winit` application handler for [`WhiteboardApp`].
//!
//! Separates operating system windowing events (create, resize, scale factor change,
//! input events) from canvas data models and presentation logic.

use crate::presenter::Presenter;
use crate::window_app::{WhiteboardApp, DEFAULT_HEIGHT, DEFAULT_WIDTH, MIN_HEIGHT, MIN_WIDTH};
use std::sync::Arc;
use tracing::{debug, info, warn};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

fn build_window_attrs(event_loop: &ActiveEventLoop, title: &str) -> WindowAttributes {
    let mut attrs = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT))
        .with_min_inner_size(LogicalSize::new(MIN_WIDTH, MIN_HEIGHT));

    if let Some(monitor) = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
    {
        let size = monitor.size();
        let scale = monitor.scale_factor();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (w, h) = (
            (f64::from(DEFAULT_WIDTH) * scale).round() as u32,
            (f64::from(DEFAULT_HEIGHT) * scale).round() as u32,
        );
        attrs = attrs.with_position(PhysicalPosition::new(
            size.width.saturating_sub(w) / 2,
            size.height.saturating_sub(h) / 2,
        ));
    }
    attrs
}

impl ApplicationHandler for WhiteboardApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = build_window_attrs(event_loop, &self.window_title());
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
            WindowEvent::CloseRequested => {
                info!(frames = self.frames_presented, "close requested; exiting");
                event_loop.exit();
            }
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
            WindowEvent::CursorMoved { position, .. } => {
                self.handle_cursor_moved(position.x, position.y);
                self.request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_input(state, button);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.handle_mouse_wheel(delta);
                self.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(&event);
            }
            WindowEvent::Focused(focused) => {
                self.handle_focused(focused);
            }
            WindowEvent::RedrawRequested => {
                self.render_frame();
            }
            _ => {}
        }
    }
}
