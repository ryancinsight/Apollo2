//! Core module for Lumidox II Controller
//!
//! This module contains foundational components that are shared across
//! the entire application, organized into sub-modules:
//! - `error`: Error handling with sub-components
//! - `operations`: Unified operation interfaces for CLI/GUI
//! - `types`: Common type definitions and aliases
//! - `calculations`: Mathematical calculations and algorithms

pub mod error;
pub mod operations;
pub mod types;
pub mod calculations;

// Re-export commonly used items for convenience
pub use error::LumidoxError;
pub use operations::{DeviceControlOperations, DeviceOperationData};
pub use types::Result;
pub use calculations::*;
