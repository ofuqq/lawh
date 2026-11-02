//! Integration tests for element Serde JSON serialization.

use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::geometry::{Point, Rect};

#[test]
fn test_serde_json_round_trips_all_variants() {
    let elements = vec![
        Element::Rectangle {
            id: 101,
            bounds: Rect::from_xywh(0.0, 0.0, 200.0, 100.0),
            style: ElementStyle::default(),
        },
        Element::Ellipse {
            id: 102,
            bounds: Rect::from_xywh(50.0, 50.0, 80.0, 80.0),
            style: ElementStyle {
                fill_color: Some("#FF0000".into()),
                fill_style: FillStyle::Hachure,
                ..ElementStyle::default()
            },
        },
        Element::Diamond {
            id: 103,
            bounds: Rect::from_xywh(20.0, 30.0, 40.0, 60.0),
            style: ElementStyle {
                fill_style: FillStyle::CrossHatch,
                ..ElementStyle::default()
            },
        },
        Element::Line {
            id: 104,
            start: Point::new(0.0, 0.0),
            end: Point::new(100.0, 100.0),
            style: ElementStyle::default(),
        },
        Element::Arrow {
            id: 105,
            start: Point::new(100.0, 100.0),
            end: Point::new(200.0, 150.0),
            style: ElementStyle {
                roughness: 2.0,
                seed: 9999,
                ..ElementStyle::default()
            },
        },
        Element::Stroke {
            id: 106,
            points: vec![Point::new(10.0, 15.0), Point::new(25.0, 40.0)],
            pressures: vec![0.5, 0.8],
            style: ElementStyle::default(),
        },
    ];

    for element in elements {
        let json = serde_json::to_string_pretty(&element).expect("serialization succeeds");
        let deserialized: Element = serde_json::from_str(&json).expect("deserialization succeeds");
        assert_eq!(element, deserialized);
    }
}

#[test]
fn test_tagged_json_representation() {
    let rect = Element::Rectangle {
        id: 42,
        bounds: Rect::from_xywh(10.0, 10.0, 20.0, 20.0),
        style: ElementStyle::default(),
    };
    let json_value: serde_json::Value =
        serde_json::to_value(&rect).expect("to_value should succeed");

    assert_eq!(json_value["type"], "rectangle");
    assert_eq!(json_value["id"], 42);
    assert_eq!(json_value["bounds"]["min"]["x"], 10.0);
}
