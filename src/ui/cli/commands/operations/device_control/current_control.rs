//! Current control operations for Lumidox II Controller
//!
//! This module handles custom current firing operations with
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

/// Current control operations handler
pub struct CurrentControlOperations;

impl CurrentControlOperations {
    /// Create a new current control operations handler
    pub fn new() -> Self {
        Self
    }

    /// Extract current value from command
    fn extract_current_value(&self, command: &Commands) -> Result<u16> {
        match command {
            Commands::Current { value } => Ok(*value),
            _ => Err(CommandError::invalid_parameters(
                command,
                "Not a current control command"
            ).into()),
        }
    }

    /// Validate current control operation
    fn validate_current_control(&self, current: u16, device: &LumidoxDevice) -> Result<()> {
        // Validate current value
        DeviceControlValidator::validate_current_value(current)?;
        
        // Validate device is ready for firing
        DeviceControlValidator::validate_device_armed(device)?;
        
        Ok(())
    }

    /// Execute current control firing operation
    fn execute_current_firing(
        &self,
        current: u16,
        device: &mut LumidoxDevice,
    ) -> Result<CommandExecutionResult> {
        // Perform the actual current firing
        match device.fire_with_current(current) {
            Ok(()) => {
                let message = format!("Fired with {}mA successfully", current);
                Ok(CommandExecutionResult::success_with_message(message))
            }
            Err(e) => {
                let message = format!("Failed to fire with {}mA: {}", current, e);
                Ok(CommandExecutionResult::failure(message))
            }
        }
    }

    /// Display current firing confirmation
    fn display_confirmation(&self, current: u16) -> Result<()> {
        println!("Firing with {}mA.", current);
        Ok(())
    }

    /// Get current range description for user feedback
    fn get_current_range_description(&self) -> &'static str {
        "Current range: 1-5000mA (recommended: 100-2000mA for most treatments)"
    }

    /// Validate current is within safe operating range
    fn validate_safe_current_range(&self, current: u16) -> Result<()> {
        if current > 3000 {
            println!("Warning: High current value ({}mA). Ensure proper safety protocols.", current);
        }
        
        if current < 50 {
            println!("Warning: Low current value ({}mA). May not be effective for treatment.", current);
        }
        
        Ok(())
    }

    /// Get recommended current for different treatment types
    fn get_recommended_current(&self, treatment_type: &str) -> Option<u16> {
        match treatment_type.to_lowercase().as_str() {
            "light" => Some(500),
            "medium" => Some(1000),
            "intensive" => Some(1500),
            "maximum" => Some(2000),
            _ => None,
        }
    }

    /// Check if current value requires additional confirmation
    fn requires_additional_confirmation(&self, current: u16) -> bool {
        current > 2500 // High current values require extra confirmation
    }
}

impl Default for CurrentControlOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor for CurrentControlOperations {
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        // Extract current value
        let current = self.extract_current_value(command)?;
        
        // Display confirmation
        self.display_confirmation(current)?;
        
        // Validate safe current range
        self.validate_safe_current_range(current)?;
        
        // Validate operation
        self.validate_current_control(current, context.device())?;
        
        // Execute the firing operation
        self.execute_current_firing(current, context.device_mut())
    }

    fn can_handle(&self, command: &Commands) -> bool {
        matches!(command, Commands::Current { .. })
    }
}

impl OperationValidator for CurrentControlOperations {
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()> {
        let current = self.extract_current_value(command)?;
        self.validate_current_control(current, device)
    }

    fn requires_device_connection(&self, _command: &Commands) -> bool {
        true
    }

    fn is_safe_operation(&self, command: &Commands) -> bool {
        // Check if current value is within safe range
        if let Ok(current) = self.extract_current_value(command) {
            current <= 2000 // Consider <= 2000mA as safe
        } else {
            false
        }
    }
}

