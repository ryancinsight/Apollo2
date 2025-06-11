//! Firing operations for Lumidox II Controller
//!
//! This module provides functions specifically for firing stages and
//! managing current-based firing operations with intelligent transitions.

use crate::core::{LumidoxError, Result};
use crate::communication::{ProtocolHandler, protocol::commands};
use crate::device::models::{DeviceMode, Stage};
use super::arming::arm_device;
use super::modes::set_mode;
use std::thread;
use std::time::Duration;

/// Fire a specific stage with intelligent mode transition
pub fn fire_stage_smart(protocol: &mut ProtocolHandler, stage_num: u8, current_mode: Option<DeviceMode>) -> Result<()> {
    let stage = Stage::new(stage_num)?;
    
    // Get the current for this stage
    let current = protocol.send_command(stage.current_command(), 0)? as u16;
    
    // Intelligent sequence based on current device state
    match current_mode {
        Some(DeviceMode::Remote) | Some(DeviceMode::Armed) => {
            // Device is already active - direct transition without turning off
            protocol.send_command(commands::SET_CURRENT, current)?;
            set_mode(protocol, DeviceMode::Remote)?;
        }
        _ => {
            // Device is off or in local mode - use full sequence
            set_mode(protocol, DeviceMode::Standby)?;
            thread::sleep(Duration::from_millis(100));
            arm_device(protocol)?;
            protocol.send_command(commands::SET_CURRENT, current)?;
            set_mode(protocol, DeviceMode::Remote)?;
        }
    }
    
    Ok(())
}

/// Fire a specific stage (legacy function for backward compatibility)
pub fn fire_stage(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<()> {
    fire_stage_smart(protocol, stage_num, None)
}

/// Fire with a specific current value with intelligent mode transition
pub fn fire_with_current_smart(protocol: &mut ProtocolHandler, current_ma: u16, current_mode: Option<DeviceMode>) -> Result<()> {
    // Validate against maximum current
    let max_current = get_max_current(protocol)?;
    if current_ma > max_current {
        return Err(LumidoxError::InvalidInput(
            format!("Cannot fire above {}mA (requested: {}mA)", max_current, current_ma)
        ));
    }
    
    // Intelligent sequence based on current device state
    match current_mode {
        Some(DeviceMode::Remote) | Some(DeviceMode::Armed) => {
            // Device is already active - direct transition without turning off
            protocol.send_command(commands::SET_CURRENT, current_ma)?;
            set_mode(protocol, DeviceMode::Remote)?;
        }
        _ => {
            // Device is off or in local mode - use full sequence
            set_mode(protocol, DeviceMode::Standby)?;
            thread::sleep(Duration::from_millis(100));
            arm_device(protocol)?;
            protocol.send_command(commands::SET_CURRENT, current_ma)?;
            set_mode(protocol, DeviceMode::Remote)?;
        }
    }
    
    Ok(())
}

/// Fire with a specific current value (legacy function for backward compatibility)
pub fn fire_with_current(protocol: &mut ProtocolHandler, current_ma: u16) -> Result<()> {
    fire_with_current_smart(protocol, current_ma, None)
}

/// Get maximum current setting
pub fn get_max_current(protocol: &mut ProtocolHandler) -> Result<u16> {
    Ok(protocol.send_command(commands::STAGE_CURRENTS[4], 0)? as u16)
}
