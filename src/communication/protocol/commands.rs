//! Protocol command definitions for Lumidox II Controller
//!
//! This module organizes all device command codes and command arrays used
//! for communicating with the Lumidox II device over serial protocol.
//!
//! Commands are organized into specialized sub-modules by category:
//! - `device_info`: Device information commands (firmware, model, serial, wavelength)
//! - `device_control`: Device control commands (mode setting, current setting)
//! - `device_state`: Device state reading commands (remote mode, current readings)
//! - `stage_parameters`: Stage-specific parameter commands (currents, voltages)

pub mod device_info;
pub mod device_control;
pub mod device_state;
pub mod stage_parameters;

// Re-export all commands for backward compatibility
pub use device_info::*;
pub use device_control::*;
pub use device_state::*;
pub use stage_parameters::*;


