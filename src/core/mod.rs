//! Core module for Lumidox II Controller
//!
//! This module contains foundational components that are shared across
//! the entire application, organized into sub-modules:
//! - `error`: Error handling with sub-components
//! - `types`: Common type definitions and aliases

pub mod error;
pub mod types;

// Re-export commonly used items for convenience
pub use error::LumidoxError;
pub use types::Result;
