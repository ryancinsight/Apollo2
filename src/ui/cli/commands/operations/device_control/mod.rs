//! Device control operations sub-module
//!
//! This module handles all device control operations including:
//! - Stage firing operations (Stage1-5)
//! - Custom current control operations
//! - Power state control operations (Arm, Off)
//!
//! Each operation type is implemented in its own specialized file
//! following the single responsibility principle.

pub mod stage_firing;
pub mod current_control;
pub mod power_control;

// Re-export operation implementations
pub use stage_firing::StageFiringOperations;
pub use current_control::CurrentControlOperations;
pub use power_control::PowerControlOperations;

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandExecutionConfig},
    enums::DeviceControlCategory,
    traits::CommandExecutor,
};

/// Device control operations coordinator
pub struct DeviceControlCoordinator;

impl DeviceControlCoordinator {
    /// Create a new device control coordinator
    pub fn new() -> Self {
        Self
    }

    /// Execute a device control command
    pub fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match DeviceControlCategory::from_command(command) {
            Some(DeviceControlCategory::StageFiring) => {
                let executor = StageFiringOperations::new();
                executor.execute(command, context, config)
            }
            Some(DeviceControlCategory::CurrentControl) => {
                let executor = CurrentControlOperations::new();
                executor.execute(command, context, config)
            }
            Some(DeviceControlCategory::PowerControl) => {
                let executor = PowerControlOperations::new();
                executor.execute(command, context, config)
            }
            None => {
                Err(crate::core::LumidoxError::InvalidInput(
                    format!("Command {:?} is not a device control command", command)
                ))
            }
        }
    }

    /// Check if a command is a device control command
    pub fn is_device_control_command(&self, command: &Commands) -> bool {
        DeviceControlCategory::from_command(command).is_some()
    }

    /// Get the sub-category of a device control command
    pub fn get_subcategory(&self, command: &Commands) -> Option<DeviceControlCategory> {
        DeviceControlCategory::from_command(command)
    }

    /// Get a description of the device control operation
    pub fn get_operation_description(&self, command: &Commands) -> Option<&'static str> {
        match DeviceControlCategory::from_command(command) {
            Some(DeviceControlCategory::StageFiring) => {
                Some("Fire a specific stage with predefined parameters")
            }
            Some(DeviceControlCategory::CurrentControl) => {
                Some("Fire with custom current setting")
            }
            Some(DeviceControlCategory::PowerControl) => {
                match command {
                    Commands::Arm => Some("Arm the device for firing"),
                    Commands::Off => Some("Turn off the device"),
                    _ => None,
                }
            }
            None => None,
        }
    }

    /// Check if the operation requires device to be in a specific state
    pub fn requires_device_state(&self, command: &Commands) -> Option<&'static str> {
        match command {
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 | Commands::Current { .. } => {
                Some("Device must be armed and ready")
            }
            Commands::Arm => {
                Some("Device must be connected and responsive")
            }
            Commands::Off => {
                Some("Device must be connected")
            }
            _ => None,
        }
    }

    /// Get the safety level for the operation
    pub fn get_safety_level(&self, command: &Commands) -> &'static str {
        match command {
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 | Commands::Current { .. } => {
                "HIGH_RISK"
            }
            Commands::Arm => {
                "MEDIUM_RISK"
            }
            Commands::Off => {
                "LOW_RISK"
            }
            _ => "UNKNOWN",
        }
    }
}

impl Default for DeviceControlCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Common validation for device control operations
pub struct DeviceControlValidator;

impl DeviceControlValidator {
    /// Validate that the device is ready for control operations
    pub fn validate_device_ready(device: &LumidoxDevice) -> Result<()> {
        // Check if device is connected and responsive
        if device.info().is_none() {
            return Err(crate::core::LumidoxError::DeviceError(
                "Device not connected or not responding".to_string()
            ));
        }

        // Additional device state validation can be added here
        Ok(())
    }

    /// Validate that the device is armed for firing operations
    pub fn validate_device_armed(device: &LumidoxDevice) -> Result<()> {
        Self::validate_device_ready(device)?;
        
        // Check device state for firing readiness
        // This would typically check the actual device state
        // For now, we assume the device state check is handled by the device operations
        Ok(())
    }

    /// Validate stage number for stage firing operations
    pub fn validate_stage_number(stage: u8) -> Result<()> {
        if !(1..=5).contains(&stage) {
            return Err(crate::core::LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be between 1 and 5", stage)
            ));
        }
        Ok(())
    }

    /// Validate current value for current control operations
    pub fn validate_current_value(current: u16) -> Result<()> {
        if current == 0 {
            return Err(crate::core::LumidoxError::InvalidInput(
                "Current value must be greater than 0".to_string()
            ));
        }
        
        // Add maximum current validation if needed
        // This would typically be based on device specifications
        if current > 5000 {
            return Err(crate::core::LumidoxError::InvalidInput(
                format!("Current value {} exceeds maximum allowed (5000mA)", current)
            ));
        }
        
        Ok(())
    }
}
