//! 2D selection and resize handle definitions and bounding box deformation.
//!
//! Provides the 8 canonical resize handles (4 corners, 4 edge midpoints) and
//! calculates proportional and unconstrained bounding box resizing with a minimum dimension clamp.

use super::point::Point;
use super::rect::Rect;

/// Minimum dimension in world units for any resized shape bounding box (5x5 px minimum).
pub const MIN_RESIZE_DIMENSION: f32 = 5.0;

/// The 8 resize handles positioned along a selection perimeter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HandleKind {
    /// Top-left corner handle (North-West).
    TopLeft,
    /// Top-center edge midpoint handle (North).
    TopCenter,
    /// Top-right corner handle (North-East).
    TopRight,
    /// Right-center edge midpoint handle (East).
    RightCenter,
    /// Bottom-right corner handle (South-East).
    BottomRight,
    /// Bottom-center edge midpoint handle (South).
    BottomCenter,
    /// Bottom-left corner handle (South-West).
    BottomLeft,
    /// Left-center edge midpoint handle (West).
    LeftCenter,
}

impl HandleKind {
    /// All 8 handles in clockwise order starting from Top-Left.
    pub const ALL: [Self; 8] = [
        Self::TopLeft,
        Self::TopCenter,
        Self::TopRight,
        Self::RightCenter,
        Self::BottomRight,
        Self::BottomCenter,
        Self::BottomLeft,
        Self::LeftCenter,
    ];

    /// Returns `true` if this handle is one of the four corner handles.
    #[must_use]
    pub const fn is_corner(&self) -> bool {
        matches!(
            self,
            Self::TopLeft | Self::TopRight | Self::BottomRight | Self::BottomLeft
        )
    }

    /// Returns `true` if this handle is one of the four edge midpoint handles.
    #[must_use]
    pub const fn is_edge(&self) -> bool {
        !self.is_corner()
    }

    /// Computes the anchor position of this handle on the given rectangle perimeter.
    #[must_use]
    pub fn point_on_rect(&self, rect: Rect) -> Point {
        let center = rect.center();
        match self {
            Self::TopLeft => rect.min,
            Self::TopCenter => Point::new(center.x, rect.min.y),
            Self::TopRight => Point::new(rect.max.x, rect.min.y),
            Self::RightCenter => Point::new(rect.max.x, center.y),
            Self::BottomRight => rect.max,
            Self::BottomCenter => Point::new(center.x, rect.max.y),
            Self::BottomLeft => Point::new(rect.min.x, rect.max.y),
            Self::LeftCenter => Point::new(rect.min.x, center.y),
        }
    }
}

