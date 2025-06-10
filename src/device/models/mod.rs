//! Device models module for Lumidox II Controller
//!
//! This module contains all data structures and types used to represent
//! device state, configuration, and information.

pub mod types;

// Re-export commonly used items for convenience
pub use types::{DeviceMode, Stage, DeviceInfo, PowerInfo};
