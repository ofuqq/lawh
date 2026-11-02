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
