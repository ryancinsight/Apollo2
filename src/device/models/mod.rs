//! Device models module for Lumidox II Controller
//!
//! This module contains all data structures and types used to represent
//! device state, configuration, and information.
//!
//! Models are organized into specialized sub-modules by category:
//! - `device_state`: Device operating modes and state-related types
//! - `device_info`: Device identification and information types
//! - `power`: Power measurement and energy-related types
//! - `parameters`: Configuration parameters and stage-related types

pub mod device_state;
pub mod device_info;
pub mod power;
pub mod parameters;

// Maintain backward compatibility by re-exporting from legacy types module
pub mod types;

// Re-export all types for backward compatibility and convenience
pub use device_state::*;
pub use device_info::*;
pub use power::*;
pub use parameters::*;
