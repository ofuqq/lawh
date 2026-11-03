//! Integration tests for sample demo board CLI flag and scene population.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::Element;
use ruqa_core::geometry::{Point, Rect, Size, Viewport};
use tiny_skia::{Color, Pixmap};

#[test]
fn test_demo_board_population_and_element_kinds() {
    let mut app = WhiteboardApp::new();
    assert_eq!(app.scene.len(), 0, "initial whiteboard scene must be empty");

    // Simulate startup with Recent Files HUD open
    app.show_welcome_hud = true;
    app.load_demo_board();
    assert!(
        !app.show_welcome_hud,
        "loading demo board must automatically dismiss welcome HUD"
    );
    assert_eq!(app.scene.len(), 6, "demo board must populate 6 primitives");

    let elements = app.scene.elements();
    let has_rect = elements
        .iter()
        .any(|e| matches!(e, Element::Rectangle { .. }));
    let has_ellipse = elements
        .iter()
        .any(|e| matches!(e, Element::Ellipse { .. }));
    let has_diamond = elements
        .iter()
        .any(|e| matches!(e, Element::Diamond { .. }));
    let has_arrow = elements.iter().any(|e| matches!(e, Element::Arrow { .. }));
    let has_line = elements.iter().any(|e| matches!(e, Element::Line { .. }));
    let has_stroke = elements.iter().any(|e| matches!(e, Element::Stroke { .. }));

    assert!(has_rect, "demo board must include a hand-drawn rectangle");
    assert!(has_ellipse, "demo board must include a hand-drawn ellipse");
    assert!(has_diamond, "demo board must include a decision diamond");
    assert!(has_arrow, "demo board must include a directional arrow");
    assert!(has_line, "demo board must include a linear line segment");
    assert!(has_stroke, "demo board must include a freehand ink stroke");
}

#[test]
fn test_demo_elements_hit_testable_and_selectable() {
    let mut app = WhiteboardApp::new();
    app.load_demo_board();

    // Hit test the rectangle centered around (-175.0, -95.0) in world coordinates.
    let rect_world = Point::new(-175.0, -95.0);
    let hit_rect = app.scene.hit_test_topmost(rect_world, 5.0);
    assert!(hit_rect.is_some(), "rectangle must be hit-testable");
    let rect_id = hit_rect.unwrap().id();

    // Select the rectangle
    app.selection.set_selection([rect_id]);
    assert!(app.selection.is_selected(rect_id));

    // Marquee drag selection enclosing the entire demo board around the origin
    let marquee_box = Rect::from_xywh(-300.0, -200.0, 600.0, 400.0);
    let enclosed_ids: Vec<u64> = app
        .scene
        .elements()
        .iter()
        .filter(|e| marquee_box.intersects(&e.bounding_box()))
        .map(Element::id)
        .collect();

    assert_eq!(
        enclosed_ids.len(),
        6,
        "marquee enclosing [-300, -200, 600, 400] must intersect all 6 demo shapes"
    );
}

#[test]
fn test_demo_board_renders_pixels_to_surface() {
    let mut app = WhiteboardApp::new();
    app.load_demo_board();

    let mut pixmap = Pixmap::new(1280, 800).expect("pixmap allocation succeeds");
    pixmap.fill(Color::BLACK);

    let viewport = Viewport::new(Point::ZERO, 1.0);
    let window_size = Size::new(1280.0, 800.0);

    let rendered_count = ruqa_render::shape_render::render_scene_viewport(
        &app.scene,
        &viewport,
        window_size,
        &mut pixmap.as_mut(),
    );

    assert_eq!(
        rendered_count, 6,
        "all 6 demo shapes should be rendered in viewport"
    );

    let non_black_pixels = pixmap
        .pixels()
        .iter()
        .filter(|p| p.red() > 0 || p.green() > 0 || p.blue() > 0)
        .count();

    assert!(
        non_black_pixels > 2000,
        "demo primitives must produce substantial rasterized vector pixels"
    );
}
