//! Information retrieval operations for Lumidox II Controller
//!
//! This module provides unified information retrieval operations that serve as the single
//! source of truth for device information across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! Following the "Code as a Database" paradigm, this module is organized into:
//! - `device_status/` - Device status operations with comprehensive state data
//! - `parameters/` - Parameter reading operations with validation
//! - `stage_info/` - Stage information operations with current/voltage data

pub mod device_status;
pub mod parameters;
pub mod stage_info;

// Re-export unified operations for backward compatibility
pub use device_status::DeviceStatusOperations;
pub use parameters::ParameterOperations;
pub use stage_info::StageInfoOperations;

// Legacy re-exports to maintain existing API compatibility
use crate::core::operations::result_types::{OperationResult, DeviceOperationData};
use crate::device::LumidoxDevice;

/// Legacy information operations manager for backward compatibility
/// 
/// This struct maintains the existing API while delegating to the new
/// hierarchical operation modules. All existing code will continue to work
/// without modification.
pub struct InformationOperations;

impl InformationOperations {
    /// Get device status information (legacy API)
    /// 
    /// Delegates to the new hierarchical device status operations while maintaining
    /// the exact same API signature and behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with status data
    pub fn get_device_status(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        DeviceStatusOperations::get_device_status_unified(device)
    }

    /// Read current parameters (legacy API)
    /// 
    /// Delegates to the new hierarchical parameter operations while maintaining
    /// the exact same API signature and behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with parameter data
    pub fn read_current_parameters(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        ParameterOperations::read_current_settings_unified(device)
    }

    /// Get stage information (legacy API)
    /// 
    /// Delegates to the new hierarchical stage info operations while maintaining
    /// the exact same API signature and behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage` - Stage number to query (1-5)
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with stage data
    pub fn get_stage_information(device: &mut LumidoxDevice, stage: u8) -> OperationResult<DeviceOperationData> {
        StageInfoOperations::get_stage_data_unified(device, stage)
    }

    /// Get device health information (legacy helper)
    /// 
    /// Provides backward compatibility for the helper function used by
    /// the original implementation.
    pub fn get_device_health_string(device: &LumidoxDevice) -> Option<String> {
        device.current_mode().map(|mode| format!("Mode: {:?}", mode))
    }
}
