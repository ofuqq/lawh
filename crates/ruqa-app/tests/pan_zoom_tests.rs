//! Integration tests for infinite canvas pan and zoom interactions in `ruqa-app`.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::geometry::{Point, Size};
use winit::event::{ElementState, MouseButton, MouseScrollDelta};

#[test]
fn middle_mouse_drag_pans_viewport() {
    let mut app = WhiteboardApp::new();
    app.handle_cursor_moved(100.0, 100.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Middle);

    // Drag right 50px, down 30px
    app.handle_cursor_moved(150.0, 130.0);
    assert_eq!(app.viewport.pan, Point::new(-50.0, -30.0));
    assert!(app.pointer.panned_during_press);

    app.handle_mouse_input(ElementState::Released, MouseButton::Middle);
    assert!(!app.pointer.middle_pressed);
    // panned_during_press stays true until the next press to suppress phantom clicks
    assert!(app.pointer.panned_during_press);
}

#[test]
fn space_left_drag_pans_viewport_identically() {
    let mut app = WhiteboardApp::new();
    app.handle_cursor_moved(200.0, 200.0);

    // Hold Space
    app.pointer.space_held = true;
    // Press Left button
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(!app.pointer.panned_during_press);

    // Drag left 40px, up 20px
    app.handle_cursor_moved(160.0, 180.0);
    assert_eq!(app.viewport.pan, Point::new(40.0, 20.0));
    assert!(app.pointer.panned_during_press);

    // Release Left button
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
    assert!(!app.pointer.left_pressed);
    assert!(app.pointer.panned_during_press);
}

#[test]
fn focus_loss_resets_inputs() {
    let mut app = WhiteboardApp::new();
    app.pointer.left_pressed = true;
    app.pointer.middle_pressed = true;
    app.pointer.space_held = true;
    app.pointer.panned_during_press = true;

    app.handle_focused(false);
    assert!(!app.pointer.left_pressed);
    assert!(!app.pointer.middle_pressed);
    assert!(!app.pointer.space_held);
    assert!(!app.pointer.panned_during_press);
}

#[test]
fn mouse_wheel_zooms_anchored_at_cursor() {
    let mut app = WhiteboardApp::new();
    // Cursor at (400, 300)
    app.handle_cursor_moved(400.0, 300.0);

    let window_size = Size::new(
        app.physical_size.width as f32,
        app.physical_size.height as f32,
    );
    let anchor = Point::new(400.0, 300.0);
    let world_before = app.viewport.screen_to_world(anchor, window_size);

    // Zoom in with line delta
    app.handle_mouse_wheel(MouseScrollDelta::LineDelta(0.0, 1.0));
    assert!((app.viewport.zoom - 1.15).abs() < 1e-5);

    let world_after = app.viewport.screen_to_world(anchor, window_size);
    assert!((world_before.x - world_after.x).abs() < 1e-4);
    assert!((world_before.y - world_after.y).abs() < 1e-4);
}
