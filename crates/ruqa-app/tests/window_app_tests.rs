//! Window application state and constraints tests.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::geometry::Viewport;

#[test]
fn whiteboard_app_initial_state() {
    let app = WhiteboardApp::new();
    assert!(app.window.is_none());
    assert!(app.presenter.is_none());
    assert_eq!(app.frames_presented, 0);
    assert_eq!(app.scale_factor, 1.0);
    assert_eq!(app.physical_size.width, 1280);
    assert_eq!(app.physical_size.height, 800);
    assert_eq!(app.viewport, Viewport::default());
    assert!(app.selection.selection.is_empty());
}

#[test]
fn window_dimensions_constraints() {
    let app = WhiteboardApp::new();
    assert!(app.physical_size.width >= 640);
    assert!(app.physical_size.height >= 480);
}
