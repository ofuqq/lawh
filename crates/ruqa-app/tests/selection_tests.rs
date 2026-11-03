//! Integration tests for selection tool click and marquee interactions in `ruqa-app`.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use winit::event::{ElementState, MouseButton};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_width: 2.0,
            ..Default::default()
        },
    }
}

#[test]
fn test_app_click_selection_and_empty_canvas_deselect() {
    let mut app = WhiteboardApp::new();
    // In default Viewport (pan=(0,0), zoom=1.0, window=(1280, 800)),
    // window center is (640, 400).
    // world_pt = screen_pt - (640, 400).
    // Place a rect centered around world (0, 0): [-50, -50, 100, 100]
    app.scene.push(sample_rect(1, -50.0, -50.0, 100.0, 100.0));
    // Another rect at world (200, 0): [150, -50, 100, 100]
    app.scene.push(sample_rect(2, 150.0, -50.0, 100.0, 100.0));

    // Screen coordinate for world boundary of rect 1:
    // (x=-50, y=-50) -> screen (590, 350)
    app.handle_cursor_moved(590.0, 350.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    assert_eq!(app.selection.selection.len(), 1);
    assert!(app.selection.selection.is_selected(1));

    // Shift-click rect 2 at world boundary: (x=150, y=-50) -> screen (790, 350)
    app.pointer.shift_held = true;

    app.handle_cursor_moved(790.0, 350.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // Both 1 and 2 selected
    assert_eq!(app.selection.selection.len(), 2);
    assert!(app.selection.selection.is_selected(1));
    assert!(app.selection.selection.is_selected(2));

    // Release Shift
    app.pointer.shift_held = false;

    // Click empty canvas at screen (300, 300) -> world (-340, -100)
    app.handle_cursor_moved(300.0, 300.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // Selection cleared
    assert!(app.selection.selection.is_empty());
}

#[test]
fn test_app_marquee_drag_selection() {
    let mut app = WhiteboardApp::new();
    // Rect 1 at world (-100, -100): [-120, -120, 40, 40]
    app.scene.push(sample_rect(10, -120.0, -120.0, 40.0, 40.0));
    // Rect 2 at world (-50, -100): [-70, -120, 40, 40]
    app.scene.push(sample_rect(20, -70.0, -120.0, 40.0, 40.0));
    // Rect 3 far away at world (500, 500)
    app.scene.push(sample_rect(30, 500.0, 500.0, 40.0, 40.0));

    // Start marquee drag at world (-150, -150) -> screen (490, 250)
    app.handle_cursor_moved(490.0, 250.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(app.selection.active_marquee().is_some());

    // Drag to world (0, -50) -> screen (640, 350)
    app.handle_cursor_moved(640.0, 350.0);
    assert!(app.selection.active_marquee().unwrap().is_significant());

    // Release mouse
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
    assert!(app.selection.active_marquee().is_none());

    // Both rects 10 and 20 selected, rect 30 not selected
    assert_eq!(app.selection.selection.len(), 2);
    assert!(app.selection.selection.is_selected(10));
    assert!(app.selection.selection.is_selected(20));
    assert!(!app.selection.selection.is_selected(30));
}

#[test]
fn test_space_pan_does_not_trigger_marquee() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));

    // Space held for panning
    app.pointer.space_held = true;

    // Left click and drag
    app.handle_cursor_moved(100.0, 100.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(150.0, 150.0);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // Marquee should NOT be active and selection should remain empty
    assert!(app.selection.active_marquee().is_none());
    assert!(app.selection.selection.is_empty());
    assert!(app.pointer.panned_during_press);
}

#[test]
fn test_drag_translate_selected_shapes_in_app() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, -50.0, -50.0, 100.0, 100.0));
    app.scene.push(sample_rect(2, 100.0, -50.0, 100.0, 100.0));

    app.selection.selection.set_selection([1, 2]);

    // Click on boundary of rect 1 at world (-50, -20) -> screen (590, 380)
    // away from the LeftCenter selection handle at (590, 400)
    app.handle_cursor_moved(590.0, 380.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(app.selection.is_translating());

    // Drag 50px right and 30px down -> screen (640, 410)
    app.handle_cursor_moved(640.0, 410.0);

    // Both shapes translated by (+50, +30)
    assert_eq!(
        app.scene.get(1).unwrap().bounds(),
        Rect::from_xywh(0.0, -20.0, 100.0, 100.0)
    );
    assert_eq!(
        app.scene.get(2).unwrap().bounds(),
        Rect::from_xywh(150.0, -20.0, 100.0, 100.0)
    );

    // Release mouse
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
    assert!(!app.selection.is_translating());
}

#[test]
fn test_selection_bounds_aggregation_in_app() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
    app.scene.push(sample_rect(2, 100.0, 100.0, 50.0, 50.0));

    // Initially none selected
    assert!(app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .is_none());

    // Select element 1
    app.selection.selection.select_single(1);
    let bounds1 = app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .expect("element 1 bounds exist");
    assert!(bounds1.contains(ruqa_core::geometry::Point::new(10.0, 10.0)));
    assert!(!bounds1.contains(ruqa_core::geometry::Point::new(100.0, 100.0)));

    // Multi-select elements 1 and 2
    app.selection.selection.extend([2]);
    let multi_bounds = app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .expect("multi-selection bounds exist");
    assert!(multi_bounds.contains(ruqa_core::geometry::Point::new(10.0, 10.0)));
    assert!(multi_bounds.contains(ruqa_core::geometry::Point::new(150.0, 150.0)));
}