/// Computes the new bounding box when dragging `handle` to `current_world_pt`.
///
/// Corner handles scale from the opposite corner anchor point.
/// Edge handles scale only their respective axis.
/// When `shift_held` is true on corner handles, scaling is constrained to preserve aspect ratio.
/// All output rectangles enforce a minimum dimension of [`MIN_RESIZE_DIMENSION`] (5x5 units).
#[must_use]
pub fn compute_resized_bounds(
    initial_bounds: Rect,
    handle: HandleKind,
    current_world_pt: Point,
    shift_held: bool,
) -> Rect {
    let b = initial_bounds;
    let pt = current_world_pt;
    let min_dim = MIN_RESIZE_DIMENSION;

    match handle {
        HandleKind::BottomRight => {
            let anchor = b.min;
            let mut w = (pt.x - anchor.x).max(min_dim);
            let mut h = (pt.y - anchor.y).max(min_dim);
            if shift_held && b.width() > 1e-4 && b.height() > 1e-4 {
                let scale = (w / b.width()).max(h / b.height());
                w = b.width() * scale;
                h = b.height() * scale;
            }
            Rect::from_xywh(anchor.x, anchor.y, w, h)
        }
        HandleKind::TopLeft => {
            let anchor = b.max;
            let mut w = (anchor.x - pt.x).max(min_dim);
            let mut h = (anchor.y - pt.y).max(min_dim);
            if shift_held && b.width() > 1e-4 && b.height() > 1e-4 {
                let scale = (w / b.width()).max(h / b.height());
                w = b.width() * scale;
                h = b.height() * scale;
            }
            Rect::from_xywh(anchor.x - w, anchor.y - h, w, h)
        }
        HandleKind::TopRight => {
            let anchor = Point::new(b.min.x, b.max.y);
            let mut w = (pt.x - anchor.x).max(min_dim);
            let mut h = (anchor.y - pt.y).max(min_dim);
            if shift_held && b.width() > 1e-4 && b.height() > 1e-4 {
                let scale = (w / b.width()).max(h / b.height());
                w = b.width() * scale;
                h = b.height() * scale;
            }
            Rect::from_xywh(anchor.x, anchor.y - h, w, h)
        }
        HandleKind::BottomLeft => {
            let anchor = Point::new(b.max.x, b.min.y);
            let mut w = (anchor.x - pt.x).max(min_dim);
            let mut h = (pt.y - anchor.y).max(min_dim);
            if shift_held && b.width() > 1e-4 && b.height() > 1e-4 {
                let scale = (w / b.width()).max(h / b.height());
                w = b.width() * scale;
                h = b.height() * scale;
            }
            Rect::from_xywh(anchor.x - w, anchor.y, w, h)
        }
        HandleKind::TopCenter => {
            let h = (b.max.y - pt.y).max(min_dim);
            Rect::from_xywh(b.min.x, b.max.y - h, b.width(), h)
        }
        HandleKind::BottomCenter => {
            let h = (pt.y - b.min.y).max(min_dim);
            Rect::from_xywh(b.min.x, b.min.y, b.width(), h)
        }
        HandleKind::LeftCenter => {
            let w = (b.max.x - pt.x).max(min_dim);
            Rect::from_xywh(b.max.x - w, b.min.y, w, b.height())
        }
        HandleKind::RightCenter => {
            let w = (pt.x - b.min.x).max(min_dim);
            Rect::from_xywh(b.min.x, b.min.y, w, b.height())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_right_corner_resizing() {
        let b = Rect::from_xywh(10.0, 10.0, 50.0, 50.0);
        // Drag to (100, 80)
        let res =
            compute_resized_bounds(b, HandleKind::BottomRight, Point::new(100.0, 80.0), false);
        assert_eq!(res.min, Point::new(10.0, 10.0));
        assert_eq!(res.width(), 90.0);
        assert_eq!(res.height(), 70.0);

        // Constrained shift: 90 / 50 = 1.8 > 70 / 50 = 1.4 -> w=90, h=90
        let res_shift =
            compute_resized_bounds(b, HandleKind::BottomRight, Point::new(100.0, 80.0), true);
        assert_eq!(res_shift.min, Point::new(10.0, 10.0));
        assert_eq!(res_shift.width(), 90.0);
        assert_eq!(res_shift.height(), 90.0);
    }

    #[test]
    fn test_other_corner_resizing() {
        let b = Rect::from_xywh(20.0, 20.0, 60.0, 40.0);

        // TopLeft anchored at max (80, 60), drag to (0, 0)
        let res_tl = compute_resized_bounds(b, HandleKind::TopLeft, Point::new(0.0, 0.0), false);
        assert_eq!(res_tl.max, Point::new(80.0, 60.0));
        assert_eq!(res_tl.width(), 80.0);
        assert_eq!(res_tl.height(), 60.0);

        // TopRight anchored at bottom-left (20, 60), drag to (100, 10)
        let res_tr =
            compute_resized_bounds(b, HandleKind::TopRight, Point::new(100.0, 10.0), false);
        assert_eq!(res_tr.min.x, 20.0);
        assert_eq!(res_tr.max.y, 60.0);
        assert_eq!(res_tr.width(), 80.0);
        assert_eq!(res_tr.height(), 50.0);

        // BottomLeft anchored at top-right (80, 20), drag to (10, 80)
        let res_bl =
            compute_resized_bounds(b, HandleKind::BottomLeft, Point::new(10.0, 80.0), false);
        assert_eq!(res_bl.max.x, 80.0);
        assert_eq!(res_bl.min.y, 20.0);
        assert_eq!(res_bl.width(), 70.0);
        assert_eq!(res_bl.height(), 60.0);
    }

    #[test]
    fn test_minimum_dimension_clamping() {
        let b = Rect::from_xywh(50.0, 50.0, 50.0, 50.0);
        // Drag past anchor to negative dimensions
        let res = compute_resized_bounds(b, HandleKind::BottomRight, Point::new(10.0, 10.0), false);
        assert_eq!(res.width(), MIN_RESIZE_DIMENSION);
        assert_eq!(res.height(), MIN_RESIZE_DIMENSION);
    }

    #[test]
    fn test_edge_midpoint_only_scales_single_axis() {
        let b = Rect::from_xywh(10.0, 10.0, 50.0, 40.0);
        let res_top =
            compute_resized_bounds(b, HandleKind::TopCenter, Point::new(200.0, -10.0), false);
        assert_eq!(res_top.width(), 50.0);
        assert_eq!(res_top.height(), 60.0);

        let res_right =
            compute_resized_bounds(b, HandleKind::RightCenter, Point::new(100.0, 300.0), false);
        assert_eq!(res_right.width(), 90.0);
        assert_eq!(res_right.height(), 40.0);
    }
}
