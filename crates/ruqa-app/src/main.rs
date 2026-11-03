//! Ruqa desktop application entry point.
//!
//! # Modes
//!
//! | Invocation | Behaviour |
//! |---|---|
//! | `cargo run -p ruqa-app` | Opens the interactive whiteboard window. |
//! | `cargo run -p ruqa-app -- --demo` | Opens the whiteboard populated with sample primitives. |
//! | `cargo run -p ruqa-app -- --smoke-test <N>` | Pumps `N` synthetic frames through the winit event loop and exits with code 0. Used by CI. |
//! | *(headless, no DISPLAY)* | Runs a synthetic mouse-motion benchmark and exits. |

use ruqa_app::event_dispatch::{PointerState, ViewportCamera};
use ruqa_app::window_app::WhiteboardApp;
use std::time::Instant;
use tracing::{error, info, warn};
use winit::event_loop::EventLoop;

fn main() {
    ruqa_app::logging::init();
    info!(version = ruqa_app::VERSION, "Ruqa starting up");

    // Parse simple CLI flags without pulling in a third-party arg parser.
    let args: Vec<String> = std::env::args().collect();
    let smoke_frames: Option<u32> = parse_smoke_test_flag(&args);
    let is_demo: bool = parse_demo_flag(&args);
    let initial_file = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-'))
        .cloned();

    let has_display = std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();

    if let Some(n) = smoke_frames {
        // Smoke-test mode: requires a display (xvfb-run in CI provides one).
        info!(frames = n, is_demo, "smoke-test mode");
        run_smoke_test(n, is_demo);
        return;
    }

    if has_display {
        info!("display detected; starting interactive window");
        match EventLoop::new() {
            Ok(event_loop) => {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                let mut app = WhiteboardApp::new();
                app.load_recent_files();
                if is_demo {
                    info!("populating sample demo primitives on canvas");
                    app.load_demo_board();
                    app.show_welcome_hud = false;
                } else if let Some(path) = initial_file {
                    if let Err(e) = app.open_recent_file(&path) {
                        warn!(?e, path = ?path, "could not open initial file argument");
                    }
                }
                if let Err(err) = event_loop.run_app(&mut app) {
                    error!(?err, "event loop exited with error");
                }
                return;
            }
            Err(err) => {
                warn!(
                    ?err,
                    "could not create event loop; falling back to headless benchmark"
                );
            }
        }
    } else {
        info!("no display; running headless pointer benchmark");
    }

    run_headless_pointer_benchmark();
}

/// Runs a real winit event loop for `n` `RedrawRequested` events then exits.
///
/// Requires a display (use `xvfb-run` in CI). The process exits 0 on success,
/// non-zero if the event loop returns an error.
fn run_smoke_test(n: u32, is_demo: bool) {
    use std::sync::atomic::AtomicU32;
    use std::sync::Arc;

    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();
    let target = n;

    match EventLoop::new() {
        Ok(event_loop) => {
            // We install an event-loop hook via the UserEvent mechanism to count frames.
            // Simplest approach: run the full app and exit after N frames via a side channel.
            let mut app = SmokeApp {
                inner: {
                    let mut a = WhiteboardApp::new();
                    if is_demo {
                        a.load_demo_board();
                    }
                    a
                },
                frame_counter: counter_clone,
                target_frames: target,
            };
            if let Err(e) = event_loop.run_app(&mut app) {
                error!(?e, "smoke-test event loop error");
                std::process::exit(1);
            }
            info!(frames = n, "smoke-test passed");
        }
        Err(e) => {
            error!(?e, "could not create event loop for smoke-test");
            std::process::exit(1);
        }
    }
}

/// Thin wrapper around [`WhiteboardApp`] that exits after `target_frames` frames.
struct SmokeApp {
    inner: WhiteboardApp,
    frame_counter: std::sync::Arc<std::sync::atomic::AtomicU32>,
    target_frames: u32,
}

impl winit::application::ApplicationHandler for SmokeApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.inner.resumed(event_loop);
        // Request the first redraw immediately.
        if let Some(w) = &self.inner.window {
            w.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Count RedrawRequested events.
        if matches!(event, winit::event::WindowEvent::RedrawRequested) {
            let count = self
                .frame_counter
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                + 1;
            // Delegate to the real app.
            self.inner.window_event(event_loop, window_id, event);
            if count >= self.target_frames {
                info!(frames = count, "smoke-test target reached; exiting");
                event_loop.exit();
                return;
            }
            // Request the next frame immediately (busy-loop for test speed).
            if let Some(w) = &self.inner.window {
                w.request_redraw();
            }
        } else {
            self.inner.window_event(event_loop, window_id, event);
        }
    }
}

/// Parses `--smoke-test <N>` from the argument list.
fn parse_smoke_test_flag(args: &[String]) -> Option<u32> {
    for i in 0..args.len() {
        if args[i] == "--smoke-test" {
            return args.get(i + 1).and_then(|v| v.parse::<u32>().ok());
        }
    }
    None
}

/// Returns `true` if the `--demo` flag is present in the arguments.
fn parse_demo_flag(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--demo")
}

/// Synthetic 10 000-event mouse motion benchmark (headless fallback).
fn run_headless_pointer_benchmark() {
    info!("running headless pointer benchmark (10 000 events)");
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
        "headless benchmark complete"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_smoke_test_flag() {
        let args = vec![
            "ruqa-app".to_string(),
            "--smoke-test".to_string(),
            "10".to_string(),
        ];
        assert_eq!(parse_smoke_test_flag(&args), Some(10));

        let args_invalid = vec![
            "ruqa-app".to_string(),
            "--smoke-test".to_string(),
            "abc".to_string(),
        ];
        assert_eq!(parse_smoke_test_flag(&args_invalid), None);

        let args_missing = vec!["ruqa-app".to_string(), "--smoke-test".to_string()];
        assert_eq!(parse_smoke_test_flag(&args_missing), None);

        let args_none = vec!["ruqa-app".to_string()];
        assert_eq!(parse_smoke_test_flag(&args_none), None);
    }

    #[test]
    fn test_parse_demo_flag() {
        let args_demo = vec!["ruqa-app".to_string(), "--demo".to_string()];
        assert!(parse_demo_flag(&args_demo));

        let args_mixed = vec![
            "ruqa-app".to_string(),
            "--smoke-test".to_string(),
            "5".to_string(),
            "--demo".to_string(),
        ];
        assert!(parse_demo_flag(&args_mixed));

        let args_none = vec!["ruqa-app".to_string()];
        assert!(!parse_demo_flag(&args_none));
    }
}
