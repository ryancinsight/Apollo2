//! Device controller creation for Lumidox II Controller CLI
//!
//! This module handles device controller creation and initialization
//! for CLI operations.

use crate::core::{LumidoxError, Result};
use crate::communication::{ProtocolHandler, protocol::constants};
use crate::device::LumidoxDevice;

/// Create a new device controller from a port name
pub fn create_device_controller(port_name: &str) -> Result<LumidoxDevice> {
    create_device_controller_with_optimization(port_name, true)
}

/// Create a new device controller from a port name with specified optimization setting
pub fn create_device_controller_with_optimization(port_name: &str, optimize_transitions: bool) -> Result<LumidoxDevice> {
    let port = serialport::new(port_name, constants::DEFAULT_BAUD_RATE)
        .timeout(constants::DEFAULT_TIMEOUT)
        .open()
        .map_err(LumidoxError::SerialError)?;

    let protocol = ProtocolHandler::new(port)?;
    let mut device = LumidoxDevice::new_with_optimization(protocol, optimize_transitions);
    device.initialize()?;

    Ok(device)
}
