//! Integration and benchmark tests for spatial AABB viewport culling.

use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect, Size, Viewport};
use ruqa_core::scene::Scene;
use ruqa_render::shape_render::render_scene_viewport;
use std::time::Instant;
use tiny_skia::{Color, Pixmap};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_color: "#ECEFF4".into(),
            stroke_width: 2.0,
            seed: id,
            ..Default::default()
        },
    }
}

#[test]
fn test_aabb_culling_with_stroke_padding() {
    let mut scene = Scene::new();

    // Visible window: 400x400 with camera at origin -> world bounds: -200..200, -200..200
    let viewport = Viewport::new(Point::ZERO, 1.0);
    let window_size = Size::new(400.0, 400.0);

    // Line starting at x=202 with stroke_width=10.0 (half_width = 5.0)
    // Geometric bounds: [202, 0] to [300, 0] (strictly > 200, would be culled by unpadded bounds)
    // AABB with padding: [197, -5] to [305, 5] (intersects visible rect [-200, 200]!)
    let line = Element::Line {
        id: 1,
        start: Point::new(202.0, 0.0),
        end: Point::new(300.0, 0.0),
        style: ElementStyle {
            stroke_color: "#FF6B6B".into(),
            stroke_width: 10.0,
            seed: 1,
            ..Default::default()
        },
    };
    scene.push(line);

    let mut pixmap = Pixmap::new(400, 400).expect("pixmap allocation succeeds");
    pixmap.fill(Color::BLACK);

    let rendered = render_scene_viewport(&scene, &viewport, window_size, &mut pixmap.as_mut());
    assert_eq!(
        rendered, 1,
        "element with stroke padding intersecting visible rect must NOT be culled"
    );
}

#[test]
fn test_completely_offscreen_elements_culled() {
    let mut scene = Scene::new();
    let viewport = Viewport::new(Point::ZERO, 1.0);
    let window_size = Size::new(400.0, 400.0);

    // 10 elements far away in world coordinates
    for i in 1..=10 {
        scene.push(sample_rect(
            i,
            5000.0 + (i as f32) * 100.0,
            5000.0,
            50.0,
            50.0,
        ));
    }

    let mut pixmap = Pixmap::new(400, 400).expect("pixmap allocation succeeds");
    pixmap.fill(Color::TRANSPARENT);

    let rendered = render_scene_viewport(&scene, &viewport, window_size, &mut pixmap.as_mut());
    assert_eq!(rendered, 0, "offscreen elements must all be culled");

    // Pixmap remains completely untouched (all zero bytes)
    let non_transparent = pixmap.data().iter().filter(|&&b| b != 0).count();
    assert_eq!(
        non_transparent, 0,
        "no pixels should be drawn for culled elements"
    );
}

#[test]
fn test_culling_dynamic_pan_and_zoom() {
    let mut scene = Scene::new();
    // Element at world (0, 0)
    scene.push(sample_rect(1, -25.0, -25.0, 50.0, 50.0));
    // Element at world (1000, 0)
    scene.push(sample_rect(2, 975.0, -25.0, 50.0, 50.0));

    let window_size = Size::new(400.0, 400.0);
    let mut pixmap = Pixmap::new(400, 400).expect("pixmap allocation succeeds");

    // Camera at origin: element 1 visible, element 2 culled
    let vp1 = Viewport::new(Point::ZERO, 1.0);
    let count1 = render_scene_viewport(&scene, &vp1, window_size, &mut pixmap.as_mut());
    assert_eq!(count1, 1);

    // Pan camera to world (1000, 0): element 1 culled, element 2 visible
    let vp2 = Viewport::new(Point::new(1000.0, 0.0), 1.0);
    let count2 = render_scene_viewport(&scene, &vp2, window_size, &mut pixmap.as_mut());
    assert_eq!(count2, 1);

    // Zoom out so both elements fit in the visible viewport (zoom 0.1x -> world span 4000x4000)
    let vp3 = Viewport::new(Point::new(500.0, 0.0), 0.1);
    let count3 = render_scene_viewport(&scene, &vp3, window_size, &mut pixmap.as_mut());
    assert_eq!(count3, 2, "zoomed-out viewport must render both elements");
}

