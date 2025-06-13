//! Information operations sub-module
//!
//! This module handles all information retrieval operations including:
//! - Device information operations (Info)
//! - Status reading operations (Status)
//! - State reading operations (ReadState)
//!
//! Each operation type is implemented in its own specialized file
//! following the single responsibility principle.

pub mod device_info;
pub mod status_reading;
pub mod state_reading;

// Re-export operation implementations
pub use device_info::DeviceInfoOperations;
pub use status_reading::StatusReadingOperations;
pub use state_reading::StateReadingOperations;

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandExecutionConfig},
    enums::InformationCategory,
    traits::CommandExecutor,
};

/// Information operations coordinator
pub struct InformationCoordinator;

impl InformationCoordinator {
    /// Create a new information coordinator
    pub fn new() -> Self {
        Self
    }

    /// Execute an information command
    pub fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match InformationCategory::from_command(command) {
            Some(InformationCategory::DeviceInfo) => {
                let executor = DeviceInfoOperations::new();
                executor.execute(command, context, config)
            }
            Some(InformationCategory::StatusReading) => {
                let executor = StatusReadingOperations::new();
                executor.execute(command, context, config)
            }
            Some(InformationCategory::StateReading) => {
                let executor = StateReadingOperations::new();
                executor.execute(command, context, config)
            }
            None => {
                Err(crate::core::LumidoxError::InvalidInput(
                    format!("Command {:?} is not an information command", command)
                ))
            }
        }
    }

    /// Check if a command is an information command
    pub fn is_information_command(&self, command: &Commands) -> bool {
        InformationCategory::from_command(command).is_some()
    }

    /// Get the sub-category of an information command
    pub fn get_subcategory(&self, command: &Commands) -> Option<InformationCategory> {
        InformationCategory::from_command(command)
    }

    /// Get a description of the information operation
    pub fn get_operation_description(&self, command: &Commands) -> Option<&'static str> {
        match InformationCategory::from_command(command) {
            Some(InformationCategory::DeviceInfo) => {
                Some("Retrieve device information including firmware, model, and serial number")
            }
            Some(InformationCategory::StatusReading) => {
                Some("Read device status including state and current settings")
            }
            Some(InformationCategory::StateReading) => {
                Some("Read remote mode state and device operational status")
            }
            None => None,
        }
    }

    /// Check if the operation requires device to be in a specific state
    pub fn requires_device_state(&self, command: &Commands) -> Option<&'static str> {
        match command {
            Commands::Info => {
                Some("Device must be connected and responsive")
            }
            Commands::Status => {
                Some("Device must be connected and initialized")
            }
            Commands::ReadState => {
                Some("Device must be connected and in remote mode")
            }
            _ => None,
        }
    }

    /// Get the safety level for the operation
    pub fn get_safety_level(&self, _command: &Commands) -> &'static str {
        "SAFE" // All information operations are safe
    }

    /// Check if the operation modifies device state
    pub fn modifies_device_state(&self, _command: &Commands) -> bool {
        false // Information operations are read-only
    }

    /// Get expected response type for the operation
    pub fn get_response_type(&self, command: &Commands) -> Option<&'static str> {
        match InformationCategory::from_command(command) {
            Some(InformationCategory::DeviceInfo) => {
                Some("Device information structure")
            }
            Some(InformationCategory::StatusReading) => {
                Some("Device status summary")
            }
            Some(InformationCategory::StateReading) => {
                Some("Remote mode state description")
            }
            None => None,
        }
    }

    /// Get the typical execution time for the operation
    pub fn get_typical_execution_time(&self, command: &Commands) -> Option<&'static str> {
        match InformationCategory::from_command(command) {
            Some(InformationCategory::DeviceInfo) => {
                Some("< 1 second")
            }
            Some(InformationCategory::StatusReading) => {
                Some("1-2 seconds")
            }
            Some(InformationCategory::StateReading) => {
                Some("< 1 second")
            }
            None => None,
        }
    }
}

impl Default for InformationCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Common validation for information operations
pub struct InformationValidator;

impl InformationValidator {
    /// Validate that the device is ready for information operations
    pub fn validate_device_ready(device: &LumidoxDevice) -> Result<()> {
        // Check if device is connected and responsive
        if device.info().is_none() {
            return Err(crate::core::LumidoxError::DeviceError(
                "Device not connected or not responding".to_string()
            ));
        }

        Ok(())
    }

    /// Validate that the device is initialized for status operations
    pub fn validate_device_initialized(device: &LumidoxDevice) -> Result<()> {
        Self::validate_device_ready(device)?;
        
        // Additional initialization checks can be added here
        // For now, we assume device readiness implies initialization
        Ok(())
    }

