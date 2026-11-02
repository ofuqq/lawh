//! Integration tests for `.ruqa` versioned document JSON persistence and file I/O.

use ruqa_core::document::{RuqaDocument, CURRENT_SCHEMA_VERSION, FORMAT_MAGIC};
use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::error::DocumentError;
use ruqa_core::geometry::{Point, Rect, Viewport};
use ruqa_core::scene::Scene;

fn complex_test_scene() -> Scene {
    let mut scene = Scene::new();

    scene.push(Element::Rectangle {
        id: 101,
        bounds: Rect::from_xywh(10.5, 20.25, 150.75, 80.125),
        style: ElementStyle {
            stroke_color: "#E0E0E0".into(),
            stroke_width: 2.5,
            fill_color: Some("#212121".into()),
            fill_style: FillStyle::Solid,
            roughness: 1.4,
            seed: 12345,
        },
    });

    scene.push(Element::Ellipse {
        id: 102,
        bounds: Rect::from_xywh(-50.0, -30.0, 100.0, 60.0),
        style: ElementStyle {
            stroke_color: "#FF5252".into(),
            stroke_width: 1.8,
            fill_color: Some("#B71C1C".into()),
            fill_style: FillStyle::Hachure,
            roughness: 1.0,
            seed: 54321,
        },
    });

    scene.push(Element::Diamond {
        id: 103,
        bounds: Rect::from_xywh(200.0, 150.0, 80.0, 80.0),
        style: ElementStyle {
            stroke_color: "#4DD0E1".into(),
            stroke_width: 3.0,
            fill_color: None,
            fill_style: FillStyle::None,
            roughness: 0.8,
            seed: 98765,
        },
    });

    scene.push(Element::Line {
        id: 104,
        start: Point::new(-100.12, 50.34),
        end: Point::new(100.56, -50.78),
        style: ElementStyle::default(),
    });

    scene.push(Element::Arrow {
        id: 105,
        start: Point::new(0.0, 0.0),
        end: Point::new(75.5, 75.5),
        style: ElementStyle {
            stroke_color: "#FFD54F".into(),
            stroke_width: 2.0,
            fill_color: None,
            fill_style: FillStyle::None,
            roughness: 1.2,
            seed: 112233,
        },
    });

    scene.push(Element::Stroke {
        id: 106,
        points: vec![
            Point::new(10.0, 10.0),
            Point::new(15.5, 22.3),
            Point::new(25.0, 40.0),
        ],
        pressures: vec![0.4, 0.8, 1.2],
        style: ElementStyle {
            stroke_color: "#81C784".into(),
            stroke_width: 4.0,
            fill_color: None,
            fill_style: FillStyle::None,
            roughness: 0.0,
            seed: 445566,
        },
    });

    scene
}

#[test]
fn test_complex_document_round_trip_preserves_floats_and_order() {
    let scene = complex_test_scene();
    let viewport = Viewport::new(Point::new(-123.456, 789.012), 2.345);

    let doc = RuqaDocument::new(&scene, viewport);
    let json = doc.to_json_string().expect("JSON serialization succeeds");

    let loaded = RuqaDocument::from_json_str(&json).expect("JSON deserialization succeeds");
    assert_eq!(doc, loaded);

    let (restored_scene, restored_viewport) = loaded.into_scene_and_viewport();
    assert_eq!(restored_scene.len(), 6);
    assert_eq!(restored_scene.elements(), scene.elements());
    assert_eq!(restored_viewport.pan, viewport.pan);
    assert_eq!(restored_viewport.zoom, viewport.zoom);
}