#[test]
fn test_culling_benchmark_scales_with_visible_elements_not_total_elements() {
    let mut large_scene = Scene::new();
    let mut visible_only_scene = Scene::new();

    // Add 30 shapes inside the camera view (-200..200, -200..200)
    for i in 0..30 {
        let x = ((i % 6) as f32) * 50.0 - 150.0;
        let y = ((i / 6) as f32) * 50.0 - 120.0;
        let elem = sample_rect(i as u64 + 1, x, y, 40.0, 30.0);
        large_scene.push(elem.clone());
        visible_only_scene.push(elem);
    }

    // Add 2,000 shapes far offscreen (scattered from 10,000 to 50,000)
    for i in 0..2000 {
        let x = 10000.0 + ((i % 50) as f32) * 200.0;
        let y = 10000.0 + ((i / 50) as f32) * 200.0;
        large_scene.push(sample_rect((i + 100) as u64, x, y, 60.0, 40.0));
    }

    assert_eq!(visible_only_scene.len(), 30);
    assert_eq!(large_scene.len(), 2030);

    let viewport = Viewport::new(Point::ZERO, 1.0);
    let window_size = Size::new(800.0, 600.0);
    let mut pixmap = Pixmap::new(800, 600).expect("pixmap allocation succeeds");

    // Warmup
    for _ in 0..3 {
        pixmap.fill(Color::BLACK);
        render_scene_viewport(&large_scene, &viewport, window_size, &mut pixmap.as_mut());
    }

    // Measure frame time for large scene (2,030 total elements, 30 visible)
    let start_large = Instant::now();
    let mut rendered_large = 0;
    for _ in 0..10 {
        pixmap.fill(Color::BLACK);
        rendered_large =
            render_scene_viewport(&large_scene, &viewport, window_size, &mut pixmap.as_mut());
    }
    let elapsed_large = start_large.elapsed();
    let per_frame_large_ms = (elapsed_large.as_secs_f64() * 1000.0) / 10.0;

    // Measure frame time for visible-only scene (30 total elements, 30 visible)
    let start_small = Instant::now();
    let mut rendered_small = 0;
    for _ in 0..10 {
        pixmap.fill(Color::BLACK);
        rendered_small = render_scene_viewport(
            &visible_only_scene,
            &viewport,
            window_size,
            &mut pixmap.as_mut(),
        );
    }
    let elapsed_small = start_small.elapsed();
    let per_frame_small_ms = (elapsed_small.as_secs_f64() * 1000.0) / 10.0;

    // Both scenes must render exactly the 30 visible elements
    assert_eq!(rendered_large, 30);
    assert_eq!(rendered_small, 30);

    // Latency for 2,030 elements with culling must maintain solid 60 FPS (< 16.6 ms in release, < 100 ms in debug)
    let target_threshold_ms = if cfg!(debug_assertions) { 100.0 } else { 16.6 };
    assert!(
        per_frame_large_ms < target_threshold_ms,
        "culled rendering of 2,030 elements took {per_frame_large_ms:.2} ms per frame (must be < {target_threshold_ms} ms)"
    );

    // Latency of 2,030 elements should be close to 30 elements (depends on visible elements, not 2,000 offscreen elements)
    let diff_threshold = if cfg!(debug_assertions) { 75.0 } else { 5.0 };
    assert!(
        (per_frame_large_ms - per_frame_small_ms).abs() < diff_threshold,
        "rendering time should depend on visible elements (large: {per_frame_large_ms:.2} ms, small: {per_frame_small_ms:.2} ms)"
    );
}
