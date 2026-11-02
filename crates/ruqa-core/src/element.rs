//! Whiteboard geometric shape primitives and element models.
//!
//! Defines the core [`Element`] algebraic enum and [`ElementStyle`] attributes
//! representing whiteboard vector shapes (rectangles, ellipses, diamonds, lines, arrows).

use crate::geometry::{Point, Rect};
use serde::{Deserialize, Serialize};

/// Unique identifier for an element in the whiteboard scene.
pub type ElementId = u64;

/// Fill pattern style for closed whiteboard shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FillStyle {
    /// No fill (transparent interior).
    #[default]
    None,
    /// Solid single-color fill.
    Solid,
    /// Hand-drawn parallel hachure lines.
    Hachure,
    /// Crossed hand-drawn hatch lines.
    CrossHatch,
    /// Stippled dot pattern fill.
    Dots,
}

/// Styling and roughness parameters for a whiteboard element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementStyle {
    /// Stroke outline color in hex format (e.g. `"#E0E0E0"`).
    pub stroke_color: String,
    /// Stroke outline width in world units (standard presets: 1.5, 3.0, 5.0).
    pub stroke_width: f32,
    /// Optional fill color in hex format (e.g. `"#1E88E5"`).
    pub fill_color: Option<String>,
    /// Fill pattern style.
    pub fill_style: FillStyle,
    /// Roughness factor (0.0 for crisp straight lines, 1.2 for hand-drawn, 2.5 for very sketchy).
    pub roughness: f32,
    /// Deterministic PRNG seed for reproducible rough lines across renders.
    pub seed: u64,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            stroke_color: "#E0E0E0".to_string(),
            stroke_width: 2.0,
            fill_color: None,
            fill_style: FillStyle::None,
            roughness: 1.2,
            seed: 0,
        }
    }
}

/// Algebraic enum representing any geometric whiteboard primitive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Element {
    /// An axis-aligned rectangular shape.
    Rectangle {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Spatial bounding box in world coordinates.
        bounds: Rect,
        /// Visual appearance and rough parameters.
        style: ElementStyle,
    },
    /// An ellipse or circle bounded by a rectangle.
    Ellipse {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Spatial bounding box in world coordinates.
        bounds: Rect,
        /// Visual appearance and rough parameters.
        style: ElementStyle,
    },
    /// A diamond (rhombus) shape oriented within bounding box corners.
    Diamond {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Spatial bounding box in world coordinates.
        bounds: Rect,
        /// Visual appearance and rough parameters.
        style: ElementStyle,
    },
    /// A two-point linear segment.
    Line {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Starting point in world coordinates.
        start: Point,
        /// Ending point in world coordinates.
        end: Point,
        /// Visual appearance and rough parameters.
        style: ElementStyle,
    },
    /// A directional arrow pointing from start to end with an arrowhead.
    Arrow {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Starting point in world coordinates.
        start: Point,
        /// Ending tip point in world coordinates.
        end: Point,
        /// Visual appearance and rough parameters.
        style: ElementStyle,
    },
    /// A continuous freehand smoothed ink stroke with dynamic pressure.
    Stroke {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Ordered sequence of smoothed path coordinates.
        points: Vec<Point>,
        /// Simulated or hardware pressure values (0.0 to 1.0) per point.
        pressures: Vec<f32>,
        /// Visual appearance and rough parameters.
        style: ElementStyle,
    },
}

impl Element {
    /// Returns the unique monotonic identifier of this element.
    #[must_use]
    pub const fn id(&self) -> ElementId {
        match *self {
            Self::Rectangle { id, .. }
            | Self::Ellipse { id, .. }
            | Self::Diamond { id, .. }
            | Self::Line { id, .. }
            | Self::Arrow { id, .. }
            | Self::Stroke { id, .. } => id,
        }
    }

    /// Returns a reference to the element's visual styling.
    #[must_use]
    pub const fn style(&self) -> &ElementStyle {
        match self {
            Self::Rectangle { style, .. }
            | Self::Ellipse { style, .. }
            | Self::Diamond { style, .. }
            | Self::Line { style, .. }
            | Self::Arrow { style, .. }
            | Self::Stroke { style, .. } => style,
        }
    }

    /// Returns a mutable reference to the element's visual styling.
    #[must_use]
    pub fn style_mut(&mut self) -> &mut ElementStyle {
        match self {
            Self::Rectangle { style, .. }
            | Self::Ellipse { style, .. }
            | Self::Diamond { style, .. }
            | Self::Line { style, .. }
            | Self::Arrow { style, .. }
            | Self::Stroke { style, .. } => style,
        }
    }

