//! Integration tests for unsaved changes confirmation dialog on quit (Issue #35).

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use std::fs;
use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_color: "#38BDF8".to_string(),
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
fn test_close_requested_when_clean_exits_immediately() {
    let mut app = WhiteboardApp::new();
    assert!(!app.is_dirty);
    assert!(!app.show_quit_dialog);
    assert!(!app.should_exit);

    app.request_close();

    assert!(!app.show_quit_dialog);
    assert!(app.should_exit);
}

#[test]
fn test_close_requested_when_dirty_prompts_dialog() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 10.0, 10.0, 50.0, 50.0));
    app.mark_dirty();
    assert!(app.is_dirty);

    app.request_close();

    assert!(app.show_quit_dialog);
    assert!(!app.should_exit);
}

#[test]
fn test_cancel_button_and_escape_key_dismisses_dialog_and_does_not_exit() {
    let mut app = WhiteboardApp::new();
    app.mark_dirty();
    app.request_close();
    assert!(app.show_quit_dialog);

    // 1. Dismiss via Escape key
    press_key(&mut app, Key::Named(NamedKey::Escape));
    assert!(!app.show_quit_dialog);
    assert!(!app.should_exit);
    assert!(app.is_dirty);

    // 2. Dismiss via clicking Cancel button
    app.request_close();
    assert!(app.show_quit_dialog);
    let layout = app.quit_dialog_layout();
    let cancel_pt = layout.cancel_button_bounds.center();
    let consumed = app.handle_quit_dialog_pointer_down(cancel_pt);
    assert!(consumed);
    assert!(!app.show_quit_dialog);
    assert!(!app.should_exit);
    assert!(app.is_dirty);
}

#[test]
fn test_dont_save_button_and_d_key_exits_without_saving() {
    let mut app = WhiteboardApp::new();
    let tmp = std::env::temp_dir().join(format!("ruqa_dont_save_{}.ruqa", std::process::id()));
    let _ = fs::remove_file(&tmp);
    app.file_path = Some(tmp.clone());
    app.mark_dirty();

    // 1. Discard via 'd' key
    app.request_close();
    assert!(app.show_quit_dialog);
    press_key(&mut app, Key::Character("d".into()));
    assert!(!app.show_quit_dialog);
    assert!(app.should_exit);
    assert!(!tmp.exists()); // File was not saved

    // 2. Discard via Don't Save button
    app.should_exit = false;
    app.mark_dirty();
    app.request_close();
    assert!(app.show_quit_dialog);
    let layout = app.quit_dialog_layout();
    let dont_save_pt = layout.dont_save_button_bounds.center();
    let consumed = app.handle_quit_dialog_pointer_down(dont_save_pt);
    assert!(consumed);
    assert!(!app.show_quit_dialog);
    assert!(app.should_exit);
    assert!(!tmp.exists());

    let _ = fs::remove_file(&tmp);
}

#[test]
fn test_save_button_and_enter_key_saves_and_exits() {
    let mut app = WhiteboardApp::new();
    let tmp = std::env::temp_dir().join(format!("ruqa_save_quit_{}.ruqa", std::process::id()));
    let _ = fs::remove_file(&tmp);
    app.file_path = Some(tmp.clone());
    app.scene.push(sample_rect(1, 20.0, 20.0, 40.0, 40.0));
    app.mark_dirty();

    // 1. Confirm Save via Enter key
    app.request_close();
    assert!(app.show_quit_dialog);
    press_key(&mut app, Key::Named(NamedKey::Enter));
    assert!(!app.show_quit_dialog);
    assert!(app.should_exit);
    assert!(!app.is_dirty);
    assert!(tmp.is_file());

    let _ = fs::remove_file(&tmp);

    // 2. Confirm Save via Save button click
    let tmp2 = std::env::temp_dir().join(format!("ruqa_save_quit2_{}.ruqa", std::process::id()));
    let _ = fs::remove_file(&tmp2);
    app.file_path = Some(tmp2.clone());
    app.should_exit = false;
    app.mark_dirty();
    app.request_close();
    assert!(app.show_quit_dialog);

    let layout = app.quit_dialog_layout();
    let save_pt = layout.save_button_bounds.center();
    let consumed = app.handle_quit_dialog_pointer_down(save_pt);
    assert!(consumed);
    assert!(!app.show_quit_dialog);
    assert!(app.should_exit);
    assert!(!app.is_dirty);
    assert!(tmp2.is_file());

    let _ = fs::remove_file(&tmp2);
}

#[test]
fn test_ctrl_q_triggers_quit_confirmation_when_dirty() {
    let mut app = WhiteboardApp::new();
    app.mark_dirty();

    // Press Ctrl+Q
    app.pointer.ctrl_held = true;
    press_key(&mut app, Key::Character("q".into()));

    assert!(app.show_quit_dialog);
    assert!(!app.should_exit);
}

#[test]
fn test_quit_dialog_isolates_mouse_clicks_from_canvas() {
    let mut app = WhiteboardApp::new();
    let elem = sample_rect(42, 100.0, 100.0, 100.0, 100.0);
    app.scene.push(elem);
    app.mark_dirty();
    app.request_close();
    assert!(app.show_quit_dialog);

    // Click inside the dialog container body (outside buttons)
    let layout = app.quit_dialog_layout();
    let dialog_center = layout.dialog_bounds.center();
    let consumed = app.handle_quit_dialog_pointer_down(dialog_center);
    assert!(consumed);
    assert!(app.show_quit_dialog); // Dialog stays open
    assert!(app.selection.is_empty()); // Canvas element was NOT selected
}
