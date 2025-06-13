//! Command execution traits for Lumidox II Controller CLI
//!
//! This module defines traits that provide common interfaces for
//! command execution, validation, and result handling.

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandExecutionConfig},
    enums::{CommandCategory, CommandPriority, CommandSafetyLevel, CommandRequirement},
};

/// Trait for executing commands with a device context
pub trait CommandExecutor {
    /// Execute a command with the given context and configuration
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult>;

    /// Check if the executor can handle the given command
    fn can_handle(&self, command: &Commands) -> bool;

    /// Get the priority level for command execution
    fn get_priority(&self, command: &Commands) -> CommandPriority {
        CommandPriority::from_command(command)
    }

    /// Get the safety level for the command
    fn get_safety_level(&self, command: &Commands) -> CommandSafetyLevel {
        CommandSafetyLevel::from_command(command)
    }
}

/// Trait for validating commands before execution
pub trait CommandValidator {
    /// Validate that a command can be executed with the current context
    fn validate(
        &self,
        command: &Commands,
        context: &CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<()>;

    /// Check if the command requires user confirmation
    fn requires_confirmation(&self, command: &Commands) -> bool {
        CommandSafetyLevel::from_command(command).requires_confirmation()
    }

    /// Get all requirements for the command
    fn get_requirements(&self, command: &Commands) -> Vec<CommandRequirement> {
        CommandRequirement::from_command(command)
    }

    /// Validate device state for the command
    fn validate_device_state(&self, command: &Commands, device: &LumidoxDevice) -> Result<()>;

    /// Validate command parameters
    fn validate_parameters(&self, command: &Commands) -> Result<()>;
}

/// Trait for handling command results and output
pub trait CommandResultHandler {
    /// Handle the result of command execution
    fn handle_result(&self, result: &CommandExecutionResult) -> Result<()>;

    /// Format the result for display
    fn format_result(&self, result: &CommandExecutionResult) -> String;

    /// Check if the result indicates success
    fn is_success(&self, result: &CommandExecutionResult) -> bool {
        result.success
    }

    /// Check if execution should continue after this result
    fn should_continue(&self, result: &CommandExecutionResult) -> bool {
        result.should_continue
    }
}

/// Trait for categorizing commands
pub trait CommandCategorizer {
    /// Get the primary category of a command
    fn get_category(&self, command: &Commands) -> CommandCategory {
        CommandCategory::from_command(command)
    }

    /// Get a human-readable description of the command
    fn get_description(&self, command: &Commands) -> &'static str;

    /// Check if the command is destructive (changes device state)
    fn is_destructive(&self, command: &Commands) -> bool {
        matches!(
            CommandSafetyLevel::from_command(command),
            CommandSafetyLevel::MediumRisk | CommandSafetyLevel::HighRisk
        )
    }

    /// Check if the command requires device connection
    fn requires_device(&self, command: &Commands) -> bool {
        CommandRequirement::from_command(command)
            .contains(&CommandRequirement::DeviceConnection)
    }
}

/// Trait for command execution logging and monitoring
pub trait CommandLogger {
    /// Log the start of command execution
    fn log_start(&self, command: &Commands, context: &CommandExecutionContext);

    /// Log the completion of command execution
    fn log_completion(&self, command: &Commands, result: &CommandExecutionResult);

    /// Log an error during command execution
    fn log_error(&self, command: &Commands, error: &str);

    /// Log validation failures
    fn log_validation_failure(&self, command: &Commands, reason: &str);
}

/// Trait for command execution metrics and performance monitoring
pub trait CommandMetrics {
    /// Record the execution time for a command
    fn record_execution_time(&self, command: &Commands, duration_ms: u64);

    /// Record command success/failure
    fn record_result(&self, command: &Commands, success: bool);

    /// Get execution statistics for a command type
    fn get_stats(&self, command: &Commands) -> CommandStats;

    /// Reset metrics for a command type
    fn reset_stats(&self, command: &Commands);
}

/// Statistics for command execution
#[derive(Debug, Clone)]
pub struct CommandStats {
    /// Total number of executions
    pub total_executions: u64,
    /// Number of successful executions
    pub successful_executions: u64,
    /// Number of failed executions
    pub failed_executions: u64,
    /// Average execution time in milliseconds
    pub average_execution_time_ms: f64,
    /// Minimum execution time in milliseconds
    pub min_execution_time_ms: u64,
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
}

impl Default for CommandStats {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_execution_time_ms: 0.0,
            min_execution_time_ms: u64::MAX,
            max_execution_time_ms: 0,
        }
    }
}

impl CommandStats {
    /// Calculate the success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            (self.successful_executions as f64 / self.total_executions as f64) * 100.0
        }
    }

    /// Calculate the failure rate as a percentage
    pub fn failure_rate(&self) -> f64 {
        100.0 - self.success_rate()
    }

    /// Check if the command has been executed
    pub fn has_executions(&self) -> bool {
        self.total_executions > 0
    }

    /// Get a summary string of the statistics
    pub fn summary(&self) -> String {
        format!(
            "Executions: {}, Success Rate: {:.1}%, Avg Time: {:.1}ms",
            self.total_executions,
            self.success_rate(),
            self.average_execution_time_ms
        )
    }
}

/// Trait for command execution context management
pub trait CommandContextManager {
    /// Prepare the execution context for a command
    fn prepare_context(
        &self,
        command: &Commands,
        device: LumidoxDevice,
        port_name: String,
        optimize_transitions: bool,
    ) -> Result<CommandExecutionContext>;

    /// Clean up the context after command execution
    fn cleanup_context(&self, context: CommandExecutionContext) -> Result<()>;

    /// Check if the context is valid for the command
    fn validate_context(&self, command: &Commands, context: &CommandExecutionContext) -> Result<()>;

    /// Update context configuration
    fn update_context_config(
        &self,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<()>;
}

/// Trait for command execution coordination
pub trait CommandCoordinator {
    /// Coordinate the execution of a command from start to finish
    fn coordinate_execution(
        &self,
        command: Commands,
        port_name: String,
        optimize_transitions: bool,
        config: CommandExecutionConfig,
    ) -> Result<CommandExecutionResult>;

    /// Handle pre-execution setup
    fn pre_execution_setup(&self, command: &Commands) -> Result<()>;

    /// Handle post-execution cleanup
    fn post_execution_cleanup(&self, command: &Commands, result: &CommandExecutionResult) -> Result<()>;
}
