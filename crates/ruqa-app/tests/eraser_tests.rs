//! Integration tests for the continuous stroke and element eraser tool and undo/redo lifecycle.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::tool::Tool;
use winit::event::{ElementState, MouseButton};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle::default(),
    }
}

fn sample_stroke(id: u64, pts: Vec<Point>) -> Element {
    let n = pts.len();
    Element::Stroke {
        id,
        points: pts,
        pressures: vec![0.5; n],
        style: ElementStyle::default(),
    }
}

// In default Viewport (pan=(0,0), zoom=1.0, window=(1280, 800)),
// screen center is (640, 400).
// screen_to_world(p) = p - (640, 400).
// world_to_screen(w) = w + (640, 400).
fn world_to_screen(world_pt: Point) -> (f64, f64) {
    (f64::from(world_pt.x + 640.0), f64::from(world_pt.y + 400.0))
}

fn click_world(app: &mut WhiteboardApp, world_pt: Point) {
    let (sx, sy) = world_to_screen(world_pt);
    app.handle_cursor_moved(sx, sy);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
}

#[test]
fn test_single_click_erases_shape_and_undo_restores() {
    let mut app = WhiteboardApp::new();
    app.active_tool = Tool::Eraser;

    let rect = sample_rect(1, 0.0, 0.0, 60.0, 40.0);
    app.scene.push(rect);
    assert_eq!(app.scene.len(), 1);

    // Click on the rectangle at world (30, 20)
    click_world(&mut app, Point::new(30.0, 20.0));

    // Element is immediately removed
    assert_eq!(app.scene.len(), 0);
    assert!(app.scene.get(1).is_none());
    assert!(app.history.can_undo());
    assert!(app.is_dirty);

    // Ctrl+Z restores the element
    app.history.undo(&mut app.scene);
    assert_eq!(app.scene.len(), 1);
    assert!(app.scene.get(1).is_some());

    // Redo deletes it again
    app.history.redo(&mut app.scene);
    assert_eq!(app.scene.len(), 0);
}

#[test]
fn test_continuous_drag_erases_multiple_elements_in_single_undoable_command() {
    let mut app = WhiteboardApp::new();
    app.active_tool = Tool::Eraser;

    // Place 3 shapes along horizontal axis:
    // Rect 1 at x: 20..80, y: -20..20
    app.scene.push(sample_rect(10, 20.0, -20.0, 60.0, 40.0));
    // Rect 2 at x: 120..180, y: -20..20
    app.scene.push(sample_rect(20, 120.0, -20.0, 60.0, 40.0));
    // Rect 3 at x: 220..280, y: -20..20
    app.scene.push(sample_rect(30, 220.0, -20.0, 60.0, 40.0));
    assert_eq!(app.scene.len(), 3);

    // Start drag at world (0, 0)
    let (s0_x, s0_y) = world_to_screen(Point::new(0.0, 0.0));
    app.handle_cursor_moved(s0_x, s0_y);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert_eq!(app.scene.len(), 3);

    // Drag through rect 1 at world (50, 0)
    let (s1_x, s1_y) = world_to_screen(Point::new(50.0, 0.0));
    app.handle_cursor_moved(s1_x, s1_y);
    assert_eq!(app.scene.len(), 2);
    assert!(app.scene.get(10).is_none());

    // Drag through rect 2 at world (150, 0)
    let (s2_x, s2_y) = world_to_screen(Point::new(150.0, 0.0));
    app.handle_cursor_moved(s2_x, s2_y);
    assert_eq!(app.scene.len(), 1);
    assert!(app.scene.get(20).is_none());

    // Drag through rect 3 at world (250, 0)
    let (s3_x, s3_y) = world_to_screen(Point::new(250.0, 0.0));
    app.handle_cursor_moved(s3_x, s3_y);
    assert_eq!(app.scene.len(), 0);
    assert!(app.scene.get(30).is_none());

    // Release mouse at world (300, 0)
    let (s4_x, s4_y) = world_to_screen(Point::new(300.0, 0.0));
    app.handle_cursor_moved(s4_x, s4_y);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // Exactly 1 atomic delete command was pushed to history
    assert_eq!(app.history.undo_len(), 1);

    // A single undo restores all 3 elements with original indices preserved
    app.history.undo(&mut app.scene);
    assert_eq!(app.scene.len(), 3);
    assert_eq!(app.scene.position(10), Some(0));
    assert_eq!(app.scene.position(20), Some(1));
    assert_eq!(app.scene.position(30), Some(2));

    // Redo re-deletes all 3 elements
    app.history.redo(&mut app.scene);
    assert_eq!(app.scene.len(), 0);
}

