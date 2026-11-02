//! # ruqa-app
//!
//! Native desktop windowing, winit 0.30 event loop, and mouse input dispatch for Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod clipboard;
pub mod drag;
pub mod event_dispatch;
pub mod file_actions;
pub mod logging;
pub mod presenter;
pub mod selection;
pub mod selection_state;
pub mod window_app;
pub mod window_handler;

/// Application version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
