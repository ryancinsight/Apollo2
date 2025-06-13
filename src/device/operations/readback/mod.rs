//! Readback operations sub-module for Lumidox II Controller
//!
//! This module organizes readback operations into focused sub-modules:
//! - `state`: Device state reading and status operations
//! - `current`: ARM/FIRE current readback and ARM current control operations

pub mod state;
pub mod current;

// Re-export commonly used functions for convenience
pub use state::{
    read_remote_mode_state,
    // Note: These functions are available but not currently used
    // is_remote_controlled,
    // is_ready_for_firing,
    get_device_state_description
};

pub use current::{
    read_arm_current, 
    read_fire_current, 
    set_arm_current, 
    get_current_settings_summary
};
