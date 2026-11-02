//! Visual rasterization of the Recent Files / Welcome HUD modal overlay.

pub use crate::recent_files_layout::{RecentFilesAction, RecentFilesLayout};
use ruqa_core::geometry::Rect;
use ruqa_core::recent_files::RecentFileEntry;
use std::path::Path;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, PixmapMut, Stroke, Transform};

/// Theme styling parameters for the Recent Files HUD overlay.
#[derive(Debug, Clone, PartialEq)]
pub struct RecentFilesTheme {
    /// Fullscreen backdrop dimming color.
    pub backdrop_color: Color,
    /// Modal dialog background color.
    pub dialog_background: Color,
    /// Modal dialog border stroke color.
    pub dialog_border: Color,
    /// Card background color for an idle recent file item.
    pub item_background: Color,
    /// Card background color when an item is hovered.
    pub item_hovered: Color,
    /// Accent color for the "New Board" action button.
    pub accent_color: Color,
}

impl Default for RecentFilesTheme {
    fn default() -> Self {
        Self {
            backdrop_color: Color::from_rgba8(0, 0, 0, 160),
            dialog_background: Color::from_rgba8(28, 28, 30, 250),
            dialog_border: Color::from_rgba8(55, 55, 60, 255),
            item_background: Color::from_rgba8(40, 40, 44, 255),
            item_hovered: Color::from_rgba8(58, 58, 65, 255),
            accent_color: Color::from_rgba8(56, 189, 248, 255), // Cyan #38BDF8
        }
    }
}

/// Helper to build a rounded rectangle [`tiny_skia::Path`].
fn rounded_rect_path(rect: Rect, radius: f32) -> Option<tiny_skia::Path> {
    let r = radius.min(rect.width() * 0.5).min(rect.height() * 0.5);
    let mut pb = PathBuilder::new();
    pb.move_to(rect.min.x + r, rect.min.y);
    pb.line_to(rect.max.x - r, rect.min.y);
    pb.quad_to(rect.max.x, rect.min.y, rect.max.x, rect.min.y + r);
    pb.line_to(rect.max.x, rect.max.y - r);
    pb.quad_to(rect.max.x, rect.max.y, rect.max.x - r, rect.max.y);
    pb.line_to(rect.min.x + r, rect.max.y);
    pb.quad_to(rect.min.x, rect.max.y, rect.min.x, rect.max.y - r);
    pb.line_to(rect.min.x, rect.min.y + r);
    pb.quad_to(rect.min.x, rect.min.y, rect.min.x + r, rect.min.y);
    pb.close();
    pb.finish()
}

