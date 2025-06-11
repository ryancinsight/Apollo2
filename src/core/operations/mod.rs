//! Core operations module for Lumidox II Controller
//!
//! This module provides unified, interface-agnostic operation interfaces that eliminate
//! duplication between CLI and GUI implementations. It defines common operation patterns,
//! result types, and error handling that both interfaces can use consistently.
//!
//! The core operations module includes:
//! - Unified device control operations (ARM, turn off, shutdown)
//! - Interface-independent result types with structured data
//! - Common error handling and validation
//! - State management coordination
//! - Operation logging and feedback coordination

pub mod device_control;
pub mod firing;
pub mod result_types;

// Re-export commonly used types
pub use device_control::DeviceControlOperations;
pub use firing::StageOperations;
pub use result_types::{OperationResult, OperationResponse, DeviceOperationData};
