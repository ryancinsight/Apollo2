//! Command-specific error types for Lumidox II Controller CLI
//!
//! This module defines error types specific to command execution,
//! providing detailed error information for different failure scenarios.

use crate::core::LumidoxError;
use super::args::Commands;
use super::enums::{CommandCategory, CommandSafetyLevel};
use thiserror::Error;

/// Command execution specific errors
#[derive(Error, Debug, Clone)]
pub enum CommandError {
    /// Command validation failed
    #[error("Command validation failed: {reason}")]
    ValidationFailed {
        command: String,
        reason: String,
    },

    /// Command execution failed
    #[error("Command execution failed: {reason}")]
    ExecutionFailed {
        command: String,
        reason: String,
        category: CommandCategory,
    },

    /// Device connection required but not available
    #[error("Device connection required for command '{command}' but not available")]
    DeviceConnectionRequired {
        command: String,
    },

    /// Device in invalid state for command
    #[error("Device in invalid state for command '{command}': {current_state}")]
    InvalidDeviceState {
        command: String,
        current_state: String,
        required_state: Option<String>,
    },

    /// Command parameters are invalid
    #[error("Invalid parameters for command '{command}': {reason}")]
    InvalidParameters {
        command: String,
        reason: String,
    },

    /// User confirmation required but not provided
    #[error("User confirmation required for command '{command}' (safety level: {safety_level:?})")]
    ConfirmationRequired {
        command: String,
        safety_level: CommandSafetyLevel,
    },

    /// Command timeout
    #[error("Command '{command}' timed out after {timeout_ms}ms")]
    Timeout {
        command: String,
        timeout_ms: u32,
    },

    /// Command not supported
    #[error("Command '{command}' is not supported in current context")]
    NotSupported {
        command: String,
        context: String,
    },

    /// Resource unavailable
    #[error("Resource unavailable for command '{command}': {resource}")]
    ResourceUnavailable {
        command: String,
        resource: String,
    },

    /// Permission denied
    #[error("Permission denied for command '{command}': {reason}")]
    PermissionDenied {
        command: String,
        reason: String,
    },

    /// Command conflict with current operation
    #[error("Command '{command}' conflicts with current operation: {current_operation}")]
    OperationConflict {
        command: String,
        current_operation: String,
    },
}

/// Result type for command operations
pub type CommandResult<T> = Result<T, CommandError>;

impl CommandError {
    /// Create a validation failed error
    pub fn validation_failed(command: &Commands, reason: impl Into<String>) -> Self {
        Self::ValidationFailed {
            command: format!("{:?}", command),
            reason: reason.into(),
        }
    }

