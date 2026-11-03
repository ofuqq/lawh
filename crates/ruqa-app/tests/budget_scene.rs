//! Reproducible benchmark for the fixed stress scene defined in `docs/product/budget.md`.
//!
//! # Benchmark Specification
//! - Viewport: 2560×1440 pixels at scale factor 1.0 with dark theme dot grid enabled.
//! - Elements:
//!   - 5,000 rough shapes (1,000 rects, 1,000 ellipses, 1,000 diamonds, 1,000 lines, 1,000 arrows).
//!   - 2,000 smoothed freehand strokes (24 points each).
//!   - 500 bilingual text boxes with mixed Arabic/Latin content.
//! - Total Elements: 7,500, all positioned within the visible viewport bounds.
//! - Execution: Rendered through [`WhiteboardApp::render_to_pixmap`] (full production pipeline
//!   including culling, grid rendering, vector roughing, and text layout).
//!
//! # Running
//! ```bash
//! cargo test --release -p ruqa-app --test budget_scene -- --ignored --nocapture
//! ```

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::geometry::{Point, Rect, Size};
use std::time::Instant;
use tiny_skia::Pixmap;
use winit::dpi::PhysicalSize;

struct SimpleLcg(u64);

impl SimpleLcg {
    const fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.0
    }

    fn next_f32(&mut self, min: f32, max: f32) -> f32 {
        let t = (self.next_u64() >> 32) as u32 as f32 / (u32::MAX as f32);
        min + t * (max - min)
    }
}

fn build_budget_stress_scene(app: &mut WhiteboardApp) {
    let mut rng = SimpleLcg::new(42);
    let mut id = 1u64;

    // 1. 5,000 rough shapes (1,000 rects, 1,000 ellipses, 1,000 diamonds, 1,000 lines, 1,000 arrows)
    for _ in 0..1000 {
        let x = rng.next_f32(60.0, 2400.0);
        let y = rng.next_f32(60.0, 1300.0);
        let w = rng.next_f32(30.0, 120.0);
        let h = rng.next_f32(20.0, 80.0);
        let style = ElementStyle {
            stroke_color: "#e0e0e0".into(),
            stroke_width: 2.0,
            roughness: 1.0,
            seed: rng.next_u64(),
            fill_style: FillStyle::None,
            ..Default::default()
        };
        app.scene.push(Element::Rectangle {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style,
        });
        id += 1;
    }

    for _ in 0..1000 {
        let x = rng.next_f32(60.0, 2400.0);
        let y = rng.next_f32(60.0, 1300.0);
        let w = rng.next_f32(30.0, 100.0);
        let h = rng.next_f32(30.0, 100.0);
        let style = ElementStyle {
            stroke_color: "#4a9eff".into(),
            stroke_width: 2.0,
            roughness: 1.0,
            seed: rng.next_u64(),
            fill_style: FillStyle::None,
            ..Default::default()
        };
        app.scene.push(Element::Ellipse {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style,
        });
        id += 1;
    }

    for _ in 0..1000 {
        let x = rng.next_f32(60.0, 2400.0);
        let y = rng.next_f32(60.0, 1300.0);
        let w = rng.next_f32(40.0, 110.0);
        let h = rng.next_f32(40.0, 110.0);
        let style = ElementStyle {
            stroke_color: "#50e3c2".into(),
            stroke_width: 2.0,
            roughness: 1.0,
            seed: rng.next_u64(),
            fill_style: FillStyle::None,
            ..Default::default()
        };
        app.scene.push(Element::Diamond {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style,
        });
        id += 1;
    }

    for _ in 0..1000 {
        let x1 = rng.next_f32(60.0, 2400.0);
        let y1 = rng.next_f32(60.0, 1300.0);
        let x2 = x1 + rng.next_f32(-80.0, 80.0);
        let y2 = y1 + rng.next_f32(-80.0, 80.0);
        let style = ElementStyle {
            stroke_color: "#ffc83b".into(),
            stroke_width: 2.0,
            roughness: 1.0,
            seed: rng.next_u64(),
            ..Default::default()
        };
        app.scene.push(Element::Line {
            id,
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
            style,
        });
        id += 1;
    }

    for _ in 0..1000 {
        let x1 = rng.next_f32(60.0, 2400.0);
        let y1 = rng.next_f32(60.0, 1300.0);
        let x2 = x1 + rng.next_f32(-90.0, 90.0);
        let y2 = y1 + rng.next_f32(-90.0, 90.0);
        let style = ElementStyle {
            stroke_color: "#ff6b6b".into(),
            stroke_width: 2.0,
            roughness: 1.0,
            seed: rng.next_u64(),
            ..Default::default()
        };
        app.scene.push(Element::Arrow {
            id,
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
            style,
        });
        id += 1;
    }

    // 2. 2,000 smoothed freehand strokes (24 points each)
    for _ in 0..2000 {
        let start_x = rng.next_f32(80.0, 2300.0);
        let start_y = rng.next_f32(80.0, 1250.0);
        let mut points = Vec::with_capacity(24);
        let mut curr = Point::new(start_x, start_y);
        points.push(curr);

        for _ in 1..24 {
            curr = Point::new(
                curr.x + rng.next_f32(-6.0, 6.0),
                curr.y + rng.next_f32(-4.0, 4.0),
            );
            points.push(curr);
        }

        let pressures = vec![0.5; 24];
        let style = ElementStyle {
            stroke_color: "#b8e986".into(),
            stroke_width: 2.5,
            roughness: 1.0,
            seed: rng.next_u64(),
            ..Default::default()
        };
        app.scene.push(Element::Stroke {
            id,
            points,
            pressures,
            style,
        });
        id += 1;
    }

    // 3. 500 bilingual text boxes
    for _ in 0..500 {
        let x = rng.next_f32(60.0, 2350.0);
        let y = rng.next_f32(60.0, 1320.0);
        let style = ElementStyle {
            stroke_color: "#ffffff".into(),
            stroke_width: 1.0,
            roughness: 0.0,
            seed: rng.next_u64(),
            ..Default::default()
        };
        app.scene.push(Element::Text {
            id,
            position: Point::new(x, y),
            content: "خوارزمية Dijkstra تعمل في O(V log V)".to_string(),
            font_size: 16.0,
            style,
        });
        id += 1;
    }
}

