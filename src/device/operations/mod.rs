//! Device operations module for Lumidox II Controller
//!
//! This module organizes device operations into focused sub-modules:
//! - `control`: Device control operations (firing, arming, modes, validation)
//! - `power`: Power measurement and stage parameter operations
//! - `readback`: Device state and current readback operations

pub mod control;
pub mod power;
pub mod readback;

// Re-export commonly used items for convenience
