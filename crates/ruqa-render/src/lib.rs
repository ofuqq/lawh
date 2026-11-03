//! # ruqa-render
//!
//! 2D vector rendering, rough path conversion, and frame presentation for the Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod export_png;
pub mod grid_render;
pub mod offscreen;
pub mod pen_render;
pub mod quit_dialog_layout;
pub mod quit_dialog_render;
pub mod recent_files_layout;
pub mod recent_files_render;
pub mod rough_convert;
pub mod rough_options;
pub mod selection_render;
pub mod shape_render;
pub mod spike_benchmark;
pub mod style_panel_layout;
pub mod style_panel_render;
pub mod text_layout;
pub mod text_render;
pub mod toolbar_icons;
pub mod toolbar_render;
pub mod vault_icons;
pub mod vault_layout;
pub mod vault_render;

pub use export_png::{
    export_elements_to_pixmap, export_elements_to_png, export_elements_to_png_file,
    export_scene_to_pixmap, export_scene_to_png, export_scene_to_png_file, PngExportOptions,
};
pub use grid_render::{render_grid, render_grid_scaled, GridConfig, GridStyle};
pub use quit_dialog_render::{
    render_quit_dialog, render_quit_dialog_scaled, QuitDialogAction, QuitDialogLayout,
    QuitDialogTheme,
};
pub use style_panel_render::{render_style_panel, StyleAction, StylePanelLayout, StylePanelTheme};
pub use text_layout::{measure_text_dimensions, measure_text_dimensions_with_max_width};
pub use text_render::{render_text_element, render_text_element_wrapped, render_ui_text};
pub use toolbar_render::{render_toolbar, ToolbarLayout, ToolbarStyle};

/// Library version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
