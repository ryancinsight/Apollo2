//! Current readback and control operations for Lumidox II Controller
//!
//! This module provides functions for reading current ARM and FIRE current settings
//! and controlling ARM current values.

use crate::core::{LumidoxError, Result};
use crate::communication::{ProtocolHandler, protocol::commands};

/// Read current ARM current setting from device
/// 
/// Uses protocol command 0x20 to read the current ARM current setting.
/// Returns the ARM current value in milliamps (mA).
pub fn read_arm_current(protocol: &mut ProtocolHandler) -> Result<u16> {
    let current_value = protocol.send_command(commands::READ_ARM_CURRENT, 0)? as u16;
    Ok(current_value)
}

/// Read current FIRE current setting from device
/// 
/// Uses protocol command 0x21 to read the current FIRE current setting.
/// Returns the FIRE current value in milliamps (mA).
pub fn read_fire_current(protocol: &mut ProtocolHandler) -> Result<u16> {
    let current_value = protocol.send_command(commands::READ_FIRE_CURRENT, 0)? as u16;
    Ok(current_value)
}

/// Set ARM current value
/// 
/// Uses protocol command 0x40 to set the ARM current.
/// The current value should be specified in milliamps (mA).
/// 
/// # Arguments
/// * `protocol` - Protocol handler for device communication
/// * `current_ma` - ARM current value in milliamps
/// 
/// # Returns
/// * `Ok(())` if the ARM current was set successfully
/// * `Err(LumidoxError)` if the operation failed or current value is invalid
pub fn set_arm_current(protocol: &mut ProtocolHandler, current_ma: u16) -> Result<()> {
    // Validate current value is not zero
    if current_ma == 0 {
        return Err(LumidoxError::InvalidInput(
            "ARM current cannot be zero".to_string()
        ));
    }
    
    protocol.send_command(commands::SET_ARM_CURRENT, current_ma)?;
    Ok(())
}

/// Get current settings summary
/// 
/// Reads both ARM and FIRE current settings and returns them as a formatted string.
/// Useful for displaying current device configuration.
pub fn get_current_settings_summary(protocol: &mut ProtocolHandler) -> Result<String> {
    let arm_current = read_arm_current(protocol)?;
    let fire_current = read_fire_current(protocol)?;
    
    Ok(format!(
        "ARM Current: {}mA, FIRE Current: {}mA", 
        arm_current, 
        fire_current
    ))
}

/// Validate ARM current against device limits
/// 
/// Checks if the proposed ARM current value is within acceptable limits.
/// This function can be extended to check against device-specific maximum values.
pub fn validate_arm_current(current_ma: u16) -> Result<()> {
    if current_ma == 0 {
        return Err(LumidoxError::InvalidInput(
            "ARM current cannot be zero".to_string()
        ));
    }
    
    // TODO: Add maximum current validation against device capabilities
    // This could be enhanced to check against the maximum current from stage 5
    
    Ok(())
}

/// Check if ARM and FIRE currents are synchronized
/// 
/// Compares ARM and FIRE current settings to determine if they match.
/// Returns true if both currents are set to the same value.
pub fn are_currents_synchronized(protocol: &mut ProtocolHandler) -> Result<bool> {
    let arm_current = read_arm_current(protocol)?;
    let fire_current = read_fire_current(protocol)?;
    
    Ok(arm_current == fire_current)
}
