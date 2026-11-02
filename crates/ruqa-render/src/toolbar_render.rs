//! Minimal HUD toolbar rasterization and layout geometry for tool selection.

use crate::toolbar_icons::{draw_icon, rounded_rect_path};
use ruqa_core::geometry::{Point, Rect};
use ruqa_core::tool::Tool;
use tiny_skia::{Color, Paint, PixmapMut, Stroke, Transform};

/// Layout and positioning geometry for the canvas HUD toolbar in logical coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolbarLayout {
    /// Bounding rectangle of the entire toolbar container in logical screen coordinates.
    pub bounds: Rect,
    /// Width and height of each tool icon button.
    pub button_size: f32,
    /// Spacing between adjacent buttons.
    pub button_spacing: f32,
    /// Horizontal padding inside the toolbar container.
    pub padding_x: f32,
    /// Vertical padding inside the toolbar container.
    pub padding_y: f32,
}

impl ToolbarLayout {
    /// Default button square dimension in logical pixels.
    pub const BUTTON_SIZE: f32 = 36.0;
    /// Default horizontal gap between tool buttons.
    pub const BUTTON_SPACING: f32 = 4.0;
    /// Default horizontal padding inside container.
    pub const PADDING_X: f32 = 6.0;
    /// Default vertical padding inside container.
    pub const PADDING_Y: f32 = 4.0;
    /// Top offset from the window top edge in logical pixels.
    pub const TOP_MARGIN: f32 = 14.0;
    /// Corner radius of the toolbar container.
    pub const CORNER_RADIUS: f32 = 8.0;
    /// Corner radius of the active tool highlight badge.
    pub const BADGE_RADIUS: f32 = 6.0;

    /// Computes the toolbar layout centered horizontally at the top of the canvas.
    #[must_use]
    pub fn new(screen_width: f32) -> Self {
        #[allow(clippy::cast_precision_loss)]
        let count = Tool::ALL.len() as f32;
        let total_w = Self::PADDING_X * 2.0
            + count * Self::BUTTON_SIZE
            + (count - 1.0) * Self::BUTTON_SPACING;
        let total_h = Self::PADDING_Y * 2.0 + Self::BUTTON_SIZE;
        let x = ((screen_width - total_w) * 0.5).max(0.0);
        let y = Self::TOP_MARGIN;

        Self {
            bounds: Rect::from_xywh(x, y, total_w, total_h),
            button_size: Self::BUTTON_SIZE,
            button_spacing: Self::BUTTON_SPACING,
            padding_x: Self::PADDING_X,
            padding_y: Self::PADDING_Y,
        }
    }

    /// Returns the logical screen rectangle for the button at `index`.
    #[must_use]
    pub fn button_rect(&self, index: usize) -> Rect {
        #[allow(clippy::cast_precision_loss)]
        let x = self.bounds.min.x
            + self.padding_x
            + index as f32 * (self.button_size + self.button_spacing);
        let y = self.bounds.min.y + self.padding_y;
        Rect::from_xywh(x, y, self.button_size, self.button_size)
    }

    /// Hit-tests a logical screen point against the toolbar buttons, returning the clicked [`Tool`] if any.
    #[must_use]
    pub fn tool_at(&self, point: Point) -> Option<Tool> {
        if !self.bounds.contains(point) {
            return None;
        }
        for (i, &tool) in Tool::ALL.iter().enumerate() {
            if self.button_rect(i).contains(point) {
                return Some(tool);
            }
        }
        None
    }

    /// Returns `true` if the point falls inside the toolbar container bounds.
    #[must_use]
    pub fn contains(&self, point: Point) -> bool {
        self.bounds.contains(point)
    }
}

/// Visual styling parameters for the floating HUD toolbar.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolbarStyle {
    /// Translucent background color of the toolbar container.
    pub background: Color,
    /// Border stroke color of the toolbar container.
    pub border_color: Color,
    /// Border stroke width in logical pixels.
    pub border_width: f32,
    /// Background color of the active tool highlight badge.
    pub active_badge_color: Color,
    /// Background color of a hovered tool button.
    pub hover_badge_color: Color,
    /// Icon color for the active tool.
    pub active_icon_color: Color,
    /// Icon color for inactive tools.
    pub inactive_icon_color: Color,
}

impl Default for ToolbarStyle {
    fn default() -> Self {
        Self {
            background: Color::from_rgba8(30, 30, 30, 242), // #1E1E1E translucent
            border_color: Color::from_rgba8(51, 51, 51, 255), // #333333
            border_width: 1.0,
            active_badge_color: Color::from_rgba8(37, 99, 235, 255), // #2563EB Blue
            hover_badge_color: Color::from_rgba8(45, 45, 45, 255),   // #2D2D2D
            active_icon_color: Color::from_rgba8(255, 255, 255, 255),
            inactive_icon_color: Color::from_rgba8(163, 163, 163, 255), // #A3A3A3
        }
    }
}

