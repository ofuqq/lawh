//! Selection outline, multi-selection bounds, and resize handles rasterization.
//!
//! Provides visual feedback for selected canvas elements with an accent-colored
//! bounding box and 8 perimeter resize handles that maintain a constant screen-pixel
//! dimension regardless of camera zoom.

use ruqa_core::geometry::{Point, Rect, Size, Viewport};
use tiny_skia::{Color, Paint, PixmapMut, Transform};

/// The 8 resize handles positioned along a selection perimeter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// A selection handle placed at a specific screen position with bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionHandle {
    /// Kind and position type of this handle.
    pub kind: HandleKind,
    /// Center anchor point in screen coordinates.
    pub center: Point,
    /// Axis-aligned bounding box of the handle square in screen coordinates.
    pub bounds: Rect,
}

impl SelectionHandle {
    /// Creates a handle centered at `center` with size `size x size`.
    #[must_use]
    pub fn new(kind: HandleKind, center: Point, size: f32) -> Self {
        let half = size * 0.5;
        let bounds = Rect::from_xywh(center.x - half, center.y - half, size, size);
        Self {
            kind,
            center,
            bounds,
        }
    }

    /// Returns `true` if the screen point hits this handle with optional tolerance padding.
    #[must_use]
    pub fn hit_test(&self, point: Point, tolerance: f32) -> bool {
        self.bounds.inflate(tolerance).contains(point)
    }
}

/// Visual styling options for rendering selection outlines and resize handles.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionStyle {
    /// Color of the bounding box outline.
    pub outline_color: Color,
    /// Width of the bounding box outline stroke.
    pub outline_width: f32,
    /// Padding in screen pixels between the element bounds and the selection outline.
    pub padding: f32,
    /// Dimension of the square resize handles in screen pixels (width and height).
    pub handle_size: f32,
    /// Fill color of the resize handle squares.
    pub handle_fill: Color,
    /// Stroke color of the resize handle borders.
    pub handle_stroke: Color,
    /// Stroke width of the resize handle borders.
    pub handle_stroke_width: f32,
}

impl Default for SelectionStyle {
    fn default() -> Self {
        Self {
            outline_color: Color::from_rgba8(25, 118, 210, 255), // Accent blue
            outline_width: 1.5,
            padding: 4.0,
            handle_size: 8.0,
            handle_fill: Color::from_rgba8(255, 255, 255, 255), // White
            handle_stroke: Color::from_rgba8(25, 118, 210, 255), // Accent blue
            handle_stroke_width: 1.5,
        }
    }
}

/// Computes the padded screen-space rectangle enclosing `world_bounds`.
#[must_use]
pub fn selection_screen_rect(
    world_bounds: Rect,
    viewport: &Viewport,
    window_size: Size,
    padding: f32,
) -> Rect {
    let p_min = viewport.world_to_screen(world_bounds.min, window_size);
    let p_max = viewport.world_to_screen(world_bounds.max, window_size);
    Rect::from_min_max(p_min, p_max).inflate(padding)
}

/// Computes the 8 selection handles on the perimeter of `screen_rect`.
#[must_use]
pub fn compute_handles(screen_rect: Rect, handle_size: f32) -> [SelectionHandle; 8] {
    let mut handles = [SelectionHandle::new(HandleKind::TopLeft, Point::ZERO, handle_size); 8];
    for (i, &kind) in HandleKind::ALL.iter().enumerate() {
        let center = kind.point_on_rect(screen_rect);
        handles[i] = SelectionHandle::new(kind, center, handle_size);
    }
    handles
}

