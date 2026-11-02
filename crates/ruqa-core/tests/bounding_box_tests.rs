//! Comprehensive bounding box and padding integration tests for all element variants.

use ruqa_core::element::{Element, ElementStyle};
use ruqa_core::geometry::{Point, Rect};

fn style_with_stroke_width(w: f32) -> ElementStyle {
    ElementStyle {
        stroke_width: w,
        ..Default::default()
    }
}

#[test]
fn test_rectangle_bounding_box_and_zero_area() {
    let style = style_with_stroke_width(4.0);
    let rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(10.0, 20.0, 100.0, 50.0),
        style: style.clone(),
    };
    let bb = rect.bounding_box();
    // Stroke width 4.0 -> padding = 2.0
    assert_eq!(bb.min, Point::new(8.0, 18.0));
    assert_eq!(bb.max, Point::new(112.0, 72.0));
    assert_eq!(bb.width(), 104.0);
    assert_eq!(bb.height(), 54.0);

    // Zero-area rectangle
    let zero_rect = Element::Rectangle {
        id: 2,
        bounds: Rect::from_xywh(50.0, 50.0, 0.0, 0.0),
        style,
    };
    let zero_bb = zero_rect.bounding_box();
    assert_eq!(zero_bb.min, Point::new(48.0, 48.0));
    assert_eq!(zero_bb.max, Point::new(52.0, 52.0));
    assert_eq!(zero_bb.width(), 4.0);
    assert_eq!(zero_bb.height(), 4.0);
}

#[test]
fn test_ellipse_and_diamond_bounding_box() {
    let style = style_with_stroke_width(2.0);
    let ellipse = Element::Ellipse {
        id: 10,
        bounds: Rect::from_xywh(0.0, 0.0, 80.0, 60.0),
        style: style.clone(),
    };
    let bb_ellipse = ellipse.bounding_box();
    // Stroke width 2.0 -> padding = 1.0
    assert_eq!(bb_ellipse.min, Point::new(-1.0, -1.0));
    assert_eq!(bb_ellipse.max, Point::new(81.0, 61.0));

    let diamond = Element::Diamond {
        id: 11,
        bounds: Rect::from_xywh(20.0, 30.0, 40.0, 60.0),
        style,
    };
    let bb_diamond = diamond.bounding_box();
    assert_eq!(bb_diamond.min, Point::new(19.0, 29.0));
    assert_eq!(bb_diamond.max, Point::new(61.0, 91.0));
}

#[test]
fn test_line_bounding_box_orientations_and_zero_length() {
    let style = style_with_stroke_width(4.0);

    // Horizontal line
    let h_line = Element::Line {
        id: 20,
        start: Point::new(10.0, 50.0),
        end: Point::new(110.0, 50.0),
        style: style.clone(),
    };
    let h_bb = h_line.bounding_box();
    assert_eq!(h_bb.min, Point::new(8.0, 48.0));
    assert_eq!(h_bb.max, Point::new(112.0, 52.0));

    // Vertical line
    let v_line = Element::Line {
        id: 21,
        start: Point::new(50.0, 10.0),
        end: Point::new(50.0, 110.0),
        style: style.clone(),
    };
    let v_bb = v_line.bounding_box();
    assert_eq!(v_bb.min, Point::new(48.0, 8.0));
    assert_eq!(v_bb.max, Point::new(52.0, 112.0));

    // Diagonal reversed line
    let diag_line = Element::Line {
        id: 22,
        start: Point::new(100.0, 80.0),
        end: Point::new(20.0, 10.0),
        style: style.clone(),
    };
    let d_bb = diag_line.bounding_box();
    assert_eq!(d_bb.min, Point::new(18.0, 8.0));
    assert_eq!(d_bb.max, Point::new(102.0, 82.0));

    // Zero-length line
    let zero_line = Element::Line {
        id: 23,
        start: Point::new(40.0, 40.0),
        end: Point::new(40.0, 40.0),
        style,
    };
    let z_bb = zero_line.bounding_box();
    assert_eq!(z_bb.min, Point::new(38.0, 38.0));
    assert_eq!(z_bb.max, Point::new(42.0, 42.0));
}

