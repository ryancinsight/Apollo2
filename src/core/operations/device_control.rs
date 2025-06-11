//! Unified device control operations
//!
//! This module provides interface-agnostic device control operations that can be used
//! by both CLI and GUI interfaces. It eliminates code duplication by centralizing
//! business logic and providing structured results that each interface can format
//! according to its presentation requirements.

use crate::core::LumidoxError;
use crate::device::LumidoxDevice;
use super::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use std::time::Instant;

/// Unified device control operations manager
pub struct DeviceControlOperations;

impl DeviceControlOperations {
    /// Arm the device for firing operations
    /// 
    /// Provides a unified interface for device arming that returns structured data
    /// suitable for both CLI and GUI presentation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with operation data
    /// 
    /// # Example
    /// ```
    /// let result = DeviceControlOperations::arm_device(&mut device)?;
    /// match result.data {
    ///     DeviceOperationData::DeviceControl { success, .. } => {
    ///         if success {
    ///             println!("CLI: {}", result.message);
    ///             // or gui_state.status = result.message;
    ///         }
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn arm_device(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Get current state before operation
        let previous_state = Self::get_device_state_string(device);
        
        // Perform the arming operation
        match device.arm() {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let new_state = Self::get_device_state_string(device);
                
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: new_state.clone(),
                    success: true,
                };
                
                let message = "Device armed successfully and ready for firing operations".to_string();
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "arm_device".to_string(),
                    duration,
                ).with_context("operation".to_string(), "device_arming".to_string()))
            }
            Err(e) => {
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: Self::get_device_state_string(device),
                    success: false,
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to arm device: {}", e)))
            }
        }
    }

    /// Turn off the device safely
    /// 
    /// Provides a unified interface for device turn-off that returns structured data
    /// suitable for both CLI and GUI presentation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with operation data
    pub fn turn_off_device(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Get current state before operation
        let previous_state = Self::get_device_state_string(device);
        
        // Perform the turn off operation
        match device.turn_off() {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let new_state = Self::get_device_state_string(device);
                
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: new_state.clone(),
                    success: true,
                };
                
                let message = "Device turned off successfully and is now in safe standby mode".to_string();
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "turn_off_device".to_string(),
                    duration,
                ).with_context("operation".to_string(), "device_turn_off".to_string()))
            }
            Err(e) => {
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: Self::get_device_state_string(device),
                    success: false,
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to turn off device: {}", e)))
            }
        }
    }

    /// Shutdown the device and return to local mode
    /// 
    /// Provides a unified interface for device shutdown that returns structured data
    /// suitable for both CLI and GUI presentation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured result with operation data
    pub fn shutdown_device(device: &mut LumidoxDevice) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Get current state before operation
        let previous_state = Self::get_device_state_string(device);
        
        // Perform the shutdown operation
        match device.shutdown() {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let new_state = Self::get_device_state_string(device);
                
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: new_state.clone(),
                    success: true,
                };
                
                let message = "Device shutdown successfully and returned to local mode".to_string();
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "shutdown_device".to_string(),
                    duration,
                ).with_context("operation".to_string(), "device_shutdown".to_string()))
            }
            Err(e) => {
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: Self::get_device_state_string(device),
                    success: false,
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to shutdown device: {}", e)))
            }
        }
    }

    /// Get device state as a string for logging/display
    fn get_device_state_string(device: &LumidoxDevice) -> Option<String> {
        device.current_mode().map(|mode| format!("{:?}", mode))
    }
}