/// Renders the complete Recent Files / Welcome HUD modal overlay into `pixmap`.
pub fn render_recent_files_hud(
    pixmap: &mut PixmapMut<'_>,
    layout: &RecentFilesLayout,
    _entries: &[RecentFileEntry],
    hovered_path: Option<&Path>,
    theme: &RecentFilesTheme,
) {
    let transform = Transform::identity();

    // 1. Semi-transparent backdrop dimming
    let mut backdrop_paint = Paint::default();
    backdrop_paint.set_color(theme.backdrop_color);
    if let Some(screen_rect) =
        tiny_skia::Rect::from_xywh(0.0, 0.0, pixmap.width() as f32, pixmap.height() as f32)
    {
        pixmap.fill_rect(screen_rect, &backdrop_paint, transform, None);
    }

    // 2. Dialog container
    if let Some(dialog_path) = rounded_rect_path(layout.dialog_bounds, 12.0) {
        let mut bg_paint = Paint::default();
        bg_paint.set_color(theme.dialog_background);
        bg_paint.anti_alias = true;
        pixmap.fill_path(&dialog_path, &bg_paint, FillRule::Winding, transform, None);

        let mut border_paint = Paint::default();
        border_paint.set_color(theme.dialog_border);
        border_paint.anti_alias = true;
        let stroke = Stroke {
            width: 1.0,
            ..Default::default()
        };
        pixmap.stroke_path(&dialog_path, &border_paint, &stroke, transform, None);
    }

    // 3. Close button (X icon)
    let cb = layout.close_button_bounds;
    let mut close_paint = Paint::default();
    close_paint.set_color(Color::from_rgba8(160, 160, 170, 255));
    close_paint.anti_alias = true;
    let stroke = Stroke {
        width: 1.5,
        ..Default::default()
    };
    let mut x_pb = PathBuilder::new();
    let pad = 8.0;
    x_pb.move_to(cb.min.x + pad, cb.min.y + pad);
    x_pb.line_to(cb.max.x - pad, cb.max.y - pad);
    x_pb.move_to(cb.max.x - pad, cb.min.y + pad);
    x_pb.line_to(cb.min.x + pad, cb.max.y - pad);
    if let Some(x_path) = x_pb.finish() {
        pixmap.stroke_path(&x_path, &close_paint, &stroke, transform, None);
    }

    // 4. Recent File item rows
    for (path, bounds) in &layout.item_bounds {
        let is_hovered = hovered_path.is_some_and(|hp| hp == path);
        let color = if is_hovered {
            theme.item_hovered
        } else {
            theme.item_background
        };

        if let Some(item_path) = rounded_rect_path(*bounds, 6.0) {
            let mut paint = Paint::default();
            paint.set_color(color);
            paint.anti_alias = true;
            pixmap.fill_path(&item_path, &paint, FillRule::Winding, transform, None);

            // Subtle border
            let mut border = Paint::default();
            border.set_color(if is_hovered {
                theme.accent_color
            } else {
                Color::from_rgba8(50, 50, 55, 255)
            });
            border.anti_alias = true;
            let stroke = Stroke {
                width: 1.0,
                ..Default::default()
            };
            pixmap.stroke_path(&item_path, &border, &stroke, transform, None);

            // Document icon on the left
            let icon_x = bounds.min.x + 14.0;
            let icon_y = bounds.min.y + 12.0;
            let mut doc_pb = PathBuilder::new();
            doc_pb.move_to(icon_x, icon_y);
            doc_pb.line_to(icon_x + 12.0, icon_y);
            doc_pb.line_to(icon_x + 16.0, icon_y + 4.0);
            doc_pb.line_to(icon_x + 16.0, icon_y + 20.0);
            doc_pb.line_to(icon_x, icon_y + 20.0);
            doc_pb.close();
            if let Some(doc_path) = doc_pb.finish() {
                let mut doc_paint = Paint::default();
                doc_paint.set_color(Color::from_rgba8(180, 180, 190, 255));
                doc_paint.anti_alias = true;
                pixmap.stroke_path(&doc_path, &doc_paint, &stroke, transform, None);
            }
        }
    }

    // 5. "New Board" button
    let nb = layout.new_board_bounds;
    if let Some(nb_path) = rounded_rect_path(nb, 6.0) {
        let mut nb_paint = Paint::default();
        nb_paint.set_color(Color::from_rgba8(45, 45, 52, 255));
        nb_paint.anti_alias = true;
        pixmap.fill_path(&nb_path, &nb_paint, FillRule::Winding, transform, None);

        let mut nb_border = Paint::default();
        nb_border.set_color(theme.accent_color);
        nb_border.anti_alias = true;
        let stroke = Stroke {
            width: 1.0,
            ..Default::default()
        };
        pixmap.stroke_path(&nb_path, &nb_border, &stroke, transform, None);

        // Plus icon
        let cx = nb.min.x + 18.0;
        let cy = nb.center().y;
        let mut plus_pb = PathBuilder::new();
        plus_pb.move_to(cx - 5.0, cy);
        plus_pb.line_to(cx + 5.0, cy);
        plus_pb.move_to(cx, cy - 5.0);
        plus_pb.line_to(cx, cy + 5.0);
        if let Some(plus_path) = plus_pb.finish() {
            let mut plus_paint = Paint::default();
            plus_paint.set_color(theme.accent_color);
            plus_paint.anti_alias = true;
            pixmap.stroke_path(&plus_path, &plus_paint, &stroke, transform, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::recent_files::RecentFileEntry;
    use std::path::PathBuf;
    use tiny_skia::Pixmap;

    #[test]
    fn test_render_recent_files_hud_produces_pixels() {
        let mut pixmap = Pixmap::new(800, 600).expect("pixmap allocation succeeds");
        pixmap.fill(Color::BLACK);

        let entries = vec![
            RecentFileEntry::new(PathBuf::from("/vault/Algorithms.ruqa")),
            RecentFileEntry::new(PathBuf::from("/vault/OS.ruqa")),
        ];

        let layout = RecentFilesLayout::new(800.0, 600.0, &entries);
        let theme = RecentFilesTheme::default();

        render_recent_files_hud(&mut pixmap.as_mut(), &layout, &entries, None, &theme);

        let non_black = pixmap
            .pixels()
            .iter()
            .filter(|p| p.red() > 0 || p.green() > 0 || p.blue() > 0)
            .count();

        assert!(
            non_black > 1000,
            "HUD overlay must rasterize visible pixels"
        );
    }
}
