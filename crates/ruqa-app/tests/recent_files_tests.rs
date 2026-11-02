//! Integration tests for recent files tracking, welcome HUD layout, interaction, and keyboard toggling.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use ruqa_core::recent_files::MAX_RECENT_FILES;
use std::fs;
use std::path::PathBuf;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::{Key, NamedKey};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle::default(),
    }
}

fn press_key(app: &mut WhiteboardApp, key: Key) {
    app.handle_key(&key, ElementState::Pressed);
    app.handle_key(&key, ElementState::Released);
}

#[test]
fn test_recent_files_lru_and_cap() {
    let mut app = WhiteboardApp::new();
    app.recent_files.clear();

    for i in 1..=15 {
        let path = PathBuf::from(format!("/tmp/board_{i}.ruqa"));
        app.record_recent_file(&path);
    }

    assert_eq!(app.recent_files.len(), MAX_RECENT_FILES);
    assert_eq!(
        app.recent_files.entries[0].path,
        PathBuf::from("/tmp/board_15.ruqa")
    );

    // Recording an existing file moves it to the front without growing the list
    app.record_recent_file(PathBuf::from("/tmp/board_10.ruqa"));
    assert_eq!(app.recent_files.len(), MAX_RECENT_FILES);
    assert_eq!(
        app.recent_files.entries[0].path,
        PathBuf::from("/tmp/board_10.ruqa")
    );
}

#[test]
fn test_save_and_open_record_in_recent_files() {
    let mut app = WhiteboardApp::new();
    app.recent_files.clear();

    let tmp_dir = std::env::temp_dir().join(format!("ruqa_rf_test_{}", std::process::id()));
    fs::create_dir_all(&tmp_dir).unwrap();
    let file1 = tmp_dir.join("Lecture1.ruqa");

    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 50.0));
    app.save_as(&file1).unwrap();

    assert_eq!(app.recent_files.len(), 1);
    assert_eq!(app.recent_files.entries[0].path, file1);

    let mut app2 = WhiteboardApp::new();
    app2.recent_files.clear();
    app2.open_file(&file1).unwrap();
    assert_eq!(app2.recent_files.len(), 1);
    assert_eq!(app2.recent_files.entries[0].path, file1);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_hud_item_click_loads_document_and_closes_hud() {
    let mut app = WhiteboardApp::new();
    let tmp_dir = std::env::temp_dir().join(format!("ruqa_hud_test_{}", std::process::id()));
    fs::create_dir_all(&tmp_dir).unwrap();
    let board_path = tmp_dir.join("Operating_Systems.ruqa");

    app.scene.push(sample_rect(42, 10.0, 20.0, 80.0, 60.0));
    app.save_as(&board_path).unwrap();

    let mut fresh_app = WhiteboardApp::new();
    fresh_app.recent_files.clear();
    fresh_app.record_recent_file(&board_path);
    fresh_app.show_welcome_hud = true;

    let layout = fresh_app.recent_files_layout();
    assert_eq!(layout.item_bounds.len(), 1);
    let item_center = layout.item_bounds[0].1.center();

    fresh_app.pointer.cursor_pos = Some((f64::from(item_center.x), f64::from(item_center.y)));
    fresh_app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);

    assert!(!fresh_app.show_welcome_hud);
    assert_eq!(fresh_app.file_path, Some(board_path));
    assert_eq!(fresh_app.scene.len(), 1);
    assert_eq!(fresh_app.scene.get(42).unwrap().id(), 42);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_hud_dismiss_button_and_outside_click() {
    let mut app = WhiteboardApp::new();
    app.recent_files.clear();
    app.record_recent_file("/tmp/doc.ruqa");
    app.show_welcome_hud = true;

    // Click on close button
    let layout = app.recent_files_layout();
    let close_center = layout.close_button_bounds.center();
    app.pointer.cursor_pos = Some((f64::from(close_center.x), f64::from(close_center.y)));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(!app.show_welcome_hud);

    // Reopen and click outside
    app.show_welcome_hud = true;
    app.pointer.cursor_pos = Some((10.0, 10.0)); // Outside modal dialog
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(!app.show_welcome_hud);
}

#[test]
fn test_hud_keyboard_escape_and_ctrl_o_toggle() {
    let mut app = WhiteboardApp::new();
    app.recent_files.clear();
    app.record_recent_file("/tmp/doc.ruqa");
    app.show_welcome_hud = true;

    // Escape dismisses
    press_key(&mut app, Key::Named(NamedKey::Escape));
    assert!(!app.show_welcome_hud);

    // Ctrl+O toggles HUD open
    app.pointer.ctrl_held = true;
    press_key(&mut app, Key::Character("o".into()));
    assert!(app.show_welcome_hud);

    // Ctrl+O toggles HUD closed
    press_key(&mut app, Key::Character("o".into()));
    assert!(!app.show_welcome_hud);
}

#[test]
fn test_hud_hover_state_updates_on_cursor_moved() {
    let mut app = WhiteboardApp::new();
    let doc_path = PathBuf::from("/tmp/hover_test.ruqa");
    app.recent_files.clear();
    app.record_recent_file(&doc_path);
    app.show_welcome_hud = true;

    let layout = app.recent_files_layout();
    let item_center = layout.item_bounds[0].1.center();

    app.handle_cursor_moved(f64::from(item_center.x), f64::from(item_center.y));
    assert_eq!(app.hovered_recent_file, Some(doc_path));

    // Move outside item
    app.handle_cursor_moved(10.0, 10.0);
    assert_eq!(app.hovered_recent_file, None);
}
