//! Device controller operations module for Lumidox II Controller
//!
//! This module organizes device controller operations into focused sub-modules:
//! - `control`: Device control operations (firing, arming, shutdown, etc.)
//! - `information`: Device information and status operations
//!
//! The operations architecture provides:
//! - Clear separation between control and information operations
//! - Specialized functionality for different operational concerns
//! - Enhanced maintainability through focused modules
//! - Comprehensive operation support with proper error handling

pub mod control;
pub mod information;

// Re-export commonly used items for convenience
pub use control::{
    DeviceControlOperations, ControlOperation, FiringReadinessResult,
    ControlRecommendations, ControlOperationResult
};
pub use information::{
    DeviceInformationOperations, DeviceStatusReport, StageInformation,
    InformationConsistencyReport
};
