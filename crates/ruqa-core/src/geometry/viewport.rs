//! 2D infinite canvas camera tracking world translation offset and scale factor.

use super::point::{Point, Size};
use super::rect::Rect;
use serde::{Deserialize, Serialize};

/// 2D infinite canvas camera tracking world translation offset and scale factor.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    /// World coordinates currently visible at the center of the viewport window.
    pub pan: Point,
    /// Zoom scale factor clamped between [`Self::MIN_ZOOM`] and [`Self::MAX_ZOOM`].
    pub zoom: f32,
}

impl Viewport {
    /// Minimum zoom scale boundary (10% scale).
    pub const MIN_ZOOM: f32 = 0.1;
    /// Maximum zoom scale boundary (1000% scale).
    pub const MAX_ZOOM: f32 = 10.0;
    /// Default zoom scale multiplier (100% scale).
    pub const DEFAULT_ZOOM: f32 = 1.0;

    /// Creates a new viewport with clamped zoom factor.
    #[must_use]
    pub fn new(pan: Point, zoom: f32) -> Self {
        Self {
            pan,
            zoom: zoom.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM),
        }
    }

    /// Updates the zoom level, clamping it between [`Self::MIN_ZOOM`] and [`Self::MAX_ZOOM`].
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
    }

    /// Translates the camera pan position by a world delta.
    pub fn pan_by(&mut self, delta: Point) {
        self.pan = self.pan + delta;
    }

    /// Converts screen pixel coordinates to infinite canvas world coordinates.
    ///
    /// # Formulas
    /// `world_pt = pan + ((screen_pt - screen_center) / zoom)`
    #[must_use]
    pub fn screen_to_world(&self, screen_pt: Point, window_size: Size) -> Point {
        let center = window_size.center();
        let offset_from_center = screen_pt - center;
        self.pan + (offset_from_center / self.zoom)
    }

    /// Converts canvas world coordinates to screen pixel coordinates.
    ///
    /// # Formulas
    /// `screen_pt = screen_center + ((world_pt - pan) * zoom)`
    #[must_use]
    pub fn world_to_screen(&self, world_pt: Point, window_size: Size) -> Point {
        let center = window_size.center();
        let offset_from_pan = world_pt - self.pan;
        center + (offset_from_pan * self.zoom)
    }

    /// Calculates the visible rectangular bounds of the infinite canvas in world coordinates.
    #[must_use]
    pub fn visible_world_rect(&self, window_size: Size) -> Rect {
        let top_left = self.screen_to_world(Point::ZERO, window_size);
        let bottom_right = self.screen_to_world(
            Point::new(window_size.width, window_size.height),
            window_size,
        );
        Rect::from_min_max(top_left, bottom_right)
    }

    /// Zooms the viewport anchored at a specific screen point.
    ///
    /// The world point under `screen_anchor` remains at the exact same screen position
    /// after zooming.
    pub fn zoom_at(&mut self, screen_anchor: Point, factor: f32, window_size: Size) {
        let target_zoom = (self.zoom * factor).clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        if (target_zoom - self.zoom).abs() < f32::EPSILON {
            return;
        }

        let world_anchor = self.screen_to_world(screen_anchor, window_size);
        self.zoom = target_zoom;
        let center = window_size.center();
        self.pan = world_anchor - ((screen_anchor - center) / self.zoom);
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(Point::ZERO, Self::DEFAULT_ZOOM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_screen_world_round_trip() {
        let viewport = Viewport::new(Point::new(150.0, -75.0), 1.5);
        let window_size = Size::new(1280.0, 800.0);

        let test_points = [
            Point::new(0.0, 0.0),
            Point::new(640.0, 400.0),
            Point::new(1280.0, 800.0),
            Point::new(123.4, 567.8),
        ];

        for &screen_pt in &test_points {
            let world_pt = viewport.screen_to_world(screen_pt, window_size);
            let round_trip = viewport.world_to_screen(world_pt, window_size);
            assert!(
                (round_trip.x - screen_pt.x).abs() < 1e-4,
                "x mismatch for {screen_pt:?}: got {round_trip:?}"
            );
            assert!(
                (round_trip.y - screen_pt.y).abs() < 1e-4,
                "y mismatch for {screen_pt:?}: got {round_trip:?}"
            );
        }
    }

    #[test]
    fn test_viewport_visible_world_rect_contains_pan_center() {
        let viewport = Viewport::new(Point::new(250.0, 100.0), 2.0);
        let window_size = Size::new(800.0, 600.0);

        let rect = viewport.visible_world_rect(window_size);
        assert!(rect.contains(viewport.pan));
        assert_eq!(rect.center(), viewport.pan);
        assert!((rect.width() - 400.0).abs() < 1e-4);
        assert!((rect.height() - 300.0).abs() < 1e-4);
    }

    #[test]
    fn test_viewport_anchored_zoom() {
        let mut viewport = Viewport::new(Point::new(0.0, 0.0), 1.0);
        let window_size = Size::new(1280.0, 800.0);
        let anchor = Point::new(300.0, 200.0);

        let world_before = viewport.screen_to_world(anchor, window_size);
        viewport.zoom_at(anchor, 2.0, window_size);
        let world_after = viewport.screen_to_world(anchor, window_size);

        assert!(
            (world_before.x - world_after.x).abs() < 1e-4,
            "anchor x drifted: before={world_before:?}, after={world_after:?}"
        );
        assert!(
            (world_before.y - world_after.y).abs() < 1e-4,
            "anchor y drifted: before={world_before:?}, after={world_after:?}"
        );
        assert_eq!(viewport.zoom, 2.0);
    }

    #[test]
    fn test_viewport_zoom_clamping() {
        let mut vp = Viewport::new(Point::ZERO, 0.01);
        assert_eq!(vp.zoom, Viewport::MIN_ZOOM);

        vp.set_zoom(100.0);
        assert_eq!(vp.zoom, Viewport::MAX_ZOOM);

        vp.set_zoom(2.5);
        assert_eq!(vp.zoom, 2.5);
    }

    #[test]
    fn test_viewport_zero_window_size_edge_case() {
        let vp = Viewport::new(Point::new(50.0, 50.0), 1.0);
        let zero_size = Size::ZERO;
        let screen = Point::new(10.0, 10.0);
        let world = vp.screen_to_world(screen, zero_size);
        assert_eq!(world, Point::new(60.0, 60.0));
        let round_trip = vp.world_to_screen(world, zero_size);
        assert_eq!(round_trip, screen);
    }
}
