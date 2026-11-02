//! Typed error enumerations for the Ruqa core library.
//!
//! Every public fallible function in `ruqa-core` returns one of these variants
//! rather than a bare `String` or `Box<dyn Error>`. The [`thiserror`] derive
//! macro generates the `std::error::Error` and `Display` implementations, keeping
//! the enum declaration as the single source of truth for error messages.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when reading or writing a `.ruqa` scene document.
#[derive(Debug, Error)]
pub enum DocumentError {
    /// The file could not be opened or read.
    #[error("I/O error reading `{path}`: {source}")]
    Io {
        /// File path that was being accessed.
        path: PathBuf,
        /// Underlying OS error.
        #[source]
        source: std::io::Error,
    },

    /// The file's JSON structure is valid UTF-8 but does not match the expected schema.
    #[error("failed to parse `.ruqa` document at `{path}`: {source}")]
    Parse {
        /// File path that was being parsed.
        path: PathBuf,
        /// Deserialisation error from `serde_json` (will be wired in a later issue).
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// The on-disk format version is newer than this build of Ruqa can understand.
    #[error(
        "unsupported document version {found} in `{path}` \
         (this build understands up to version {max_supported})"
    )]
    UnsupportedVersion {
        /// File path.
        path: PathBuf,
        /// Version string that was found in the file.
        found: String,
        /// Highest version string this binary can handle.
        max_supported: String,
    },
}

/// Errors that can occur during rendering operations.
#[derive(Debug, Error)]
pub enum RenderError {
    /// The requested canvas dimensions exceed what the rasteriser can allocate.
    #[error("canvas dimensions {width}x{height} exceed maximum supported size")]
    DimensionsTooLarge {
        /// Requested width in pixels.
        width: u32,
        /// Requested height in pixels.
        height: u32,
    },

    /// A golden-image comparison found pixel differences.
    #[error("golden image mismatch: {mismatch_count} pixel(s) differ (threshold {threshold})")]
    GoldenMismatch {
        /// Number of pixels that did not match.
        mismatch_count: u32,
        /// Maximum allowed mismatches (configured per test).
        threshold: u32,
    },

    /// Writing a PNG or raw bitmap to disk failed.
    #[error("failed to write image to `{path}`: {source}")]
    ImageWrite {
        /// Destination file path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_io_error_display() {
        let err = DocumentError::Io {
            path: PathBuf::from("/tmp/board.ruqa"),
            source: std::io::Error::other("file not found"),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("/tmp/board.ruqa"),
            "path missing from message: {msg}"
        );
        assert!(
            msg.contains("file not found"),
            "source missing from message: {msg}"
        );
    }

    #[test]
    fn render_golden_mismatch_display() {
        let err = RenderError::GoldenMismatch {
            mismatch_count: 42,
            threshold: 10,
        };
        let msg = err.to_string();
        assert!(msg.contains("42"), "count missing: {msg}");
        assert!(msg.contains("10"), "threshold missing: {msg}");
    }

    #[test]
    fn unsupported_version_display() {
        let err = DocumentError::UnsupportedVersion {
            path: PathBuf::from("board.ruqa"),
            found: "99".into(),
            max_supported: "1".into(),
        };
        assert!(err.to_string().contains("99"));
    }
}
