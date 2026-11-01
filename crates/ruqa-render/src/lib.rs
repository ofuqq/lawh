//! # ruqa-render
//!
//! 2D vector rendering, rough path conversion, and frame presentation for the Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod rough_convert;
pub mod spike_benchmark;

/// Library version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
