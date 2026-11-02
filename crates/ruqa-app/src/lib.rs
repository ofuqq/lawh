//! # ruqa-app
//!
//! Native desktop windowing, winit 0.30 event loop, and mouse input dispatch for Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod event_dispatch;
pub mod logging;
pub mod presenter;
pub mod selection;
pub mod window_app;

/// Application version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