impl OperationExecutor for CurrentControlOperations {
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        let current = self.extract_current_value(command)?;
        self.execute_current_firing(current, device)
    }

    fn operation_name(&self) -> &'static str {
        "Current Control"
    }

    fn operation_description(&self) -> &'static str {
        "Fire with custom current setting in milliamps"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_current_value() {
        let ops = CurrentControlOperations::new();
        
        assert_eq!(ops.extract_current_value(&Commands::Current { value: 100 }).unwrap(), 100);
        assert_eq!(ops.extract_current_value(&Commands::Current { value: 1500 }).unwrap(), 1500);
        assert_eq!(ops.extract_current_value(&Commands::Current { value: 2000 }).unwrap(), 2000);
        
        // Should fail for non-current commands
        assert!(ops.extract_current_value(&Commands::Stage1).is_err());
        assert!(ops.extract_current_value(&Commands::Arm).is_err());
    }

    #[test]
    fn test_can_handle() {
        let ops = CurrentControlOperations::new();
        
        assert!(ops.can_handle(&Commands::Current { value: 100 }));
        assert!(ops.can_handle(&Commands::Current { value: 2000 }));
        
        assert!(!ops.can_handle(&Commands::Stage1));
        assert!(!ops.can_handle(&Commands::Arm));
        assert!(!ops.can_handle(&Commands::Off));
    }

    #[test]
    fn test_requires_device_connection() {
        let ops = CurrentControlOperations::new();
        assert!(ops.requires_device_connection(&Commands::Current { value: 100 }));
    }

    #[test]
    fn test_is_safe_operation() {
        let ops = CurrentControlOperations::new();
        
        // Safe current values
        assert!(ops.is_safe_operation(&Commands::Current { value: 100 }));
        assert!(ops.is_safe_operation(&Commands::Current { value: 1000 }));
        assert!(ops.is_safe_operation(&Commands::Current { value: 2000 }));
        
        // Unsafe current values
        assert!(!ops.is_safe_operation(&Commands::Current { value: 2500 }));
        assert!(!ops.is_safe_operation(&Commands::Current { value: 3000 }));
        assert!(!ops.is_safe_operation(&Commands::Current { value: 5000 }));
    }

    #[test]
    fn test_operation_metadata() {
        let ops = CurrentControlOperations::new();
        assert_eq!(ops.operation_name(), "Current Control");
        assert!(!ops.operation_description().is_empty());
    }

    #[test]
    fn test_get_current_range_description() {
        let ops = CurrentControlOperations::new();
        let description = ops.get_current_range_description();
        assert!(description.contains("1-5000mA"));
        assert!(description.contains("100-2000mA"));
    }

    #[test]
    fn test_get_recommended_current() {
        let ops = CurrentControlOperations::new();
        
        assert_eq!(ops.get_recommended_current("light"), Some(500));
        assert_eq!(ops.get_recommended_current("medium"), Some(1000));
        assert_eq!(ops.get_recommended_current("intensive"), Some(1500));
        assert_eq!(ops.get_recommended_current("maximum"), Some(2000));
        assert_eq!(ops.get_recommended_current("unknown"), None);
        
        // Test case insensitive
        assert_eq!(ops.get_recommended_current("LIGHT"), Some(500));
        assert_eq!(ops.get_recommended_current("Medium"), Some(1000));
    }

    #[test]
    fn test_requires_additional_confirmation() {
        let ops = CurrentControlOperations::new();
        
        // Normal current values don't require additional confirmation
        assert!(!ops.requires_additional_confirmation(100));
        assert!(!ops.requires_additional_confirmation(1000));
        assert!(!ops.requires_additional_confirmation(2000));
        assert!(!ops.requires_additional_confirmation(2500));
        
        // High current values require additional confirmation
        assert!(ops.requires_additional_confirmation(2501));
        assert!(ops.requires_additional_confirmation(3000));
        assert!(ops.requires_additional_confirmation(5000));
    }

    #[test]
    fn test_default_implementation() {
        let ops1 = CurrentControlOperations::new();
        let ops2 = CurrentControlOperations::default();
        
        // Both should have same behavior
        assert_eq!(ops1.operation_name(), ops2.operation_name());
        assert_eq!(ops1.operation_description(), ops2.operation_description());
    }
}
