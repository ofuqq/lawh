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

    // Click empty canvas at screen (100, 100) -> world (-540, -300)
    app.handle_cursor_moved(100.0, 100.0);
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
    assert!(app.selection.active_marquee.is_some());

    // Drag to world (0, -50) -> screen (640, 350)
    app.handle_cursor_moved(640.0, 350.0);
    assert!(app.selection.active_marquee.unwrap().is_significant());

    // Release mouse
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
    assert!(app.selection.active_marquee.is_none());

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
    assert!(app.selection.active_marquee.is_none());
    assert!(app.selection.selection.is_empty());
    assert!(app.pointer.panned_during_press);
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
