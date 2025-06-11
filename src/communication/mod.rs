//! Communication module for Lumidox II Controller
//!
//! This module handles all communication-related functionality,
//! including serial protocol handling, automated port detection,
//! baud rate detection, and low-level device communication.

pub mod protocol;
pub mod port_detection;
pub mod baud_detection;
pub mod auto_connect;

// Re-export commonly used items for convenience
pub use protocol::ProtocolHandler;
pub use port_detection::{PortDetector, PortDetectionConfig, PortCandidate};
pub use baud_detection::{BaudDetector, BaudDetectionConfig, BaudTestResult};
pub use auto_connect::{AutoConnector, AutoConnectConfig, AutoConnectResult, ConnectionMethod};
