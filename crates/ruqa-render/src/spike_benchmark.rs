//! Spike 1 Benchmark: Headless CPU rendering of 1,000 hand-drawn shapes and freehand strokes.

use rough_tiny_skia::ToSkiaDrawable;
use roughr::core::OptionsBuilder;
use roughr::generator::Generator;
use std::time::Instant;
use tiny_skia::{Color, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

/// Results of the Spike 1 rendering benchmark.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Number of elements rendered per frame.
    pub element_count: usize,
    /// Total number of tiny-skia paths across all elements.
    pub total_paths: usize,
    /// Time taken to generate the scene paths in milliseconds.
    pub generation_ms: f64,
    /// Number of frames measured.
    pub frame_count: usize,
    /// Minimum frame duration in milliseconds.
    pub min_ms: f64,
    /// Mean frame duration in milliseconds.
    pub mean_ms: f64,
    /// 50th percentile (median) frame duration in milliseconds.
    pub p50_ms: f64,
    /// 95th percentile frame duration in milliseconds.
    pub p95_ms: f64,
    /// Maximum frame duration in milliseconds.
    pub max_ms: f64,
    /// Calculated frames per second based on mean duration.
    pub fps: f64,
    /// Resident Set Size (RSS) in megabytes during benchmark.
    pub rss_mb: f64,
    /// Whether the 60 FPS (<= 16.6 ms) target was achieved.
    pub target_met: bool,
}

/// Helper to read Linux resident set size (RSS) in megabytes.
#[must_use]
pub fn current_rss_mb() -> f64 {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<f64>() {
                        return kb / 1024.0;
                    }
                }
            }
        }
    }
    0.0
}

/// Pre-generated test shape for the benchmark.
pub struct BenchmarkShape {
    paths: Vec<Path>,
    paint: Paint<'static>,
    stroke: Stroke,
}

/// Generates `count` synthetic hand-drawn shapes (rectangles, ellipses, and pen strokes).
#[must_use]
pub fn generate_synthetic_scene(count: usize) -> Vec<BenchmarkShape> {
    let generator = Generator::default();
    let mut shapes = Vec::with_capacity(count);

    let colors = [
        Color::from_rgba8(236, 239, 244, 255), // Off-white
        Color::from_rgba8(77, 208, 225, 255),  // Cyan
        Color::from_rgba8(255, 138, 101, 255), // Coral
        Color::from_rgba8(255, 213, 79, 255),  // Yellow
        Color::from_rgba8(129, 199, 132, 255), // Green
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
        let x = ((i * 37) % 1800) as f32 + 20.0;
        let y = ((i * 59) % 960) as f32 + 20.0;

        match i % 3 {
            0 => {
                // Rough Rectangle
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
                shapes.push(BenchmarkShape {
                    paths,
                    paint,
                    stroke,
                });
            }
            1 => {
                // Rough Ellipse
                let options = OptionsBuilder::default()
                    .seed(seed)
                    .roughness(1.0)
                    .build()
                    .ok();
                let drawable = generator.ellipse(x + 60.0, y + 40.0, 100.0, 70.0, &options);
                let paths: Vec<Path> = drawable
                    .to_skia_drawable()
                    .sets
                    .into_iter()
                    .map(|set| set.ops)
                    .collect();
                shapes.push(BenchmarkShape {
                    paths,
                    paint,
                    stroke,
                });
            }
            _ => {
                // Multi-point freehand stroke
                let mut pb = PathBuilder::new();
                pb.move_to(x, y);
                for step in 1..8 {
                    let sx = x + (step as f32 * 12.0);
                    let sy = y + ((step as f32 * 0.8).sin() * 20.0);
                    pb.line_to(sx, sy);
                }
                if let Some(path) = pb.finish() {
                    shapes.push(BenchmarkShape {
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

/// Runs the Spike 1 rendering benchmark over `frames` iterations with `element_count` elements.
#[must_use]
pub fn run_spike_benchmark(element_count: usize, frames: usize) -> BenchmarkResult {
    let mut pixmap = Pixmap::new(1920, 1080).expect("failed to allocate 1920x1080 pixmap");
    let gen_start = Instant::now();
    let scene = generate_synthetic_scene(element_count);
    let generation_ms = gen_start.elapsed().as_secs_f64() * 1000.0;

    let total_paths = scene.iter().map(|s| s.paths.len()).sum();
    let mut frame_times = Vec::with_capacity(frames);

    let background = Color::from_rgba8(18, 18, 18, 255); // #121212 dark matte

    for _ in 0..frames {
        let start = Instant::now();
        pixmap.fill(background);

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

        let duration = start.elapsed();
        frame_times.push(duration.as_secs_f64() * 1000.0);
    }

    frame_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let min_ms = frame_times[0];
    let max_ms = frame_times[frame_times.len() - 1];
    let sum: f64 = frame_times.iter().sum();
    let mean_ms = sum / (frames as f64);
    let p50_ms = frame_times[frames / 2];
    let p95_ms = frame_times[(frames * 95) / 100];
    let fps = 1000.0 / mean_ms;
    let target_met = mean_ms <= 16.667;
    let rss_mb = current_rss_mb();

    BenchmarkResult {
        element_count,
        total_paths,
        generation_ms,
        frame_count: frames,
        min_ms,
        mean_ms,
        p50_ms,
        p95_ms,
        max_ms,
        fps,
        rss_mb,
        target_met,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_synthetic_scene_generates_shapes() {
        let scene = generate_synthetic_scene(30);
        assert_eq!(scene.len(), 30);
    }

    #[test]
    fn test_benchmark_runs_and_measures_latencies() {
        let result = run_spike_benchmark(50, 5);
        assert_eq!(result.element_count, 50);
        assert_eq!(result.frame_count, 5);
        assert!(result.mean_ms > 0.0);
        assert!(result.fps > 0.0);
    }
}