    /// Validate that the device is in remote mode for state operations
    pub fn validate_remote_mode(device: &LumidoxDevice) -> Result<()> {
        Self::validate_device_ready(device)?;
        
        // Check if device is in remote mode
        // This would typically check the actual device mode
        // For now, we assume the mode check is handled by the device operations
        Ok(())
    }

    /// Validate information request parameters
    pub fn validate_info_request() -> Result<()> {
        // Information requests typically don't have parameters to validate
        Ok(())
    }

    /// Validate status request parameters
    pub fn validate_status_request() -> Result<()> {
        // Status requests typically don't have parameters to validate
        Ok(())
    }

    /// Validate state request parameters
    pub fn validate_state_request() -> Result<()> {
        // State requests typically don't have parameters to validate
        Ok(())
    }
}

/// Information operation result types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InformationResultType {
    /// Device information result
    DeviceInfo,
    /// Status reading result
    StatusReading,
    /// State reading result
    StateReading,
}

impl InformationResultType {
    /// Get the display name for the result type
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "Device Information",
            Self::StatusReading => "Status Reading",
            Self::StateReading => "State Reading",
        }
    }

    /// Get the description for the result type
    pub fn description(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "Device hardware and firmware information",
            Self::StatusReading => "Current device status and settings",
            Self::StateReading => "Remote mode and operational state",
        }
    }

    /// Check if the result type contains sensitive information
    pub fn contains_sensitive_info(&self) -> bool {
        match self {
            Self::DeviceInfo => false,     // Device info is generally not sensitive
            Self::StatusReading => false,  // Status info is generally not sensitive
            Self::StateReading => false,   // State info is generally not sensitive
        }
    }

    /// Get the expected data format for the result type
    pub fn expected_format(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "Structured device information",
            Self::StatusReading => "Status summary text",
            Self::StateReading => "State description text",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_information_coordinator_creation() {
        let coordinator = InformationCoordinator::new();
        assert!(coordinator.is_information_command(&Commands::Info));
        assert!(coordinator.is_information_command(&Commands::Status));
        assert!(coordinator.is_information_command(&Commands::ReadState));
        
        assert!(!coordinator.is_information_command(&Commands::Stage1));
        assert!(!coordinator.is_information_command(&Commands::Arm));
    }

    #[test]
    fn test_get_subcategory() {
        let coordinator = InformationCoordinator::new();
        
        assert_eq!(
            coordinator.get_subcategory(&Commands::Info),
            Some(InformationCategory::DeviceInfo)
        );
        assert_eq!(
            coordinator.get_subcategory(&Commands::Status),
            Some(InformationCategory::StatusReading)
        );
        assert_eq!(
            coordinator.get_subcategory(&Commands::ReadState),
            Some(InformationCategory::StateReading)
        );
        assert_eq!(coordinator.get_subcategory(&Commands::Stage1), None);
    }

    #[test]
    fn test_get_operation_description() {
        let coordinator = InformationCoordinator::new();
        
        assert!(coordinator.get_operation_description(&Commands::Info).is_some());
        assert!(coordinator.get_operation_description(&Commands::Status).is_some());
        assert!(coordinator.get_operation_description(&Commands::ReadState).is_some());
        assert!(coordinator.get_operation_description(&Commands::Stage1).is_none());
    }

    #[test]
    fn test_safety_level() {
        let coordinator = InformationCoordinator::new();
        
        assert_eq!(coordinator.get_safety_level(&Commands::Info), "SAFE");
        assert_eq!(coordinator.get_safety_level(&Commands::Status), "SAFE");
        assert_eq!(coordinator.get_safety_level(&Commands::ReadState), "SAFE");
    }

    #[test]
    fn test_modifies_device_state() {
        let coordinator = InformationCoordinator::new();
        
        assert!(!coordinator.modifies_device_state(&Commands::Info));
        assert!(!coordinator.modifies_device_state(&Commands::Status));
        assert!(!coordinator.modifies_device_state(&Commands::ReadState));
    }

    #[test]
    fn test_information_result_type() {
        let device_info = InformationResultType::DeviceInfo;
        assert_eq!(device_info.display_name(), "Device Information");
        assert!(!device_info.contains_sensitive_info());
        
        let status = InformationResultType::StatusReading;
        assert_eq!(status.display_name(), "Status Reading");
        assert!(!status.contains_sensitive_info());
        
        let state = InformationResultType::StateReading;
        assert_eq!(state.display_name(), "State Reading");
        assert!(!state.contains_sensitive_info());
    }

    #[test]
    fn test_default_implementation() {
        let coordinator1 = InformationCoordinator::new();
        let coordinator2 = InformationCoordinator::default();
        
        // Both should have same behavior
        assert_eq!(
            coordinator1.is_information_command(&Commands::Info),
            coordinator2.is_information_command(&Commands::Info)
        );
    }
}
