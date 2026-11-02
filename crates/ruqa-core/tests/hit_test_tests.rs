//! Integration tests for precise shape hit-testing and distance tolerance.

use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::hit_test::DEFAULT_HIT_TOLERANCE;
use ruqa_core::scene::Scene;

fn style_stroke_only(w: f32) -> ElementStyle {
    ElementStyle {
        stroke_width: w,
        fill_color: None,
        fill_style: FillStyle::None,
        ..Default::default()
    }
}

fn style_filled(w: f32) -> ElementStyle {
    ElementStyle {
        stroke_width: w,
        fill_color: Some("#336699".into()),
        fill_style: FillStyle::Solid,
        ..Default::default()
    }
}

#[test]
fn test_unfilled_rectangle_hit_testing() {
    let rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(100.0, 100.0, 200.0, 100.0),
        style: style_stroke_only(2.0),
    };

    // Center interior: unfilled rectangle must MISS
    assert!(!rect.hit_test(Point::new(200.0, 150.0), DEFAULT_HIT_TOLERANCE));

    // Exactly on edge (x=100, y=150): must HIT
    assert!(rect.hit_test(Point::new(100.0, 150.0), 2.0));

    // Within default 8px tolerance of edge (x=95, y=150): must HIT
    assert!(rect.hit_test(Point::new(95.0, 150.0), DEFAULT_HIT_TOLERANCE));

    // Outside tolerance (x=80, y=150): must MISS
    assert!(!rect.hit_test(Point::new(80.0, 150.0), DEFAULT_HIT_TOLERANCE));
}

#[test]
fn test_filled_rectangle_hit_testing() {
    let rect = Element::Rectangle {
        id: 2,
        bounds: Rect::from_xywh(100.0, 100.0, 200.0, 100.0),
        style: style_filled(2.0),
    };

    // Center interior: filled rectangle must HIT even with 0 tolerance
    assert!(rect.hit_test(Point::new(200.0, 150.0), 0.0));

    // Just outside edge: within tolerance must HIT
    assert!(rect.hit_test(Point::new(95.0, 150.0), DEFAULT_HIT_TOLERANCE));

    // Outside tolerance: must MISS
    assert!(!rect.hit_test(Point::new(80.0, 150.0), DEFAULT_HIT_TOLERANCE));
}

#[test]
fn test_ellipse_hit_testing_filled_and_unfilled() {
    let unfilled = Element::Ellipse {
        id: 10,
        bounds: Rect::from_xywh(100.0, 100.0, 200.0, 100.0),
        style: style_stroke_only(2.0),
    };
    // Center of unfilled ellipse must MISS
    assert!(!unfilled.hit_test(Point::new(200.0, 150.0), DEFAULT_HIT_TOLERANCE));
    // Perimeter at (300, 150) must HIT
    assert!(unfilled.hit_test(Point::new(300.0, 150.0), 2.0));

    let filled = Element::Ellipse {
        id: 11,
        bounds: Rect::from_xywh(100.0, 100.0, 200.0, 100.0),
        style: style_filled(2.0),
    };
    // Center of filled ellipse must HIT
    assert!(filled.hit_test(Point::new(200.0, 150.0), 0.0));
}

#[test]
fn test_diamond_hit_testing() {
    let diamond = Element::Diamond {
        id: 20,
        bounds: Rect::from_xywh(100.0, 100.0, 100.0, 100.0),
        style: style_stroke_only(2.0),
    };
    // Center of unfilled diamond must MISS
    assert!(!diamond.hit_test(Point::new(150.0, 150.0), 5.0));
    // Top vertex (150, 100) must HIT
    assert!(diamond.hit_test(Point::new(150.0, 100.0), 2.0));
    // Far corner (105, 105) is outside diamond perimeter and outside tolerance -> MISS
    assert!(!diamond.hit_test(Point::new(105.0, 105.0), 5.0));

    let filled = Element::Diamond {
        id: 21,
        bounds: Rect::from_xywh(100.0, 100.0, 100.0, 100.0),
        style: style_filled(2.0),
    };
    // Center of filled diamond must HIT
    assert!(filled.hit_test(Point::new(150.0, 150.0), 0.0));
}

#[test]
fn test_line_and_arrowhead_hit_testing() {
    let line = Element::Line {
        id: 30,
        start: Point::new(10.0, 20.0),
        end: Point::new(100.0, 20.0),
        style: style_stroke_only(4.0),
    };
    // Point on segment
    assert!(line.hit_test(Point::new(50.0, 20.0), 0.0));
    // Point 5px away (with 8px tolerance)
    assert!(line.hit_test(Point::new(50.0, 25.0), DEFAULT_HIT_TOLERANCE));
    // Point 20px away
    assert!(!line.hit_test(Point::new(50.0, 40.0), DEFAULT_HIT_TOLERANCE));

    let arrow = Element::Arrow {
        id: 31,
        start: Point::new(0.0, 50.0),
        end: Point::new(100.0, 50.0),
        style: style_stroke_only(2.0),
    };
    // Hitting near the arrowhead (barb extends backward at 30 deg to approx (86, 42) and (86, 58))
    assert!(arrow.hit_test(Point::new(88.0, 44.0), DEFAULT_HIT_TOLERANCE));
}

#[test]
fn test_freehand_stroke_hit_testing() {
    let stroke = Element::Stroke {
        id: 40,
        points: vec![
            Point::new(0.0, 0.0),
            Point::new(50.0, 20.0),
            Point::new(100.0, 80.0),
        ],
        pressures: vec![1.0, 1.0, 1.0],
        style: style_stroke_only(4.0),
    };

    // Point near middle segment
    assert!(stroke.hit_test(Point::new(25.0, 10.0), 2.0));
    // Point within 8px tolerance
    assert!(stroke.hit_test(Point::new(25.0, 16.0), DEFAULT_HIT_TOLERANCE));
    // Far away point
    assert!(!stroke.hit_test(Point::new(25.0, 40.0), DEFAULT_HIT_TOLERANCE));
}

#[test]
fn test_scene_topmost_hit_test_respects_z_order() {
    let mut scene = Scene::new();
    let bottom_rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(50.0, 50.0, 100.0, 100.0),
        style: style_filled(2.0),
    };
    let top_rect = Element::Rectangle {
        id: 2,
        bounds: Rect::from_xywh(80.0, 80.0, 100.0, 100.0),
        style: style_filled(2.0),
    };

    scene.push(bottom_rect);
    scene.push(top_rect);

    // Click in overlap region (100, 100): topmost element (id 2) should be returned
    let hit = scene.hit_test_topmost(Point::new(100.0, 100.0), 0.0);
    assert_eq!(hit.map(Element::id), Some(2));

    // Click exclusively in bottom rect (60, 60): bottom element (id 1) should be returned
    let hit_bottom = scene.hit_test_topmost(Point::new(60.0, 60.0), 0.0);
    assert_eq!(hit_bottom.map(Element::id), Some(1));

    // hit_test_all in overlap should yield [2, 1]
    let all_hits: Vec<u64> = scene
        .hit_test_all(Point::new(100.0, 100.0), 0.0)
        .map(Element::id)
        .collect();
    assert_eq!(all_hits, vec![2, 1]);
}
