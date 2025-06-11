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
pub use control::{fire_stage, fire_stage_smart, fire_with_current, fire_with_current_smart,
                  arm_device, set_mode, turn_off, shutdown, get_max_current};
pub use power::get_power_info;
pub use readback::{read_remote_mode_state, read_arm_current, read_fire_current, set_arm_current,
                   get_device_state_description, get_current_settings_summary};
