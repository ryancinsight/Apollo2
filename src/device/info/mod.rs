//! Device information module for Lumidox II Controller
//!
//! This module handles device information retrieval and management,
//! including firmware version, model number, serial number, and wavelength.

pub mod reader;

// Re-export commonly used items for convenience
pub use reader::*;
