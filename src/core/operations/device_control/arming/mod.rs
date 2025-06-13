//! Device arming operations for Lumidox II Controller
//!
//! This module provides unified device arming operations that serve as the single
//! source of truth for device arming across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! The arming operations provide:
//! - Unified device arming with readiness validation
//! - Structured operation responses with state transition data
//! - Consistent error handling and device state management
//! - Interface-independent business logic

use crate::core::LumidoxError;
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

mod device_traits;
pub use device_traits::{DeviceStateProvider, ArmingCapable};

#[cfg(test)]
mod tests;

/// Arming operations for unified device arming functionality
pub struct ArmingOperations;

impl ArmingOperations {
    /// Arm the device using unified operation pattern
    ///
    /// This function provides the single source of truth for device arming operations
    /// across all interfaces (CLI, GUI). It performs validation, executes the arming
    /// operation, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for arming operations
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::DeviceControl` with:
    /// - `previous_state`: The device state before arming
    /// - `new_state`: The device state after arming
    /// - `success`: Whether the arming operation succeeded
    ///
    /// # Example
    /// ```
    /// let response = ArmingOperations::arm_device_unified(&mut device)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::DeviceControl { success, new_state, .. } = response.data {
    ///     if success {
    ///         println!("Device armed successfully. New state: {:?}", new_state);
    ///     }
    /// }
    /// ```
    pub fn arm_device_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Get current state before operation
        let previous_state = Self::get_device_state_string(device);
        
        // Perform the arming operation using existing device method
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
                let _data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: Self::get_device_state_string(device),
                    success: false,
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to arm device: {}", e)))
            }
        }
    }

    /// Validate device readiness for arming operations
    ///
    /// Provides centralized validation logic for device arming readiness
    /// used across all arming operations.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to validate
    ///
    /// # Returns
    /// * `Result<()>` - Success if ready for arming, error if not ready
    ///
    /// # Example
    /// ```
    /// ArmingOperations::validate_arming_readiness(&device)?; // OK if ready
    /// ```
    pub fn validate_arming_readiness<T: DeviceStateProvider>(device: &T) -> crate::core::Result<()> {
        // Check if device is in a state that allows arming
        match device.current_mode() {
            Some(mode) => {
                use crate::device::models::DeviceMode;
                match mode {
                    DeviceMode::Local => {
                        Err(LumidoxError::InvalidInput(
                            "Device is in local mode. Cannot arm device remotely.".to_string()
                        ))
                    }
                    DeviceMode::Armed => {
                        Err(LumidoxError::InvalidInput(
                            "Device is already armed.".to_string()
                        ))
                    }
                    DeviceMode::Remote => {
                        Err(LumidoxError::InvalidInput(
                            "Device is in remote mode. Turn off device before arming.".to_string()
                        ))
                    }
                    DeviceMode::Standby => {
                        Ok(()) // Standby is the correct state for arming
                    }
                }
            }
            None => {
                Err(LumidoxError::InvalidInput(
                    "Device mode is unknown. Cannot determine arming readiness.".to_string()
                ))
            }
        }
    }

    /// Get device state as a string for logging/display
    ///
    /// Helper function to convert device state to string representation
    /// for consistent logging and display across interfaces.
    ///
    /// # Arguments
    /// * `device` - Reference to any device that implements DeviceStateProvider
    ///
    /// # Returns
    /// * `Option<String>` - Device state string if available
    fn get_device_state_string<T: DeviceStateProvider>(device: &T) -> Option<String> {
        device.current_mode().map(|mode| format!("{:?}", mode))
    }
}
