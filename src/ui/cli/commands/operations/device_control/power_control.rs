//! Power control operations for Lumidox II Controller
//!
//! This module handles power state control operations (Arm, Off) with
//! proper validation, execution, and result handling.

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::super::super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandExecutionConfig},
    traits::CommandExecutor,
    errors::CommandError,
};
use super::{DeviceControlValidator, OperationValidator, OperationExecutor};

/// Power control operations handler
pub struct PowerControlOperations;

impl PowerControlOperations {
    /// Create a new power control operations handler
    pub fn new() -> Self {
        Self
    }

    /// Extract power operation type from command
    fn extract_power_operation(&self, command: &Commands) -> Result<PowerOperation> {
        match command {
            Commands::Arm => Ok(PowerOperation::Arm),
            Commands::Off => Ok(PowerOperation::Off),
            _ => Err(CommandError::invalid_parameters(
                command,
                "Not a power control command"
            ).into()),
        }
    }

    /// Validate power control operation
    fn validate_power_control(&self, operation: &PowerOperation, device: &LumidoxDevice) -> Result<()> {
        // Validate device is ready for power operations
        DeviceControlValidator::validate_device_ready(device)?;
        
        // Additional validation based on operation type
        match operation {
            PowerOperation::Arm => {
                // Validate device can be armed
                self.validate_arm_operation(device)?;
            }
            PowerOperation::Off => {
                // Validate device can be turned off
                self.validate_off_operation(device)?;
            }
        }
        
        Ok(())
    }

    /// Validate ARM operation
    fn validate_arm_operation(&self, _device: &LumidoxDevice) -> Result<()> {
        // Check if device is in a state that allows arming
        // For now, we assume the device state check is handled by the device operations
        Ok(())
    }

    /// Validate OFF operation
    fn validate_off_operation(&self, _device: &LumidoxDevice) -> Result<()> {
        // OFF operation is generally always safe
        Ok(())
    }

    /// Execute power control operation
    fn execute_power_operation(
        &self,
        operation: &PowerOperation,
        device: &mut LumidoxDevice,
    ) -> Result<CommandExecutionResult> {
        match operation {
            PowerOperation::Arm => {
                match device.arm() {
                    Ok(()) => {
                        let message = "Device armed successfully".to_string();
                        Ok(CommandExecutionResult::success_with_message(message))
                    }
                    Err(e) => {
                        let message = format!("Failed to arm device: {}", e);
                        Ok(CommandExecutionResult::failure(message))
                    }
                }
            }
            PowerOperation::Off => {
                match device.turn_off() {
                    Ok(()) => {
                        let message = "Device turned off successfully".to_string();
                        Ok(CommandExecutionResult::success_with_message(message))
                    }
                    Err(e) => {
                        let message = format!("Failed to turn off device: {}", e);
                        Ok(CommandExecutionResult::failure(message))
                    }
                }
            }
        }
    }

    /// Display power operation confirmation
    fn display_confirmation(&self, operation: &PowerOperation) -> Result<()> {
        match operation {
            PowerOperation::Arm => println!("Arming device."),
            PowerOperation::Off => println!("Turning off device."),
        }
        Ok(())
    }

    /// Get operation description for user feedback
    fn get_operation_description(&self, operation: &PowerOperation) -> &'static str {
        match operation {
            PowerOperation::Arm => "Arm the device for firing operations",
            PowerOperation::Off => "Turn off the device and disable all outputs",
        }
    }

    /// Get safety level for the operation
    fn get_safety_level(&self, operation: &PowerOperation) -> PowerSafetyLevel {
        match operation {
            PowerOperation::Arm => PowerSafetyLevel::Medium,
            PowerOperation::Off => PowerSafetyLevel::Low,
        }
    }

    /// Check if operation requires confirmation
    fn requires_confirmation(&self, operation: &PowerOperation) -> bool {
        match operation {
            PowerOperation::Arm => true,  // Arming requires confirmation
            PowerOperation::Off => false, // Turning off is generally safe
        }
    }

    /// Get expected device state after operation
    fn get_expected_state(&self, operation: &PowerOperation) -> &'static str {
        match operation {
            PowerOperation::Arm => "Armed and ready for firing",
            PowerOperation::Off => "Off and safe",
        }
    }
}

impl Default for PowerControlOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor for PowerControlOperations {
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        // Extract power operation
        let operation = self.extract_power_operation(command)?;
        
        // Display confirmation
        self.display_confirmation(&operation)?;
        
        // Validate operation
        self.validate_power_control(&operation, context.device())?;
        
        // Execute the power operation
        self.execute_power_operation(&operation, context.device_mut())
    }

    fn can_handle(&self, command: &Commands) -> bool {
        matches!(command, Commands::Arm | Commands::Off)
    }
}

impl OperationValidator for PowerControlOperations {
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()> {
        let operation = self.extract_power_operation(command)?;
        self.validate_power_control(&operation, device)
    }

    fn requires_device_connection(&self, _command: &Commands) -> bool {
        true
    }

