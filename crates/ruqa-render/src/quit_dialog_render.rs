//! Visual rasterization of the unsaved changes quit confirmation dialog overlay.

pub use crate::quit_dialog_layout::{QuitDialogAction, QuitDialogLayout, QuitDialogTheme};
use ruqa_core::geometry::Rect;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, PixmapMut, Stroke, Transform};

/// Helper to construct a rounded rectangle [`tiny_skia::Path`].
fn rounded_rect(rect: Rect, radius: f32) -> Option<tiny_skia::Path> {
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

/// Renders the complete quit confirmation dialog overlay into `pixmap`.
pub fn render_quit_dialog(
    pixmap: &mut PixmapMut<'_>,
    layout: &QuitDialogLayout,
    hovered_action: Option<QuitDialogAction>,
    theme: &QuitDialogTheme,
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

    // 2. Dialog box container
    if let Some(dialog_path) = rounded_rect(layout.dialog_bounds, 12.0) {
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

    // 3. Warning Alert Icon (Amber Triangle with exclamation point)
    let icon_left = layout.dialog_bounds.min.x + 24.0;
    let icon_top = layout.dialog_bounds.min.y + 24.0;
    let mut tri_pb = PathBuilder::new();
    tri_pb.move_to(icon_left + 16.0, icon_top);
    tri_pb.line_to(icon_left + 32.0, icon_top + 28.0);
    tri_pb.line_to(icon_left, icon_top + 28.0);
    tri_pb.close();

    if let Some(tri_path) = tri_pb.finish() {
        let mut warn_paint = Paint::default();
        warn_paint.set_color(Color::from_rgba8(245, 158, 11, 255)); // Amber #F59E0B
        warn_paint.anti_alias = true;
        pixmap.fill_path(&tri_path, &warn_paint, FillRule::Winding, transform, None);

        // Exclamation point in dark ink
        let mut excl_paint = Paint::default();
        excl_paint.set_color(Color::from_rgba8(20, 20, 22, 255));
        let stroke = Stroke {
            width: 2.0,
            ..Default::default()
        };
        let mut excl_pb = PathBuilder::new();
        excl_pb.move_to(icon_left + 16.0, icon_top + 10.0);
        excl_pb.line_to(icon_left + 16.0, icon_top + 18.0);
        excl_pb.move_to(icon_left + 16.0, icon_top + 22.0);
        excl_pb.line_to(icon_left + 16.0, icon_top + 24.0);
        if let Some(excl_path) = excl_pb.finish() {
            pixmap.stroke_path(&excl_path, &excl_paint, &stroke, transform, None);
        }
    }

    // 4. Title & message vector bars (visual placeholders representing text lines)
    let text_x = icon_left + 44.0;
    let title_y = icon_top + 4.0;
    let mut title_paint = Paint::default();
    title_paint.set_color(Color::from_rgba8(240, 240, 245, 255));
    if let Some(r) = tiny_skia::Rect::from_xywh(text_x, title_y, 180.0, 10.0) {
        pixmap.fill_rect(r, &title_paint, transform, None);
    }

    let msg_y = title_y + 20.0;
    let mut msg_paint = Paint::default();
    msg_paint.set_color(Color::from_rgba8(160, 160, 170, 255));
    if let Some(r) = tiny_skia::Rect::from_xywh(text_x, msg_y, 240.0, 8.0) {
        pixmap.fill_rect(r, &msg_paint, transform, None);
    }
    if let Some(r) = tiny_skia::Rect::from_xywh(text_x, msg_y + 14.0, 160.0, 8.0) {
        pixmap.fill_rect(r, &msg_paint, transform, None);
    }

    // 5. Buttons: Cancel, Don't Save, Save
    render_button(
        pixmap,
        layout.cancel_button_bounds,
        if hovered_action == Some(QuitDialogAction::Cancel) {
            theme.cancel_hovered
        } else {
            theme.cancel_background
        },
        Color::from_rgba8(80, 80, 88, 255),
        Color::from_rgba8(200, 200, 210, 255),
    );

    render_button(
        pixmap,
        layout.dont_save_button_bounds,
        if hovered_action == Some(QuitDialogAction::DontSave) {
            theme.dont_save_hovered
        } else {
            theme.dont_save_background
        },
        Color::from_rgba8(220, 38, 38, 255),
        Color::from_rgba8(255, 255, 255, 255),
    );

    render_button(
        pixmap,
        layout.save_button_bounds,
        if hovered_action == Some(QuitDialogAction::Save) {
            theme.save_hovered
        } else {
            theme.save_background
        },
        Color::from_rgba8(14, 165, 233, 255),
        Color::from_rgba8(10, 15, 25, 255),
    );
}

/// Helper to render a stylized rounded action button.
fn render_button(
    pixmap: &mut PixmapMut<'_>,
    bounds: Rect,
    bg_color: Color,
    border_color: Color,
    label_color: Color,
) {
    let transform = Transform::identity();
    if let Some(btn_path) = rounded_rect(bounds, 6.0) {
        let mut bg_paint = Paint::default();
        bg_paint.set_color(bg_color);
        bg_paint.anti_alias = true;
        pixmap.fill_path(&btn_path, &bg_paint, FillRule::Winding, transform, None);

        let mut border_paint = Paint::default();
        border_paint.set_color(border_color);
        border_paint.anti_alias = true;
        let stroke = Stroke {
            width: 1.0,
            ..Default::default()
        };
        pixmap.stroke_path(&btn_path, &border_paint, &stroke, transform, None);

        // Center label indicator bar
        let label_w = bounds.width() * 0.5;
        let label_h = 6.0;
        let label_x = bounds.min.x + (bounds.width() - label_w) * 0.5;
        let label_y = bounds.min.y + (bounds.height() - label_h) * 0.5;
        if let Some(lr) = tiny_skia::Rect::from_xywh(label_x, label_y, label_w, label_h) {
            let mut lp = Paint::default();
            lp.set_color(label_color);
            pixmap.fill_rect(lr, &lp, transform, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_skia::Pixmap;

    #[test]
    fn test_render_quit_dialog_produces_non_zero_pixels() {
        let mut pixmap = Pixmap::new(800, 600).unwrap();
        pixmap.fill(Color::BLACK);

        let layout = QuitDialogLayout::new(800.0, 600.0);
        let theme = QuitDialogTheme::default();

        render_quit_dialog(
            &mut pixmap.as_mut(),
            &layout,
            Some(QuitDialogAction::Save),
            &theme,
        );

        let non_zero_pixels = pixmap.data().iter().filter(|&&b| b > 0).count();
        assert!(non_zero_pixels > 1000);
    }
}
