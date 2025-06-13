//! Control operations sub-module for Lumidox II Controller
//!
//! This module organizes control operations into focused sub-modules:
//! - `firing`: Stage and current-based firing operations
//! - `arming`: Device arming operations
//! - `modes`: Device mode management
//! - `validation`: Input validation functions

pub mod firing;
pub mod arming;
pub mod modes;
pub mod validation;

// Re-export commonly used functions for backward compatibility
pub use firing::{fire_stage, fire_stage_smart, fire_with_current, fire_with_current_smart, get_max_current};
pub use arming::arm_device;
pub use modes::{set_mode, turn_off, shutdown};
