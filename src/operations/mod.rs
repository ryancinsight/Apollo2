//! Shared operations module for Lumidox II Controller
//!
//! This module provides unified operation interfaces and result types that can be
//! used by both CLI and GUI components, eliminating code duplication and improving
//! modularity. It serves as the central coordination point for all device operations,
//! validation, and result handling across different interface modes.
//!
//! The operations module includes:
//! - Unified operation interfaces for both CLI and GUI
//! - Shared result types that work with both interface patterns
//! - Common validation layer for input validation and error handling
//! - Device operation management with sync/async compatibility
//! - Configuration types that work with both CLI args and GUI state

// Import operation modules
pub mod device_operations;
pub mod validation;

// Re-export operation components for easy access
pub use device_operations::{DeviceOperationManager, DeviceOperationConfig};
pub use validation::{ValidationManager, ValidationResult, ValidationError};

use crate::device::{LumidoxDevice, models::{DeviceMode, DeviceInfo}};
use crate::core::{LumidoxError, Result};
use std::fmt;

/// Unified operation result type
/// 
/// Represents the result of any operation that can be performed by both
/// CLI and GUI interfaces. This type provides a common interface for
/// handling operation outcomes across different UI modes.
#[derive(Debug, Clone)]
pub enum OperationResult<T> {
    /// Operation completed successfully
    Success {
        /// The successful result data
        data: T,
        /// Optional success message for user feedback
        message: Option<String>,
        /// Operation execution time in milliseconds
        duration_ms: Option<u64>,
    },
    /// Operation failed with an error
    Error {
        /// The error that occurred
        error: LumidoxError,
        /// Optional context information
        context: Option<String>,
        /// Whether the operation can be retried
        retryable: bool,
    },
    /// Operation was cancelled by user
    Cancelled {
        /// Reason for cancellation
        reason: String,
    },
    /// Operation is in progress (for async operations)
    InProgress {
        /// Progress percentage (0-100)
        progress: Option<u8>,
        /// Current operation status message
        status: String,
    },
}

impl<T> OperationResult<T> {
    /// Create a successful result
    /// 
    /// # Arguments
    /// * `data` - The successful result data
    /// 
    /// # Returns
    /// * `OperationResult<T>` - Success result
    /// 
    /// # Example
    /// ```
    /// let result = OperationResult::success(device_info);
    /// ```
    pub fn success(data: T) -> Self {
        Self::Success {
            data,
            message: None,
            duration_ms: None,
        }
    }
    
    /// Create a successful result with message
    /// 
    /// # Arguments
    /// * `data` - The successful result data
    /// * `message` - Success message for user feedback
    /// 
    /// # Returns
    /// * `OperationResult<T>` - Success result with message
    /// 
    /// # Example
    /// ```
    /// let result = OperationResult::success_with_message(device_info, "Device connected successfully");
    /// ```
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self::Success {
            data,
            message: Some(message.into()),
            duration_ms: None,
        }
    }
    
    /// Create an error result
    /// 
    /// # Arguments
    /// * `error` - The error that occurred
    /// 
    /// # Returns
    /// * `OperationResult<T>` - Error result
    /// 
    /// # Example
    /// ```
    /// let result = OperationResult::error(LumidoxError::DeviceNotFound);
    /// ```
    pub fn error(error: LumidoxError) -> Self {
        Self::Error {
            error,
            context: None,
            retryable: false,
        }
    }
    
    /// Create a retryable error result
    /// 
    /// # Arguments
    /// * `error` - The error that occurred
    /// * `context` - Optional context information
    /// 
    /// # Returns
    /// * `OperationResult<T>` - Retryable error result
    /// 
    /// # Example
    /// ```
    /// let result = OperationResult::retryable_error(error, "Connection timeout");
    /// ```
    pub fn retryable_error(error: LumidoxError, context: Option<String>) -> Self {
        Self::Error {
            error,
            context,
            retryable: true,
        }
    }
    
    /// Create a cancelled result
    /// 
    /// # Arguments
    /// * `reason` - Reason for cancellation
    /// 
    /// # Returns
    /// * `OperationResult<T>` - Cancelled result
    /// 
    /// # Example
    /// ```
    /// let result = OperationResult::cancelled("User cancelled operation");
    /// ```
    pub fn cancelled(reason: impl Into<String>) -> Self {
        Self::Cancelled {
            reason: reason.into(),
        }
    }
    
    /// Create an in-progress result
    /// 
    /// # Arguments
    /// * `status` - Current operation status
    /// 
    /// # Returns
    /// * `OperationResult<T>` - In-progress result
    /// 
    /// # Example
    /// ```
    /// let result = OperationResult::in_progress("Connecting to device...");
    /// ```
    pub fn in_progress(status: impl Into<String>) -> Self {
        Self::InProgress {
            progress: None,
            status: status.into(),
        }
    }
    
    /// Check if the result is successful
    /// 
    /// # Returns
    /// * `bool` - True if result is successful
    /// 
    /// # Example
    /// ```
    /// if result.is_success() {
    ///     println!("Operation succeeded");
    /// }
    /// ```
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }
    
    /// Check if the result is an error
    /// 
    /// # Returns
    /// * `bool` - True if result is an error
    /// 
    /// # Example
    /// ```
    /// if result.is_error() {
    ///     println!("Operation failed");
    /// }
    /// ```
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }
    
    /// Check if the result is retryable
    /// 
    /// # Returns
    /// * `bool` - True if result is retryable
    /// 
    /// # Example
    /// ```
    /// if result.is_retryable() {
    ///     println!("Operation can be retried");
    /// }
    /// ```
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Error { retryable, .. } => *retryable,
            _ => false,
        }
    }
    
    /// Get the success data if available
    /// 
    /// # Returns
    /// * `Option<&T>` - Success data if available
    /// 
    /// # Example
    /// ```
    /// if let Some(data) = result.data() {
    ///     println!("Got data: {:?}", data);
    /// }
    /// ```
    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Success { data, .. } => Some(data),
            _ => None,
        }
    }
    
    /// Get the error if available
    ///
    /// # Returns
    /// * `Option<&LumidoxError>` - Error if available
    ///
    /// # Example
    /// ```
    /// if let Some(error) = result.get_error() {
    ///     eprintln!("Error: {}", error);
    /// }
    /// ```
    pub fn get_error(&self) -> Option<&LumidoxError> {
        match self {
            Self::Error { error, .. } => Some(error),
            _ => None,
        }
    }
    
    /// Get a user-friendly message for the result
    /// 
    /// # Returns
    /// * `String` - User-friendly message
    /// 
    /// # Example
    /// ```
    /// println!("{}", result.message());
    /// ```
    pub fn message(&self) -> String {
        match self {
            Self::Success { message, .. } => {
                message.clone().unwrap_or_else(|| "Operation completed successfully".to_string())
            }
            Self::Error { error, context, .. } => {
                if let Some(ctx) = context {
                    format!("{}: {}", ctx, error)
                } else {
                    error.to_string()
                }
            }
            Self::Cancelled { reason } => format!("Operation cancelled: {}", reason),
            Self::InProgress { status, .. } => status.clone(),
        }
    }
    
    /// Convert to a standard Result type
    /// 
    /// # Returns
    /// * `Result<T>` - Standard result type
    /// 
    /// # Example
    /// ```
    /// let std_result = operation_result.to_result()?;
    /// ```
    pub fn to_result(self) -> Result<T> {
        match self {
            Self::Success { data, .. } => Ok(data),
            Self::Error { error, .. } => Err(error),
            Self::Cancelled { reason } => Err(LumidoxError::OperationCancelled(reason)),
            Self::InProgress { .. } => Err(LumidoxError::OperationInProgress),
        }
    }
}

