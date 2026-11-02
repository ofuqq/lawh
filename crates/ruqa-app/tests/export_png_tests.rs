//! Integration tests for exporting canvas diagrams to high-resolution PNG images.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::Rect;
use ruqa_render::export_png::PngExportOptions;
use std::fs;
use winit::event::ElementState;
use winit::keyboard::Key;

fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
    Element::Rectangle {
        id,
        bounds: Rect::from_xywh(x, y, w, h),
        style: ElementStyle {
            stroke_color: "#38BDF8".to_string(), // Cyan
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
fn test_export_png_creates_valid_file_with_png_signature() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 20.0, 30.0, 100.0, 60.0));

    let tmp = std::env::temp_dir().join(format!("ruqa_export_valid_{}.png", std::process::id()));
    let _ = fs::remove_file(&tmp);

    app.export_png(&tmp, &PngExportOptions::default()).unwrap();
    assert!(tmp.is_file());

    let bytes = fs::read(&tmp).unwrap();
    assert!(bytes.len() > 100);
    // Standard PNG magic bytes \x89PNG\r\n\x1a\n
    assert_eq!(&bytes[0..8], b"\x89PNG\r\n\x1a\n");

    let _ = fs::remove_file(&tmp);
}

#[test]
fn test_export_png_high_res_and_transparent_presets() {
    let mut app = WhiteboardApp::new();
    app.scene.push(sample_rect(1, 0.0, 0.0, 100.0, 100.0));

    let tmp_dark =
        std::env::temp_dir().join(format!("ruqa_export_dark_{}.png", std::process::id()));
    let tmp_trans =
        std::env::temp_dir().join(format!("ruqa_export_trans_{}.png", std::process::id()));

    // 2x high-res dark export
    app.export_png(&tmp_dark, &PngExportOptions::high_res_dark())
        .unwrap();
    assert!(tmp_dark.is_file());

    // Transparent export
    app.export_png(&tmp_trans, &PngExportOptions::transparent())
        .unwrap();
    assert!(tmp_trans.is_file());

    let dark_bytes = fs::read(&tmp_dark).unwrap();
    let trans_bytes = fs::read(&tmp_trans).unwrap();

    assert_eq!(&dark_bytes[0..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(&trans_bytes[0..8], b"\x89PNG\r\n\x1a\n");

    let _ = fs::remove_file(&tmp_dark);
    let _ = fs::remove_file(&tmp_trans);
}

#[test]
fn test_ctrl_e_shortcut_triggers_default_export() {
    let mut app = WhiteboardApp::new();
    let tmp_board =
        std::env::temp_dir().join(format!("ruqa_export_shortcut_{}.ruqa", std::process::id()));
    let expected_png = tmp_board.with_extension("png");
    let _ = fs::remove_file(&expected_png);

    app.file_path = Some(tmp_board);
    app.scene.push(sample_rect(10, 10.0, 10.0, 80.0, 40.0));

    // Ctrl+E shortcut
    app.pointer.ctrl_held = true;
    press_key(&mut app, Key::Character("e".into()));

    assert!(expected_png.is_file());
    let _ = fs::remove_file(&expected_png);
}

#[test]
fn test_empty_scene_export_succeeds_without_panic() {
    let app = WhiteboardApp::new();
    let tmp = std::env::temp_dir().join(format!("ruqa_export_empty_{}.png", std::process::id()));
    let _ = fs::remove_file(&tmp);

    app.export_png(&tmp, &PngExportOptions::default()).unwrap();
    assert!(tmp.is_file());
    let _ = fs::remove_file(&tmp);
}

#[test]
fn test_export_png_tight_selection_bounds_vs_full_diagram() {
    let mut app = WhiteboardApp::new();
    let rect_a = sample_rect(1, 10.0, 10.0, 50.0, 50.0);
    let rect_b = sample_rect(2, 500.0, 500.0, 100.0, 100.0);
    app.scene.push(rect_a);
    app.scene.push(rect_b);

    // 1. With no selection, export bounds enclose both elements (full canvas diagram)
    assert!(!app.has_active_selection());
    let full_bounds = app.export_bounds();
    assert_eq!(full_bounds.min.x, 9.0);
    assert_eq!(full_bounds.min.y, 9.0);
    assert_eq!(full_bounds.max.x, 601.0);
    assert_eq!(full_bounds.max.y, 601.0);
    assert_eq!(app.export_elements().len(), 2);

    // 2. Select only Rect A
    app.selection.set_selection(vec![1]);
    assert!(app.has_active_selection());
    let sel_bounds = app.export_bounds();
    assert_eq!(sel_bounds.min.x, 9.0);
    assert_eq!(sel_bounds.min.y, 9.0);
    assert_eq!(sel_bounds.max.x, 61.0);
    assert_eq!(sel_bounds.max.y, 61.0);
    let sel_elements = app.export_elements();
    assert_eq!(sel_elements.len(), 1);
    assert_eq!(sel_elements[0].id(), 1);

    // 3. Export selection to PNG and verify file is created
    let tmp = std::env::temp_dir().join(format!("ruqa_export_sel_{}.png", std::process::id()));
    let _ = fs::remove_file(&tmp);

    app.export_png(&tmp, &PngExportOptions::default()).unwrap();
    assert!(tmp.is_file());
    let bytes = fs::read(&tmp).unwrap();
    assert_eq!(&bytes[0..8], b"\x89PNG\r\n\x1a\n");
    let _ = fs::remove_file(&tmp);
}
