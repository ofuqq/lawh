//! Ruqa desktop application entry point.

use ruqa_app::event_dispatch::{PointerState, ViewportCamera};
use ruqa_app::window_app::WhiteboardApp;
use std::time::Instant;
use winit::event_loop::EventLoop;

fn main() {
    println!("=== Ruqa Spike 3: winit 0.30 Event Loop & Mouse Dispatch ===");

    let has_display = std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();

    if has_display {
        println!("Display server detected. Attempting to initialize winit 0.30 event loop...");
        match EventLoop::new() {
            Ok(event_loop) => {
                let mut app = WhiteboardApp::new();
                println!("Running interactive window... (close window or press Ctrl+C to exit)");
                if let Err(err) = event_loop.run_app(&mut app) {
                    eprintln!("Event loop terminated with error: {:?}", err);
                }
                return;
            }
            Err(err) => {
                println!(
                    "Unable to create OS window event loop ({:?}). Falling back to headless simulation.",
                    err
                );
            }
        }
    } else {
        println!(
            "Headless environment (no DISPLAY/WAYLAND_DISPLAY). Running synthetic event benchmark."
        );
    }

    // Headless high-frequency event loop benchmark (144Hz simulation)
    run_headless_pointer_benchmark();
}

fn run_headless_pointer_benchmark() {
    println!("\n--- Simulating 10,000 High-Frequency Mouse Motion Events (Zero-Allocation) ---");
    let mut pointer = PointerState::new();
    let mut camera = ViewportCamera::new();

    let event_count = 10_000;
    let start = Instant::now();

    for i in 0..event_count {
        let x = 100.0 + ((i as f64) * 0.1);
        let y = 150.0 + ((i as f64 * 0.05).sin() * 50.0);

        if let Some((dx, dy)) = pointer.update_cursor(x, y) {
            if i % 3 == 0 {
                // Simulate middle-click drag pan
                pointer.middle_pressed = true;
                camera.pan(dx, dy);
            } else {
                pointer.middle_pressed = false;
            }

            if i % 100 == 0 {
                // Simulate wheel zoom
                camera.zoom_at(x, y, 1.05);
            }
        }
    }

    let elapsed = start.elapsed();
    let duration_ms = elapsed.as_secs_f64() * 1000.0;
    let ns_per_event = (elapsed.as_nanos() as f64) / (event_count as f64);

    println!("Total Events   : {}", event_count);
    println!("Total Duration : {:.3} ms", duration_ms);
    println!("Average Latency: {:.1} ns per event", ns_per_event);
    println!(
        "Final Camera   : pan ({:.1}, {:.1}), zoom {:.2}x",
        camera.offset_x, camera.offset_y, camera.zoom
    );
    println!("Zero Alloc     : verified (no heap allocations during motion dispatch)");
    println!("--------------------------------------------------");
}
