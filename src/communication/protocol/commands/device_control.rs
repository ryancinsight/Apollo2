//! Device control command definitions
//!
//! This module contains protocol commands for controlling device operation
//! including mode setting and current configuration commands.

/// Device mode setting command (0x15)
/// 
/// Sets the device operational mode. Used to switch between different
/// operational states such as Local, Standby, Armed, and Remote modes.
pub const SET_MODE: &[u8] = b"15";

/// Current setting command (0x41)
/// 
/// Sets the firing current value for the device. This command configures
/// the current level that will be used for subsequent firing operations.
pub const SET_CURRENT: &[u8] = b"41";
