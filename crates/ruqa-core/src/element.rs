//! Whiteboard geometric shape primitives and element models.
//!
//! Defines the core [`Element`] algebraic enum and [`ElementStyle`] attributes
//! representing whiteboard vector shapes (rectangles, ellipses, diamonds, lines, arrows).

use crate::geometry::{Point, Rect};
pub use crate::style::{ElementStyle, FillStyle};
use serde::{Deserialize, Serialize};

/// Unique identifier for an element in the whiteboard scene.
pub type ElementId = u64;

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
    /// An on-canvas multiline text element with font size and styling.
    Text {
        /// Unique monotonic element identifier.
        id: ElementId,
        /// Top-left anchor point in canvas world coordinates.
        position: Point,
        /// Text content string (UTF-8, supporting mixed Arabic/Latin/numbers).
        content: String,
        /// Font size in world pixels.
        font_size: f32,
        /// Visual appearance and color parameters.
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
            | Self::Stroke { id, .. }
            | Self::Text { id, .. } => id,
        }
    }

    /// Sets the unique identifier of this element.
    pub fn set_id(&mut self, new_id: ElementId) {
        match self {
            Self::Rectangle { id, .. }
            | Self::Ellipse { id, .. }
            | Self::Diamond { id, .. }
            | Self::Line { id, .. }
            | Self::Arrow { id, .. }
            | Self::Stroke { id, .. }
            | Self::Text { id, .. } => *id = new_id,
        }
    }

    /// Returns a duplicate of this element with a fresh ID, new PRNG roughness seed, and translated by `offset`.
    #[must_use]
    pub fn duplicate_with(&self, new_id: ElementId, new_seed: u64, offset: Point) -> Self {
        let mut cloned = self.clone();
        cloned.set_id(new_id);
        cloned.style_mut().seed = new_seed;
        cloned.translate(offset);
        cloned
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
            | Self::Stroke { style, .. }
            | Self::Text { style, .. } => style,
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
            | Self::Stroke { style, .. }
            | Self::Text { style, .. } => style,
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
            Self::Text { position, .. } => {
                *position = *position + delta;
            }
        }
    }

    /// Remaps the element geometry proportionally from `old_bounds` into `new_bounds`.
    pub fn scale_from_bounds(&mut self, old_bounds: Rect, new_bounds: Rect) {
        let old_w = old_bounds.width();
        let old_h = old_bounds.height();
        if old_w < 1e-4 || old_h < 1e-4 {
            return;
        }

        let remap_point = |p: Point| -> Point {
            let u = (p.x - old_bounds.min.x) / old_w;
            let v = (p.y - old_bounds.min.y) / old_h;
            Point::new(
                new_bounds.min.x + (u * new_bounds.width()),
                new_bounds.min.y + (v * new_bounds.height()),
            )
        };

        match self {
            Self::Rectangle { bounds, .. }
            | Self::Ellipse { bounds, .. }
            | Self::Diamond { bounds, .. } => {
                let p1 = remap_point(bounds.min);
                let p2 = remap_point(bounds.max);
                *bounds = Rect::from_min_max(p1, p2);
            }
            Self::Line { start, end, .. } | Self::Arrow { start, end, .. } => {
                *start = remap_point(*start);
                *end = remap_point(*end);
            }
            Self::Stroke { points, .. } => {
                for p in points {
                    *p = remap_point(*p);
                }
            }
            Self::Text {
                position,
                font_size,
                ..
            } => {
                *position = remap_point(*position);
                let scale_y = new_bounds.height() / old_bounds.height();
                *font_size = (*font_size * scale_y).clamp(8.0, 144.0);
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
            Self::Text {
                position,
                content,
                font_size,
                ..
            } => {
                let lines: Vec<&str> = if content.ends_with('\n') {
                    content.split('\n').collect()
                } else {
                    content.lines().collect()
                };
                let line_count = lines.len().max(1);
                let max_len = lines
                    .iter()
                    .map(|l| l.chars().count())
                    .max()
                    .unwrap_or(0)
                    .max(1);
                #[allow(clippy::cast_precision_loss)]
                let w = (max_len as f32 * font_size * 0.6).max(*font_size);
                #[allow(clippy::cast_precision_loss)]
                let h = (line_count as f32 * font_size * 1.3).max(*font_size);
                Rect::from_xywh(position.x, position.y, w, h)
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
            Self::Text { .. } => self.bounds(),
        }
    }

    /// Tests whether a world coordinate point hits this element within the specified tolerance.
    #[must_use]
    pub fn hit_test(&self, point: Point, tolerance: f32) -> bool {
        crate::hit_test::hit_test_element(self, point, tolerance)
    }
}
