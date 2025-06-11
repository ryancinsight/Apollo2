//! Arming operations for Lumidox II Controller
//!
//! This module provides functions specifically for arming the device
//! and managing ARM-related operations.

use crate::core::Result;
use crate::communication::ProtocolHandler;
use crate::device::models::DeviceMode;
use super::modes::set_mode;
use std::thread;
use std::time::Duration;

/// Arm the device (prepare for firing)
pub fn arm_device(protocol: &mut ProtocolHandler) -> Result<()> {
    set_mode(protocol, DeviceMode::Armed)?;
    thread::sleep(Duration::from_millis(100));
    Ok(())
}
