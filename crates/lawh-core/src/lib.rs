//! # lawh-core
//!
//! Core data structures, canvas geometry, and serialization for the Lawh whiteboard.

#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Version indicator for the lawh-core library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the library greeting message.
#[must_use]
pub fn greeting() -> &'static str {
    "Lawh Core - Lightweight Whiteboard Foundation"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "Lawh Core - Lightweight Whiteboard Foundation");
    }

    #[test]
    fn test_version_present() {
        assert!(!VERSION.is_empty());
    }
}
