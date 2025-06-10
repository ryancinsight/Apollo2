//! Error context utilities for Lumidox II Controller
//!
//! This module provides error context extension traits and utilities
//! for better error reporting and debugging.

use super::types::LumidoxError;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, LumidoxError>;

/// Error context extension trait for better error reporting
pub trait ErrorContext<T> {
    /// Add context to an error
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<LumidoxError>,
{
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            match base_error {
                LumidoxError::DeviceError(msg) => {
                    LumidoxError::DeviceError(format!("{}: {}", f(), msg))
                }
                other => other,
            }
        })
    }
}
