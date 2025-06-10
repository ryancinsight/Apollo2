//! Device information reader for Lumidox II Controller
//!
//! This module provides functions for reading device information
//! including firmware version, model details, and device specifications.

use crate::core::Result;
use crate::communication::{ProtocolHandler, protocol::{commands, utils}};
use crate::device::models::DeviceInfo;
use crate::device::operations::control::get_max_current;

/// Read all device information
pub fn read_device_info(protocol: &mut ProtocolHandler) -> Result<DeviceInfo> {
    let firmware_version = format!("1.{}", 
        protocol.send_command(commands::FIRMWARE_VERSION, 0)?);
    
    let model_number = utils::read_string_data(
        protocol, 
        &commands::MODEL_COMMANDS
    )?;
    
    let serial_number = utils::read_string_data(
        protocol, 
        &commands::SERIAL_COMMANDS
    )?;
    
    let wavelength = utils::read_string_data(
        protocol, 
        &commands::WAVELENGTH_COMMANDS
    )?;
    
    let max_current_ma = get_max_current(protocol)?;
    
    Ok(DeviceInfo {
        firmware_version,
        model_number,
        serial_number,
        wavelength,
        max_current_ma,
    })
}
