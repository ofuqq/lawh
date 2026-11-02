//! # ruqa-render
//!
//! 2D vector rendering, rough path conversion, and frame presentation for the Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod offscreen;
pub mod pen_render;
pub mod rough_convert;
pub mod selection_render;
pub mod shape_render;
pub mod spike_benchmark;
pub mod text_layout;
pub mod toolbar_icons;
pub mod toolbar_render;

pub use toolbar_render::{render_toolbar, ToolbarLayout, ToolbarStyle};

/// Library version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