#[test]
fn test_file_save_and_load_round_trip() {
    let scene = complex_test_scene();
    let viewport = Viewport::new(Point::new(50.0, 50.0), 1.0);
    let doc = RuqaDocument::new(&scene, viewport);

    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join("test_board_round_trip.ruqa");

    doc.save_to_file(&file_path)
        .expect("saving document succeeds");
    assert!(file_path.exists());

    let loaded = RuqaDocument::load_from_file(&file_path).expect("loading document succeeds");
    assert_eq!(doc, loaded);

    // Clean up temporary file
    let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_load_nonexistent_file_returns_io_error() {
    let nonexistent = std::env::temp_dir().join("nonexistent_board_12345.ruqa");
    let err = RuqaDocument::load_from_file(&nonexistent).unwrap_err();

    match err {
        DocumentError::Io { path, .. } => {
            assert_eq!(path, nonexistent);
        }
        other => panic!("expected DocumentError::Io, got {other:?}"),
    }
}

#[test]
fn test_rejects_unsupported_future_version() {
    let json = format!(
        r#"{{
            "format": "{FORMAT_MAGIC}",
            "schema_version": {},
            "app_version": "2.0.0",
            "canvas": {{}},
            "elements": []
        }}"#,
        CURRENT_SCHEMA_VERSION + 1
    );

    let err = RuqaDocument::from_json_str(&json).unwrap_err();
    match err {
        DocumentError::UnsupportedVersion {
            found,
            max_supported,
            ..
        } => {
            assert_eq!(found, (CURRENT_SCHEMA_VERSION + 1).to_string());
            assert_eq!(max_supported, CURRENT_SCHEMA_VERSION.to_string());
        }
        other => panic!("expected UnsupportedVersion, got {other:?}"),
    }
}

#[test]
fn test_rejects_invalid_format_magic() {
    let json = r#"{
        "format": "excalidraw",
        "schema_version": 1,
        "app_version": "0.1.0",
        "elements": []
    }"#;

    let err = RuqaDocument::from_json_str(json).unwrap_err();
    match err {
        DocumentError::Parse { source, .. } => {
            assert!(source.to_string().contains("invalid format identifier"));
        }
        other => panic!("expected Parse error, got {other:?}"),
    }
}

#[test]
fn test_forward_compatibility_ignores_unknown_properties() {
    let json = r##"{
        "format": "ruqa",
        "schema_version": 1,
        "app_version": "0.1.0",
        "extra_future_top_level": { "field": 123 },
        "canvas": {
            "background_color": "#121212",
            "viewport": { "pan": { "x": 0.0, "y": 0.0 }, "zoom": 1.0 },
            "custom_canvas_option": "dark_matrix"
        },
        "elements": [
            {
                "id": 1,
                "type": "rectangle",
                "bounds": { "min": { "x": 0.0, "y": 0.0 }, "max": { "x": 50.0, "y": 50.0 } },
                "style": { "stroke_color": "#FFF", "stroke_width": 2.0, "roughness": 1.0, "seed": 0 },
                "future_element_tag": "pinned"
            }
        ]
    }"##;

    let doc = RuqaDocument::from_json_str(json).expect("should ignore unknown fields");
    assert_eq!(doc.elements.len(), 1);
    assert_eq!(doc.elements[0].id(), 1);
}

#[test]
fn test_load_older_v0_schema_runs_migration_into_v1() {
    let json = r##"{
        "format": "ruqa",
        "schema_version": 0,
        "app_version": "0.0.1",
        "pan_x": 100.0,
        "pan_y": -50.0,
        "zoom": 2.0,
        "background_color": "#000000",
        "elements": [
            {
                "id": 42,
                "type": "rectangle",
                "bounds": { "min": { "x": 10.0, "y": 10.0 }, "max": { "x": 60.0, "y": 60.0 } },
                "style": { "stroke_color": "#FFF", "stroke_width": 2.0 }
            }
        ]
    }"##;

    let doc = RuqaDocument::from_json_str(json).expect("migration succeeds");
    assert_eq!(doc.schema_version, 1);
    assert_eq!(doc.canvas.viewport.pan, Point::new(100.0, -50.0));
    assert_eq!(doc.canvas.viewport.zoom, 2.0);
    assert_eq!(doc.canvas.background_color, "#000000");
    assert_eq!(doc.elements.len(), 1);
    assert_eq!(doc.elements[0].id(), 42);
}
