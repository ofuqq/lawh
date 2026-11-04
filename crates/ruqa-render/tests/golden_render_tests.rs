//! Visual regression golden-image integration tests for ruqa-render.

use rough_tiny_skia::ToSkiaDrawable;
use roughr::core::OptionsBuilder;
use roughr::generator::Generator;
use ruqa_render::offscreen::{compare_pixmaps, OffscreenCanvas};
use std::path::Path;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

fn render_deterministic_test_board() -> Pixmap {
    let mut canvas = OffscreenCanvas::new(400, 300).expect("failed to allocate 400x300 canvas");
    canvas.clear(Color::from_rgba8(18, 18, 18, 255)); // #121212 dark canvas

    let generator = Generator::default();

    // 1. Seeded rough rectangle
    let rect_options = OptionsBuilder::default()
        .seed(42)
        .roughness(1.0)
        .build()
        .ok();
    let rect = generator.rectangle(30.0, 30.0, 140.0, 90.0, &rect_options);
    let mut rect_paint = Paint::default();
    rect_paint.set_color(Color::from_rgba8(77, 208, 225, 255)); // Cyan
    rect_paint.anti_alias = true;
    let stroke = Stroke {
        width: 2.0,
        ..Default::default()
    };
    for set in rect.to_skia_drawable().sets {
        canvas.stroke_path(&set.ops, &rect_paint, &stroke, Transform::identity());
    }

    // 2. Seeded rough ellipse
    let ellipse_options = OptionsBuilder::default()
        .seed(101)
        .roughness(1.0)
        .build()
        .ok();
    let ellipse = generator.ellipse(280.0, 80.0, 100.0, 70.0, &ellipse_options);
    let mut ellipse_paint = Paint::default();
    ellipse_paint.set_color(Color::from_rgba8(255, 138, 101, 255)); // Coral
    ellipse_paint.anti_alias = true;
    for set in ellipse.to_skia_drawable().sets {
        canvas.stroke_path(&set.ops, &ellipse_paint, &stroke, Transform::identity());
    }

    // 3. Freehand smooth curve
    let mut pb = PathBuilder::new();
    pb.move_to(50.0, 220.0);
    pb.cubic_to(120.0, 160.0, 240.0, 280.0, 350.0, 210.0);
    if let Some(path) = pb.finish() {
        let mut curve_paint = Paint::default();
        curve_paint.set_color(Color::from_rgba8(255, 213, 79, 255)); // Yellow
        curve_paint.anti_alias = true;
        canvas.stroke_path(&path, &curve_paint, &stroke, Transform::identity());
    }

    canvas.pixmap().clone()
}

#[test]
fn test_golden_image_deterministic_primitives() {
    let actual = render_deterministic_test_board();
    let golden_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden");
    let golden_path = golden_dir.join("basic_primitives.png");
    let diff_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/failed_diffs");

    let should_update = std::env::var("UPDATE_GOLDEN").as_deref() == Ok("1");

    if should_update || !golden_path.exists() {
        std::fs::create_dir_all(&golden_dir).unwrap();
        actual
            .save_png(&golden_path)
            .expect("failed to save golden image");
        return;
    }

    let expected = Pixmap::load_png(&golden_path).expect("failed to load golden image");
    let (report, diff_pixmap) = compare_pixmaps(&actual, &expected, 0, 0);

    if !report.is_match {
        std::fs::create_dir_all(&diff_dir).unwrap();
        let actual_fail_path = diff_dir.join("basic_primitives_actual.png");
        let diff_fail_path = diff_dir.join("basic_primitives_diff.png");

        let _ = actual.save_png(&actual_fail_path);
        if let Some(diff) = diff_pixmap {
            let _ = diff.save_png(&diff_fail_path);
        }

        panic!(
            "Golden visual regression mismatch! Mismatched pixels: {} / {} ({:.2}%), max delta: {}. \
             Inspect diff images at {:?}",
            report.mismatched_pixels,
            report.total_pixels,
            report.mismatch_percentage,
            report.max_channel_delta,
            diff_dir
        );
    }
}
