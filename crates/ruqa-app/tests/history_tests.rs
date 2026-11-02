//! Integration tests for undo/redo history and keyboard shortcuts in `ruqa-app`.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::{Key, NamedKey};

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

fn press_key(app: &mut WhiteboardApp, key: Key) {
    app.handle_key(&key, ElementState::Pressed);
    app.handle_key(&key, ElementState::Released);
}

#[test]
fn test_undo_and_redo_translation_via_keyboard() {
    let mut app = WhiteboardApp::new();
    // Rect at world (0, 0, 100, 100). Screen center is (640, 400).
    // Bounds in screen: [640..740, 400..500]
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));
    app.selection.selection.select_single(1);

    // Drag from screen (640, 420) to (690, 450) -> world dx=+50, dy=+30
    app.handle_cursor_moved(640.0, 420.0);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(690.0, 450.0);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    assert_eq!(
        app.scene.get(1).unwrap().bounds(),
        Rect::from_xywh(50.0, 30.0, 100.0, 100.0)
    );
    assert_eq!(app.history.undo_len(), 1);

    // Press Ctrl (hold down)
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);

    // Press 'z' -> Undo
    press_key(&mut app, Key::Character("z".into()));
    assert_eq!(
        app.scene.get(1).unwrap().bounds(),
        Rect::from_xywh(0.0, 0.0, 100.0, 100.0)
    );
    assert_eq!(app.history.redo_len(), 1);

    // Press 'y' -> Redo
    press_key(&mut app, Key::Character("y".into()));
    assert_eq!(
        app.scene.get(1).unwrap().bounds(),
        Rect::from_xywh(50.0, 30.0, 100.0, 100.0)
    );

    // Undo again with Ctrl+Z
    press_key(&mut app, Key::Character("z".into()));
    assert_eq!(
        app.scene.get(1).unwrap().bounds(),
        Rect::from_xywh(0.0, 0.0, 100.0, 100.0)
    );

    // Redo with Ctrl+Shift+Z
    app.handle_key(&Key::Named(NamedKey::Shift), ElementState::Pressed);
    press_key(&mut app, Key::Character("z".into()));
    assert_eq!(
        app.scene.get(1).unwrap().bounds(),
        Rect::from_xywh(50.0, 30.0, 100.0, 100.0)
    );

    // Release modifiers
    app.handle_key(&Key::Named(NamedKey::Shift), ElementState::Released);
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Released);
}

#[test]
fn test_undo_and_redo_resize_via_keyboard() {
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
    let br = ruqa_core::geometry::HandleKind::BottomRight.point_on_rect(screen_rect);

    // Drag BottomRight handle +50px right and +50px down
    app.handle_cursor_moved(f64::from(br.x), f64::from(br.y));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_cursor_moved(f64::from(br.x + 50.0), f64::from(br.y + 50.0));
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);

    let resized_width = app.scene.get(1).unwrap().bounds().width();
    assert!(resized_width > 140.0);
    assert_eq!(app.history.undo_len(), 1);

    // Undo with Ctrl+Z
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("z".into()));
    assert_eq!(app.scene.get(1).unwrap().bounds().width(), 100.0);

    // Redo with Ctrl+Y
    press_key(&mut app, Key::Character("y".into()));
    assert_eq!(app.scene.get(1).unwrap().bounds().width(), resized_width);
}

#[test]
fn test_delete_key_removes_selected_elements_and_undo_restores() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));
    app.scene.push(sample_rect(2, 60.0, 0.0, 50.0, 50.0));
    app.selection.selection.select_single(1);

    // Press Delete key
    press_key(&mut app, Key::Named(NamedKey::Delete));

    assert_eq!(app.scene.len(), 1);
    assert!(app.scene.get(1).is_none());
    assert!(app.scene.get(2).is_some());
    assert!(app.selection.is_empty());
    assert_eq!(app.history.undo_len(), 1);

    // Ctrl+Z restores element 1
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("z".into()));
    assert_eq!(app.scene.len(), 2);
    assert!(app.scene.get(1).is_some());
    assert_eq!(app.scene.position(1), Some(0));

    // Ctrl+Y deletes element 1 again
    press_key(&mut app, Key::Character("y".into()));
    assert_eq!(app.scene.len(), 1);
    assert!(app.scene.get(1).is_none());
}

#[test]
fn test_backspace_key_removes_multiple_elements_preserving_z_order() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(10, 0.0, 0.0, 50.0, 50.0));
    app.scene.push(sample_rect(20, 60.0, 0.0, 50.0, 50.0));
    app.scene.push(sample_rect(30, 120.0, 0.0, 50.0, 50.0));

    // Multi-select elements 10 and 20
    app.selection.selection.set_selection([10, 20]);

    // Press Backspace key
    press_key(&mut app, Key::Named(NamedKey::Backspace));

    assert_eq!(app.scene.len(), 1);
    assert_eq!(
        app.scene.get(30),
        Some(&sample_rect(30, 120.0, 0.0, 50.0, 50.0))
    );
    assert!(app.selection.is_empty());

    // Ctrl+Z restores both elements in original z-order
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("z".into()));

    assert_eq!(app.scene.len(), 3);
    assert_eq!(app.scene.position(10), Some(0));
    assert_eq!(app.scene.position(20), Some(1));
    assert_eq!(app.scene.position(30), Some(2));
}

#[test]
fn test_delete_with_empty_selection_is_noop() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));

    // No selection
    press_key(&mut app, Key::Named(NamedKey::Delete));
    press_key(&mut app, Key::Named(NamedKey::Backspace));

    assert_eq!(app.scene.len(), 1);
    assert_eq!(app.history.undo_len(), 0);
}
