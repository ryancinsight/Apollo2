//! Error handling sub-module for Lumidox II Controller
//!
//! This module organizes error handling functionality into logical components:
//! - types: Error type definitions and variants
//! - context: Error context utilities and traits
//! - device_errors: Device-specific error handling utilities
//! - communication_errors: Protocol and communication error utilities
//! - validation_errors: Input validation error utilities
//! - system_errors: System and I/O error utilities

pub mod types;
pub mod context;
pub mod device_errors;
pub mod communication_errors;
pub mod validation_errors;
pub mod system_errors;

// Re-export commonly used items for convenience
pub use types::LumidoxError;
pub use device_errors::{DeviceErrorCategory, DeviceErrorUtils};
pub use communication_errors::{CommunicationErrorCategory, CommunicationErrorUtils};
pub use validation_errors::{ValidationErrorCategory, ValidationErrorUtils};
pub use system_errors::{SystemErrorCategory, SystemErrorUtils};
