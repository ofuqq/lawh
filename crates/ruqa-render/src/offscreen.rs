//! Offscreen canvas rasterization and visual regression test utilities.

use std::path::Path;
use tiny_skia::{Color, Paint, Path as SkiaPath, Pixmap, PremultipliedColorU8, Stroke, Transform};

/// Offscreen canvas for headless rendering and visual regression verification.
pub struct OffscreenCanvas {
    pixmap: Pixmap,
}

impl OffscreenCanvas {
    /// Creates a new offscreen canvas with given dimensions.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Option<Self> {
        Pixmap::new(width, height).map(|pixmap| Self { pixmap })
    }

    /// Returns canvas pixel width.
    #[must_use]
    pub fn width(&self) -> u32 {
        self.pixmap.width()
    }

    /// Returns canvas pixel height.
    #[must_use]
    pub fn height(&self) -> u32 {
        self.pixmap.height()
    }

    /// Borrow underlying Pixmap reference.
    #[must_use]
    pub fn pixmap(&self) -> &Pixmap {
        &self.pixmap
    }

    /// Borrow underlying mutable Pixmap reference.
    pub fn pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }

    /// Clears canvas to solid color.
    pub fn clear(&mut self, color: Color) {
        self.pixmap.fill(color);
    }

    /// Strokes a path onto the canvas.
    pub fn stroke_path(
        &mut self,
        path: &SkiaPath,
        paint: &Paint,
        stroke: &Stroke,
        transform: Transform,
    ) {
        self.pixmap
            .stroke_path(path, paint, stroke, transform, None);
    }

    /// Reads RGBA pixel at (x, y).
    #[must_use]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<PremultipliedColorU8> {
        self.pixmap.pixel(x, y)
    }

    /// Encodes canvas to PNG file.
    pub fn save_png(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        self.pixmap
            .save_png(path)
            .map_err(|e| std::io::Error::other(e.to_string()))
    }
}

/// Result of comparing two pixmaps.
#[derive(Debug, Clone)]
pub struct DiffReport {
    /// Total pixels evaluated.
    pub total_pixels: usize,
    /// Number of pixels exceeding per-channel color tolerance.
    pub mismatched_pixels: usize,
    /// Maximum channel difference observed across all pixels.
    pub max_channel_delta: u8,
    /// Percentage of pixels that mismatched.
    pub mismatch_percentage: f64,
    /// Whether difference is within the allowed threshold.
    pub is_match: bool,
}

/// Compares two pixmaps pixel-by-pixel with tolerance and generates a visual diff pixmap.
///
/// In the diff pixmap:
/// - Matching pixels are rendered in dark muted gray.
/// - Mismatched pixels are highlighted in vibrant magenta (`#FF00FF`).
#[must_use]
pub fn compare_pixmaps(
    actual: &Pixmap,
    expected: &Pixmap,
    channel_tolerance: u8,
    max_allowed_mismatch: usize,
) -> (DiffReport, Option<Pixmap>) {
    let width = actual.width();
    let height = actual.height();

    if width != expected.width() || height != expected.height() {
        return (
            DiffReport {
                total_pixels: (width * height) as usize,
                mismatched_pixels: (width * height) as usize,
                max_channel_delta: 255,
                mismatch_percentage: 100.0,
                is_match: false,
            },
            None,
        );
    }

    let total_pixels = (width * height) as usize;
    let mut mismatched_pixels = 0;
    let mut max_channel_delta = 0u8;

    let mut diff_pixmap = Pixmap::new(width, height);

    let actual_pixels = actual.pixels();
    let expected_pixels = expected.pixels();

    let magenta = PremultipliedColorU8::from_rgba(255, 0, 255, 255).unwrap();
    let dim_gray = PremultipliedColorU8::from_rgba(25, 25, 25, 255).unwrap();

    for (i, (act, exp)) in actual_pixels.iter().zip(expected_pixels.iter()).enumerate() {
        let dr = (act.red() as i16 - exp.red() as i16).unsigned_abs() as u8;
        let dg = (act.green() as i16 - exp.green() as i16).unsigned_abs() as u8;
        let db = (act.blue() as i16 - exp.blue() as i16).unsigned_abs() as u8;
        let da = (act.alpha() as i16 - exp.alpha() as i16).unsigned_abs() as u8;

        let delta = dr.max(dg).max(db).max(da);
        if delta > max_channel_delta {
            max_channel_delta = delta;
        }

        let is_pixel_mismatch = delta > channel_tolerance;
        if is_pixel_mismatch {
            mismatched_pixels += 1;
        }

        if let Some(diff) = &mut diff_pixmap {
            diff.pixels_mut()[i] = if is_pixel_mismatch { magenta } else { dim_gray };
        }
    }

    let mismatch_percentage = (mismatched_pixels as f64 / total_pixels as f64) * 100.0;
    let is_match = mismatched_pixels <= max_allowed_mismatch;

    let report = DiffReport {
        total_pixels,
        mismatched_pixels,
        max_channel_delta,
        mismatch_percentage,
        is_match,
    };

    (report, diff_pixmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offscreen_canvas_clear_and_get_pixel() {
        let mut canvas = OffscreenCanvas::new(100, 100).expect("failed to create canvas");
        assert_eq!(canvas.width(), 100);
        assert_eq!(canvas.height(), 100);

        let red = Color::from_rgba8(255, 0, 0, 255);
        canvas.clear(red);

        let pixel = canvas.get_pixel(50, 50).expect("pixel out of bounds");
        assert_eq!(pixel.red(), 255);
        assert_eq!(pixel.green(), 0);
        assert_eq!(pixel.blue(), 0);
        assert_eq!(pixel.alpha(), 255);
    }

    #[test]
    fn test_compare_identical_pixmaps() {
        let mut p1 = Pixmap::new(50, 50).unwrap();
        let mut p2 = Pixmap::new(50, 50).unwrap();
        p1.fill(Color::from_rgba8(18, 18, 18, 255));
        p2.fill(Color::from_rgba8(18, 18, 18, 255));

        let (report, _) = compare_pixmaps(&p1, &p2, 0, 0);
        assert!(report.is_match);
        assert_eq!(report.mismatched_pixels, 0);
        assert_eq!(report.max_channel_delta, 0);
    }

    #[test]
    fn test_compare_differing_pixmaps_detects_mismatch() {
        let mut p1 = Pixmap::new(50, 50).unwrap();
        let mut p2 = Pixmap::new(50, 50).unwrap();
        p1.fill(Color::from_rgba8(18, 18, 18, 255));
        p2.fill(Color::from_rgba8(255, 255, 255, 255));

        let (report, diff) = compare_pixmaps(&p1, &p2, 5, 0);
        assert!(!report.is_match);
        assert_eq!(report.mismatched_pixels, 2500);
        assert!(diff.is_some());
    }
}