    /// Create an execution failed error
    pub fn execution_failed(command: &Commands, reason: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            command: format!("{:?}", command),
            reason: reason.into(),
            category: CommandCategory::from_command(command),
        }
    }

    /// Create a device connection required error
    pub fn device_connection_required(command: &Commands) -> Self {
        Self::DeviceConnectionRequired {
            command: format!("{:?}", command),
        }
    }

    /// Create an invalid device state error
    pub fn invalid_device_state(
        command: &Commands,
        current_state: impl Into<String>,
        required_state: Option<impl Into<String>>,
    ) -> Self {
        Self::InvalidDeviceState {
            command: format!("{:?}", command),
            current_state: current_state.into(),
            required_state: required_state.map(|s| s.into()),
        }
    }

    /// Create an invalid parameters error
    pub fn invalid_parameters(command: &Commands, reason: impl Into<String>) -> Self {
        Self::InvalidParameters {
            command: format!("{:?}", command),
            reason: reason.into(),
        }
    }

    /// Create a confirmation required error
    pub fn confirmation_required(command: &Commands) -> Self {
        Self::ConfirmationRequired {
            command: format!("{:?}", command),
            safety_level: CommandSafetyLevel::from_command(command),
        }
    }

    /// Create a timeout error
    pub fn timeout(command: &Commands, timeout_ms: u32) -> Self {
        Self::Timeout {
            command: format!("{:?}", command),
            timeout_ms,
        }
    }

    /// Create a not supported error
    pub fn not_supported(command: &Commands, context: impl Into<String>) -> Self {
        Self::NotSupported {
            command: format!("{:?}", command),
            context: context.into(),
        }
    }

    /// Create a resource unavailable error
    pub fn resource_unavailable(command: &Commands, resource: impl Into<String>) -> Self {
        Self::ResourceUnavailable {
            command: format!("{:?}", command),
            resource: resource.into(),
        }
    }

    /// Create a permission denied error
    pub fn permission_denied(command: &Commands, reason: impl Into<String>) -> Self {
        Self::PermissionDenied {
            command: format!("{:?}", command),
            reason: reason.into(),
        }
    }

    /// Create an operation conflict error
    pub fn operation_conflict(command: &Commands, current_operation: impl Into<String>) -> Self {
        Self::OperationConflict {
            command: format!("{:?}", command),
            current_operation: current_operation.into(),
        }
    }

    /// Get the command name from the error
    pub fn command_name(&self) -> &str {
        match self {
            Self::ValidationFailed { command, .. } => command,
            Self::ExecutionFailed { command, .. } => command,
            Self::DeviceConnectionRequired { command } => command,
            Self::InvalidDeviceState { command, .. } => command,
            Self::InvalidParameters { command, .. } => command,
            Self::ConfirmationRequired { command, .. } => command,
            Self::Timeout { command, .. } => command,
            Self::NotSupported { command, .. } => command,
            Self::ResourceUnavailable { command, .. } => command,
            Self::PermissionDenied { command, .. } => command,
            Self::OperationConflict { command, .. } => command,
        }
    }

    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ValidationFailed { .. } => false,
            Self::ExecutionFailed { .. } => false,
            Self::DeviceConnectionRequired { .. } => true,
            Self::InvalidDeviceState { .. } => true,
            Self::InvalidParameters { .. } => false,
            Self::ConfirmationRequired { .. } => true,
            Self::Timeout { .. } => true,
            Self::NotSupported { .. } => false,
            Self::ResourceUnavailable { .. } => true,
            Self::PermissionDenied { .. } => false,
            Self::OperationConflict { .. } => true,
        }
    }

    /// Get the error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::ValidationFailed { .. } => ErrorSeverity::Error,
            Self::ExecutionFailed { .. } => ErrorSeverity::Error,
            Self::DeviceConnectionRequired { .. } => ErrorSeverity::Warning,
            Self::InvalidDeviceState { .. } => ErrorSeverity::Warning,
            Self::InvalidParameters { .. } => ErrorSeverity::Error,
            Self::ConfirmationRequired { .. } => ErrorSeverity::Info,
            Self::Timeout { .. } => ErrorSeverity::Warning,
            Self::NotSupported { .. } => ErrorSeverity::Error,
            Self::ResourceUnavailable { .. } => ErrorSeverity::Warning,
            Self::PermissionDenied { .. } => ErrorSeverity::Error,
            Self::OperationConflict { .. } => ErrorSeverity::Warning,
        }
    }

    /// Get suggested recovery actions
    pub fn recovery_suggestions(&self) -> Vec<&'static str> {
        match self {
            Self::DeviceConnectionRequired { .. } => vec![
                "Check device connection",
                "Verify COM port settings",
                "Try auto-detection with --auto flag",
            ],
            Self::InvalidDeviceState { .. } => vec![
                "Check device status",
                "Reset device if necessary",
                "Wait for device to reach ready state",
            ],
            Self::ConfirmationRequired { .. } => vec![
                "Add --confirm flag to bypass confirmation",
                "Review command safety implications",
            ],
            Self::Timeout { .. } => vec![
                "Check device responsiveness",
                "Increase timeout value",
                "Retry the operation",
            ],
            Self::ResourceUnavailable { .. } => vec![
                "Wait for resource to become available",
                "Check system resources",
                "Retry after a delay",
            ],
            Self::OperationConflict { .. } => vec![
                "Wait for current operation to complete",
                "Cancel conflicting operation if possible",
                "Retry after conflict resolution",
            ],
            _ => vec!["Check command parameters and try again"],
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    /// Warning that doesn't prevent operation
    Warning,
    /// Error that prevents operation
    Error,
    /// Critical error that may require immediate attention
    Critical,
}

impl ErrorSeverity {
    /// Get the display string for the severity
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
        }
    }

    /// Get the color code for terminal display
    pub fn color_code(&self) -> &'static str {
        match self {
            Self::Info => "\x1b[36m",      // Cyan
            Self::Warning => "\x1b[33m",   // Yellow
            Self::Error => "\x1b[31m",     // Red
            Self::Critical => "\x1b[35m",  // Magenta
        }
    }
}

/// Convert CommandError to LumidoxError for compatibility
impl From<CommandError> for LumidoxError {
    fn from(err: CommandError) -> Self {
        match err {
            CommandError::ValidationFailed { reason, .. } => {
                LumidoxError::InvalidInput(reason)
            }
            CommandError::ExecutionFailed { reason, .. } => {
                LumidoxError::DeviceError(reason)
            }
            CommandError::DeviceConnectionRequired { .. } => {
                LumidoxError::CommunicationError("Device connection required".to_string())
            }
            CommandError::InvalidDeviceState { current_state, .. } => {
                LumidoxError::DeviceError(format!("Invalid device state: {}", current_state))
            }
            CommandError::InvalidParameters { reason, .. } => {
                LumidoxError::InvalidInput(reason)
            }
            CommandError::ConfirmationRequired { command, .. } => {
                LumidoxError::InvalidInput(format!("Confirmation required for {}", command))
            }
            CommandError::Timeout { timeout_ms, .. } => {
                LumidoxError::CommunicationError(format!("Operation timed out after {}ms", timeout_ms))
            }
            CommandError::NotSupported { command, .. } => {
                LumidoxError::InvalidInput(format!("Command not supported: {}", command))
            }
            CommandError::ResourceUnavailable { resource, .. } => {
                LumidoxError::DeviceError(format!("Resource unavailable: {}", resource))
            }
            CommandError::PermissionDenied { reason, .. } => {
                LumidoxError::InvalidInput(format!("Permission denied: {}", reason))
            }
            CommandError::OperationConflict { current_operation, .. } => {
                LumidoxError::DeviceError(format!("Operation conflict: {}", current_operation))
            }
        }
    }
}
