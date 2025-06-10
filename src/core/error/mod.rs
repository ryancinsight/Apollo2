//! Error handling sub-module for Lumidox II Controller
//!
//! This module organizes error handling functionality into logical components:
//! - types: Error type definitions and variants
//! - context: Error context utilities and traits

pub mod types;
pub mod context;

// Re-export commonly used items for convenience
pub use types::LumidoxError;
