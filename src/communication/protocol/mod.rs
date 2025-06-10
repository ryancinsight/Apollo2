//! Protocol sub-module for Lumidox II Controller communication
//!
//! This module organizes protocol-related functionality into logical components:
//! - Constants: Protocol markers, timeouts, and configuration values
//! - Commands: Device command definitions and command arrays
//! - Handler: Core protocol communication logic
//! - Utils: Protocol utility functions for data processing

pub mod constants;
pub mod commands;
pub mod handler;
pub mod utils;

// Re-export commonly used items for convenience
pub use handler::ProtocolHandler;
