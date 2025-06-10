//! Device module for Lumidox II Controller
//!
//! This module provides high-level device abstraction and control functionality,
//! organized into sub-modules for better maintainability:
//! - `models`: Device data structures and types
//! - `operations`: Device control and power operations
//! - `info`: Device information retrieval
//! - `controller`: Main device controller orchestrating all operations

pub mod models;
pub mod operations;
pub mod info;
pub mod controller;

// Re-export commonly used items for convenience
pub use controller::LumidoxDevice;
