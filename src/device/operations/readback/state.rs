//! Device state readback operations for Lumidox II Controller
//!
//! This module provides functions for reading device operational state
//! including remote mode status and device configuration.

use crate::core::Result;
use crate::communication::{ProtocolHandler, protocol::commands};
use crate::device::models::DeviceMode;

/// Read current remote mode state from device
/// 
/// Uses protocol command 0x13 to read the current operational state.
/// Returns the DeviceMode corresponding to the device's current state:
/// - 0x0000: Local mode (device controlled locally)
/// - 0x0001: Standby mode (On, Output Off)
/// - 0x0002: Armed mode (On, Arm)
/// - 0x0003: Remote mode (On, Fire)
pub fn read_remote_mode_state(protocol: &mut ProtocolHandler) -> Result<DeviceMode> {
    let state_value = protocol.send_command(commands::READ_REMOTE_MODE, 0)?;
    
    let mode = match state_value {
        0 => DeviceMode::Local,
        1 => DeviceMode::Standby,
        2 => DeviceMode::Armed,
        3 => DeviceMode::Remote,
        _ => {
            // Default to Local mode for unknown values
            DeviceMode::Local
        }
    };
    
    Ok(mode)
}

/// Check if device is in remote control mode
/// 
/// Returns true if device is in any remote-controlled state (Standby, Armed, or Remote),
/// false if in Local mode.
pub fn is_remote_controlled(protocol: &mut ProtocolHandler) -> Result<bool> {
    let mode = read_remote_mode_state(protocol)?;
    Ok(matches!(mode, DeviceMode::Standby | DeviceMode::Armed | DeviceMode::Remote))
}

/// Check if device is ready for firing
/// 
/// Returns true if device is in Armed or Remote (Fire) mode,
/// false otherwise.
pub fn is_ready_for_firing(protocol: &mut ProtocolHandler) -> Result<bool> {
    let mode = read_remote_mode_state(protocol)?;
    Ok(matches!(mode, DeviceMode::Armed | DeviceMode::Remote))
}

/// Get device state as human-readable string
/// 
/// Returns a descriptive string of the current device operational state.
pub fn get_device_state_description(protocol: &mut ProtocolHandler) -> Result<String> {
    let mode = read_remote_mode_state(protocol)?;
    
    let description = match mode {
        DeviceMode::Local => "Local Control (device controlled locally)",
        DeviceMode::Standby => "Remote Standby (on, output off)",
        DeviceMode::Armed => "Remote Armed (on, ready for firing)",
        DeviceMode::Remote => "Remote Firing (on, output active)",
    };
    
    Ok(description.to_string())
}
