//! Tracing subscriber initialisation for the Ruqa desktop application.
//!
//! Call [`init`] once at the start of `main`. It reads the `RUST_LOG` environment
//! variable (defaulting to `info`) and installs a compact, coloured
//! formatter that writes to stderr. No allocations occur inside per-frame or
//! per-event paths because `tracing` macros compile to a static `Callsite` probe;
//! the string formatting only runs when the event is actually enabled.
//!
//! ## Choosing log levels
//!
//! | Level     | Use for |
//! |-----------|---------|
//! | `error!`  | Unrecoverable failures (bad file, render crash) |
//! | `warn!`   | Degraded quality (font fallback, slow frame) |
//! | `info!`   | Startup lifecycle, window open/close |
//! | `debug!`  | Tool state changes, document saves |
//! | `trace!`  | High-frequency paths — **disabled in release builds** |

use tracing_subscriber::{fmt, EnvFilter};

/// Initialise a compact stderr subscriber.
///
/// Reads `RUST_LOG` from the environment; if unset defaults to `"info"`.
///
/// # Panics
/// Panics if called more than once per process (the global subscriber can only
/// be set once; subsequent calls are a programming error).
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .compact()
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_env_filter_is_info() {
        let filter = EnvFilter::new("info");
        // Verify filter parses correctly
        assert_eq!(filter.to_string(), "info");
    }
}
