//! 2D point and size primitives.

use std::ops::{Add, Div, Mul, Sub};

/// A point in 2D continuous coordinate space.
#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: f32,
    /// Vertical coordinate.
    pub y: f32,
}

impl Point {
    /// The origin point `(0.0, 0.0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Creates a new 2D point.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Computes Euclidean distance between this point and another.
    #[must_use]
    pub fn distance(&self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.hypot(dy)
    }

    /// Computes the minimum Euclidean distance from this point to a finite 2D line segment `[a, b]`.
    #[must_use]
    pub fn distance_to_segment(self, a: Self, b: Self) -> f32 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let l2 = dx * dx + dy * dy;

        if l2 < 1e-6 {
            return self.distance(a);
        }

        let t = (((self.x - a.x) * dx + (self.y - a.y) * dy) / l2).clamp(0.0, 1.0);
        let proj = Self::new(a.x + t * dx, a.y + t * dy);
        self.distance(proj)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Point {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl From<(f32, f32)> for Point {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

/// A 2D dimension representing width and height.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    /// Width extent.
    pub width: f32,
    /// Height extent.
    pub height: f32,
}

impl Size {
    /// Zero size `(0.0, 0.0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Creates a new 2D size.
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Center point of this size starting from origin `(width / 2.0, height / 2.0)`.
    #[must_use]
    pub fn center(&self) -> Point {
        Point::new(self.width * 0.5, self.height * 0.5)
    }
}

impl From<(f32, f32)> for Size {
    #[inline]
    fn from((width, height): (f32, f32)) -> Self {
        Self::new(width, height)
    }
}

impl From<(u32, u32)> for Size {
    #[inline]
    fn from((width, height): (u32, u32)) -> Self {
        Self::new(width as f32, height as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_arithmetic_and_distance() {
        let p1 = Point::new(10.0, 20.0);
        let p2 = Point::new(4.0, 12.0);

        assert_eq!(p1 + p2, Point::new(14.0, 32.0));
        assert_eq!(p1 - p2, Point::new(6.0, 8.0));
        assert_eq!(p2 * 2.5, Point::new(10.0, 30.0));
        assert_eq!(p1 / 2.0, Point::new(5.0, 10.0));
        assert!((p1.distance(p2) - 10.0).abs() < 1e-6);

        // Point-to-segment distance
        let a = Point::new(0.0, 0.0);
        let b = Point::new(100.0, 0.0);
        // Point above segment
        assert!((Point::new(50.0, 15.0).distance_to_segment(a, b) - 15.0).abs() < 1e-6);
        // Point before segment origin
        assert!((Point::new(-10.0, 0.0).distance_to_segment(a, b) - 10.0).abs() < 1e-6);
        // Point past segment end
        assert!((Point::new(105.0, 0.0).distance_to_segment(a, b) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_size_center_and_conversions() {
        let s = Size::new(800.0, 600.0);
        assert_eq!(s.center(), Point::new(400.0, 300.0));

        let s_f: Size = (100.0, 50.0).into();
        assert_eq!(s_f, Size::new(100.0, 50.0));

        let s_u: Size = (1280u32, 720u32).into();
        assert_eq!(s_u, Size::new(1280.0, 720.0));
    }
}
