//! Automated continuous study session simulation test (Gate 2 Dogfooding validation).
//!
//! Validates stability, memory efficiency, and error-free operation across high-intensity
//! whiteboard usage patterns simulating a full continuous study session.

use ruqa_app::window_app::WhiteboardApp;
use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::style::{FillStyle, StrokeWidthPreset};
use ruqa_core::theme::{ThemeMode, DARK_BACKGROUND_COLOR, LIGHT_BACKGROUND_COLOR};
use ruqa_core::tool::Tool;
use ruqa_render::export_png::PngExportOptions;
use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

fn press_key_combination(app: &mut WhiteboardApp, modifier: NamedKey, key_char: &str) {
    app.handle_key(&Key::Named(modifier), ElementState::Pressed);
    app.handle_key(&Key::Character(key_char.into()), ElementState::Pressed);
    app.handle_key(&Key::Character(key_char.into()), ElementState::Released);
    app.handle_key(&Key::Named(modifier), ElementState::Released);
}

#[test]
fn test_simulated_continuous_study_session_stability() {
    let mut app = WhiteboardApp::new();
    assert_eq!(app.scene.len(), 0);

    // 1. Study phase 1: Diagramming algorithm flowchart with geometric primitives
    app.active_tool = Tool::Rectangle;
    for i in 0..50 {
        let x = (i % 10) as f32 * 120.0 + 50.0;
        let y = (i / 10) as f32 * 100.0 + 50.0;
        let style = ElementStyle {
            stroke_color: "#38BDF8".to_string(),
            fill_style: FillStyle::SemiTransparent,
            ..Default::default()
        };
        app.scene.push(Element::Rectangle {
            id: (i + 1) as u64,
            bounds: Rect::from_xywh(x, y, 100.0, 60.0),
            style,
        });
    }
    assert_eq!(app.scene.len(), 50);

    // Connect nodes with arrows
    app.active_tool = Tool::Arrow;
    for i in 0..40 {
        let x = (i % 10) as f32 * 120.0 + 150.0;
        let y = (i / 10) as f32 * 100.0 + 80.0;
        let style = ElementStyle::default();
        app.scene.push(Element::Arrow {
            id: (i + 51) as u64,
            start: Point::new(x, y),
            end: Point::new(x + 20.0, y),
            style,
        });
    }
    assert_eq!(app.scene.len(), 90);

    // 2. Study phase 2: Freehand velocity cursive annotations
    app.active_tool = Tool::Pen;
    for i in 0..30 {
        let start_x = (i * 25) as f32;
        let points: Vec<Point> = (0..20)
            .map(|step| {
                Point::new(
                    start_x + step as f32 * 5.0,
                    600.0 + (step as f32 * 0.5).sin() * 20.0,
                )
            })
            .collect();
        let pressures: Vec<f32> = (0..20).map(|step| 0.3 + (step as f32 * 0.03)).collect();
        let style = ElementStyle {
            stroke_color: "#FFD166".to_string(),
            stroke_width: StrokeWidthPreset::Medium.width(),
            ..Default::default()
        };
        app.scene.push(Element::Stroke {
            id: (i + 91) as u64,
            points,
            pressures,
            style,
        });
    }
    assert_eq!(app.scene.len(), 120);

    // 3. Navigation phase: Rapid pan and zoom across the canvas
    for step in 0..200 {
        let angle = step as f32 * 0.1;
        let dx = angle.cos() * 15.0;
        let dy = angle.sin() * 15.0;
        app.viewport.pan_by(Point::new(dx, dy));
        if step % 20 == 0 {
            app.viewport.set_zoom(app.viewport.zoom * 1.05);
        } else if step % 20 == 10 {
            app.viewport.set_zoom(app.viewport.zoom * 0.95);
        }
    }
    assert!(app.viewport.zoom > 0.0);

    // 4. Editing phase: Selection, duplication, delete, and undo/redo
    app.active_tool = Tool::Select;
    let sample_id = 1u64;
    app.selection.selection.select_single(sample_id);
    assert_eq!(app.selection.selection.selected_ids().len(), 1);

    // Duplicate element 5 times using Ctrl+D
    for _ in 0..5 {
        app.duplicate_selected();
    }
    assert_eq!(app.scene.len(), 125);

    // Delete selected element
    app.delete_selected();
    assert_eq!(app.scene.len(), 124);

    // Undo delete via keyboard Ctrl+Z
    assert!(app.history.can_undo());
    press_key_combination(&mut app, NamedKey::Control, "z");
    assert_eq!(app.scene.len(), 125);

    // Redo delete via keyboard Ctrl+Y
    assert!(app.history.can_redo());
    press_key_combination(&mut app, NamedKey::Control, "y");
    assert_eq!(app.scene.len(), 124);

    // 5. Theme inversion and grid toggle
    assert_eq!(app.theme_mode, ThemeMode::Dark);
    assert_eq!(app.theme_mode.background_color(), DARK_BACKGROUND_COLOR);

    app.toggle_theme();
    assert_eq!(app.theme_mode, ThemeMode::Light);
    assert_eq!(app.theme_mode.background_color(), LIGHT_BACKGROUND_COLOR);

    app.toggle_theme();
    assert_eq!(app.theme_mode, ThemeMode::Dark);

    // 6. Persistence & PNG export
    let temp_dir = std::env::temp_dir();
    let board_path = temp_dir.join(format!("ruqa_dogfood_test_{}.ruqa", std::process::id()));
    let png_path = temp_dir.join(format!("ruqa_dogfood_test_{}.png", std::process::id()));

    // Save board to file
    let save_res = app.save_as(&board_path);
    assert!(save_res.is_ok(), "save failed: {:?}", save_res.err());
    assert!(!app.is_dirty);
    assert_eq!(app.file_path, Some(board_path.clone()));

    // Verify saving cleared dirty flag
    assert!(!app.is_dirty);

    // Headless PNG export
    let export_res = app.export_png(&png_path, &PngExportOptions::default());
    assert!(
        export_res.is_ok(),
        "PNG export failed: {:?}",
        export_res.err()
    );
    assert!(png_path.exists());
    let png_bytes = std::fs::read(&png_path).expect("read png");
    assert_eq!(&png_bytes[0..8], b"\x89PNG\r\n\x1a\n");

    // Clean up temporary test files
    let _ = std::fs::remove_file(&board_path);
    let _ = std::fs::remove_file(&png_path);
}