#[test]
fn test_arrow_bounding_box_includes_arrowhead_extent() {
    let style = style_with_stroke_width(2.0);
    // Right-pointing arrow
    let arrow_right = Element::Arrow {
        id: 30,
        start: Point::new(0.0, 50.0),
        end: Point::new(100.0, 50.0),
        style: style.clone(),
    };
    let bb_right = arrow_right.bounding_box();
    // Length = 100, barb length = 12 + 4 = 16. Angle = 0.
    // Barbs extend backward at +/- 30 degrees: end.x + 16 * cos(pi +/- pi/6) = 100 - 16*cos(30) ~ 86.14
    // end.y + 16 * sin(pi +/- pi/6) = 50 +/- 8 = 42 and 58.
    // Bounding box must enclose y in [42, 58] padded by 1.0 -> y in [41, 59]
    assert!(bb_right.min.x <= -1.0);
    assert!(bb_right.max.x >= 101.0);
    assert!(bb_right.min.y <= 41.5);
    assert!(bb_right.max.y >= 58.5);

    // Diagonal arrow
    let arrow_diag = Element::Arrow {
        id: 31,
        start: Point::new(0.0, 0.0),
        end: Point::new(100.0, 100.0),
        style: style.clone(),
    };
    let bb_diag = arrow_diag.bounding_box();
    assert!(bb_diag.min.x <= -1.0);
    assert!(bb_diag.min.y <= -1.0);
    assert!(bb_diag.max.x >= 101.0);
    assert!(bb_diag.max.y >= 101.0);

    // Zero-length arrow
    let zero_arrow = Element::Arrow {
        id: 32,
        start: Point::new(30.0, 30.0),
        end: Point::new(30.0, 30.0),
        style,
    };
    let bb_zero = zero_arrow.bounding_box();
    assert_eq!(bb_zero.min, Point::new(29.0, 29.0));
    assert_eq!(bb_zero.max, Point::new(31.0, 31.0));
}

#[test]
fn test_stroke_bounding_box_empty_single_and_variable_pressure() {
    let style = style_with_stroke_width(4.0);

    // Empty stroke
    let empty_stroke = Element::Stroke {
        id: 40,
        points: Vec::new(),
        pressures: Vec::new(),
        style: style.clone(),
    };
    assert_eq!(
        empty_stroke.bounding_box(),
        Rect::from_xywh(0.0, 0.0, 0.0, 0.0)
    );

    // Single-point stroke dab (center (50, 50), pressure 1.0, half_width 2.0 -> radius 2.0)
    let single_stroke = Element::Stroke {
        id: 41,
        points: vec![Point::new(50.0, 50.0)],
        pressures: vec![1.0],
        style: style.clone(),
    };
    let bb_single = single_stroke.bounding_box();
    assert_eq!(bb_single.min, Point::new(48.0, 48.0));
    assert_eq!(bb_single.max, Point::new(52.0, 52.0));

    // Multi-point stroke with variable pressures
    let var_stroke = Element::Stroke {
        id: 42,
        points: vec![
            Point::new(10.0, 20.0),
            Point::new(50.0, 20.0),
            Point::new(100.0, 20.0),
        ],
        pressures: vec![0.5, 1.5, 0.5],
        style,
    };
    let bb_var = var_stroke.bounding_box();
    // At center point (50, 20), pressure = 1.5 -> radius = 2.0 * 1.5 = 3.0 -> y extends from 17 to 23
    assert!(bb_var.min.y <= 17.0);
    assert!(bb_var.max.y >= 23.0);
    // At x=10, radius = 1.0 -> min.x = 9.0
    assert_eq!(bb_var.min.x, 9.0);
    // At x=100, radius = 1.0 -> max.x = 101.0
    assert_eq!(bb_var.max.x, 101.0);
}
