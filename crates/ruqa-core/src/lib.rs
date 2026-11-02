//! # ruqa-core
//!
//! Core data structures, canvas geometry, and serialization for the Ruqa whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod document;
pub mod element;
pub mod error;
pub mod geometry;
pub mod history;
pub mod hit_test;
pub mod migration;
pub mod scene;
pub mod stroke;
pub mod style;
pub mod tool;

pub use document::RuqaDocument;
pub use history::{Command, HistoryStack};
pub use tool::Tool;

/// Version indicator for the ruqa-core library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the library greeting message.
#[must_use]
pub fn greeting() -> &'static str {
    "Ruqa Core - Lightweight Whiteboard Foundation"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "Ruqa Core - Lightweight Whiteboard Foundation");
    }

    #[test]
    fn test_version_present() {
        assert!(!VERSION.is_empty());
    }
}