impl<T> fmt::Display for OperationResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

/// Operation configuration
/// 
/// Contains configuration parameters that can be used by both CLI and GUI
/// operations to customize behavior and provide consistent operation patterns.
#[derive(Debug, Clone)]
pub struct OperationConfig {
    /// Enable verbose output
    pub verbose: bool,
    /// Enable optimized stage transitions
    pub optimize_transitions: bool,
    /// Operation timeout in seconds
    pub timeout_seconds: u32,
    /// Maximum retry attempts for retryable operations
    pub max_retries: u8,
    /// Whether to auto-detect device ports
    pub auto_detect: bool,
    /// Specific port name to use (if not auto-detecting)
    pub port_name: Option<String>,
}

impl OperationConfig {
    /// Create default operation configuration
    /// 
    /// # Returns
    /// * `OperationConfig` - Default configuration
    /// 
    /// # Example
    /// ```
    /// let config = OperationConfig::default();
    /// ```
    pub fn default() -> Self {
        Self {
            verbose: false,
            optimize_transitions: true,
            timeout_seconds: 30,
            max_retries: 3,
            auto_detect: true,
            port_name: None,
        }
    }
    
    /// Create configuration from CLI arguments
    /// 
    /// # Arguments
    /// * `port_name` - Optional port name
    /// * `auto_detect` - Enable auto-detection
    /// * `verbose` - Enable verbose output
    /// * `optimize_transitions` - Enable optimized transitions
    /// 
    /// # Returns
    /// * `OperationConfig` - Configuration from CLI args
    /// 
    /// # Example
    /// ```
    /// let config = OperationConfig::from_cli_args(None, true, false, true);
    /// ```
    pub fn from_cli_args(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> Self {
        Self {
            verbose,
            optimize_transitions,
            timeout_seconds: 30,
            max_retries: 3,
            auto_detect,
            port_name,
        }
    }
    
    /// Create configuration with specific port
    /// 
    /// # Arguments
    /// * `port_name` - Specific port name
    /// 
    /// # Returns
    /// * `OperationConfig` - Configuration with specific port
    /// 
    /// # Example
    /// ```
    /// let config = OperationConfig::with_port("COM3");
    /// ```
    pub fn with_port(port_name: impl Into<String>) -> Self {
        Self {
            port_name: Some(port_name.into()),
            auto_detect: false,
            ..Self::default()
        }
    }
    
    /// Enable verbose mode
    /// 
    /// # Returns
    /// * `OperationConfig` - Configuration with verbose enabled
    /// 
    /// # Example
    /// ```
    /// let config = OperationConfig::default().with_verbose();
    /// ```
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    
    /// Set timeout
    /// 
    /// # Arguments
    /// * `seconds` - Timeout in seconds
    /// 
    /// # Returns
    /// * `OperationConfig` - Configuration with timeout set
    /// 
    /// # Example
    /// ```
    /// let config = OperationConfig::default().with_timeout(60);
    /// ```
    pub fn with_timeout(mut self, seconds: u32) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}
