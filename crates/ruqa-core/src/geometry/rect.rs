//! 2D bounding box rectangle primitive.

use super::point::Point;

/// An axis-aligned 2D bounding box defined by minimum and maximum corner points.
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Rect {
    /// Top-left (minimum x and y) corner in world coordinates.
    pub min: Point,
    /// Bottom-right (maximum x and y) corner in world coordinates.
    pub max: Point,
}

impl Rect {
    /// Creates a rectangle from minimum and maximum corner points.
    ///
    /// Normalizes corners so `min.x <= max.x` and `min.y <= max.y`.
    #[must_use]
    pub fn from_min_max(p1: Point, p2: Point) -> Self {
        Self {
            min: Point::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            max: Point::new(p1.x.max(p2.x), p1.y.max(p2.y)),
        }
    }

    /// Creates a rectangle from top-left origin `(x, y)` and dimensions `(width, height)`.
    #[must_use]
    pub fn from_xywh(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::from_min_max(Point::new(x, y), Point::new(x + width, y + height))
    }

    /// Returns the width of this rectangle.
    #[must_use]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Returns the height of this rectangle.
    #[must_use]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Returns the center point of this rectangle.
    #[must_use]
    pub fn center(&self) -> Point {
        Point::new(
            self.min.x + (self.width() * 0.5),
            self.min.y + (self.height() * 0.5),
        )
    }

    /// Returns true if the given point lies inside or on the boundaries of this rectangle.
    #[must_use]
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Returns true if two axis-aligned bounding boxes overlap.
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Returns the smallest bounding box enclosing both `self` and `other`.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Point::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: Point::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }

    /// Returns a new rectangle expanded symmetrically on all sides by `padding`.
    ///
    /// If `padding` is negative, the rectangle shrinks (insets). If shrinking would cause
    /// bounds to invert (`min > max`), returns a zero-area point rectangle at the center.
    #[must_use]
    pub fn inflate(&self, padding: f32) -> Self {
        let min_x = self.min.x - padding;
        let max_x = self.max.x + padding;
        let min_y = self.min.y - padding;
        let max_y = self.max.y + padding;

        if min_x > max_x || min_y > max_y {
            let c = self.center();
            Self::from_min_max(c, c)
        } else {
            Self {
                min: Point::new(min_x, min_y),
                max: Point::new(max_x, max_y),
            }
        }
    }

    /// Returns a new rectangle translated by `delta`.
    #[must_use]
    pub fn translate(&self, delta: Point) -> Self {
        Self {
            min: self.min + delta,
            max: self.max + delta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_bounds_and_intersection() {
        let r1 = Rect::from_xywh(10.0, 20.0, 100.0, 50.0);
        assert_eq!(r1.min, Point::new(10.0, 20.0));
        assert_eq!(r1.max, Point::new(110.0, 70.0));
        assert_eq!(r1.width(), 100.0);
        assert_eq!(r1.height(), 50.0);
        assert_eq!(r1.center(), Point::new(60.0, 45.0));

        assert!(r1.contains(Point::new(60.0, 45.0)));
        assert!(!r1.contains(Point::new(5.0, 45.0)));

        let r2 = Rect::from_xywh(50.0, 40.0, 80.0, 60.0);
        let r3 = Rect::from_xywh(200.0, 200.0, 30.0, 30.0);
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));

        let u = r1.union(&r3);
        assert_eq!(u.min, Point::new(10.0, 20.0));
        assert_eq!(u.max, Point::new(230.0, 230.0));

        let inflated = r1.inflate(5.0);
        assert_eq!(inflated.min, Point::new(5.0, 15.0));
        assert_eq!(inflated.max, Point::new(115.0, 75.0));

        let over_shrunk = r1.inflate(-100.0);
        assert_eq!(over_shrunk.width(), 0.0);
        assert_eq!(over_shrunk.height(), 0.0);
    }
}
