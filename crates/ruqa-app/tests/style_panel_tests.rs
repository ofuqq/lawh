//! Integration tests for the contextual styling panel, swatch/preset selection, and undoable commands.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use ruqa_core::style::{RoughnessPreset, StrokeWidthPreset, PALETTE_COLORS};
use winit::event::{ElementState, MouseButton};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.5,
            roughness: 1.2,
            ..Default::default()
        },
    }
}

fn click_at(app: &mut WhiteboardApp, x: f64, y: f64) {
    app.handle_cursor_moved(x, y);
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    app.handle_mouse_input(ElementState::Released, MouseButton::Left);
}

#[test]
fn test_default_style_panel_state() {
    let app = WhiteboardApp::new();
    assert!(app.show_style_panel);
    assert!(!app.style_panel_pressed);
    assert_eq!(app.current_style.stroke_color, PALETTE_COLORS[0]);
    assert_eq!(
        app.current_style.stroke_width,
        StrokeWidthPreset::Medium.width()
    );
    assert_eq!(
        app.current_style.roughness,
        RoughnessPreset::Sketchy.factor()
    );
}

#[test]
fn test_click_color_swatches_updates_current_style_when_no_selection() {
    let mut app = WhiteboardApp::new();
    let layout = app.style_panel_layout();

    for (i, &hex) in PALETTE_COLORS.iter().enumerate() {
        let swatch = layout.color_swatch_rect(i);
        let cx = f64::from(swatch.min.x + swatch.width() * 0.5);
        let cy = f64::from(swatch.min.y + swatch.height() * 0.5);

        click_at(&mut app, cx, cy);
        assert_eq!(app.current_style.stroke_color, hex);
        assert!(!app.style_panel_pressed);
        // No elements selected, so no command pushed to history
        assert!(!app.history.can_undo());
    }
}

#[test]
fn test_click_stroke_width_buttons_updates_current_style_when_no_selection() {
    let mut app = WhiteboardApp::new();
    let layout = app.style_panel_layout();

    for (i, &preset) in StrokeWidthPreset::ALL.iter().enumerate() {
        let btn = layout.width_button_rect(i);
        let cx = f64::from(btn.min.x + btn.width() * 0.5);
        let cy = f64::from(btn.min.y + btn.height() * 0.5);

        click_at(&mut app, cx, cy);
        assert_eq!(app.current_style.stroke_width, preset.width());
        assert!(!app.style_panel_pressed);
        assert!(!app.history.can_undo());
    }
}

#[test]
fn test_click_roughness_buttons_updates_current_style_when_no_selection() {
    let mut app = WhiteboardApp::new();
    let layout = app.style_panel_layout();

    for (i, &preset) in RoughnessPreset::ALL.iter().enumerate() {
        let btn = layout.roughness_button_rect(i);
        let cx = f64::from(btn.min.x + btn.width() * 0.5);
        let cy = f64::from(btn.min.y + btn.height() * 0.5);

        click_at(&mut app, cx, cy);
        assert_eq!(app.current_style.roughness, preset.factor());
        assert!(!app.style_panel_pressed);
        assert!(!app.history.can_undo());
    }
}

#[test]
fn test_click_style_panel_with_selection_modifies_elements_and_creates_undoable_command() {
    let mut app = WhiteboardApp::new();
    let elem = sample_rect(1, 200.0, 200.0, 100.0, 100.0);
    app.scene.push(elem);
    app.selection.set_selection(vec![1]);

    let layout = app.style_panel_layout();

    // 1. Change color to Coral (index 1: #FF6B6B)
    let swatch1 = layout.color_swatch_rect(1);
    click_at(
        &mut app,
        f64::from(swatch1.min.x + swatch1.width() * 0.5),
        f64::from(swatch1.min.y + swatch1.height() * 0.5),
    );
    assert_eq!(app.scene.get(1).unwrap().style().stroke_color, "#FF6B6B");
    assert!(app.history.can_undo());
    assert!(app.is_dirty);

    // 2. Change stroke width to Bold (index 2: 4.0)
    let btn_bold = layout.width_button_rect(2);
    click_at(
        &mut app,
        f64::from(btn_bold.min.x + btn_bold.width() * 0.5),
        f64::from(btn_bold.min.y + btn_bold.height() * 0.5),
    );
    assert_eq!(app.scene.get(1).unwrap().style().stroke_width, 4.0);

    // 3. Change roughness to Clean (index 0: 0.0)
    let btn_clean = layout.roughness_button_rect(0);
    click_at(
        &mut app,
        f64::from(btn_clean.min.x + btn_clean.width() * 0.5),
        f64::from(btn_clean.min.y + btn_clean.height() * 0.5),
    );
    assert_eq!(app.scene.get(1).unwrap().style().roughness, 0.0);

    // 4. Test undoing all 3 changes in reverse order
    app.history.undo(&mut app.scene);
    assert_eq!(app.scene.get(1).unwrap().style().roughness, 1.2);

    app.history.undo(&mut app.scene);
    assert_eq!(app.scene.get(1).unwrap().style().stroke_width, 2.5);

    app.history.undo(&mut app.scene);
    assert_eq!(app.scene.get(1).unwrap().style().stroke_color, "#ECECEC");

    // 5. Test redoing changes forward
    app.history.redo(&mut app.scene);
    assert_eq!(app.scene.get(1).unwrap().style().stroke_color, "#FF6B6B");

    app.history.redo(&mut app.scene);
    assert_eq!(app.scene.get(1).unwrap().style().stroke_width, 4.0);

    app.history.redo(&mut app.scene);
    assert_eq!(app.scene.get(1).unwrap().style().roughness, 0.0);
}

#[test]
fn test_style_panel_clicks_do_not_interact_with_canvas_underneath() {
    let mut app = WhiteboardApp::new();
    let rect_behind = sample_rect(42, 16.0, 68.0, 150.0, 100.0);
    app.scene.push(rect_behind);

    let layout = app.style_panel_layout();
    let swatch0 = layout.color_swatch_rect(0);

    click_at(
        &mut app,
        f64::from(swatch0.min.x + swatch0.width() * 0.5),
        f64::from(swatch0.min.y + swatch0.height() * 0.5),
    );

    // The element behind was NOT clicked / selected
    assert!(app.selection.is_empty());
}

#[test]
fn test_selecting_element_synchronizes_current_style() {
    let mut app = WhiteboardApp::new();
    let mut elem = sample_rect(1, 0.0, 0.0, 100.0, 100.0);
    elem.style_mut().stroke_color = "#FFD166".to_string(); // Yellow
    elem.style_mut().stroke_width = 1.0; // Thin
    elem.style_mut().roughness = 0.0; // Clean
    app.scene.push(elem);

    app.selection.set_selection(vec![1]);
    app.sync_style_from_selection();

    assert_eq!(app.current_style.stroke_color, "#FFD166");
    assert_eq!(app.current_style.stroke_width, 1.0);
    assert_eq!(app.current_style.roughness, 0.0);
}
