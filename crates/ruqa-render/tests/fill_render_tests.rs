//! Integration tests for organic sketchy fill patterns (Solid, SemiTransparent, Hachure, CrossHatch).

use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::geometry::{Point, Rect};
use ruqa_render::shape_render::render_element;
use tiny_skia::{Color, Pixmap, Transform};

fn count_colored_pixels(pixmap: &Pixmap, target: Color) -> usize {
    let tr = target.red();
    let tg = target.green();
    let tb = target.blue();

    pixmap
        .pixels()
        .iter()
        .filter(|p| {
            // Premultiplied ColorU8 comparison with small tolerance
            let pr = f32::from(p.red()) / 255.0;
            let pg = f32::from(p.green()) / 255.0;
            let pb = f32::from(p.blue()) / 255.0;
            (pr - tr).abs() < 0.15 && (pg - tg).abs() < 0.15 && (pb - tb).abs() < 0.15
        })
        .count()
}

#[test]
fn test_rectangle_solid_and_semitransparent_fill() {
    let mut solid_pixmap = Pixmap::new(200, 200).unwrap();
    solid_pixmap.fill(Color::BLACK);

    let rect_solid = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(40.0, 40.0, 120.0, 120.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(), // White outline
            stroke_width: 2.0,
            fill_color: Some("#FF6B6B".to_string()), // Coral fill
            fill_style: FillStyle::Solid,
            roughness: 1.0,
            seed: 42,
        },
    };
    render_element(
        &rect_solid,
        &mut solid_pixmap.as_mut(),
        Transform::identity(),
    );

    // Center pixel should be coral fill
    let center_pixel = solid_pixmap.pixel(100, 100).expect("center pixel");
    assert!(
        center_pixel.red() > 200,
        "center pixel should be coral red: {:?}",
        center_pixel
    );

    // Semi-transparent should have lower alpha/brightness against black background
    let mut semi_pixmap = Pixmap::new(200, 200).unwrap();
    semi_pixmap.fill(Color::BLACK);

    let rect_semi = Element::Rectangle {
        id: 2,
        bounds: Rect::from_xywh(40.0, 40.0, 120.0, 120.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FF6B6B".to_string()),
            fill_style: FillStyle::SemiTransparent,
            roughness: 1.0,
            seed: 42,
        },
    };
    render_element(&rect_semi, &mut semi_pixmap.as_mut(), Transform::identity());

    let semi_center = semi_pixmap.pixel(100, 100).expect("semi center pixel");
    assert!(semi_center.red() > 50, "semi center should have color");
    assert!(
        semi_center.red() < center_pixel.red(),
        "semi-transparent fill red component should be less than solid ({} vs {})",
        semi_center.red(),
        center_pixel.red()
    );
}

#[test]
fn test_hachure_pattern_fill_produces_lines_and_gaps() {
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill(Color::BLACK);

    let rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(20.0, 20.0, 160.0, 160.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#38BDF8".to_string()), // Cyan fill
            fill_style: FillStyle::Hachure,
            roughness: 1.2,
            seed: 12345,
        },
    };
    render_element(&rect, &mut pixmap.as_mut(), Transform::identity());

    let cyan = Color::from_rgba8(56, 189, 248, 255);
    let cyan_pixels = count_colored_pixels(&pixmap, cyan);
    assert!(
        cyan_pixels > 200,
        "hachure pattern must draw substantial fill pixels (got {cyan_pixels})"
    );

    // Inside the shape, verify there exist both colored hachure pixels and black gaps
    let mut black_gaps_inside = 0;
    let mut colored_inside = 0;
    for y in 50..150 {
        for x in 50..150 {
            let p = pixmap.pixel(x, y).unwrap();
            if p.red() == 0 && p.green() == 0 && p.blue() == 0 {
                black_gaps_inside += 1;
            } else {
                colored_inside += 1;
            }
        }
    }
    assert!(
        black_gaps_inside > 500,
        "hachure fill must preserve gaps between sketchy lines (got {black_gaps_inside})"
    );
    assert!(
        colored_inside > 500,
        "hachure fill must contain sketchy lines (got {colored_inside})"
    );
}

