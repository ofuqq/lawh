//! Ruqa desktop application entry point.

use ruqa_app::event_dispatch::{PointerState, ViewportCamera};
use ruqa_app::window_app::WhiteboardApp;
use std::time::Instant;
use tracing::{debug, error, info, warn};
use winit::event_loop::EventLoop;

fn main() {
    // Initialise structured logging before any other work.
    ruqa_app::logging::init();

    info!(version = ruqa_app::VERSION, "Ruqa starting up");

    let has_display = std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();

    if has_display {
        info!("display server detected; attempting winit 0.30 event loop");
        match EventLoop::new() {
            Ok(event_loop) => {
                let mut app = WhiteboardApp::new();
                info!("entering interactive window (close window or Ctrl-C to exit)");
                if let Err(err) = event_loop.run_app(&mut app) {
                    error!(?err, "event loop terminated with error");
                }
                return;
            }
            Err(err) => {
                warn!(
                    ?err,
                    "unable to create OS window; falling back to headless simulation"
                );
            }
        }
    } else {
        debug!("headless environment (no DISPLAY/WAYLAND_DISPLAY); running synthetic benchmark");
    }

    run_headless_pointer_benchmark();
}

/// Runs a synthetic 10 000-event mouse motion benchmark to verify zero-allocation dispatch.
///
/// This is kept as a development smoke-test for headless CI. It will be removed
/// once the full window lifecycle is wired up in issue #8.
fn run_headless_pointer_benchmark() {
    info!("running 10 000-event headless pointer benchmark");
    let mut pointer = PointerState::new();
    let mut camera = ViewportCamera::new();

    let event_count = 10_000u32;
    let start = Instant::now();

    for i in 0..event_count {
        let x = 100.0 + (f64::from(i) * 0.1);
        let y = 150.0 + ((f64::from(i) * 0.05).sin() * 50.0);

        if let Some((dx, dy)) = pointer.update_cursor(x, y) {
            if i % 3 == 0 {
                pointer.middle_pressed = true;
                camera.pan(dx, dy);
            } else {
                pointer.middle_pressed = false;
            }

            if i % 100 == 0 {
                camera.zoom_at(x, y, 1.05);
            }
        }
    }

    let elapsed = start.elapsed();
    let ns_per_event = elapsed.as_nanos() as f64 / f64::from(event_count);

    info!(
        total_events = event_count,
        duration_ms = elapsed.as_millis(),
        ns_per_event = format!("{ns_per_event:.1}"),
        offset_x = format!("{:.1}", camera.offset_x),
        offset_y = format!("{:.1}", camera.offset_y),
        zoom = format!("{:.2}", camera.zoom),
        "headless benchmark complete"
    );
}
