//! Power operations sub-module for Lumidox II Controller
//!
//! This module organizes power operations into focused sub-modules:
//! - `measurement`: Power reading and unit decoding operations
//! - `parameters`: Stage parameter operations (future expansion for missing protocol commands)

pub mod measurement;
pub mod parameters;

// Re-export commonly used functions for backward compatibility
pub use measurement::{get_power_info, decode_total_units, decode_per_units};
pub use parameters::{StageParameters, get_stage_parameters, get_stage_arm_current, get_stage_volt_limit, get_stage_volt_start};
