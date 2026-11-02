//! PNG image export actions and shortcut handling for [`WhiteboardApp`].

use crate::window_app::WhiteboardApp;
use ruqa_core::element::Element;
use ruqa_core::error::RenderError;
use ruqa_core::geometry::Rect;
use ruqa_render::export_png::{export_elements_to_png_file, PngExportOptions};
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

    /// Returns `true` if one or more canvas elements are actively selected.
    #[must_use]
    pub fn has_active_selection(&self) -> bool {
        !self.selection.is_empty()
    }

    /// Calculates the tight export bounding box.
    ///
    /// If elements are selected, returns the bounding box enclosing the selection.
    /// If nothing is selected, returns the bounding box enclosing all scene elements.
    #[must_use]
    pub fn export_bounds(&self) -> Rect {
        if self.has_active_selection() {
            if let Some(bounds) = self.scene.selection_bounds(self.selection.selected_ids()) {
                return bounds;
            }
        }

        let ids: Vec<u64> = self.scene.elements().iter().map(Element::id).collect();
        self.scene
            .selection_bounds(&ids)
            .unwrap_or_else(|| Rect::from_xywh(0.0, 0.0, 200.0, 150.0))
    }

    /// Returns the collection of elements to be rendered in the export.
    ///
    /// If elements are selected, yields only the selected elements.
    /// If nothing is selected, yields all elements on the canvas.
    #[must_use]
    pub fn export_elements(&self) -> Vec<&Element> {
        if self.has_active_selection() {
            self.scene
                .elements()
                .iter()
                .filter(|e| self.selection.is_selected(e.id()))
                .collect()
        } else {
            self.scene.elements().iter().collect()
        }
    }

    /// Exports the current canvas or selection to a high-resolution PNG file.
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
        let elements = self.export_elements();
        export_elements_to_png_file(elements, bounds, path, options)?;
        info!(
            path = ?path,
            is_selection = self.has_active_selection(),
            scale = options.scale,
            "exported canvas to PNG image"
        );
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
