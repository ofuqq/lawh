//! Vector icon paths and drawing primitives for the canvas HUD toolbar.

use ruqa_core::tool::Tool;
use tiny_skia::{Color, Paint, Path, PathBuilder, PixmapMut, Stroke, Transform};

/// Builds a closed rounded rectangle path.
#[must_use]
pub fn rounded_rect_path(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<Path> {
    let r = r.min(w * 0.5).min(h * 0.5);
    let mut pb = PathBuilder::new();
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.quad_to(x + w, y, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.quad_to(x + w, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.quad_to(x, y + h, x, y + h - r);
    pb.line_to(x, y + r);
    pb.quad_to(x, y, x + r, y);
    pb.close();
    pb.finish()
}

fn path_select(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx - 5.0, cy - 6.5);
    pb.line_to(cx - 5.0, cy + 6.5);
    pb.line_to(cx - 1.5, cy + 2.5);
    pb.line_to(cx + 2.0, cy + 6.5);
    pb.line_to(cx + 3.8, cy + 5.0);
    pb.line_to(cx + 0.5, cy + 1.2);
    pb.line_to(cx + 4.5, cy + 1.2);
    pb.close();
    pb.finish()
}

fn path_pen(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx - 6.0, cy + 6.0);
    pb.line_to(cx - 6.0, cy + 2.5);
    pb.line_to(cx + 2.5, cy - 6.0);
    pb.line_to(cx + 6.0, cy - 2.5);
    pb.line_to(cx - 2.5, cy + 6.0);
    pb.close();
    pb.finish()
}

fn path_rectangle(cx: f32, cy: f32) -> Option<Path> {
    rounded_rect_path(cx - 7.0, cy - 5.5, 14.0, 11.0, 1.5)
}

fn path_ellipse(cx: f32, cy: f32) -> Option<Path> {
    let skia_rect = tiny_skia::Rect::from_xywh(cx - 7.0, cy - 5.5, 14.0, 11.0)?;
    PathBuilder::from_oval(skia_rect)
}

fn path_diamond(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx, cy - 6.5);
    pb.line_to(cx + 6.5, cy);
    pb.line_to(cx, cy + 6.5);
    pb.line_to(cx - 6.5, cy);
    pb.close();
    pb.finish()
}

fn path_line(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx - 6.0, cy + 5.0);
    pb.line_to(cx + 6.0, cy - 5.0);
    pb.finish()
}

fn path_arrow(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx - 6.0, cy + 5.0);
    pb.line_to(cx + 5.5, cy - 4.5);
    pb.move_to(cx + 1.0, cy - 4.5);
    pb.line_to(cx + 5.5, cy - 4.5);
    pb.line_to(cx + 5.5, cy + 0.0);
    pb.finish()
}

fn path_eraser(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx - 6.0, cy + 1.0);
    pb.line_to(cx - 1.0, cy + 6.0);
    pb.line_to(cx + 6.0, cy - 1.0);
    pb.line_to(cx + 1.0, cy - 6.0);
    pb.close();
    pb.move_to(cx - 2.5, cy - 2.5);
    pb.line_to(cx + 2.5, cy + 2.5);
    pb.finish()
}

fn path_text(cx: f32, cy: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx - 5.5, cy - 6.0);
    pb.line_to(cx + 5.5, cy - 6.0);
    pb.move_to(cx, cy - 6.0);
    pb.line_to(cx, cy + 6.0);
    pb.finish()
}

/// Draws the vector icon for a specific [`Tool`] centered at `(cx, cy)`.
pub fn draw_icon(
    pixmap: &mut PixmapMut,
    tool: Tool,
    cx: f32,
    cy: f32,
    color: Color,
    transform: Transform,
) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let stroke = Stroke {
        width: 1.5,
        line_cap: tiny_skia::LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..Default::default()
    };

    match tool {
        Tool::Select => {
            if let Some(path) = path_select(cx, cy) {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, transform, None);
            }
        }
        Tool::Pen => {
            if let Some(path) = path_pen(cx, cy) {
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
        }
        Tool::Rectangle => {
            if let Some(path) = path_rectangle(cx, cy) {
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
        }
        Tool::Ellipse => {
            if let Some(path) = path_ellipse(cx, cy) {
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
        }
        Tool::Diamond => {
            if let Some(path) = path_diamond(cx, cy) {
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
        }
        Tool::Line => {
            if let Some(path) = path_line(cx, cy) {
                let mut line_stroke = stroke;
                line_stroke.width = 2.0;
                pixmap.stroke_path(&path, &paint, &line_stroke, transform, None);
            }
        }
        Tool::Arrow => {
            if let Some(path) = path_arrow(cx, cy) {
                let mut arrow_stroke = stroke;
                arrow_stroke.width = 1.8;
                pixmap.stroke_path(&path, &paint, &arrow_stroke, transform, None);
            }
        }
        Tool::Eraser => {
            if let Some(path) = path_eraser(cx, cy) {
                pixmap.stroke_path(&path, &paint, &stroke, transform, None);
            }
        }
        Tool::Text => {
            if let Some(path) = path_text(cx, cy) {
                let mut text_stroke = stroke;
                text_stroke.width = 1.8;
                pixmap.stroke_path(&path, &paint, &text_stroke, transform, None);
            }
        }
    }
}
