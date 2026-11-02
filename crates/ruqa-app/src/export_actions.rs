//! PNG image export actions and shortcut handling for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::error::RenderError;
use ruqa_core::geometry::Rect;
use ruqa_render::export_png::{export_scene_to_png_file, PngExportOptions};
use std::path::{Path, PathBuf};
use tracing::info;

impl WhiteboardApp {
    /// Resolves the default destination path for exporting a PNG image.
    #[must_use]
    pub fn default_export_path(&self) -> PathBuf {
        if let Some(ref path) = self.file_path {
            path.with_extension("png")
        } else {
            PathBuf::from("Ruqa_Board.png")
        }
    }

    /// Calculates the diagram bounding box for export (whole board aggregate bounds).
    #[must_use]
    pub fn export_bounds(&self) -> Rect {
        let ids: Vec<u64> = self
            .scene
            .elements()
            .iter()
            .map(ruqa_core::element::Element::id)
            .collect();
        self.scene
            .selection_bounds(&ids)
            .unwrap_or_else(|| Rect::from_xywh(0.0, 0.0, 200.0, 150.0))
    }

    /// Exports the current canvas scene to a high-resolution PNG file.
    ///
    /// # Errors
    /// Returns [`RenderError`] if rendering, PNG encoding, or filesystem write fails.
    pub fn export_png(
        &self,
        path: impl AsRef<Path>,
        options: &PngExportOptions,
    ) -> Result<(), RenderError> {
        let path = path.as_ref();
        let bounds = self.export_bounds();
        export_scene_to_png_file(&self.scene, bounds, path, options)?;
        info!(path = ?path, scale = options.scale, "exported canvas to PNG image");
        Ok(())
    }

    /// Exports the current canvas to its default file location using 2x high-resolution dark mode.
    ///
    /// # Errors
    /// Returns [`RenderError`] if rendering or saving fails.
    pub fn export_high_res_png_default(&self) -> Result<(), RenderError> {
        let path = self.default_export_path();
        self.export_png(&path, &PngExportOptions::high_res_dark())
    }
}