    fn is_safe_operation(&self, command: &Commands) -> bool {
        if let Ok(operation) = self.extract_power_operation(command) {
            matches!(self.get_safety_level(&operation), PowerSafetyLevel::Low)
        } else {
            false
        }
    }
}

impl OperationExecutor for PowerControlOperations {
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        let operation = self.extract_power_operation(command)?;
        self.execute_power_operation(&operation, device)
    }

    fn operation_name(&self) -> &'static str {
        "Power Control"
    }

    fn operation_description(&self) -> &'static str {
        "Control device power state (arm/off)"
    }
}

/// Power operation types
#[derive(Debug, Clone, PartialEq, Eq)]
enum PowerOperation {
    /// Arm the device for firing
    Arm,
    /// Turn off the device
    Off,
}

/// Safety levels for power operations
#[derive(Debug, Clone, PartialEq, Eq)]
enum PowerSafetyLevel {
    /// Low risk operation
    Low,
    /// Medium risk operation
    Medium,
    /// High risk operation
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_power_operation() {
        let ops = PowerControlOperations::new();
        
        assert_eq!(ops.extract_power_operation(&Commands::Arm).unwrap(), PowerOperation::Arm);
        assert_eq!(ops.extract_power_operation(&Commands::Off).unwrap(), PowerOperation::Off);
        
        // Should fail for non-power commands
        assert!(ops.extract_power_operation(&Commands::Stage1).is_err());
        assert!(ops.extract_power_operation(&Commands::Current { value: 100 }).is_err());
    }

    #[test]
    fn test_can_handle() {
        let ops = PowerControlOperations::new();
        
        assert!(ops.can_handle(&Commands::Arm));
        assert!(ops.can_handle(&Commands::Off));
        
        assert!(!ops.can_handle(&Commands::Stage1));
        assert!(!ops.can_handle(&Commands::Current { value: 100 }));
        assert!(!ops.can_handle(&Commands::Info));
    }

    #[test]
    fn test_requires_device_connection() {
        let ops = PowerControlOperations::new();
        assert!(ops.requires_device_connection(&Commands::Arm));
        assert!(ops.requires_device_connection(&Commands::Off));
    }

    #[test]
    fn test_is_safe_operation() {
        let ops = PowerControlOperations::new();
        
        // OFF is considered safe
        assert!(ops.is_safe_operation(&Commands::Off));
        
        // ARM is considered medium risk (not safe)
        assert!(!ops.is_safe_operation(&Commands::Arm));
    }

    #[test]
    fn test_operation_metadata() {
        let ops = PowerControlOperations::new();
        assert_eq!(ops.operation_name(), "Power Control");
        assert!(!ops.operation_description().is_empty());
    }

    #[test]
    fn test_get_operation_description() {
        let ops = PowerControlOperations::new();
        
        let arm_desc = ops.get_operation_description(&PowerOperation::Arm);
        assert!(arm_desc.contains("Arm"));
        assert!(arm_desc.contains("firing"));
        
        let off_desc = ops.get_operation_description(&PowerOperation::Off);
        assert!(off_desc.contains("Turn off"));
        assert!(off_desc.contains("disable"));
    }

    #[test]
    fn test_get_safety_level() {
        let ops = PowerControlOperations::new();
        
        assert_eq!(ops.get_safety_level(&PowerOperation::Arm), PowerSafetyLevel::Medium);
        assert_eq!(ops.get_safety_level(&PowerOperation::Off), PowerSafetyLevel::Low);
    }

    #[test]
    fn test_requires_confirmation() {
        let ops = PowerControlOperations::new();
        
        assert!(ops.requires_confirmation(&PowerOperation::Arm));
        assert!(!ops.requires_confirmation(&PowerOperation::Off));
    }

    #[test]
    fn test_get_expected_state() {
        let ops = PowerControlOperations::new();
        
        let arm_state = ops.get_expected_state(&PowerOperation::Arm);
        assert!(arm_state.contains("Armed"));
        assert!(arm_state.contains("ready"));
        
        let off_state = ops.get_expected_state(&PowerOperation::Off);
        assert!(off_state.contains("Off"));
        assert!(off_state.contains("safe"));
    }

    #[test]
    fn test_default_implementation() {
        let ops1 = PowerControlOperations::new();
        let ops2 = PowerControlOperations::default();
        
        // Both should have same behavior
        assert_eq!(ops1.operation_name(), ops2.operation_name());
        assert_eq!(ops1.operation_description(), ops2.operation_description());
    }

    #[test]
    fn test_power_operation_equality() {
        assert_eq!(PowerOperation::Arm, PowerOperation::Arm);
        assert_eq!(PowerOperation::Off, PowerOperation::Off);
        assert_ne!(PowerOperation::Arm, PowerOperation::Off);
    }

    #[test]
    fn test_power_safety_level_equality() {
        assert_eq!(PowerSafetyLevel::Low, PowerSafetyLevel::Low);
        assert_eq!(PowerSafetyLevel::Medium, PowerSafetyLevel::Medium);
        assert_eq!(PowerSafetyLevel::High, PowerSafetyLevel::High);
        assert_ne!(PowerSafetyLevel::Low, PowerSafetyLevel::Medium);
    }
}