#[test]
fn test_fast_cursor_motion_interpolates_drag_trail_without_missing_elements() {
    let mut app = WhiteboardApp::new();
    app.active_tool = Tool::Eraser;

    // Thin line at x = 100, y from -50 to 50
    let line = Element::Line {
        id: 42,
        start: Point::new(100.0, -50.0),
        end: Point::new(100.0, 50.0),
        style: ElementStyle::default(),
    };
    app.scene.push(line);
    assert_eq!(app.scene.len(), 1);

    // Large single cursor jump: from world (20, 0) to world (180, 0) -> dx = 160px
    let (s1_x, s1_y) = world_to_screen(Point::new(20.0, 0.0));
    let (s2_x, s2_y) = world_to_screen(Point::new(180.0, 0.0));

    app.handle_cursor_moved(s1_x, s1_y);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    // Instant jump across the line
    app.handle_cursor_moved(s2_x, s2_y);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // The line was intercepted and erased along the trail interpolation
    assert_eq!(app.scene.len(), 0);
    assert!(app.scene.get(42).is_none());
    assert!(app.history.can_undo());
}

#[test]
fn test_erasing_freehand_pen_strokes() {
    let mut app = WhiteboardApp::new();
    app.active_tool = Tool::Eraser;

    let stroke = sample_stroke(
        99,
        vec![
            Point::new(10.0, 10.0),
            Point::new(20.0, 30.0),
            Point::new(40.0, 50.0),
            Point::new(60.0, 80.0),
        ],
    );
    app.scene.push(stroke);
    assert_eq!(app.scene.len(), 1);

    // Click near one of the stroke points at world (20, 30)
    click_world(&mut app, Point::new(20.0, 30.0));

    assert_eq!(app.scene.len(), 0);
    assert!(app.scene.get(99).is_none());

    // Undo restores the exact freehand stroke
    app.history.undo(&mut app.scene);
    assert_eq!(app.scene.len(), 1);
    if let Some(Element::Stroke { points, .. }) = app.scene.get(99) {
        assert_eq!(points.len(), 4);
    } else {
        panic!("restored element must be a Stroke");
    }
}

#[test]
fn test_empty_drag_does_not_commit_history_command() {
    let mut app = WhiteboardApp::new();
    app.active_tool = Tool::Eraser;
    app.scene.push(sample_rect(1, 200.0, 200.0, 50.0, 50.0));

    // Drag in empty area: from world (-200, -200) to (-100, -100)
    let (s1_x, s1_y) = world_to_screen(Point::new(-200.0, -200.0));
    let (s2_x, s2_y) = world_to_screen(Point::new(-100.0, -100.0));

    app.handle_cursor_moved(s1_x, s1_y);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(s2_x, s2_y);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    // Scene untouched and no undo command created
    assert_eq!(app.scene.len(), 1);
    assert!(!app.history.can_undo());
}

#[test]
fn test_erasing_selected_element_clears_selection() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));
    app.selection.set_selection(vec![1]);
    assert!(app.selection.selected_ids().contains(&1));

    app.active_tool = Tool::Eraser;
    click_world(&mut app, Point::new(50.0, 50.0));

    assert_eq!(app.scene.len(), 0);
    assert!(!app.selection.selected_ids().contains(&1));
    assert!(app.selection.is_empty());
}
