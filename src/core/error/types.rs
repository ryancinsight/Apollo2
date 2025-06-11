//! Error type definitions for Lumidox II Controller
//!
//! This module defines all error types used throughout the application,
//! providing centralized error type definitions with proper error propagation.

use thiserror::Error;

/// Main error type for the Lumidox II Controller application
#[derive(Error, Debug)]
pub enum LumidoxError {
    /// Serial communication errors
    #[error("Serial communication error: {0}")]
    SerialError(#[from] serialport::Error),

    /// Standard I/O errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid user input or parameter validation errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Device-specific communication or protocol errors
    #[error("Device communication error: {0}")]
    DeviceError(String),

    /// Configuration or initialization errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Protocol parsing or validation errors
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Operation cancelled by user
    #[error("Operation cancelled: {0}")]
    OperationCancelled(String),

    /// Operation is currently in progress
    #[error("Operation in progress")]
    OperationInProgress,

    /// Device not found or not connected
    #[error("Device not found")]
    DeviceNotFound,
}

// Implement Clone manually for the parts that need it
impl Clone for LumidoxError {
    fn clone(&self) -> Self {
        match self {
            Self::SerialError(e) => Self::SerialError(serialport::Error::new(
                serialport::ErrorKind::Unknown,
                e.to_string(),
            )),
            Self::IoError(e) => Self::IoError(std::io::Error::new(
                e.kind(),
                e.to_string(),
            )),
            Self::InvalidInput(s) => Self::InvalidInput(s.clone()),
            Self::DeviceError(s) => Self::DeviceError(s.clone()),
            Self::ConfigError(s) => Self::ConfigError(s.clone()),
            Self::ProtocolError(s) => Self::ProtocolError(s.clone()),
            Self::ValidationError(s) => Self::ValidationError(s.clone()),
            Self::OperationCancelled(s) => Self::OperationCancelled(s.clone()),
            Self::OperationInProgress => Self::OperationInProgress,
            Self::DeviceNotFound => Self::DeviceNotFound,
        }
    }
}
