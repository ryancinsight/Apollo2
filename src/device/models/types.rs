//! Device type definitions for Lumidox II Controller
//!
//! This module defines all data structures used to represent device state,
//! configuration, and information including modes, stages, and device info.

use crate::core::{LumidoxError, Result};
use crate::communication::protocol::commands;

/// Device operating modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceMode {
    /// Local mode (device controlled locally) - 0x0000
    Local = 0,
    /// Standby mode (On, Output Off) - 0x0001
    Standby = 1,
    /// Armed mode (On, Arm) - 0x0002
    Armed = 2,
    /// Remote firing mode (On, Fire) - 0x0003
    Remote = 3,
}

/// Device stage configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stage {
    pub number: u8,
    pub current_ma: u16,
}

impl Stage {
    /// Create a new stage configuration
    pub fn new(number: u8) -> Result<Self> {
        if !(1..=5).contains(&number) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5", number)
            ));
        }
        Ok(Stage { number, current_ma: 0 })
    }
    
    /// Get the command for reading this stage's current
    pub fn current_command(&self) -> &'static [u8] {
        commands::STAGE_CURRENTS[(self.number - 1) as usize]
    }
}

/// Device information structure
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub firmware_version: String,
    pub model_number: String,
    pub serial_number: String,
    pub wavelength: String,
    pub max_current_ma: u16,
}

/// Power measurement data
#[derive(Debug, Clone)]
pub struct PowerInfo {
    pub total_power: f32,
    pub total_units: String,
    pub per_power: f32,
    pub per_units: String,
}
