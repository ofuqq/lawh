//! Mouse event dispatch, viewport transformations, and zero-allocation pointer tracking.

/// State of mouse pointers and modifier keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct PointerState {
    /// Last known cursor position in screen coordinates (x, y).
    pub cursor_pos: Option<(f64, f64)>,
    /// Whether the primary left mouse button is pressed.
    pub left_pressed: bool,
    /// Whether the middle mouse button is pressed.
    pub middle_pressed: bool,
    /// Whether the Space key is held down (for panning with left click).
    pub space_held: bool,
}

impl PointerState {
    /// Creates a new default pointer state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cursor_pos: None,
            left_pressed: false,
            middle_pressed: false,
            space_held: false,
        }
    }

    /// Updates cursor coordinates with zero heap allocation.
    pub fn update_cursor(&mut self, x: f64, y: f64) -> Option<(f64, f64)> {
        let prev = self.cursor_pos;
        self.cursor_pos = Some((x, y));
        prev.map(|(px, py)| (x - px, y - py))
    }

    /// Whether current inputs indicate a view panning gesture (middle-drag or Space+left-drag).
    #[must_use]
    pub const fn is_panning(&self) -> bool {
        self.middle_pressed || (self.space_held && self.left_pressed)
    }
}

/// Viewport camera tracking 2D pan offsets and zoom level.
#[derive(Debug, Clone, Copy)]
pub struct ViewportCamera {
    /// Horizontal pan offset in screen pixels.
    pub offset_x: f64,
    /// Vertical pan offset in screen pixels.
    pub offset_y: f64,
    /// Current zoom scale multiplier (default 1.0 = 100%).
    pub zoom: f64,
    /// Minimum zoom boundary.
    pub min_zoom: f64,
    /// Maximum zoom boundary.
    pub max_zoom: f64,
}

impl Default for ViewportCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewportCamera {
    /// Creates a new viewport camera at default pan (0, 0) and 1.0x zoom.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 1.0,
            min_zoom: 0.1,
            max_zoom: 10.0,
        }
    }

    /// Translates camera position by screen delta (dx, dy).
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.offset_x += dx;
        self.offset_y += dy;
    }

    /// Zooms the camera centered at screen coordinate (cursor_x, cursor_y).
    ///
    /// Preserves the canvas point under the cursor so the zoom feels anchored:
    /// canvas_x = (cursor_x - offset_x) / zoom
    /// offset_x' = cursor_x - canvas_x * zoom'
    pub fn zoom_at(&mut self, cursor_x: f64, cursor_y: f64, factor: f64) {
        let old_zoom = self.zoom;
        let new_zoom = (old_zoom * factor).clamp(self.min_zoom, self.max_zoom);

        if (new_zoom - old_zoom).abs() < f64::EPSILON {
            return;
        }

        // World coordinates under cursor prior to zoom change
        let world_x = (cursor_x - self.offset_x) / old_zoom;
        let world_y = (cursor_y - self.offset_y) / old_zoom;

        self.zoom = new_zoom;
        self.offset_x = cursor_x - (world_x * new_zoom);
        self.offset_y = cursor_y - (world_y * new_zoom);
    }

    /// Converts screen coordinates to infinite canvas world coordinates.
    #[must_use]
    pub fn screen_to_world(&self, screen_x: f64, screen_y: f64) -> (f64, f64) {
        let world_x = (screen_x - self.offset_x) / self.zoom;
        let world_y = (screen_y - self.offset_y) / self.zoom;
        (world_x, world_y)
    }

    /// Converts canvas world coordinates to screen pixel coordinates.
    #[must_use]
    pub fn world_to_screen(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        let screen_x = (world_x * self.zoom) + self.offset_x;
        let screen_y = (world_y * self.zoom) + self.offset_y;
        (screen_x, screen_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_state_tracks_motion_deltas_without_allocation() {
        let mut pointer = PointerState::new();
        assert_eq!(pointer.cursor_pos, None);

        let delta1 = pointer.update_cursor(100.0, 150.0);
        assert_eq!(delta1, None);
        assert_eq!(pointer.cursor_pos, Some((100.0, 150.0)));

        let delta2 = pointer.update_cursor(125.0, 160.0);
        assert_eq!(delta2, Some((25.0, 10.0)));
        assert_eq!(pointer.cursor_pos, Some((125.0, 160.0)));
    }

    #[test]
    fn test_pointer_state_panning_detection() {
        let mut pointer = PointerState::new();
        assert!(!pointer.is_panning());

        // Middle button triggers panning
        pointer.middle_pressed = true;
        assert!(pointer.is_panning());
        pointer.middle_pressed = false;

        // Space + Left click triggers panning
        pointer.space_held = true;
        assert!(!pointer.is_panning());
        pointer.left_pressed = true;
        assert!(pointer.is_panning());
    }

    #[test]
    fn test_viewport_camera_pan_and_screen_mapping() {
        let mut camera = ViewportCamera::new();
        camera.pan(50.0, -30.0);
        assert_eq!(camera.offset_x, 50.0);
        assert_eq!(camera.offset_y, -30.0);

        let (wx, wy) = camera.screen_to_world(50.0, -30.0);
        assert_eq!(wx, 0.0);
        assert_eq!(wy, 0.0);

        let (sx, sy) = camera.world_to_screen(0.0, 0.0);
        assert_eq!(sx, 50.0);
        assert_eq!(sy, -30.0);
    }

    #[test]
    fn test_viewport_camera_anchored_zoom() {
        let mut camera = ViewportCamera::new();
        let cursor_x = 400.0;
        let cursor_y = 300.0;

        // Canvas point under cursor at 1.0x
        let (w_before_x, w_before_y) = camera.screen_to_world(cursor_x, cursor_y);

        // Zoom in by 2.0x
        camera.zoom_at(cursor_x, cursor_y, 2.0);
        assert_eq!(camera.zoom, 2.0);

        // Canvas point under cursor after zoom should be identical
        let (w_after_x, w_after_y) = camera.screen_to_world(cursor_x, cursor_y);
        assert!((w_before_x - w_after_x).abs() < 1e-6);
        assert!((w_before_y - w_after_y).abs() < 1e-6);
    }
}
