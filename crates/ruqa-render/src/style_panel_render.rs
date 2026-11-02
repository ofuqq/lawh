//! Contextual styling HUD panel rasterization for stroke colors, widths, and roughness.

use crate::shape_render::parse_hex_color;
pub use crate::style_panel_layout::{StyleAction, StylePanelLayout};
use crate::toolbar_icons::rounded_rect_path;
use ruqa_core::style::{RoughnessPreset, StrokeWidthPreset, PALETTE_COLORS};
use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

/// Visual styling options for the contextual styling HUD panel.
#[derive(Debug, Clone, PartialEq)]
pub struct StylePanelTheme {
    /// Translucent background color of the panel.
    pub background: Color,
    /// Border stroke color of the panel container.
    pub border_color: Color,
    /// Active button highlight badge color.
    pub active_badge_color: Color,
    /// Button background color when inactive.
    pub button_bg: Color,
    /// Button border color.
    pub button_border: Color,
    /// Line / icon color for controls.
    pub foreground: Color,
}

impl Default for StylePanelTheme {
    fn default() -> Self {
        Self {
            background: Color::from_rgba8(30, 30, 30, 242), // #1E1E1E ~95%
            border_color: Color::from_rgba8(51, 51, 51, 255), // #333333
            active_badge_color: Color::from_rgba8(37, 99, 235, 255), // #2563EB Blue
            button_bg: Color::from_rgba8(42, 42, 42, 255),  // #2A2A2A
            button_border: Color::from_rgba8(60, 60, 60, 255), // #3C3C3C
            foreground: Color::from_rgba8(236, 236, 236, 255), // #ECECEC
        }
    }
}

