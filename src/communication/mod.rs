//! Communication module for Lumidox II Controller
//!
//! This module handles all communication-related functionality,
//! including serial protocol handling and low-level device communication.

pub mod protocol;

// Re-export commonly used items for convenience
pub use protocol::ProtocolHandler;