    /// Translates the element by `delta` in world coordinates.
    pub fn translate(&mut self, delta: Point) {
        match self {
            Self::Rectangle { bounds, .. }
            | Self::Ellipse { bounds, .. }
            | Self::Diamond { bounds, .. } => *bounds = bounds.translate(delta),
            Self::Line { start, end, .. } | Self::Arrow { start, end, .. } => {
                *start = *start + delta;
                *end = *end + delta;
            }
            Self::Stroke { points, .. } => {
                for p in points {
                    *p = *p + delta;
                }
            }
        }
    }

    /// Computes the unpadded geometric bounds enclosing this element in world coordinates.
    #[must_use]
    pub fn bounds(&self) -> Rect {
        match self {
            Self::Rectangle { bounds, .. }
            | Self::Ellipse { bounds, .. }
            | Self::Diamond { bounds, .. } => *bounds,
            Self::Line { start, end, .. } | Self::Arrow { start, end, .. } => {
                Rect::from_min_max(*start, *end)
            }
            Self::Stroke { points, .. } => {
                if points.is_empty() {
                    Rect::from_xywh(0.0, 0.0, 0.0, 0.0)
                } else {
                    let mut min_x = points[0].x;
                    let mut max_x = points[0].x;
                    let mut min_y = points[0].y;
                    let mut max_y = points[0].y;
                    for pt in &points[1..] {
                        min_x = min_x.min(pt.x);
                        max_x = max_x.max(pt.x);
                        min_y = min_y.min(pt.y);
                        max_y = max_y.max(pt.y);
                    }
                    Rect::from_min_max(Point::new(min_x, min_y), Point::new(max_x, max_y))
                }
            }
        }
    }

    /// Computes the tight axis-aligned bounding box enclosing this element in world coordinates,
    /// including stroke width padding and arrowhead geometry.
    #[must_use]
    pub fn bounding_box(&self) -> Rect {
        let style = self.style();
        let half_width = (style.stroke_width * 0.5).max(0.5);

        match self {
            Self::Rectangle { bounds, .. }
            | Self::Ellipse { bounds, .. }
            | Self::Diamond { bounds, .. } => bounds.inflate(half_width),
            Self::Line { start, end, .. } => Rect::from_min_max(*start, *end).inflate(half_width),
            Self::Arrow { start, end, .. } => {
                let dx = end.x - start.x;
                let dy = end.y - start.y;
                let length = dx.hypot(dy);
                if length > 1e-4 {
                    let angle = dy.atan2(dx);
                    let barb_len = (12.0 + style.stroke_width * 2.0).min(length * 0.5);
                    let barb_angle = std::f32::consts::FRAC_PI_6; // 30 degrees in radians

                    let a1 = angle - std::f32::consts::PI + barb_angle;
                    let a2 = angle - std::f32::consts::PI - barb_angle;

                    let p1 = Point::new(end.x + barb_len * a1.cos(), end.y + barb_len * a1.sin());
                    let p2 = Point::new(end.x + barb_len * a2.cos(), end.y + barb_len * a2.sin());

                    let min_x = start.x.min(end.x).min(p1.x).min(p2.x);
                    let max_x = start.x.max(end.x).max(p1.x).max(p2.x);
                    let min_y = start.y.min(end.y).min(p1.y).min(p2.y);
                    let max_y = start.y.max(end.y).max(p1.y).max(p2.y);

                    Rect::from_min_max(Point::new(min_x, min_y), Point::new(max_x, max_y))
                        .inflate(half_width)
                } else {
                    Rect::from_min_max(*start, *end).inflate(half_width)
                }
            }
            Self::Stroke {
                points, pressures, ..
            } => {
                if points.is_empty() {
                    Rect::from_xywh(0.0, 0.0, 0.0, 0.0)
                } else {
                    let mut min_x = f32::INFINITY;
                    let mut max_x = f32::NEG_INFINITY;
                    let mut min_y = f32::INFINITY;
                    let mut max_y = f32::NEG_INFINITY;

                    for (i, pt) in points.iter().enumerate() {
                        let pressure = pressures.get(i).copied().unwrap_or(1.0).clamp(0.05, 2.0);
                        let r = (half_width * pressure).max(0.5);
                        min_x = min_x.min(pt.x - r);
                        max_x = max_x.max(pt.x + r);
                        min_y = min_y.min(pt.y - r);
                        max_y = max_y.max(pt.y + r);
                    }

                    Rect::from_min_max(Point::new(min_x, min_y), Point::new(max_x, max_y))
                }
            }
        }
    }

    /// Tests whether a world coordinate point hits this element within the specified tolerance.
    #[must_use]
    pub fn hit_test(&self, point: Point, tolerance: f32) -> bool {
        crate::hit_test::hit_test_element(self, point, tolerance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_style_defaults() {
        let style = ElementStyle::default();
        assert_eq!(style.stroke_color, "#E0E0E0");
        assert_eq!(style.stroke_width, 2.0);
        assert!(style.fill_color.is_none());
        assert_eq!(style.fill_style, FillStyle::None);
        assert_eq!(style.roughness, 1.2);
        assert_eq!(style.seed, 0);
    }

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
}
