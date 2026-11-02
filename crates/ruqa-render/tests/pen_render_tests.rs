//! Integration tests for freehand pen stroke rasterization.

use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Size, Viewport};
use ruqa_core::scene::Scene;
use ruqa_render::pen_render::StrokeOutlineBuilder;
use ruqa_render::shape_render::{render_element, render_scene_viewport};
use tiny_skia::{Color, Pixmap, Transform};

#[test]
fn test_stroke_outline_builder_buffer_reuse() {
    let mut builder = StrokeOutlineBuilder::new();

    // Frame 1: 5 points
    let pts1 = [
        Point::new(0.0, 0.0),
        Point::new(10.0, 10.0),
        Point::new(20.0, 15.0),
        Point::new(30.0, 20.0),
        Point::new(40.0, 25.0),
    ];
    let press1 = [0.5, 0.6, 0.7, 0.8, 0.9];
    let path1 = builder.build_outline(&pts1, &press1, 4.0);
    assert!(path1.is_some());

    // Frame 2: 7 points (reusing allocated internal buffers)
    let pts2 = [
        Point::new(0.0, 0.0),
        Point::new(10.0, 10.0),
        Point::new(20.0, 15.0),
        Point::new(30.0, 20.0),
        Point::new(40.0, 25.0),
        Point::new(50.0, 30.0),
        Point::new(60.0, 35.0),
    ];
    let press2 = [0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.0];
    let path2 = builder.build_outline(&pts2, &press2, 4.0);
    assert!(path2.is_some());
}

#[test]
fn test_render_pen_stroke_via_scene_and_viewport() {
    let mut scene = Scene::new();
    let stroke = Element::Stroke {
        id: 1,
        points: vec![
            Point::new(50.0, 50.0),
            Point::new(100.0, 50.0),
            Point::new(150.0, 50.0),
        ],
        pressures: vec![1.0, 1.0, 1.0],
        style: ElementStyle {
            stroke_color: "#00FF00".into(),
            stroke_width: 10.0,
            ..Default::default()
        },
    };
    scene.push(stroke);

    let viewport = Viewport::new(Point::new(100.0, 50.0), 1.0);
    let window_size = Size::new(200.0, 200.0);

    let mut pixmap = Pixmap::new(200, 200).expect("valid pixmap");
    pixmap.fill(Color::BLACK);

    render_scene_viewport(&scene, &viewport, window_size, &mut pixmap.as_mut());

    // Canvas center (100, 100) corresponds to world coordinate (100, 50), which is in the stroke center
    let center_pixel = pixmap.pixel(100, 100).expect("pixel within bounds");
    assert_eq!(center_pixel.green(), 255);
    assert_eq!(center_pixel.red(), 0);
    assert_eq!(center_pixel.blue(), 0);
}

#[test]
fn test_render_element_dispatches_stroke_correctly() {
    let stroke = Element::Stroke {
        id: 42,
        points: vec![Point::new(30.0, 30.0), Point::new(70.0, 30.0)],
        pressures: vec![0.8, 0.8],
        style: ElementStyle {
            stroke_color: "#0000FF".into(),
            stroke_width: 6.0,
            ..Default::default()
        },
    };

    let mut pixmap = Pixmap::new(100, 100).expect("valid pixmap");
    pixmap.fill(Color::TRANSPARENT);

    render_element(&stroke, &mut pixmap.as_mut(), Transform::identity());

    // Midpoint of stroke (50, 30) should be colored blue
    let mid_pixel = pixmap.pixel(50, 30).expect("pixel within bounds");
    assert_eq!(mid_pixel.blue(), 255);
    assert_eq!(mid_pixel.red(), 0);
    assert_eq!(mid_pixel.green(), 0);
}
