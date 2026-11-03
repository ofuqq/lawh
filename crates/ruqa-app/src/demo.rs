//! Sample demo board generator for visual exploration and interaction testing.

use ruqa_core::element::{Element, ElementStyle, FillStyle};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::scene::Scene;
use ruqa_core::style::PALETTE_COLORS;

/// Populates `scene` with representative geometric primitives and a freehand ink stroke.
///
/// Places shapes around the world coordinate origin with diverse curated palette colors,
/// fill patterns, and deterministic seeds, enabling visual inspection and interaction testing.
pub fn populate_demo_board(scene: &mut Scene) {
    // 1. Hand-drawn rectangle (Cyan #38BDF8 with semi-transparent fill)
    let rect_id = scene.next_element_id();
    scene.push(Element::Rectangle {
        id: rect_id,
        bounds: Rect::from_xywh(-250.0, -140.0, 150.0, 90.0),
        style: ElementStyle {
            stroke_color: PALETTE_COLORS[4].into(), // Cyan
            fill_color: Some(PALETTE_COLORS[4].into()),
            fill_style: FillStyle::SemiTransparent,
            stroke_width: 2.5,
            roughness: 1.0,
            seed: 1001,
        },
    });

    // 2. Hand-drawn ellipse (Green #06D6A0 with hachure fill)
    let ellipse_id = scene.next_element_id();
    scene.push(Element::Ellipse {
        id: ellipse_id,
        bounds: Rect::from_xywh(-60.0, -140.0, 140.0, 90.0),
        style: ElementStyle {
            stroke_color: PALETTE_COLORS[3].into(), // Green
            fill_color: Some(PALETTE_COLORS[3].into()),
            fill_style: FillStyle::Hachure,
            stroke_width: 2.5,
            roughness: 1.0,
            seed: 1002,
        },
    });

    // 3. Hand-drawn decision diamond (Yellow #FFD166 with solid fill)
    let diamond_id = scene.next_element_id();
    scene.push(Element::Diamond {
        id: diamond_id,
        bounds: Rect::from_xywh(120.0, -140.0, 130.0, 90.0),
        style: ElementStyle {
            stroke_color: PALETTE_COLORS[2].into(), // Yellow
            fill_color: Some(PALETTE_COLORS[2].into()),
            fill_style: FillStyle::Solid,
            stroke_width: 2.5,
            roughness: 1.0,
            seed: 1003,
        },
    });

    // 4. Directional connector arrow (Coral #FF6B6B)
    let arrow_id = scene.next_element_id();
    scene.push(Element::Arrow {
        id: arrow_id,
        start: Point::new(-175.0, -10.0),
        end: Point::new(-175.0, 90.0),
        style: ElementStyle {
            stroke_color: PALETTE_COLORS[1].into(), // Coral
            fill_color: None,
            fill_style: FillStyle::None,
            stroke_width: 2.5,
            roughness: 1.0,
            seed: 1004,
        },
    });

    // 5. Straight line segment (Lavender #A78BFA)
    let line_id = scene.next_element_id();
    scene.push(Element::Line {
        id: line_id,
        start: Point::new(10.0, -10.0),
        end: Point::new(10.0, 90.0),
        style: ElementStyle {
            stroke_color: PALETTE_COLORS[5].into(), // Lavender
            fill_color: None,
            fill_style: FillStyle::None,
            stroke_width: 2.5,
            roughness: 1.0,
            seed: 1005,
        },
    });

    // 6. Freehand smoothed pen stroke (Crisp Off-White #ECEFF4)
    let stroke_id = scene.next_element_id();
    let points = vec![
        Point::new(80.0, 20.0),
        Point::new(105.0, 35.0),
        Point::new(135.0, 15.0),
        Point::new(165.0, 45.0),
        Point::new(195.0, 30.0),
        Point::new(225.0, 60.0),
    ];
    let pressures = vec![0.3, 0.6, 0.9, 0.8, 0.5, 0.3];
    scene.push(Element::Stroke {
        id: stroke_id,
        points,
        pressures,
        style: ElementStyle {
            stroke_color: PALETTE_COLORS[0].into(), // Off-White
            fill_color: None,
            fill_style: FillStyle::None,
            stroke_width: 3.0,
            roughness: 1.0,
            seed: 1006,
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate_demo_board_adds_primitives() {
        let mut scene = Scene::new();
        assert_eq!(scene.len(), 0);

        populate_demo_board(&mut scene);
        assert_eq!(scene.len(), 6);

        // Verify each required primitive kind is present
        assert!(scene
            .elements()
            .iter()
            .any(|e| matches!(e, Element::Rectangle { .. })));
        assert!(scene
            .elements()
            .iter()
            .any(|e| matches!(e, Element::Ellipse { .. })));
        assert!(scene
            .elements()
            .iter()
            .any(|e| matches!(e, Element::Diamond { .. })));
        assert!(scene
            .elements()
            .iter()
            .any(|e| matches!(e, Element::Arrow { .. })));
        assert!(scene
            .elements()
            .iter()
            .any(|e| matches!(e, Element::Line { .. })));
        assert!(scene
            .elements()
            .iter()
            .any(|e| matches!(e, Element::Stroke { .. })));
    }
}
