//! # ruqa-app
//!
//! Native desktop windowing, winit 0.30 event loop, and mouse input dispatch for Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod clipboard;
pub mod drag;
pub mod eraser;
pub mod event_dispatch;
pub mod export_actions;
pub mod file_actions;
pub mod frame_render;
pub mod keyboard;
pub mod logging;
pub mod presenter;
pub mod recent_files_actions;
pub mod selection;
pub mod selection_state;
pub mod style_panel;
pub mod toolbar;
pub mod vault_actions;
pub mod window_app;
pub mod window_handler;

/// Application version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