/// Renders the floating HUD toolbar with tool icon buttons and active highlight badge.
pub fn render_toolbar(
    pixmap: &mut PixmapMut,
    layout: &ToolbarLayout,
    active_tool: Tool,
    hovered_tool: Option<Tool>,
    scale_factor: f64,
    style: &ToolbarStyle,
) {
    #[allow(clippy::cast_possible_truncation)]
    let transform = Transform::from_scale(scale_factor as f32, scale_factor as f32);

    // 1. Draw toolbar background container
    if let Some(bg_path) = rounded_rect_path(
        layout.bounds.min.x,
        layout.bounds.min.y,
        layout.bounds.width(),
        layout.bounds.height(),
        ToolbarLayout::CORNER_RADIUS,
    ) {
        let mut bg_paint = Paint::default();
        bg_paint.set_color(style.background);
        bg_paint.anti_alias = true;
        pixmap.fill_path(
            &bg_path,
            &bg_paint,
            tiny_skia::FillRule::Winding,
            transform,
            None,
        );

        let mut border_paint = Paint::default();
        border_paint.set_color(style.border_color);
        border_paint.anti_alias = true;
        let stroke = Stroke {
            width: style.border_width,
            ..Default::default()
        };
        pixmap.stroke_path(&bg_path, &border_paint, &stroke, transform, None);
    }

    // 2. Draw each button
    for (i, &tool) in Tool::ALL.iter().enumerate() {
        let btn_rect = layout.button_rect(i);
        let cx = btn_rect.min.x + btn_rect.width() * 0.5;
        let cy = btn_rect.min.y + btn_rect.height() * 0.5;
        let is_active = tool == active_tool;
        let is_hovered = hovered_tool == Some(tool);

        // Draw highlight badge if active or hovered
        if is_active || is_hovered {
            let badge_color = if is_active {
                style.active_badge_color
            } else {
                style.hover_badge_color
            };
            if let Some(badge_path) = rounded_rect_path(
                btn_rect.min.x,
                btn_rect.min.y,
                btn_rect.width(),
                btn_rect.height(),
                ToolbarLayout::BADGE_RADIUS,
            ) {
                let mut badge_paint = Paint::default();
                badge_paint.set_color(badge_color);
                badge_paint.anti_alias = true;
                pixmap.fill_path(
                    &badge_path,
                    &badge_paint,
                    tiny_skia::FillRule::Winding,
                    transform,
                    None,
                );
            }
        }

        // Draw icon
        let icon_color = if is_active {
            style.active_icon_color
        } else {
            style.inactive_icon_color
        };
        draw_icon(pixmap, tool, cx, cy, icon_color, transform);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_layout_centering() {
        let layout = ToolbarLayout::new(1280.0);
        assert_eq!(layout.bounds.min.y, ToolbarLayout::TOP_MARGIN);
        let total_w = layout.bounds.width();
        let expected_x = (1280.0 - total_w) * 0.5;
        assert!((layout.bounds.min.x - expected_x).abs() < 0.01);
    }

    #[test]
    fn test_toolbar_button_hit_testing() {
        let layout = ToolbarLayout::new(1000.0);

        // Test hitting each button
        for (i, &tool) in Tool::ALL.iter().enumerate() {
            let rect = layout.button_rect(i);
            let center = Point::new(
                rect.min.x + rect.width() * 0.5,
                rect.min.y + rect.height() * 0.5,
            );
            assert_eq!(layout.tool_at(center), Some(tool));
        }

        // Outside toolbar
        assert_eq!(layout.tool_at(Point::new(10.0, 10.0)), None);
        assert_eq!(layout.tool_at(Point::new(500.0, 200.0)), None);
    }

    #[test]
    fn test_toolbar_rasterization_produces_pixels() {
        let layout = ToolbarLayout::new(800.0);
        let mut pixmap = tiny_skia::Pixmap::new(800, 600).unwrap();
        pixmap.fill(Color::BLACK);

        render_toolbar(
            &mut pixmap.as_mut(),
            &layout,
            Tool::Pen,
            Some(Tool::Rectangle),
            1.0,
            &ToolbarStyle::default(),
        );

        // Verify pixels in toolbar region are non-black
        let center_x = (layout.bounds.min.x + layout.bounds.width() * 0.5) as u32;
        let center_y = (layout.bounds.min.y + layout.bounds.height() * 0.5) as u32;
        let pixel = pixmap.pixel(center_x, center_y).unwrap();
        assert!(pixel.red() > 0 || pixel.green() > 0 || pixel.blue() > 0);
    }
}
