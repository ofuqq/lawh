//! Automated microbenchmarks for canvas rendering frame times.
//!
//! Measures headless 1920x1080 rasterization across deterministic scenes of 1,000, 5,000,
//! and 10,000 mixed elements (rectangles, ellipses, arrows, and freehand strokes).

use rough_tiny_skia::ToSkiaDrawable;
use roughr::core::OptionsBuilder;
use roughr::generator::Generator;
use std::time::Instant;
use tiny_skia::{Color, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

/// Synthetic shape representation for benchmarking.
struct BenchShape {
    paths: Vec<Path>,
    paint: Paint<'static>,
    stroke: Stroke,
}

/// Generates a deterministic scene of `count` mixed shapes.
fn generate_deterministic_scene(count: usize) -> Vec<BenchShape> {
    let generator = Generator::default();
    let mut shapes = Vec::with_capacity(count);

    let colors = [
        Color::from_rgba8(236, 239, 244, 255), // #ECEFF4 White
        Color::from_rgba8(77, 208, 225, 255),  // #4DD0E1 Cyan
        Color::from_rgba8(255, 138, 101, 255), // #FF8A65 Coral
        Color::from_rgba8(255, 213, 79, 255),  // #FFD54F Yellow
        Color::from_rgba8(129, 199, 132, 255), // #81C784 Green
    ];

    for i in 0..count {
        let color = colors[i % colors.len()];
        let mut paint = Paint::default();
        paint.set_color(color);
        paint.anti_alias = true;

        let stroke = Stroke {
            width: 1.5,
            ..Default::default()
        };

        let seed = (i as u64).wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = ((i * 47) % 1800) as f32 + 20.0;
        let y = ((i * 73) % 960) as f32 + 20.0;

        match i % 4 {
            0 => {
                // Rectangle
                let options = OptionsBuilder::default()
                    .seed(seed)
                    .roughness(1.0)
                    .build()
                    .ok();
                let drawable = generator.rectangle(x, y, 120.0, 80.0, &options);
                let paths: Vec<Path> = drawable
                    .to_skia_drawable()
                    .sets
                    .into_iter()
                    .map(|set| set.ops)
                    .collect();
                shapes.push(BenchShape {
                    paths,
                    paint,
                    stroke,
                });
            }
            1 => {
                // Ellipse
                let options = OptionsBuilder::default()
                    .seed(seed)
                    .roughness(1.0)
                    .build()
                    .ok();
                let drawable = generator.ellipse(x + 50.0, y + 40.0, 100.0, 70.0, &options);
                let paths: Vec<Path> = drawable
                    .to_skia_drawable()
                    .sets
                    .into_iter()
                    .map(|set| set.ops)
                    .collect();
                shapes.push(BenchShape {
                    paths,
                    paint,
                    stroke,
                });
            }
            2 => {
                // Linear Arrow
                let options = OptionsBuilder::default()
                    .seed(seed)
                    .roughness(1.0)
                    .build()
                    .ok();
                let x2 = x + 140.0;
                let y2 = y + 60.0;
                let line = generator.line(x, y, x2, y2, &options);
                let mut paths: Vec<Path> = line
                    .to_skia_drawable()
                    .sets
                    .into_iter()
                    .map(|set| set.ops)
                    .collect();
                // Arrow head lines
                let h1 = generator.line(x2, y2, x2 - 20.0, y2 - 10.0, &options);
                let h2 = generator.line(x2, y2, x2 - 10.0, y2 - 20.0, &options);
                paths.extend(
                    h1.to_skia_drawable()
                        .sets
                        .into_iter()
                        .chain(h2.to_skia_drawable().sets)
                        .map(|set| set.ops),
                );
                shapes.push(BenchShape {
                    paths,
                    paint,
                    stroke,
                });
            }
            _ => {
                // Smooth Freehand Stroke
                let mut pb = PathBuilder::new();
                pb.move_to(x, y);
                for step in 1..10 {
                    let sx = x + (step as f32 * 14.0);
                    let sy = y + ((step as f32 * 0.7).sin() * 25.0);
                    pb.line_to(sx, sy);
                }
                if let Some(path) = pb.finish() {
                    shapes.push(BenchShape {
                        paths: vec![path],
                        paint,
                        stroke,
                    });
                }
            }
        }
    }

    shapes
}

/// Statistics collected for a benchmark tier.
struct TierResult {
    element_count: usize,
    frame_count: usize,
    min_ms: f64,
    mean_ms: f64,
    p50_ms: f64,
    p95_ms: f64,
    max_ms: f64,
    fps: f64,
}

fn benchmark_tier(element_count: usize, frames: usize) -> TierResult {
    let mut pixmap = Pixmap::new(1920, 1080).expect("failed to allocate 1080p pixmap");
    let scene = generate_deterministic_scene(element_count);
    let mut times = Vec::with_capacity(frames);
    let bg = Color::from_rgba8(18, 18, 18, 255);

    // Warmup
    for _ in 0..5 {
        pixmap.fill(bg);
        for shape in &scene {
            for path in &shape.paths {
                pixmap.stroke_path(
                    path,
                    &shape.paint,
                    &shape.stroke,
                    Transform::identity(),
                    None,
                );
            }
        }
    }

    for _ in 0..frames {
        let start = Instant::now();
        pixmap.fill(bg);

        for shape in &scene {
            for path in &shape.paths {
                pixmap.stroke_path(
                    path,
                    &shape.paint,
                    &shape.stroke,
                    Transform::identity(),
                    None,
                );
            }
        }

        let elapsed = start.elapsed();
        times.push(elapsed.as_secs_f64() * 1000.0);
    }

    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let min_ms = times[0];
    let max_ms = times[times.len() - 1];
    let sum: f64 = times.iter().sum();
    let mean_ms = sum / (frames as f64);
    let p50_ms = times[frames / 2];
    let p95_ms = times[(frames * 95) / 100];
    let fps = 1000.0 / mean_ms;

    TierResult {
        element_count,
        frame_count: frames,
        min_ms,
        mean_ms,
        p50_ms,
        p95_ms,
        max_ms,
        fps,
    }
}

fn main() {
    println!("=== Ruqa Canvas Rendering Microbenchmark Suite ===");
    println!("Target Resolution: 1920x1080 32-bit RGBA Pixmap (Headless CPU)");
    println!("Workloads: 1,000, 5,000, and 10,000 Mixed Elements\n");

    let tiers = [
        (1_000, 30), // 1k elements, 30 frames
        (5_000, 10), // 5k elements, 10 frames
        (10_000, 5), // 10k elements, 5 frames
    ];

    let mut results = Vec::new();

    for &(count, frames) in &tiers {
        print!("Benchmarking {} elements ({} frames)... ", count, frames);
        let res = benchmark_tier(count, frames);
        println!("done ({:.2} ms / {:.1} FPS)", res.mean_ms, res.fps);
        results.push(res);
    }

    println!("\n### Microbenchmark Summary Table\n");
    println!(
        "| Elements | Frames | Min (ms) | Mean (ms) | Median (p50) | p95 (ms) | Max (ms) | Calculated FPS |"
    );
    println!("|---|---|---|---|---|---|---|---|");
    for r in &results {
        println!(
            "| **{}** | {} | {:.2} | **{:.2}** | {:.2} | {:.2} | {:.2} | **{:.1} FPS** |",
            r.element_count,
            r.frame_count,
            r.min_ms,
            r.mean_ms,
            r.p50_ms,
            r.p95_ms,
            r.max_ms,
            r.fps
        );
    }
    println!("\n==================================================");
}