/// Renders the selection bounding box and 8 resize handles into the pixmap surface.
///
/// Converts the `world_bounds` to screen space using `viewport.world_to_screen`,
/// applies visual padding, draws the outline, and draws 8 constant-size square handles.
pub fn render_selection(
    world_bounds: Rect,
    viewport: &Viewport,
    window_size: Size,
    pixmap: &mut PixmapMut<'_>,
    style: &SelectionStyle,
) {
    let screen_rect = selection_screen_rect(world_bounds, viewport, window_size, style.padding);

    // 1. Draw selection bounding box outline
    if let Some(skia_rect) = tiny_skia::Rect::from_xywh(
        screen_rect.min.x,
        screen_rect.min.y,
        screen_rect.width(),
        screen_rect.height(),
    ) {
        let mut stroke_paint = Paint::default();
        stroke_paint.set_color(style.outline_color);
        stroke_paint.anti_alias = true;
        let stroke = tiny_skia::Stroke {
            width: style.outline_width,
            ..Default::default()
        };
        let path = tiny_skia::PathBuilder::from_rect(skia_rect);
        pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
    }

    // 2. Draw 8 square resize handles
    let handles = compute_handles(screen_rect, style.handle_size);
    let mut fill_paint = Paint::default();
    fill_paint.set_color(style.handle_fill);
    fill_paint.anti_alias = false;

    let mut stroke_paint = Paint::default();
    stroke_paint.set_color(style.handle_stroke);
    stroke_paint.anti_alias = false;
    let handle_stroke = tiny_skia::Stroke {
        width: style.handle_stroke_width,
        ..Default::default()
    };

    for handle in &handles {
        if let Some(skia_handle) = tiny_skia::Rect::from_xywh(
            handle.bounds.min.x,
            handle.bounds.min.y,
            handle.bounds.width(),
            handle.bounds.height(),
        ) {
            pixmap.fill_rect(skia_handle, &fill_paint, Transform::identity(), None);
            let path = tiny_skia::PathBuilder::from_rect(skia_handle);
            pixmap.stroke_path(
                &path,
                &stroke_paint,
                &handle_stroke,
                Transform::identity(),
                None,
            );
        }
    }
}

/// Returns the handle (if any) hit by the given screen point.
#[must_use]
pub fn hit_test_handle(
    screen_point: Point,
    world_bounds: Rect,
    viewport: &Viewport,
    window_size: Size,
    style: &SelectionStyle,
) -> Option<HandleKind> {
    let screen_rect = selection_screen_rect(world_bounds, viewport, window_size, style.padding);
    let handles = compute_handles(screen_rect, style.handle_size);
    handles
        .iter()
        .find(|h| h.hit_test(screen_point, 2.0))
        .map(|h| h.kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_skia::Pixmap;

    #[test]
    fn test_handle_positions_on_rect() {
        let rect = Rect::from_xywh(10.0, 20.0, 100.0, 60.0);
        let handles = compute_handles(rect, 8.0);
        assert_eq!(handles[0].kind, HandleKind::TopLeft);
        assert_eq!(handles[0].center, Point::new(10.0, 20.0));
        assert_eq!(handles[0].bounds, Rect::from_xywh(6.0, 16.0, 8.0, 8.0));
        assert_eq!(handles[2].kind, HandleKind::TopRight);
        assert_eq!(handles[4].kind, HandleKind::BottomRight);
        assert_eq!(handles[6].kind, HandleKind::BottomLeft);
    }

    #[test]
    fn test_constant_screen_size_across_zooms() {
        let world_bounds = Rect::from_xywh(0.0, 0.0, 50.0, 50.0);
        let window_size = Size::new(800.0, 600.0);
        let style = SelectionStyle::default();

        for zoom in [0.2, 0.5, 1.0, 2.0, 5.0, 10.0] {
            let viewport = Viewport::new(Point::ZERO, zoom);
            let screen_rect =
                selection_screen_rect(world_bounds, &viewport, window_size, style.padding);
            let handles = compute_handles(screen_rect, style.handle_size);
            for handle in &handles {
                assert_eq!(handle.bounds.width(), 8.0);
                assert_eq!(handle.bounds.height(), 8.0);
            }
        }
    }

    #[test]
    fn test_render_selection_draws_pixels() {
        let mut pixmap = Pixmap::new(200, 200).unwrap();
        let world_bounds = Rect::from_xywh(-20.0, -20.0, 40.0, 40.0);
        let window_size = Size::new(200.0, 200.0);
        let viewport = Viewport::new(Point::ZERO, 1.0);
        let style = SelectionStyle::default();

        render_selection(
            world_bounds,
            &viewport,
            window_size,
            &mut pixmap.as_mut(),
            &style,
        );

        let non_zero_count = pixmap.data().iter().filter(|&&b| b != 0).count();
        assert!(non_zero_count > 0);
    }
}
