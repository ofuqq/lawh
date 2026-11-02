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

    /// Computes the axis-aligned bounding box enclosing this element in world coordinates.
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

        let stroke_empty = Element::Stroke {
            id: 3,
            points: Vec::new(),
            pressures: Vec::new(),
            style: style.clone(),
        };
        assert_eq!(stroke_empty.bounds(), Rect::from_xywh(0.0, 0.0, 0.0, 0.0));

        let stroke = Element::Stroke {
            id: 4,
            points: vec![
                Point::new(10.0, 50.0),
                Point::new(30.0, 20.0),
                Point::new(5.0, 40.0),
            ],
            pressures: vec![0.5, 0.6, 0.7],
            style,
        };
        assert_eq!(stroke.bounds().min, Point::new(5.0, 20.0));
        assert_eq!(stroke.bounds().max, Point::new(30.0, 50.0));
    }

    #[test]
    fn test_serde_json_round_trips() {
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
            let deserialized: Element =
                serde_json::from_str(&json).expect("deserialization succeeds");
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
}