#[test]
#[ignore = "performance budget benchmark: run explicitly via cargo test --release -p ruqa-app --test budget_scene -- --ignored --nocapture"]
fn benchmark_fixed_stress_scene_budget() {
    let mut app = WhiteboardApp::new();
    app.physical_size = PhysicalSize::new(2560, 1440);
    app.scale_factor = 1.0;
    app.viewport.pan = Point::new(1280.0, 720.0);
    app.viewport.zoom = 1.0;
    app.show_grid = true;

    build_budget_stress_scene(&mut app);

    // Assert exact composition matches docs/product/budget.md
    assert_eq!(app.scene.len(), 7500, "scene must contain 7,500 elements");
    assert_eq!(app.logical_size(), Size::new(2560.0, 1440.0));
    assert!(app.show_grid, "grid must be enabled");

    let count_shapes = app
        .scene
        .elements()
        .iter()
        .filter(|e| {
            matches!(
                e,
                Element::Rectangle { .. }
                    | Element::Ellipse { .. }
                    | Element::Diamond { .. }
                    | Element::Line { .. }
                    | Element::Arrow { .. }
            )
        })
        .count();
    let count_strokes = app
        .scene
        .elements()
        .iter()
        .filter(|e| matches!(e, Element::Stroke { .. }))
        .count();
    let count_text = app
        .scene
        .elements()
        .iter()
        .filter(|e| matches!(e, Element::Text { .. }))
        .count();

    assert_eq!(count_shapes, 5000, "must have 5,000 rough shapes");
    assert_eq!(count_strokes, 2000, "must have 2,000 smoothed strokes");
    assert_eq!(count_text, 500, "must have 500 bilingual text boxes");

    let mut pixmap = Pixmap::new(2560, 1440).expect("allocate 2560x1440 pixmap");

    // Warmup run
    app.render_to_pixmap(&mut pixmap.as_mut());

    // Measurement runs (N = 5)
    let iterations = 5;
    let mut times_ms = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = Instant::now();
        app.render_to_pixmap(&mut pixmap.as_mut());
        let elapsed = start.elapsed();
        times_ms.push(elapsed.as_secs_f64() * 1000.0);
    }

    times_ms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median_ms = times_ms[iterations / 2];

    println!("\n============================================================");
    println!("Ruqa Performance Budget Benchmark: Fixed Stress Scene (#159)");
    println!("============================================================");
    println!("Canvas Dimensions : 2560 x 1440 (scale factor 1.0)");
    println!("Orientation Grid  : Enabled (dark mode dot grid)");
    println!("Scene Composition : 5,000 rough shapes, 2,000 strokes, 500 text boxes (7,500 total)");
    println!("Pipeline Path     : WhiteboardApp::render_to_pixmap (culling, grid, text layout)");
    println!("Iterations        : {iterations} passes");
    for (i, t) in times_ms.iter().enumerate() {
        println!("  Pass {}: {t:.2} ms", i + 1);
    }
    println!("------------------------------------------------------------");
    println!("Measured Median   : {median_ms:.2} ms");
    println!("Budget Baseline   : 225.5 ms");
    println!("Budget Limit      : 260.0 ms (Target reference on reference hardware)");
    println!("============================================================\n");

    // Sanity assertion that rendering completed and produced non-zero frame time
    assert!(
        median_ms > 0.0,
        "measured frame time must be greater than zero"
    );
}
