//! Integration tests for the Study Vault folder drawer sidebar (`Ctrl+B`), tree browsing, and board loading.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use std::fs;
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
fn test_ctrl_b_and_escape_toggle_vault_sidebar() {
    let mut app = WhiteboardApp::new();
    assert!(!app.show_vault_sidebar);

    // Ctrl+B opens sidebar
    app.pointer.ctrl_held = true;
    press_key(&mut app, Key::Character("b".into()));
    assert!(app.show_vault_sidebar);

    // Ctrl+B closes sidebar
    press_key(&mut app, Key::Character("b".into()));
    assert!(!app.show_vault_sidebar);

    // Open again and press Escape to close
    press_key(&mut app, Key::Character("b".into()));
    assert!(app.show_vault_sidebar);
    app.pointer.ctrl_held = false;
    press_key(&mut app, Key::Named(NamedKey::Escape));
    assert!(!app.show_vault_sidebar);
}

#[test]
fn test_vault_tree_navigation_and_expansion() {
    let tmp = std::env::temp_dir().join(format!("ruqa_vault_ui_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(tmp.join("CS101")).unwrap();
    fs::write(tmp.join("CS101/Algorithms.ruqa"), "{}").unwrap();
    fs::write(tmp.join("Overview.ruqa"), "{}").unwrap();

    let mut app = WhiteboardApp::new();
    app.set_vault_root(&tmp);
    app.show_vault_sidebar = true;

    assert_eq!(app.vault_entries.len(), 2); // CS101 folder and Overview board
    assert!(app.vault_entries[0].is_folder);
    assert_eq!(app.vault_entries[0].name, "CS101");
    assert!(!app.vault_entries[0].is_expanded);

    // Click on CS101 folder row to expand
    let layout = app.vault_layout();
    let folder_center = layout.rows[0].row_bounds.center();
    app.pointer.cursor_pos = Some((f64::from(folder_center.x), f64::from(folder_center.y)));
    let consumed = app.handle_vault_pointer_down(folder_center);
    assert!(consumed);

    // Now expanded: contains CS101, Algorithms, Overview
    assert_eq!(app.vault_entries.len(), 3);
    assert!(app.vault_entries[0].is_expanded);
    assert_eq!(app.vault_entries[1].name, "Algorithms");
    assert_eq!(app.vault_entries[1].depth, 1);

    // Click again to collapse
    let consumed = app.handle_vault_pointer_down(folder_center);
    assert!(consumed);
    assert_eq!(app.vault_entries.len(), 2);
    assert!(!app.vault_entries[0].is_expanded);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_vault_board_click_loads_document() {
    let tmp = std::env::temp_dir().join(format!("ruqa_vault_load_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    let board_path = tmp.join("DataStructures.ruqa");

    // Save a board first
    let mut setup_app = WhiteboardApp::new();
    setup_app
        .scene
        .push(sample_rect(777, 50.0, 50.0, 120.0, 80.0));
    setup_app.save_as(&board_path).unwrap();

    // Start with fresh app pointing at vault
    let mut app = WhiteboardApp::new();
    app.set_vault_root(&tmp);
    app.show_vault_sidebar = true;

    assert_eq!(app.vault_entries.len(), 1);
    assert_eq!(app.vault_entries[0].name, "DataStructures");

    // Click on board row
    let layout = app.vault_layout();
    let board_center = layout.rows[0].row_bounds.center();
    app.pointer.cursor_pos = Some((f64::from(board_center.x), f64::from(board_center.y)));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);

    assert_eq!(app.file_path, Some(board_path));
    assert_eq!(app.scene.len(), 1);
    assert_eq!(app.scene.get(777).unwrap().id(), 777);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_vault_new_board_and_close_buttons() {
    let tmp = std::env::temp_dir().join(format!("ruqa_vault_btns_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    let mut app = WhiteboardApp::new();
    app.set_vault_root(&tmp);
    app.scene.push(sample_rect(1, 0.0, 0.0, 50.0, 50.0));
    app.show_vault_sidebar = true;

    // Click New Board button
    let layout = app.vault_layout();
    let new_pt = layout.new_board_bounds.center();
    app.pointer.cursor_pos = Some((f64::from(new_pt.x), f64::from(new_pt.y)));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert_eq!(app.scene.len(), 0);
    assert_eq!(app.file_path, None);

    // Click Close button
    let close_pt = layout.close_button_bounds.center();
    app.pointer.cursor_pos = Some((f64::from(close_pt.x), f64::from(close_pt.y)));
    app.handle_mouse_input(ElementState::Pressed, MouseButton::Left);
    assert!(!app.show_vault_sidebar);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_vault_hover_updates_on_cursor_moved() {
    let tmp = std::env::temp_dir().join(format!("ruqa_vault_hover_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let board_path = tmp.join("Notes.ruqa");
    fs::write(&board_path, "{}").unwrap();

    let mut app = WhiteboardApp::new();
    app.set_vault_root(&tmp);
    app.show_vault_sidebar = true;

    let layout = app.vault_layout();
    let row_center = layout.rows[0].row_bounds.center();

    app.handle_cursor_moved(f64::from(row_center.x), f64::from(row_center.y));
    assert_eq!(app.hovered_vault_item, Some(board_path));

    // Move outside row
    app.handle_cursor_moved(10.0, 500.0);
    assert_eq!(app.hovered_vault_item, None);

    let _ = fs::remove_dir_all(&tmp);
}
