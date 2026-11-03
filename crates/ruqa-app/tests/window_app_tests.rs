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

#[test]
fn test_idle_app_presents_zero_frames_and_avoids_spurious_redraws() {
    let mut app = WhiteboardApp::new();
    assert_eq!(app.frames_presented, 0);

    // Initial cursor move onto neutral canvas without clicking returns false
    let moved_neutral = app.handle_cursor_moved(400.0, 400.0);
    assert!(
        !moved_neutral,
        "Neutral cursor motion without buttons or hover change must not request redraw"
    );

    // Subsequent motions across neutral canvas also return false
    for offset in 1..=10 {
        let moved = app.handle_cursor_moved(400.0 + f64::from(offset), 400.0 + f64::from(offset));
        assert!(
            !moved,
            "Idle cursor motion across empty canvas must not request redraw"
        );
    }
    assert_eq!(
        app.frames_presented, 0,
        "Idle state must present exactly zero frames"
    );
}

#[test]
fn test_hover_state_change_triggers_redraw_demand_without_continuous_loop() {
    let mut app = WhiteboardApp::new();
    // Toolbar is at top center: Y in 16.0..60.0, X centered
    let toolbar_center_x = f64::from(app.logical_size().width) * 0.5;

    // Moving onto a toolbar button changes hover state -> returns true
    let hover_entered = app.handle_cursor_moved(toolbar_center_x, 36.0);
    assert!(
        hover_entered,
        "Entering toolbar button must request redraw to show hover outline"
    );

    // Moving slightly within the same button does NOT change hover state -> returns false
    let stay_in_button = app.handle_cursor_moved(toolbar_center_x + 1.0, 36.0);
    assert!(
        !stay_in_button,
        "Moving within the same button without state change must not request redraw"
    );
}

#[test]
fn test_clean_document_autosave_deadline_is_none() {
    let app = WhiteboardApp::new();
    assert!(!app.is_dirty);
    assert!(
        app.next_autosave_deadline().is_none(),
        "Clean document must not set autosave wakeups, allowing event loop to sleep in ControlFlow::Wait"
    );
}
