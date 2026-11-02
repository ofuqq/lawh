//! Integration tests for the floating HUD toolbar, hit-testing, and tool switching.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use ruqa_core::tool::Tool;
use winit::event::{ElementState, MouseButton};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle::default(),
    }
}

fn click_at(app: &mut WhiteboardApp, x: f64, y: f64) {
    app.handle_cursor_moved(x, y);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
}

#[test]
fn test_default_tool_is_select() {
    let app = WhiteboardApp::new();
    assert_eq!(app.active_tool, Tool::Select);
    assert_eq!(app.hovered_tool, None);
    assert!(!app.toolbar_pressed);
}

#[test]
fn test_click_toolbar_buttons_switches_active_tool() {
    let mut app = WhiteboardApp::new();
    let layout = app.toolbar_layout();

    for (i, &tool) in Tool::ALL.iter().enumerate() {
        let btn = layout.button_rect(i);
        let cx = f64::from(btn.min.x + btn.width() * 0.5);
        let cy = f64::from(btn.min.y + btn.height() * 0.5);

        click_at(&mut app, cx, cy);
        assert_eq!(app.active_tool, tool);
        assert!(!app.toolbar_pressed);
    }
}

#[test]
fn test_toolbar_clicks_do_not_interact_with_canvas_underneath() {
    let mut app = WhiteboardApp::new();
    let layout = app.toolbar_layout();

    // Place an element on the canvas positioned directly behind the toolbar
    let rect_behind = sample_rect(999, -50.0, -350.0, 100.0, 80.0);
    app.scene.push(rect_behind);

    // Compute screen coordinates for the Pen tool button
    let pen_btn = layout.button_rect(1);
    let click_x = f64::from(pen_btn.min.x + pen_btn.width() * 0.5);
    let click_y = f64::from(pen_btn.min.y + pen_btn.height() * 0.5);

    // Click on the toolbar
    click_at(&mut app, click_x, click_y);

    // Active tool changed to Pen
    assert_eq!(app.active_tool, Tool::Pen);

    // Canvas element underneath was NOT selected
    assert!(
        app.selection.is_empty(),
        "canvas element behind toolbar must not be selected"
    );
    assert!(
        app.selection.active_marquee().is_none(),
        "toolbar click must not start marquee drag"
    );
}

#[test]
fn test_hovering_toolbar_updates_hovered_tool() {
    let mut app = WhiteboardApp::new();
    let layout = app.toolbar_layout();

    let rect_rect = layout.button_rect(2); // Rectangle button
    let cx = f64::from(rect_rect.min.x + rect_rect.width() * 0.5);
    let cy = f64::from(rect_rect.min.y + rect_rect.height() * 0.5);

    app.handle_cursor_moved(cx, cy);
    assert_eq!(app.hovered_tool, Some(Tool::Rectangle));

    // Move away to empty canvas
    app.handle_cursor_moved(100.0, 500.0);
    assert_eq!(app.hovered_tool, None);
}

#[test]
fn test_toolbar_recalculates_on_window_resize() {
    let mut app = WhiteboardApp::new();
    let l1 = app.toolbar_layout();

    // Simulate window resizing to 1920x1080
    app.physical_size = winit::dpi::PhysicalSize::new(1920, 1080);
    let l2 = app.toolbar_layout();

    assert_eq!(l1.bounds.width(), l2.bounds.width());
    assert!(
        l2.bounds.min.x > l1.bounds.min.x,
        "wider window centers toolbar further to the right"
    );
}

fn press_key(app: &mut WhiteboardApp, key: winit::keyboard::Key) {
    app.handle_key(&key, ElementState::Pressed);
    app.handle_key(&key, ElementState::Released);
}

#[test]
fn test_single_key_shortcuts_switch_active_tool() {
    use winit::keyboard::Key;

    let mut app = WhiteboardApp::new();
    assert_eq!(app.active_tool, Tool::Select);

    press_key(&mut app, Key::Character("p".into()));
    assert_eq!(app.active_tool, Tool::Pen);

    press_key(&mut app, Key::Character("r".into()));
    assert_eq!(app.active_tool, Tool::Rectangle);

    press_key(&mut app, Key::Character("o".into()));
    assert_eq!(app.active_tool, Tool::Ellipse);

    press_key(&mut app, Key::Character("c".into()));
    assert_eq!(app.active_tool, Tool::Ellipse);

    press_key(&mut app, Key::Character("d".into()));
    assert_eq!(app.active_tool, Tool::Diamond);

    press_key(&mut app, Key::Character("l".into()));
    assert_eq!(app.active_tool, Tool::Line);

    press_key(&mut app, Key::Character("a".into()));
    assert_eq!(app.active_tool, Tool::Arrow);

    press_key(&mut app, Key::Character("e".into()));
    assert_eq!(app.active_tool, Tool::Eraser);

    press_key(&mut app, Key::Character("t".into()));
    assert_eq!(app.active_tool, Tool::Text);

    press_key(&mut app, Key::Character("v".into()));
    assert_eq!(app.active_tool, Tool::Select);

    // Uppercase shortcuts
    press_key(&mut app, Key::Character("R".into()));
    assert_eq!(app.active_tool, Tool::Rectangle);
    press_key(&mut app, Key::Character("P".into()));
    assert_eq!(app.active_tool, Tool::Pen);
}

#[test]
fn test_shortcuts_suppressed_when_editing_text() {
    use winit::keyboard::Key;

    let mut app = WhiteboardApp::new();
    assert_eq!(app.active_tool, Tool::Select);

    // Enable text editing mode
    app.is_editing_text = true;

    // Pressing 'r' or 'p' must NOT switch active tool
    press_key(&mut app, Key::Character("r".into()));
    assert_eq!(app.active_tool, Tool::Select);

    press_key(&mut app, Key::Character("p".into()));
    assert_eq!(app.active_tool, Tool::Select);

    // Disable text editing mode
    app.is_editing_text = false;

    // Now single-key shortcut switches tool
    press_key(&mut app, Key::Character("r".into()));
    assert_eq!(app.active_tool, Tool::Rectangle);
}

#[test]
fn test_ctrl_combinations_do_not_switch_tools() {
    use winit::keyboard::{Key, NamedKey};

    let mut app = WhiteboardApp::new();
    app.active_tool = Tool::Select;

    // Press Ctrl+D (duplicate) with Ctrl held down
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("d".into()));
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Released);

    // Should still be Select, not Diamond!
    assert_eq!(app.active_tool, Tool::Select);

    // Press Ctrl+C (copy)
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Pressed);
    press_key(&mut app, Key::Character("c".into()));
    app.handle_key(&Key::Named(NamedKey::Control), ElementState::Released);

    // Should still be Select, not Ellipse!
    assert_eq!(app.active_tool, Tool::Select);
}
