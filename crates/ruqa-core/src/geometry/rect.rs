//! 2D bounding box rectangle primitive.

use super::point::Point;

/// An axis-aligned 2D bounding box defined by minimum and maximum corner points.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
    }
}