#[test]
fn test_cross_hatch_has_higher_density_than_single_hachure() {
    let mut hachure_pixmap = Pixmap::new(200, 200).unwrap();
    hachure_pixmap.fill(Color::BLACK);

    let hachure_elem = Element::Ellipse {
        id: 1,
        bounds: Rect::from_xywh(30.0, 30.0, 140.0, 140.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FFD166".to_string()), // Yellow
            fill_style: FillStyle::Hachure,
            roughness: 1.0,
            seed: 999,
        },
    };
    render_element(
        &hachure_elem,
        &mut hachure_pixmap.as_mut(),
        Transform::identity(),
    );

    let mut cross_pixmap = Pixmap::new(200, 200).unwrap();
    cross_pixmap.fill(Color::BLACK);

    let cross_elem = Element::Ellipse {
        id: 2,
        bounds: Rect::from_xywh(30.0, 30.0, 140.0, 140.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FFD166".to_string()),
            fill_style: FillStyle::CrossHatch,
            roughness: 1.0,
            seed: 999,
        },
    };
    render_element(
        &cross_elem,
        &mut cross_pixmap.as_mut(),
        Transform::identity(),
    );

    let yellow = Color::from_rgba8(255, 209, 102, 255);
    let hachure_count = count_colored_pixels(&hachure_pixmap, yellow);
    let cross_count = count_colored_pixels(&cross_pixmap, yellow);

    assert!(
        cross_count > hachure_count,
        "cross-hatch ({cross_count}) should have higher pixel density than single hachure ({hachure_count})"
    );
}

#[test]
fn test_diamond_fill_rendering() {
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill(Color::BLACK);

    let diamond = Element::Diamond {
        id: 3,
        bounds: Rect::from_xywh(30.0, 30.0, 140.0, 140.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#06D6A0".to_string()), // Green
            fill_style: FillStyle::Hachure,
            roughness: 1.2,
            seed: 321,
        },
    };
    render_element(&diamond, &mut pixmap.as_mut(), Transform::identity());

    let green = Color::from_rgba8(6, 214, 160, 255);
    let green_pixels = count_colored_pixels(&pixmap, green);
    assert!(
        green_pixels > 150,
        "diamond hachure must render green lines (got {green_pixels})"
    );
}

#[test]
fn test_none_fill_leaves_interior_unpainted() {
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill(Color::BLACK);

    let rect = Element::Rectangle {
        id: 1,
        bounds: Rect::from_xywh(30.0, 30.0, 140.0, 140.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FF6B6B".to_string()),
            fill_style: FillStyle::None, // None suppresses fill
            roughness: 1.0,
            seed: 1,
        },
    };
    render_element(&rect, &mut pixmap.as_mut(), Transform::identity());

    // Center pixel must remain pure black
    let center = pixmap.pixel(100, 100).expect("center");
    assert_eq!(center.red(), 0);
    assert_eq!(center.green(), 0);
    assert_eq!(center.blue(), 0);
}

#[test]
fn test_open_shapes_ignore_fill_style() {
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill(Color::BLACK);

    let line = Element::Line {
        id: 1,
        start: Point::new(20.0, 100.0),
        end: Point::new(180.0, 100.0),
        style: ElementStyle {
            stroke_color: "#ECECEC".to_string(),
            stroke_width: 2.0,
            fill_color: Some("#FF6B6B".to_string()),
            fill_style: FillStyle::Solid,
            roughness: 1.0,
            seed: 1,
        },
    };
    render_element(&line, &mut pixmap.as_mut(), Transform::identity());

    // No coral red pixels anywhere
    let coral = Color::from_rgba8(255, 107, 107, 255);
    assert_eq!(count_colored_pixels(&pixmap, coral), 0);
}
