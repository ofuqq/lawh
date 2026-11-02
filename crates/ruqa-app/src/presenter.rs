//! Frame presentation: owns a softbuffer surface + tiny-skia pixmap and blits
//! CPU-rendered pixels to the OS window on every `RedrawRequested` event.
//!
//! # Architecture
//!
//! ```text
//!  tiny-skia PixmapMut  ──fill/draw──►  Pixmap (RGBA bytes)
//!                                             │
//!                                         blit (XRGB repack)
//!                                             │
//!                                        softbuffer Surface  ──►  OS window
//! ```
//!
//! The pixmap is reallocated only when the surface size changes (resize or
//! scale-factor change). On steady frames no heap allocation occurs.
//!
//! All window concerns stay in `ruqa-app`; `ruqa-render` remains window-free
//! so it can be used in headless tests.

use ruqa_core::error::RenderError;
use softbuffer::{Context, Surface};
use std::sync::Arc;
use tiny_skia::{Color, Pixmap};
use tracing::{debug, warn};
use winit::window::Window;

/// Owns the softbuffer surface and the CPU pixmap for one window.
///
/// On each frame:
/// 1. Call [`clear_dark`](Presenter::clear_dark) (or paint into [`pixmap_mut`](Presenter::pixmap_mut)).
/// 2. Call [`present`](Presenter::present) to blit pixels to the OS.
pub struct Presenter {
    // The context must stay alive as long as the surface exists.
    _context: Context<Arc<Window>>,
    surface: Surface<Arc<Window>, Arc<Window>>,
    pixmap: Pixmap,
    /// Physical width of the surface in pixels (matches softbuffer allocation).
    pub width: u32,
    /// Physical height of the surface in pixels (matches softbuffer allocation).
    pub height: u32,
}

impl Presenter {
    /// Creates a new presenter attached to the given window.
    ///
    /// Returns `None` if the softbuffer context or surface cannot be created
    /// (e.g., no GPU driver, headless, or missing Wayland compositor).
    #[must_use]
    pub fn new(window: Arc<Window>) -> Option<Self> {
        let context = Context::new(window.clone()).ok()?;
        let mut surface = Surface::new(&context, window.clone()).ok()?;
        let size = window.inner_size();
        let (w, h) = (size.width.max(1), size.height.max(1));
        let (w_nz, h_nz) = (std::num::NonZeroU32::new(w)?, std::num::NonZeroU32::new(h)?);
        surface.resize(w_nz, h_nz).ok()?;
        let pixmap = Pixmap::new(w, h)?;
        debug!(width = w, height = h, "Presenter created");
        Some(Self {
            _context: context,
            surface,
            pixmap,
            width: w,
            height: h,
        })
    }

    /// Resizes the pixmap and softbuffer surface to match new physical dimensions.
    ///
    /// Call from `WindowEvent::Resized` and `WindowEvent::ScaleFactorChanged`.
    /// No-ops if the dimensions are unchanged or zero (minimised window).
    ///
    /// # Errors
    /// Returns [`RenderError::DimensionsTooLarge`] if the pixmap allocation fails.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), RenderError> {
        // Zero size = minimised; keep existing allocation rather than erroring.
        if width == 0 || height == 0 {
            return Ok(());
        }
        if width == self.width && height == self.height {
            return Ok(());
        }

        let Some(new_pixmap) = Pixmap::new(width, height) else {
            return Err(RenderError::DimensionsTooLarge { width, height });
        };

        if let (Some(w_nz), Some(h_nz)) = (
            std::num::NonZeroU32::new(width),
            std::num::NonZeroU32::new(height),
        ) {
            if let Err(e) = self.surface.resize(w_nz, h_nz) {
                warn!(
                    ?e,
                    "softbuffer surface resize failed; continuing with old size"
                );
            }
        }

        self.pixmap = new_pixmap;
        self.width = width;
        self.height = height;
        debug!(width, height, "Presenter resized");
        Ok(())
    }

    /// Borrow the pixmap for painting.
    ///
    /// Paint into this, then call [`present`](Presenter::present).
    #[must_use]
    pub fn pixmap_mut(&mut self) -> &mut Pixmap {
        &mut self.pixmap
    }

    /// Fills the pixmap with Ruqa's matte dark background (`#121212`).
    pub fn clear_dark(&mut self) {
        self.pixmap.fill(Color::from_rgba8(0x12, 0x12, 0x12, 0xFF));
    }

    /// Blits the pixmap to the softbuffer surface and presents it to the OS.
    ///
    /// **Pixel format conversion**: tiny-skia stores premultiplied RGBA.
    /// softbuffer expects `0x00RRGGBB` (XRGB, alpha ignored). We reorder channels.
    ///
    /// # Errors
    /// Returns [`RenderError::ImageWrite`] if the softbuffer blit or present fails.
    pub fn present(&mut self) -> Result<(), RenderError> {
        let mut buffer = self
            .surface
            .buffer_mut()
            .map_err(|e| RenderError::ImageWrite {
                path: std::path::PathBuf::from("<softbuffer>"),
                source: std::io::Error::other(e.to_string()),
            })?;

        for (dst, src) in buffer.iter_mut().zip(self.pixmap.pixels().iter()) {
            *dst = premultiplied_to_xrgb(*src);
        }

        buffer.present().map_err(|e| RenderError::ImageWrite {
            path: std::path::PathBuf::from("<softbuffer>"),
            source: std::io::Error::other(e.to_string()),
        })?;
        Ok(())
    }
}

/// Converts a tiny-skia premultiplied color pixel to softbuffer `0x00RRGGBB` format.
#[inline]
#[must_use]
pub const fn premultiplied_to_xrgb(pixel: tiny_skia::PremultipliedColorU8) -> u32 {
    ((pixel.red() as u32) << 16) | ((pixel.green() as u32) << 8) | (pixel.blue() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_skia::PremultipliedColorU8;

    #[test]
    fn test_premultiplied_to_xrgb_conversions() {
        let dark = PremultipliedColorU8::from_rgba(0x12, 0x12, 0x12, 0xFF).unwrap();
        assert_eq!(premultiplied_to_xrgb(dark), 0x0012_1212);

        let red = PremultipliedColorU8::from_rgba(0xFF, 0x00, 0x00, 0xFF).unwrap();
        assert_eq!(premultiplied_to_xrgb(red), 0x00FF_0000);

        let green = PremultipliedColorU8::from_rgba(0x00, 0xFF, 0x00, 0xFF).unwrap();
        assert_eq!(premultiplied_to_xrgb(green), 0x0000_FF00);

        let blue = PremultipliedColorU8::from_rgba(0x00, 0x00, 0xFF, 0xFF).unwrap();
        assert_eq!(premultiplied_to_xrgb(blue), 0x0000_00FF);

        let white = PremultipliedColorU8::from_rgba(0xFF, 0xFF, 0xFF, 0xFF).unwrap();
        assert_eq!(premultiplied_to_xrgb(white), 0x00FF_FFFF);
    }
}
