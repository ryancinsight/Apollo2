//! Stage firing operations for Lumidox II Controller
//!
//! This module handles stage firing operations (Stage1-5) with
//! proper validation, execution, and result handling.

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::super::super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandResultData},
    traits::CommandExecutor,
    errors::CommandError,
};
use super::{DeviceControlValidator, OperationValidator, OperationExecutor};

/// Stage firing operations handler
pub struct StageFiringOperations;

impl StageFiringOperations {
    /// Create a new stage firing operations handler
    pub fn new() -> Self {
        Self
    }

    /// Extract stage number from command
    fn extract_stage_number(&self, command: &Commands) -> Result<u8> {
        match command {
            Commands::Stage1 => Ok(1),
            Commands::Stage2 => Ok(2),
            Commands::Stage3 => Ok(3),
            Commands::Stage4 => Ok(4),
            Commands::Stage5 => Ok(5),
            _ => Err(CommandError::invalid_parameters(
                command,
                "Not a stage firing command"
            ).into()),
        }
    }

    /// Validate stage firing operation
    fn validate_stage_firing(&self, stage: u8, device: &LumidoxDevice) -> Result<()> {
        // Validate stage number
        DeviceControlValidator::validate_stage_number(stage)?;
        
        // Validate device is ready for firing
        DeviceControlValidator::validate_device_armed(device)?;
        
        Ok(())
    }

    /// Execute stage firing operation
    fn execute_stage_firing(
        &self,
        stage: u8,
        device: &mut LumidoxDevice,
    ) -> Result<CommandExecutionResult> {
        // Perform the actual stage firing
        match device.fire_stage(stage) {
            Ok(()) => {
                let message = format!("Stage {} fired successfully", stage);
                Ok(CommandExecutionResult::success_with_message(message))
            }
            Err(e) => {
                let message = format!("Failed to fire stage {}: {}", stage, e);
                Ok(CommandExecutionResult::failure(message))
            }
        }
    }

    /// Display stage firing confirmation
    fn display_confirmation(&self, stage: u8) -> Result<()> {
        println!("Firing stage {}.", stage);
        Ok(())
    }

    /// Get stage description for user feedback
    fn get_stage_description(&self, stage: u8) -> &'static str {
        match stage {
            1 => "Stage 1 - Initial treatment phase",
            2 => "Stage 2 - Secondary treatment phase", 
            3 => "Stage 3 - Intermediate treatment phase",
            4 => "Stage 4 - Advanced treatment phase",
            5 => "Stage 5 - Final treatment phase",
            _ => "Unknown stage",
        }
    }
}

impl Default for StageFiringOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor for StageFiringOperations {
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        _config: &super::super::super::types::CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        // Extract stage number
        let stage = self.extract_stage_number(command)?;
        
        // Display confirmation
        self.display_confirmation(stage)?;
        
        // Validate operation
        self.validate_stage_firing(stage, context.device())?;
        
        // Execute the firing operation
        self.execute_stage_firing(stage, context.device_mut())
    }

    fn can_handle(&self, command: &Commands) -> bool {
        matches!(
            command,
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5
        )
    }
}

impl OperationValidator for StageFiringOperations {
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()> {
        let stage = self.extract_stage_number(command)?;
        self.validate_stage_firing(stage, device)
    }

    fn requires_device_connection(&self, _command: &Commands) -> bool {
        true
    }

    fn is_safe_operation(&self, _command: &Commands) -> bool {
        false // Stage firing is considered high-risk
    }
}

impl OperationExecutor for StageFiringOperations {
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        _config: &super::super::super::types::CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        let stage = self.extract_stage_number(command)?;
        self.execute_stage_firing(stage, device)
    }

    fn operation_name(&self) -> &'static str {
        "Stage Firing"
    }

    fn operation_description(&self) -> &'static str {
        "Fire a specific stage with predefined parameters"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_stage_number() {
        let ops = StageFiringOperations::new();
        
        assert_eq!(ops.extract_stage_number(&Commands::Stage1).unwrap(), 1);
        assert_eq!(ops.extract_stage_number(&Commands::Stage2).unwrap(), 2);
        assert_eq!(ops.extract_stage_number(&Commands::Stage3).unwrap(), 3);
        assert_eq!(ops.extract_stage_number(&Commands::Stage4).unwrap(), 4);
        assert_eq!(ops.extract_stage_number(&Commands::Stage5).unwrap(), 5);
    }

    #[test]
    fn test_can_handle() {
        let ops = StageFiringOperations::new();
        
        assert!(ops.can_handle(&Commands::Stage1));
        assert!(ops.can_handle(&Commands::Stage2));
        assert!(ops.can_handle(&Commands::Stage3));
        assert!(ops.can_handle(&Commands::Stage4));
        assert!(ops.can_handle(&Commands::Stage5));
        
        assert!(!ops.can_handle(&Commands::Current { value: 100 }));
        assert!(!ops.can_handle(&Commands::Arm));
        assert!(!ops.can_handle(&Commands::Off));
    }

    #[test]
    fn test_requires_device_connection() {
        let ops = StageFiringOperations::new();
        assert!(ops.requires_device_connection(&Commands::Stage1));
    }

    #[test]
    fn test_is_safe_operation() {
        let ops = StageFiringOperations::new();
        assert!(!ops.is_safe_operation(&Commands::Stage1)); // High-risk operation
    }

    #[test]
    fn test_operation_metadata() {
        let ops = StageFiringOperations::new();
        assert_eq!(ops.operation_name(), "Stage Firing");
        assert!(!ops.operation_description().is_empty());
    }

    #[test]
    fn test_get_stage_description() {
        let ops = StageFiringOperations::new();
        
        assert!(ops.get_stage_description(1).contains("Stage 1"));
        assert!(ops.get_stage_description(2).contains("Stage 2"));
        assert!(ops.get_stage_description(3).contains("Stage 3"));
        assert!(ops.get_stage_description(4).contains("Stage 4"));
        assert!(ops.get_stage_description(5).contains("Stage 5"));
        assert_eq!(ops.get_stage_description(99), "Unknown stage");
    }
}
