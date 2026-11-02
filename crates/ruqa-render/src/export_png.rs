//! High-resolution PNG image export pipeline for Ruqa canvas scenes.

use crate::shape_render::render_element;
use ruqa_core::error::RenderError;
use ruqa_core::geometry::Rect;
use ruqa_core::scene::Scene;
use std::fs;
use std::path::Path;
use tiny_skia::{Color, Pixmap, Transform};

/// Configuration options for rasterizing and encoding a canvas scene into a PNG image.
#[derive(Debug, Clone, PartialEq)]
pub struct PngExportOptions {
    /// Resolution scale multiplier (1.0 for standard 1x, 2.0 for 2x HiDPI/Retina).
    pub scale: f32,
    /// Whether the exported PNG image has a transparent alpha channel.
    pub transparent_background: bool,
    /// Background fill color when `transparent_background` is false (defaults to dark theme #121212).
    pub background_color: Option<Color>,
    /// Outer margin padding in world coordinate units added around exported bounds.
    pub padding: f32,
}

impl Default for PngExportOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            transparent_background: false,
            background_color: Some(Color::from_rgba8(18, 18, 18, 255)),
            padding: 20.0,
        }
    }
}

impl PngExportOptions {
    /// Creates preset options for 2x high-resolution dark-background export.
    #[must_use]
    pub fn high_res_dark() -> Self {
        Self {
            scale: 2.0,
            transparent_background: false,
            background_color: Some(Color::from_rgba8(18, 18, 18, 255)),
            padding: 20.0,
        }
    }

    /// Creates preset options for transparent background export (for pasting into slides/notes).
    #[must_use]
    pub fn transparent() -> Self {
        Self {
            scale: 1.0,
            transparent_background: true,
            background_color: None,
            padding: 20.0,
        }
    }

    /// Creates preset options for 2x high-resolution transparent background export.
    #[must_use]
    pub fn high_res_transparent() -> Self {
        Self {
            scale: 2.0,
            transparent_background: true,
            background_color: None,
            padding: 20.0,
        }
    }
}

/// Renders the specified bounding region of a scene into an allocated [`tiny_skia::Pixmap`].
#[must_use]
pub fn export_scene_to_pixmap(
    scene: &Scene,
    bounds: Rect,
    options: &PngExportOptions,
) -> Option<Pixmap> {
    let scale = options.scale.clamp(0.1, 16.0);
    let padding = options.padding.max(0.0);

    let padded_w = (bounds.width() + (padding * 2.0)).max(1.0);
    let padded_h = (bounds.height() + (padding * 2.0)).max(1.0);

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let pixel_w = (padded_w * scale).ceil() as u32;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let pixel_h = (padded_h * scale).ceil() as u32;

    // Reject extreme allocations exceeding 16384x16384 px
    if pixel_w == 0 || pixel_h == 0 || pixel_w > 16_384 || pixel_h > 16_384 {
        return None;
    }

    let mut pixmap = Pixmap::new(pixel_w, pixel_h)?;

    if options.transparent_background {
        pixmap.fill(Color::TRANSPARENT);
    } else {
        let bg = options
            .background_color
            .unwrap_or(Color::from_rgba8(18, 18, 18, 255));
        pixmap.fill(bg);
    }

    let tx = (padding - bounds.min.x) * scale;
    let ty = (padding - bounds.min.y) * scale;
    let transform = Transform::from_row(scale, 0.0, 0.0, scale, tx, ty);

    for elem in scene.elements() {
        render_element(elem, &mut pixmap.as_mut(), transform);
    }

    Some(pixmap)
}

/// Renders a canvas scene and encodes it directly into a PNG byte buffer.
///
/// # Errors
/// Returns [`RenderError`] if dimensions are invalid or PNG encoding fails.
pub fn export_scene_to_png(
    scene: &Scene,
    bounds: Rect,
    options: &PngExportOptions,
) -> Result<Vec<u8>, RenderError> {
    let pixmap = export_scene_to_pixmap(scene, bounds, options).ok_or_else(|| {
        let scale = options.scale.clamp(0.1, 16.0);
        let padded_w = bounds.width() + (options.padding.max(0.0) * 2.0);
        let padded_h = bounds.height() + (options.padding.max(0.0) * 2.0);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        RenderError::DimensionsTooLarge {
            width: (padded_w * scale).ceil() as u32,
            height: (padded_h * scale).ceil() as u32,
        }
    })?;

    pixmap.encode_png().map_err(|e| RenderError::PngEncode {
        reason: e.to_string(),
    })
}

/// Renders a canvas scene to PNG and writes it atomically to the specified file path.
///
/// # Errors
/// Returns [`RenderError`] if rendering, PNG encoding, or filesystem write fails.
pub fn export_scene_to_png_file(
    scene: &Scene,
    bounds: Rect,
    path: impl AsRef<Path>,
    options: &PngExportOptions,
) -> Result<(), RenderError> {
    let path = path.as_ref();
    let png_bytes = export_scene_to_png(scene, bounds, options)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| RenderError::ImageWrite {
            path: path.to_path_buf(),
            source: e,
        })?;
    }

    fs::write(path, png_bytes).map_err(|e| RenderError::ImageWrite {
        path: path.to_path_buf(),
        source: e,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruqa_core::element::{Element, ElementStyle};

    fn sample_rect(id: u64, x: f32, y: f32, w: f32, h: f32) -> Element {
        Element::Rectangle {
            id,
            bounds: Rect::from_xywh(x, y, w, h),
            style: ElementStyle {
                stroke_color: "#FF6B6B".to_string(),
                stroke_width: 2.0,
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_export_scene_to_pixmap_dimensions_and_scale() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 100.0, 100.0, 200.0, 100.0));

        let bounds = Rect::from_xywh(100.0, 100.0, 200.0, 100.0);
        let options_1x = PngExportOptions {
            scale: 1.0,
            padding: 10.0,
            transparent_background: true,
            ..Default::default()
        };

        let pixmap_1x = export_scene_to_pixmap(&scene, bounds, &options_1x).unwrap();
        // width = 200 + 20 = 220, height = 100 + 20 = 120
        assert_eq!(pixmap_1x.width(), 220);
        assert_eq!(pixmap_1x.height(), 120);

        let options_2x = PngExportOptions {
            scale: 2.0,
            padding: 10.0,
            transparent_background: false,
            ..Default::default()
        };
        let pixmap_2x = export_scene_to_pixmap(&scene, bounds, &options_2x).unwrap();
        assert_eq!(pixmap_2x.width(), 440);
        assert_eq!(pixmap_2x.height(), 240);
    }

    #[test]
    fn test_export_scene_to_png_magic_header_and_file_write() {
        let mut scene = Scene::new();
        scene.push(sample_rect(1, 0.0, 0.0, 80.0, 60.0));

        let bounds = Rect::from_xywh(0.0, 0.0, 80.0, 60.0);
        let options = PngExportOptions::default();

        let bytes = export_scene_to_png(&scene, bounds, &options).unwrap();
        assert!(bytes.len() > 100);
        // Verify standard PNG magic header \x89PNG\r\n\x1a\n
        assert_eq!(&bytes[0..8], b"\x89PNG\r\n\x1a\n");

        // Write to temporary file
        let tmp = std::env::temp_dir().join(format!("ruqa_export_test_{}.png", std::process::id()));
        export_scene_to_png_file(&scene, bounds, &tmp, &options).unwrap();
        assert!(tmp.is_file());
        let read_bytes = fs::read(&tmp).unwrap();
        assert_eq!(read_bytes, bytes);
        let _ = fs::remove_file(&tmp);
    }
}
