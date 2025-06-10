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
}
