//! Mode management operations for Lumidox II Controller
//!
//! This module provides functions for managing device operating modes
//! including local, standby, armed, and remote modes.

use crate::core::Result;
use crate::communication::{ProtocolHandler, protocol::commands};
use crate::device::models::DeviceMode;
use std::thread;
use std::time::Duration;

/// Set device operating mode
pub fn set_mode(protocol: &mut ProtocolHandler, mode: DeviceMode) -> Result<()> {
    protocol.send_command(commands::SET_MODE, mode as u16)?;
    Ok(())
}

/// Turn off the device
pub fn turn_off(protocol: &mut ProtocolHandler) -> Result<()> {
    set_mode(protocol, DeviceMode::Standby)?;
    thread::sleep(Duration::from_millis(1000));
    Ok(())
}

/// Shutdown and return to local mode
pub fn shutdown(protocol: &mut ProtocolHandler) -> Result<()> {
    turn_off(protocol)?;
    set_mode(protocol, DeviceMode::Local)?;
    thread::sleep(Duration::from_millis(1000));
    Ok(())
}