#[test]
fn test_resize_corner_handle_in_app() {
    let mut app = WhiteboardApp::new();
    // Default window 1280x800 -> center is (640, 400).
    // Rect at world (0, 0, 100, 100).
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));
    app.selection.selection.select_single(1);

    let bounds = app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .unwrap();
    let style = ruqa_render::selection_render::SelectionStyle::default();
    let screen_rect = ruqa_render::selection_render::selection_screen_rect(
        bounds,
        &app.viewport,
        app.logical_size(),
        style.padding,
    );
    let br_handle = ruqa_core::geometry::HandleKind::BottomRight.point_on_rect(screen_rect);

    // Click on BottomRight handle
    app.handle_cursor_moved(f64::from(br_handle.x), f64::from(br_handle.y));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(app.selection.is_resizing());

    // Drag further right and down by 50px
    app.handle_cursor_moved(f64::from(br_handle.x + 50.0), f64::from(br_handle.y + 30.0));
    let resized_elem = app.scene.get(1).unwrap();
    assert!(resized_elem.bounds().width() > 100.0);
    assert!(resized_elem.bounds().height() > 100.0);

    // Release mouse
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
    assert!(!app.selection.is_resizing());
}

#[test]
fn test_resize_shift_held_aspect_ratio_constraint_in_app() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));
    app.selection.selection.select_single(1);

    let bounds = app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .unwrap();
    let style = ruqa_render::selection_render::SelectionStyle::default();
    let screen_rect = ruqa_render::selection_render::selection_screen_rect(
        bounds,
        &app.viewport,
        app.logical_size(),
        style.padding,
    );
    let br_handle = ruqa_core::geometry::HandleKind::BottomRight.point_on_rect(screen_rect);

    // Hold Shift
    app.pointer.shift_held = true;

    // Click on BottomRight handle and drag unevenly (dx=80, dy=20)
    app.handle_cursor_moved(f64::from(br_handle.x), f64::from(br_handle.y));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(f64::from(br_handle.x + 80.0), f64::from(br_handle.y + 20.0));

    let resized_bounds = app.scene.get(1).unwrap().bounds();
    // Square element scaled with shift held preserves 1:1 aspect ratio
    assert!((resized_bounds.width() - resized_bounds.height()).abs() < 1e-2);

    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
}

#[test]
fn test_resize_edge_midpoint_only_scales_single_axis_in_app() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));
    app.selection.selection.select_single(1);

    let bounds = app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .unwrap();
    let style = ruqa_render::selection_render::SelectionStyle::default();
    let screen_rect = ruqa_render::selection_render::selection_screen_rect(
        bounds,
        &app.viewport,
        app.logical_size(),
        style.padding,
    );
    let rc_handle = ruqa_core::geometry::HandleKind::RightCenter.point_on_rect(screen_rect);

    // Drag RightCenter handle right by 60px and down by 40px
    app.handle_cursor_moved(f64::from(rc_handle.x), f64::from(rc_handle.y));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(f64::from(rc_handle.x + 60.0), f64::from(rc_handle.y + 40.0));

    let resized_bounds = app.scene.get(1).unwrap().bounds();
    // Width grew, but height remained strictly 100.0
    assert!(resized_bounds.width() > 100.0);
    assert!((resized_bounds.height() - 100.0).abs() < 1e-3);

    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
}

#[test]
fn test_resize_clamps_to_minimum_dimension_in_app() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));
    app.selection.selection.select_single(1);

    let bounds = app
        .scene
        .selection_bounds(app.selection.selected_ids())
        .unwrap();
    let style = ruqa_render::selection_render::SelectionStyle::default();
    let screen_rect = ruqa_render::selection_render::selection_screen_rect(
        bounds,
        &app.viewport,
        app.logical_size(),
        style.padding,
    );
    let br_handle = ruqa_core::geometry::HandleKind::BottomRight.point_on_rect(screen_rect);

    // Drag BottomRight handle far past anchor into negative coordinates
    app.handle_cursor_moved(f64::from(br_handle.x), f64::from(br_handle.y));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(
        f64::from(br_handle.x - 200.0),
        f64::from(br_handle.y - 200.0),
    );

    let clamped = app.scene.get(1).unwrap().bounds();
    assert!(clamped.width() >= ruqa_core::geometry::MIN_RESIZE_DIMENSION - 1.0);
    assert!(clamped.height() >= ruqa_core::geometry::MIN_RESIZE_DIMENSION - 1.0);

    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
}

#[test]
fn test_marquee_cleared_immediately_on_mouse_release() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));

    // Start drag on empty space to start marquee (clear of HUD overlays)
    app.handle_cursor_moved(490.0, 250.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(640.0, 350.0);

    // Marquee must be active while dragging
    assert!(
        app.selection.active_marquee().is_some(),
        "marquee should be active during drag"
    );

    // Release mouse button without moving cursor
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // Marquee must be immediately cleared
    assert!(
        app.selection.active_marquee().is_none(),
        "active marquee must be cleared immediately upon mouse button release"
    );
}
