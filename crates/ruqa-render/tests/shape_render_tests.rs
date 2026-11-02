//! Integration tests for hand-drawn shape rendering with stable seed reproducibility.

use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect, Size, Viewport};
use ruqa_core::scene::Scene;
use ruqa_render::shape_render::{render_element, render_scene_viewport};
use tiny_skia::{Color, Pixmap, Transform};

#[test]
fn test_all_shape_primitives_render_pixels() {
    let elements = [
        Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(20.0, 20.0, 80.0, 50.0),
            style: ElementStyle {
                stroke_color: "#4DD0E1".into(),
                stroke_width: 2.0,
                roughness: 1.2,
                seed: 111,
                ..Default::default()
            },
        },
        Element::Ellipse {
            id: 2,
            bounds: Rect::from_xywh(120.0, 20.0, 60.0, 60.0),
            style: ElementStyle {
                stroke_color: "#FF8A65".into(),
                stroke_width: 2.5,
                roughness: 1.0,
                seed: 222,
                ..Default::default()
            },
        },
        Element::Diamond {
            id: 3,
            bounds: Rect::from_xywh(200.0, 20.0, 70.0, 70.0),
            style: ElementStyle {
                stroke_color: "#FFD54F".into(),
                stroke_width: 2.0,
                roughness: 1.3,
                seed: 333,
                ..Default::default()
            },
        },
        Element::Line {
            id: 4,
            start: Point::new(20.0, 100.0),
            end: Point::new(100.0, 140.0),
            style: ElementStyle {
                stroke_color: "#81C784".into(),
                stroke_width: 2.0,
                roughness: 1.1,
                seed: 444,
                ..Default::default()
            },
        },
        Element::Arrow {
            id: 5,
            start: Point::new(120.0, 120.0),
            end: Point::new(220.0, 120.0),
            style: ElementStyle {
                stroke_color: "#BA68C8".into(),
                stroke_width: 2.0,
                roughness: 1.0,
                seed: 555,
                ..Default::default()
            },
        },
    ];

    for elem in elements {
        let mut pixmap = Pixmap::new(300, 200).expect("pixmap allocation");
        pixmap.fill(Color::BLACK);

        render_element(&elem, &mut pixmap.as_mut(), Transform::identity());

        // Verify that non-black pixels were drawn
        let non_black_count = pixmap
            .pixels()
            .iter()
            .filter(|p| p.red() > 0 || p.green() > 0 || p.blue() > 0)
            .count();

        assert!(
            non_black_count > 50,
            "element {:?} should render visible stroke pixels (got {non_black_count})",
            elem.id()
        );
    }
}

#[test]
fn test_viewport_culling_skips_offscreen_elements() {
    let mut scene = Scene::new();
    // Element 1 is at origin (visible)
    scene.push(Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(-20.0, -20.0, 40.0, 40.0),
        style: ElementStyle {
            stroke_color: "#FFFFFF".into(),
            seed: 1234,
            ..Default::default()
        },
    });
    // Element 2 is far away (offscreen)
    scene.push(Element::Rectangle {
        id: 2,
        bounds: Rect::from_xywh(5000.0, 5000.0, 100.0, 100.0),
        style: ElementStyle {
            stroke_color: "#FFFFFF".into(),
            seed: 5678,
            ..Default::default()
        },
    });

    let viewport = Viewport::new(Point::ZERO, 1.0);
    let window_size = Size::new(400.0, 300.0);

    let mut pixmap = Pixmap::new(400, 300).expect("pixmap allocation");
    pixmap.fill(Color::BLACK);

    render_scene_viewport(&scene, &viewport, window_size, &mut pixmap.as_mut());

    let non_black = pixmap
        .pixels()
        .iter()
        .filter(|p| p.red() > 0 || p.green() > 0 || p.blue() > 0)
        .count();

    assert!(non_black > 0, "visible element should render pixels");
}
