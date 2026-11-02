//! Precise point-in-shape and boundary distance hit-testing.
//!
//! Evaluates whether canvas interaction coordinates intersect vector elements,
//! distinguishing between filled interior regions and outline stroke tolerances.

use crate::element::{Element, FillStyle};
use crate::geometry::Point;
use std::f32::consts::FRAC_PI_6;

/// Default pixel tolerance radius for selecting elements with a mouse click (8 logical pixels).
pub const DEFAULT_HIT_TOLERANCE: f32 = 8.0;

/// Tests whether a world coordinate point hits an element within the given tolerance.
///
/// For filled shapes, tests point containment inside the shape boundary.
/// For unfilled shapes, lines, arrows, and strokes, tests Euclidean distance to the
/// perimeter or path segments within `tolerance + stroke_width / 2.0`.
#[must_use]
pub fn hit_test_element(element: &Element, point: Point, tolerance: f32) -> bool {
    let tolerance = tolerance.max(0.0);

    // Fast rejection: coarse axis-aligned bounding box check
    if !element.bounding_box().inflate(tolerance).contains(point) {
        return false;
    }

    let style = element.style();
    let stroke_radius = (style.stroke_width * 0.5).max(0.5);
    let effective_tolerance = tolerance + stroke_radius;
    let is_filled = style.fill_color.is_some() && style.fill_style != FillStyle::None;

    match element {
        Element::Rectangle { bounds, .. } => {
            if is_filled {
                bounds.inflate(tolerance).contains(point)
            } else {
                let p1 = bounds.min;
                let p2 = Point::new(bounds.max.x, bounds.min.y);
                let p3 = bounds.max;
                let p4 = Point::new(bounds.min.x, bounds.max.y);

                point.distance_to_segment(p1, p2) <= effective_tolerance
                    || point.distance_to_segment(p2, p3) <= effective_tolerance
                    || point.distance_to_segment(p3, p4) <= effective_tolerance
                    || point.distance_to_segment(p4, p1) <= effective_tolerance
            }
        }
        Element::Ellipse { bounds, .. } => {
            let center = bounds.center();
            let rx = (bounds.width() * 0.5).max(1e-4);
            let ry = (bounds.height() * 0.5).max(1e-4);

            let dx = point.x - center.x;
            let dy = point.y - center.y;
            let norm_dist_sq = (dx / rx).powi(2) + (dy / ry).powi(2);

            let angle = dy.atan2(dx);
            let boundary_pt = Point::new(center.x + rx * angle.cos(), center.y + ry * angle.sin());

            if is_filled {
                norm_dist_sq <= 1.0 || point.distance(boundary_pt) <= tolerance
            } else {
                point.distance(boundary_pt) <= effective_tolerance
            }
        }
        Element::Diamond { bounds, .. } => {
            let center = bounds.center();
            let v0 = Point::new(center.x, bounds.min.y);
            let v1 = Point::new(bounds.max.x, center.y);
            let v2 = Point::new(center.x, bounds.max.y);
            let v3 = Point::new(bounds.min.x, center.y);

            let dist_to_edges = point
                .distance_to_segment(v0, v1)
                .min(point.distance_to_segment(v1, v2))
                .min(point.distance_to_segment(v2, v3))
                .min(point.distance_to_segment(v3, v0));

            if is_filled {
                let rx = (bounds.width() * 0.5).max(1e-4);
                let ry = (bounds.height() * 0.5).max(1e-4);
                let inside_diamond =
                    (point.x - center.x).abs() / rx + (point.y - center.y).abs() / ry <= 1.0;
                inside_diamond || dist_to_edges <= tolerance
            } else {
                dist_to_edges <= effective_tolerance
            }
        }
        Element::Line { start, end, .. } => {
            point.distance_to_segment(*start, *end) <= effective_tolerance
        }
        Element::Arrow { start, end, .. } => {
            if point.distance_to_segment(*start, *end) <= effective_tolerance {
                return true;
            }

            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let length = dx.hypot(dy);
            if length > 1e-4 {
                let angle = dy.atan2(dx);
                let barb_len = (12.0 + style.stroke_width * 2.0).min(length * 0.5);

                let a1 = angle - std::f32::consts::PI + FRAC_PI_6;
                let a2 = angle - std::f32::consts::PI - FRAC_PI_6;

                let p1 = Point::new(end.x + barb_len * a1.cos(), end.y + barb_len * a1.sin());
                let p2 = Point::new(end.x + barb_len * a2.cos(), end.y + barb_len * a2.sin());

                point.distance_to_segment(*end, p1) <= effective_tolerance
                    || point.distance_to_segment(*end, p2) <= effective_tolerance
            } else {
                false
            }
        }
        Element::Stroke {
            points, pressures, ..
        } => {
            if points.is_empty() {
                return false;
            }
            if points.len() == 1 {
                let r = (stroke_radius * pressures.first().copied().unwrap_or(1.0)).max(0.5);
                return point.distance(points[0]) <= tolerance + r;
            }

            for i in 0..(points.len() - 1) {
                let p_curr = pressures.get(i).copied().unwrap_or(1.0);
                let p_next = pressures.get(i + 1).copied().unwrap_or(1.0);
                let r = (stroke_radius * (p_curr + p_next) * 0.5).max(0.5);

                if point.distance_to_segment(points[i], points[i + 1]) <= tolerance + r {
                    return true;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::ElementStyle;
    use crate::geometry::Rect;

    #[test]
    fn test_unfilled_rectangle_hit_boundary_vs_miss_interior() {
        let rect = Element::Rectangle {
            id: 1,
            bounds: Rect::from_xywh(100.0, 100.0, 200.0, 100.0),
            style: ElementStyle {
                fill_color: None,
                stroke_width: 2.0,
                ..Default::default()
            },
        };

        // Center of unfilled rectangle: should MISS with tolerance 5.0
        assert!(!hit_test_element(&rect, Point::new(200.0, 150.0), 5.0));

        // On top edge: should HIT
        assert!(hit_test_element(&rect, Point::new(150.0, 100.0), 2.0));

        // Just outside tolerance of top edge: should MISS
        assert!(!hit_test_element(&rect, Point::new(150.0, 85.0), 5.0));
    }

    #[test]
    fn test_filled_rectangle_hit_interior() {
        let rect = Element::Rectangle {
            id: 2,
            bounds: Rect::from_xywh(100.0, 100.0, 200.0, 100.0),
            style: ElementStyle {
                fill_color: Some("#FFFFFF".into()),
                fill_style: FillStyle::Solid,
                stroke_width: 2.0,
                ..Default::default()
            },
        };

        // Center of filled rectangle: should HIT
        assert!(hit_test_element(&rect, Point::new(200.0, 150.0), 0.0));
    }

    #[test]
    fn test_line_and_arrow_hit_testing() {
        let line = Element::Line {
            id: 3,
            start: Point::new(0.0, 0.0),
            end: Point::new(100.0, 100.0),
            style: ElementStyle {
                stroke_width: 2.0,
                ..Default::default()
            },
        };

        // Midpoint (50, 50): should HIT
        assert!(hit_test_element(&line, Point::new(50.0, 50.0), 2.0));

        // Point near line (50, 54) with tolerance 5.0: should HIT
        assert!(hit_test_element(&line, Point::new(50.0, 54.0), 5.0));

        // Far from line: should MISS
        assert!(!hit_test_element(&line, Point::new(50.0, 80.0), 5.0));
    }
}
