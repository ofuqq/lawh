//! Integration tests for Element accessors, translation, and duplication.

use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::geometry::{Point, Rect};

#[test]
fn test_element_accessors_and_bounds() {
    let style = ElementStyle {
        stroke_color: "#FFFFFF".into(),
        stroke_width: 3.0,
        fill_color: Some("#333333".into()),
        fill_style: FillStyle::Solid,
        roughness: 1.0,
        seed: 12345,
    };

    let rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(10.0, 20.0, 100.0, 50.0),
        style: style.clone(),
    };
    assert_eq!(rect.id(), 1);
    assert_eq!(rect.bounds().width(), 100.0);
    assert_eq!(rect.bounds().height(), 50.0);
    assert_eq!(rect.style(), &style);

    let line = Element::Line {
        id: 2,
        start: Point::new(100.0, 200.0),
        end: Point::new(50.0, 150.0),
        style: style.clone(),
    };
    assert_eq!(line.id(), 2);
    assert_eq!(line.bounds().min, Point::new(50.0, 150.0));
    assert_eq!(line.bounds().max, Point::new(100.0, 200.0));

    let rect_bb = rect.bounding_box();
    assert_eq!(rect_bb.min, Point::new(8.5, 18.5));
    assert_eq!(rect_bb.max, Point::new(111.5, 71.5));
}

#[test]
fn test_element_translation() {
    let mut rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(10.0, 20.0, 30.0, 40.0),
        style: ElementStyle::default(),
    };
    rect.translate(Point::new(5.0, -10.0));
    assert_eq!(rect.bounds(), Rect::from_xywh(15.0, 10.0, 30.0, 40.0));
}

#[test]
fn test_element_duplicate_with() {
    let rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(10.0, 10.0, 50.0, 50.0),
        style: ElementStyle {
            seed: 100,
            ..Default::default()
        },
    };
    let dup = rect.duplicate_with(2, 999, Point::new(20.0, 20.0));
    assert_eq!(dup.id(), 2);
    assert_eq!(dup.style().seed, 999);
    assert_eq!(dup.bounds(), Rect::from_xywh(30.0, 30.0, 50.0, 50.0));
}

#[test]
fn test_text_element_bounds_and_translation() {
    let mut text = Element::Text {
        id: 5,
        position: Point::new(100.0, 50.0),
        content: "Hello World".into(),
        font_size: 20.0,
        style: ElementStyle::default(),
    };
    assert_eq!(text.id(), 5);
    assert!(text.bounds().width() > 0.0);
    assert!(text.bounds().height() > 0.0);
    assert!(text.bounding_box().contains(Point::new(120.0, 60.0)));

    text.translate(Point::new(50.0, 25.0));
    assert_eq!(text.bounds().min, Point::new(150.0, 75.0));
}
