//! Vector rasterization of text elements using Parley layout and Skrifa outline extraction.

use crate::shape_render::parse_hex_color;
use crate::text_layout::CACHED_SHAPER;
use ruqa_core::element::ElementStyle;
use ruqa_core::geometry::Point;
use skrifa::instance::{LocationRef, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::{FontRef, GlyphId, MetadataProvider};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, PixmapMut, Transform};

/// An [`OutlinePen`] implementation that builds [`tiny_skia::Path`] geometry from TrueType outlines.
struct SkiaOutlinePen {
    builder: PathBuilder,
    offset_x: f32,
    baseline_y: f32,
}

impl SkiaOutlinePen {
    fn new(offset_x: f32, baseline_y: f32) -> Self {
        Self {
            builder: PathBuilder::new(),
            offset_x,
            baseline_y,
        }
    }

    fn set_offsets(&mut self, offset_x: f32, baseline_y: f32) {
        self.offset_x = offset_x;
        self.baseline_y = baseline_y;
    }
}

impl OutlinePen for SkiaOutlinePen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(self.offset_x + x, self.baseline_y - y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(self.offset_x + x, self.baseline_y - y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.builder.quad_to(
            self.offset_x + cx0,
            self.baseline_y - cy0,
            self.offset_x + x,
            self.baseline_y - y,
        );
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.builder.cubic_to(
            self.offset_x + cx0,
            self.baseline_y - cy0,
            self.offset_x + cx1,
            self.baseline_y - cy1,
            self.offset_x + x,
            self.baseline_y - y,
        );
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

/// Renders shaped UI text directly to a pixmap at the given coordinates with optional wrapping.
#[allow(clippy::too_many_arguments)]
pub fn render_ui_text(
    pixmap: &mut PixmapMut<'_>,
    content: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    transform: Transform,
    max_width: Option<f32>,
) {
    if content.trim().is_empty() {
        return;
    }

    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    CACHED_SHAPER.with(|shaper_cell| {
        let mut shaper = shaper_cell.borrow_mut();
        let layout = shaper.layout_with_max_width(content, font_size, max_width);

        let mut pen = SkiaOutlinePen::new(x, y);

        for line in layout.lines() {
            for item in line.items() {
                if let parley::layout::PositionedLayoutItem::GlyphRun(glyph_run) = item {
                    let run = glyph_run.run();
                    let font_data = run.font();
                    let run_font_size = run.font_size();

                    let Ok(font_ref) =
                        FontRef::from_index(font_data.data.as_ref(), font_data.index)
                    else {
                        continue;
                    };

                    let outlines = font_ref.outline_glyphs();
                    for glyph in glyph_run.positioned_glyphs() {
                        if let Some(outline_glyph) = outlines.get(GlyphId::new(glyph.id)) {
                            pen.set_offsets(x + glyph.x, y + glyph.y);
                            let settings = DrawSettings::unhinted(
                                Size::new(run_font_size),
                                LocationRef::default(),
                            );
                            let _ = outline_glyph.draw(settings, &mut pen);
                        }
                    }
                }
            }
        }

        if let Some(path) = pen.builder.finish() {
            pixmap.fill_path(&path, &paint, FillRule::Winding, transform, None);
        }
    });
}

/// Renders a text element at the specified canvas world position with optional max width wrapping.
pub fn render_text_element_wrapped(
    position: Point,
    content: &str,
    font_size: f32,
    style: &ElementStyle,
    pixmap: &mut PixmapMut<'_>,
    transform: Transform,
    max_width: Option<f32>,
) {
    let color = parse_hex_color(&style.stroke_color)
        .unwrap_or_else(|| Color::from_rgba8(236, 236, 236, 255));
    render_ui_text(
        pixmap, content, position.x, position.y, font_size, color, transform, max_width,
    );
}

/// Renders a text element at the specified canvas world position.
pub fn render_text_element(
    position: Point,
    content: &str,
    font_size: f32,
    style: &ElementStyle,
    pixmap: &mut PixmapMut<'_>,
    transform: Transform,
) {
    render_text_element_wrapped(position, content, font_size, style, pixmap, transform, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_skia::Pixmap;

    #[test]
    fn test_render_text_element_rasterizes_pixels() {
        let mut pixmap = Pixmap::new(200, 100).expect("pixmap creation");
        let style = ElementStyle {
            stroke_color: "#FFFFFF".to_string(),
            ..Default::default()
        };

        render_text_element(
            Point::new(10.0, 10.0),
            "Hello World",
            24.0,
            &style,
            &mut pixmap.as_mut(),
            Transform::identity(),
        );

        let non_zero_pixels = pixmap.data().chunks_exact(4).filter(|p| p[3] > 0).count();
        assert!(
            non_zero_pixels > 50,
            "Rendered text should produce visible raster pixels, got {non_zero_pixels}"
        );
    }

    #[test]
    fn test_render_arabic_text_rasterizes_pixels() {
        let mut pixmap = Pixmap::new(200, 100).expect("pixmap creation");
        let style = ElementStyle {
            stroke_color: "#FF5555".to_string(),
            ..Default::default()
        };

        render_text_element(
            Point::new(10.0, 10.0),
            "مرحبا بك",
            24.0,
            &style,
            &mut pixmap.as_mut(),
            Transform::identity(),
        );

        let non_zero_pixels = pixmap.data().chunks_exact(4).filter(|p| p[3] > 0).count();
        assert!(
            non_zero_pixels > 50,
            "Rendered Arabic text should produce visible raster pixels, got {non_zero_pixels}"
        );
    }

    #[test]
    fn test_render_bilingual_wrapped_text_produces_pixels() {
        let mut pixmap = Pixmap::new(300, 200).expect("pixmap creation");
        let style = ElementStyle {
            stroke_color: "#38BDF8".to_string(),
            ..Default::default()
        };

        let sample = "خوارزمية Dijkstra تعمل في تعقيد O(V log V) للرسم البياني";
        render_text_element_wrapped(
            Point::new(10.0, 20.0),
            sample,
            20.0,
            &style,
            &mut pixmap.as_mut(),
            Transform::identity(),
            Some(180.0),
        );

        let non_zero_pixels = pixmap.data().chunks_exact(4).filter(|p| p[3] > 0).count();
        assert!(
            non_zero_pixels > 100,
            "Rendered wrapped bilingual text should produce pixels across wrapped lines, got {non_zero_pixels}"
        );
    }
}
