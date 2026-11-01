//! Standalone runner for Spike 1 benchmark.

use ruqa_render::spike_benchmark::run_spike_benchmark;

fn main() {
    println!("=== Ruqa Spike 1: 2D CPU Rendering Benchmark ===");
    println!("Canvas Resolution: 1920x1080 (32-bit RGBA)");
    println!("Hardware Profiling: CPU, Memory RSS, and Scaling Curve\n");

    println!("Warmup run (20 frames, 100 elements)...");
    let _ = run_spike_benchmark(100, 20);

    let test_counts = [50, 100, 250, 500, 1000];

    for &count in &test_counts {
        let frames = if count <= 250 { 100 } else { 50 };
        println!("Profiling {} elements ({} frames)...", count, frames);
        let res = run_spike_benchmark(count, frames);

        println!("--------------------------------------------------");
        println!("Element Count : {}", res.element_count);
        println!("Total Paths   : {}", res.total_paths);
        println!("Gen Time      : {:.2} ms", res.generation_ms);
        println!("Frame Count   : {}", res.frame_count);
        println!("Min Duration  : {:.3} ms", res.min_ms);
        println!("Mean Duration : {:.3} ms", res.mean_ms);
        println!("Median (p50)  : {:.3} ms", res.p50_ms);
        println!("p95 Duration  : {:.3} ms", res.p95_ms);
        println!("Max Duration  : {:.3} ms", res.max_ms);
        println!("Calculated FPS: {:.1} FPS", res.fps);
        println!("Process RSS   : {:.2} MB", res.rss_mb);
        println!(
            "Target Met    : {} (Target: <= 16.667 ms / >= 60 FPS)",
            res.target_met
        );
        println!("--------------------------------------------------\n");
    }
}
