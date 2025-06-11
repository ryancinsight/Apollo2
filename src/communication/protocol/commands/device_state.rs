//! Device state reading command definitions
//!
//! This module contains protocol commands for reading current device state
//! and configuration including remote mode status and current settings.

/// Read remote mode state command (0x13)
/// 
/// Reads the current remote mode state of the device. Returns a value
/// indicating the current operational mode and readiness status.
pub const READ_REMOTE_MODE: &[u8] = b"13";

/// Read ARM current command (0x20)
/// 
/// Reads the current ARM current setting from the device. This returns
/// the current value configured for arming operations.
pub const READ_ARM_CURRENT: &[u8] = b"20";

/// Read FIRE current command (0x21)
/// 
/// Reads the current FIRE current setting from the device. This returns
/// the current value configured for firing operations.
pub const READ_FIRE_CURRENT: &[u8] = b"21";

/// Set ARM current command (0x40)
/// 
/// Sets the ARM current value for the device. This command configures
/// the current level that will be used for arming operations.
pub const SET_ARM_CURRENT: &[u8] = b"40";
