//! Device controller for Lumidox II Controller
//!
//! This module provides the main LumidoxDevice controller that orchestrates
//! device operations using the sub-modules for models, operations, and info.

use crate::core::Result;
use crate::communication::ProtocolHandler;
use crate::device::models::{DeviceMode, DeviceInfo};
use crate::device::operations::{control, power};
use crate::device::info;
use std::thread;
use std::time::Duration;

/// High-level device controller
pub struct LumidoxDevice {
    protocol: ProtocolHandler,
    info: Option<DeviceInfo>,
    current_mode: Option<DeviceMode>,
    /// Whether to use optimized stage transitions (true) or always use full safety sequence (false)
    optimize_transitions: bool,
}

impl LumidoxDevice {
    /// Create a new device controller with optimized transitions enabled by default
    pub fn new(protocol: ProtocolHandler) -> Self {
        LumidoxDevice {
            protocol,
            info: None,
            current_mode: None,
            optimize_transitions: true, // Enable optimized transitions by default
        }
    }

    /// Create a new device controller with specified transition optimization setting
    pub fn new_with_optimization(protocol: ProtocolHandler, optimize_transitions: bool) -> Self {
        LumidoxDevice {
            protocol,
            info: None,
            current_mode: None,
            optimize_transitions,
        }
    }

    /// Enable or disable optimized stage transitions
    pub fn set_optimize_transitions(&mut self, optimize: bool) {
        self.optimize_transitions = optimize;
    }

    /// Check if optimized transitions are enabled
    pub fn is_optimize_transitions(&self) -> bool {
        self.optimize_transitions
    }

    /// Initialize the device and retrieve basic information
    pub fn initialize(&mut self) -> Result<()> {
        // Set to standby mode first
        self.set_mode(DeviceMode::Standby)?;
        thread::sleep(Duration::from_millis(100));

        // Retrieve device information
        let device_info = info::read_device_info(&mut self.protocol)?;
        self.info = Some(device_info);

        Ok(())
    }

    /// Get device information (cached after initialization)
    pub fn info(&self) -> Option<&DeviceInfo> {
        self.info.as_ref()
    }

    /// Set device operating mode
    pub fn set_mode(&mut self, mode: DeviceMode) -> Result<()> {
        control::set_mode(&mut self.protocol, mode)?;
        self.current_mode = Some(mode);
        Ok(())
    }

    /// Get current device mode
    pub fn current_mode(&self) -> Option<DeviceMode> {
        self.current_mode
    }

    /// Arm the device (prepare for firing)
    pub fn arm(&mut self) -> Result<()> {
        control::arm_device(&mut self.protocol)?;
        self.current_mode = Some(DeviceMode::Armed);
        Ok(())
    }
    
    /// Fire a specific stage
    pub fn fire_stage(&mut self, stage_num: u8) -> Result<()> {
        if self.optimize_transitions {
            control::fire_stage_smart(&mut self.protocol, stage_num, self.current_mode)?;
        } else {
            control::fire_stage(&mut self.protocol, stage_num)?;
        }
        // Update current mode after firing
        self.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }

    /// Fire with a specific current value
    pub fn fire_with_current(&mut self, current_ma: u16) -> Result<()> {
        if self.optimize_transitions {
            control::fire_with_current_smart(&mut self.protocol, current_ma, self.current_mode)?;
        } else {
            control::fire_with_current(&mut self.protocol, current_ma)?;
        }
        // Update current mode after firing
        self.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }

    /// Turn off the device
    pub fn turn_off(&mut self) -> Result<()> {
        control::turn_off(&mut self.protocol)?;
        self.current_mode = Some(DeviceMode::Standby);
        Ok(())
    }

    /// Shutdown and return to local mode
    pub fn shutdown(&mut self) -> Result<()> {
        control::shutdown(&mut self.protocol)?;
        self.current_mode = Some(DeviceMode::Local);
        Ok(())
    }

    /// Get maximum current setting
    pub fn get_max_current(&mut self) -> Result<u16> {
        control::get_max_current(&mut self.protocol)
    }
    
    /// Get power information for a specific stage
    pub fn get_power_info(&mut self, stage_num: u8) -> Result<crate::device::models::PowerInfo> {
        power::get_power_info(&mut self.protocol, stage_num)
    }
}