/// Renders the floating contextual styling HUD panel.
pub fn render_style_panel(
    pixmap: &mut PixmapMut,
    layout: &StylePanelLayout,
    current_color: &str,
    current_width: f32,
    current_roughness: f32,
    scale_factor: f64,
    theme: &StylePanelTheme,
) {
    #[allow(clippy::cast_possible_truncation)]
    let transform = Transform::from_scale(scale_factor as f32, scale_factor as f32);

    // 1. Draw panel background and outer border
    if let Some(bg_path) = rounded_rect_path(
        layout.bounds.min.x,
        layout.bounds.min.y,
        layout.bounds.width(),
        layout.bounds.height(),
        StylePanelLayout::CORNER_RADIUS,
    ) {
        let mut bg_paint = Paint::default();
        bg_paint.set_color(theme.background);
        bg_paint.anti_alias = true;
        pixmap.fill_path(
            &bg_path,
            &bg_paint,
            tiny_skia::FillRule::Winding,
            transform,
            None,
        );

        let mut border_paint = Paint::default();
        border_paint.set_color(theme.border_color);
        border_paint.anti_alias = true;
        let stroke = Stroke {
            width: 1.0,
            ..Default::default()
        };
        pixmap.stroke_path(&bg_path, &border_paint, &stroke, transform, None);
    }

    // 2. Draw 6 color swatches
    for (i, &hex) in PALETTE_COLORS.iter().enumerate() {
        let swatch_rect = layout.color_swatch_rect(i);
        let is_active = current_color.eq_ignore_ascii_case(hex);

        if let Some(color) = parse_hex_color(hex) {
            if let Some(path) = rounded_rect_path(
                swatch_rect.min.x,
                swatch_rect.min.y,
                swatch_rect.width(),
                swatch_rect.height(),
                5.0,
            ) {
                let mut p = Paint::default();
                p.set_color(color);
                p.anti_alias = true;
                pixmap.fill_path(&path, &p, tiny_skia::FillRule::Winding, transform, None);

                if is_active {
                    let mut ring_paint = Paint::default();
                    ring_paint.set_color(Color::WHITE);
                    ring_paint.anti_alias = true;
                    let stroke = Stroke {
                        width: 2.0,
                        ..Default::default()
                    };
                    pixmap.stroke_path(&path, &ring_paint, &stroke, transform, None);
                }
            }
        }
    }

    // 3. Draw 3 stroke width buttons
    let active_preset = StrokeWidthPreset::from_width(current_width);
    for (i, &preset) in StrokeWidthPreset::ALL.iter().enumerate() {
        let btn_rect = layout.width_button_rect(i);
        let is_active = preset == active_preset;

        if let Some(btn_path) = rounded_rect_path(
            btn_rect.min.x,
            btn_rect.min.y,
            btn_rect.width(),
            btn_rect.height(),
            6.0,
        ) {
            let mut bg_paint = Paint::default();
            bg_paint.set_color(if is_active {
                theme.active_badge_color
            } else {
                theme.button_bg
            });
            bg_paint.anti_alias = true;
            pixmap.fill_path(
                &btn_path,
                &bg_paint,
                tiny_skia::FillRule::Winding,
                transform,
                None,
            );

            // Draw horizontal sample stroke line centered in button
            let cx = btn_rect.min.x + btn_rect.width() * 0.5;
            let cy = btn_rect.min.y + btn_rect.height() * 0.5;
            let mut pb = PathBuilder::new();
            pb.move_to(cx - 14.0, cy);
            pb.line_to(cx + 14.0, cy);
            if let Some(line_path) = pb.finish() {
                let mut stroke_paint = Paint::default();
                stroke_paint.set_color(theme.foreground);
                stroke_paint.anti_alias = true;
                let stroke = Stroke {
                    width: preset.width(),
                    line_cap: tiny_skia::LineCap::Round,
                    ..Default::default()
                };
                pixmap.stroke_path(&line_path, &stroke_paint, &stroke, transform, None);
            }
        }
    }

    // 4. Draw 2 roughness buttons (Clean vs Sketchy)
    let active_roughness = RoughnessPreset::from_roughness(current_roughness);
    for (i, &preset) in RoughnessPreset::ALL.iter().enumerate() {
        let btn_rect = layout.roughness_button_rect(i);
        let is_active = preset == active_roughness;

        if let Some(btn_path) = rounded_rect_path(
            btn_rect.min.x,
            btn_rect.min.y,
            btn_rect.width(),
            btn_rect.height(),
            6.0,
        ) {
            let mut bg_paint = Paint::default();
            bg_paint.set_color(if is_active {
                theme.active_badge_color
            } else {
                theme.button_bg
            });
            bg_paint.anti_alias = true;
            pixmap.fill_path(
                &btn_path,
                &bg_paint,
                tiny_skia::FillRule::Winding,
                transform,
                None,
            );

            // Draw line indicator: Clean = straight line, Sketchy = double or zigzag line
            let cx = btn_rect.min.x + btn_rect.width() * 0.5;
            let cy = btn_rect.min.y + btn_rect.height() * 0.5;
            let mut pb = PathBuilder::new();
            match preset {
                RoughnessPreset::Clean => {
                    pb.move_to(cx - 22.0, cy);
                    pb.line_to(cx + 22.0, cy);
                }
                RoughnessPreset::Sketchy => {
                    pb.move_to(cx - 22.0, cy - 2.0);
                    pb.line_to(cx - 8.0, cy + 2.0);
                    pb.line_to(cx + 6.0, cy - 2.0);
                    pb.line_to(cx + 22.0, cy + 1.0);
                }
            }
            if let Some(rough_path) = pb.finish() {
                let mut stroke_paint = Paint::default();
                stroke_paint.set_color(theme.foreground);
                stroke_paint.anti_alias = true;
                let stroke = Stroke {
                    width: 1.8,
                    line_cap: tiny_skia::LineCap::Round,
                    line_join: tiny_skia::LineJoin::Round,
                    ..Default::default()
                };
                pixmap.stroke_path(&rough_path, &stroke_paint, &stroke, transform, None);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::geometry::Point;

    #[test]
    fn test_style_panel_layout_hit_testing() {
        let layout = StylePanelLayout::new();

        // 1. Color swatches hit-testing
        for (i, &hex) in PALETTE_COLORS.iter().enumerate() {
            let rect = layout.color_swatch_rect(i);
            let center = Point::new(
                rect.min.x + rect.width() * 0.5,
                rect.min.y + rect.height() * 0.5,
            );
            assert_eq!(layout.action_at(center), Some(StyleAction::SetColor(hex)));
        }

        // 2. Width buttons hit-testing
        for (i, &preset) in StrokeWidthPreset::ALL.iter().enumerate() {
            let rect = layout.width_button_rect(i);
            let center = Point::new(
                rect.min.x + rect.width() * 0.5,
                rect.min.y + rect.height() * 0.5,
            );
            assert_eq!(
                layout.action_at(center),
                Some(StyleAction::SetStrokeWidth(preset))
            );
        }

        // 3. Roughness buttons hit-testing
        for (i, &preset) in RoughnessPreset::ALL.iter().enumerate() {
            let rect = layout.roughness_button_rect(i);
            let center = Point::new(
                rect.min.x + rect.width() * 0.5,
                rect.min.y + rect.height() * 0.5,
            );
            assert_eq!(
                layout.action_at(center),
                Some(StyleAction::SetRoughness(preset))
            );
        }

        // Outside panel
        assert_eq!(layout.action_at(Point::new(0.0, 0.0)), None);
        assert_eq!(layout.action_at(Point::new(500.0, 500.0)), None);
    }

    #[test]
    fn test_style_panel_rasterization_produces_pixels() {
        let layout = StylePanelLayout::new();
        let mut pixmap = tiny_skia::Pixmap::new(800, 600).unwrap();
        pixmap.fill(Color::BLACK);

        render_style_panel(
            &mut pixmap.as_mut(),
            &layout,
            "#38BDF8",
            2.5,
            1.2,
            1.0,
            &StylePanelTheme::default(),
        );

        let center_x = (layout.bounds.min.x + layout.bounds.width() * 0.5) as u32;
        let center_y = (layout.bounds.min.y + layout.bounds.height() * 0.5) as u32;
        let pixel = pixmap.pixel(center_x, center_y).unwrap();
        assert!(pixel.red() > 0 || pixel.green() > 0 || pixel.blue() > 0);
    }
}
