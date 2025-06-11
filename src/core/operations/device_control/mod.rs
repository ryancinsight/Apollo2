//! Device control operations for Lumidox II Controller
//!
//! This module provides unified device control operations that serve as the single
//! source of truth for device control across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! Following the "Code as a Database" paradigm, this module is organized into:
//! - `arming/` - Device arming operations with readiness validation
//! - `power_control/` - Turn off operations with safety confirmation
//! - `shutdown/` - Device shutdown operations with state preservation

pub mod arming;
pub mod power_control;
pub mod shutdown;

// Re-export unified operations for backward compatibility
pub use arming::ArmingOperations;
pub use power_control::PowerControlOperations;
pub use shutdown::ShutdownOperations;

// Legacy re-exports to maintain existing API compatibility
use crate::core::operations::result_types::{OperationResult, DeviceOperationData};
use crate::device::LumidoxDevice;

/// Legacy device control operations manager for backward compatibility
/// 
/// This struct maintains the existing API while delegating to the new
/// hierarchical operation modules. All existing code will continue to work
/// without modification.
pub struct DeviceControlOperations;

impl DeviceControlOperations {
    /// Arm the device for firing operations (legacy API)
    /// 
    /// Delegates to the new hierarchical arming operations while maintaining
    /// the exact same API signature and behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with operation data
    pub fn arm_device(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        ArmingOperations::arm_device_unified(device)
    }

    /// Turn off the device safely (legacy API)
    /// 
    /// Delegates to the new hierarchical power control operations while maintaining
    /// the exact same API signature and behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with operation data
    pub fn turn_off_device(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        PowerControlOperations::turn_off_device_unified(device)
    }

    /// Shutdown the device and return to local mode (legacy API)
    /// 
    /// Delegates to the new hierarchical shutdown operations while maintaining
    /// the exact same API signature and behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with operation data
    pub fn shutdown_device(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        ShutdownOperations::shutdown_device_unified(device)
    }

    /// Get device state as a string for logging/display (legacy helper)
    /// 
    /// Provides backward compatibility for the helper function used by
    /// the original implementation.
    pub fn get_device_state_string(device: &LumidoxDevice) -> Option<String> {
        device.current_mode().map(|mode| format!("{:?}", mode))
    }
}
