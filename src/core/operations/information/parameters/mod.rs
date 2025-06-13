//! Parameter operations for Lumidox II Controller
//!
//! This module provides unified parameter operations that serve as the single
//! source of truth for parameter reading across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! The parameter operations provide:
//! - Unified current parameter reading with validation
//! - Structured operation responses with parameter metadata
//! - Consistent error handling and range validation
//! - Interface-independent business logic

use crate::core::LumidoxError;
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

// TODO: Create tests module when needed
// #[cfg(test)]
// mod tests;

/// Parameter operations for unified parameter reading functionality
pub struct ParameterOperations;

impl ParameterOperations {
    /// Read current settings using unified operation pattern
    ///
    /// This function provides the single source of truth for current parameter reading
    /// across all interfaces (CLI, GUI). It performs validation, executes the parameter
    /// reading, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for parameter operations
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::ParameterInfo` with:
    /// - `parameter_name`: Name of the parameter being read
    /// - `value`: Parameter value as string
    /// - `units`: Parameter units (mA for current)
    /// - `valid_range`: Whether parameter is within valid range
    /// - `metadata`: Additional parameter information
    ///
    /// # Example
    /// ```
    /// let response = ParameterOperations::read_current_settings_unified(&mut device)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::ParameterInfo { parameter_name, value, .. } = response.data {
    ///     println!("Parameter: {}, Value: {:?}", parameter_name, value);
    /// }
    /// ```
    pub fn read_current_settings_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Read ARM current parameter
        match device.read_arm_current() {
            Ok(arm_current) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let valid_range = Self::validate_current_range(arm_current);
                
                let data = DeviceOperationData::ParameterInfo {
                    parameter_name: "ARM Current".to_string(),
                    value: Some(arm_current.to_string()),
                    units: Some("mA".to_string()),
                    valid_range,
                    metadata: Some(format!("Range: 0-{} mA", Self::get_max_current_range())),
                };
                
                let message = format!("ARM current parameter: {}mA ({})", 
                    arm_current, 
                    if valid_range { "Valid" } else { "Out of range" }
                );
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "read_current_settings".to_string(),
                    duration,
                ).with_context("operation".to_string(), "parameter_reading".to_string()))
            }
            Err(e) => {
                let _data = DeviceOperationData::ParameterInfo {
                    parameter_name: "ARM Current".to_string(),
                    value: None,
                    units: Some("mA".to_string()),
                    valid_range: false,
                    metadata: Some("Failed to read parameter".to_string()),
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to read ARM current: {}", e)))
            }
        }
    }

    /// Read ARM current using unified operation pattern
    ///
    /// This function provides centralized ARM current reading
    /// used across all parameter operations.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for ARM current reading
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Example
    /// ```
    /// let response = ParameterOperations::read_arm_current_unified(&mut device)?;
    /// ```
    pub fn read_arm_current_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        match device.read_arm_current() {
            Ok(arm_current) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let valid_range = Self::validate_current_range(arm_current);
                
                let data = DeviceOperationData::ParameterInfo {
                    parameter_name: "ARM Current".to_string(),
                    value: Some(arm_current.to_string()),
                    units: Some("mA".to_string()),
                    valid_range,
                    metadata: Some(format!("Valid range: 0-{} mA", Self::get_max_current_range())),
                };
                
                let message = format!("ARM current: {}mA", arm_current);
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "read_arm_current".to_string(),
                    duration,
                ).with_context("operation".to_string(), "arm_current_reading".to_string()))
            }
            Err(e) => {
                Err(LumidoxError::DeviceError(format!("Failed to read ARM current: {}", e)))
            }
        }
    }

    /// Get configuration using unified operation pattern
    ///
    /// This function provides centralized configuration reading
    /// used across all parameter operations.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for configuration reading
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Example
    /// ```
    /// let response = ParameterOperations::get_configuration_unified(&mut device)?;
    /// ```
    pub fn get_configuration_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Read multiple configuration parameters
        let arm_current = device.read_arm_current().ok();
        let fire_current = device.read_fire_current().ok();
        let remote_mode = device.read_remote_mode().ok();
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let config_summary = format!(
            "ARM: {}mA, FIRE: {}mA, Mode: {:?}",
            arm_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string()),
            fire_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string()),
            remote_mode.unwrap_or_else(|| crate::device::models::DeviceMode::Local)
        );
        
        let data = DeviceOperationData::ParameterInfo {
            parameter_name: "Device Configuration".to_string(),
            value: Some(config_summary.clone()),
            units: None,
            valid_range: arm_current.map(|c| Self::validate_current_range(c)).unwrap_or(false),
            metadata: Some("Complete device configuration summary".to_string()),
        };
        
        let message = format!("Device configuration: {}", config_summary);
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_configuration".to_string(),
            duration,
        ).with_context("operation".to_string(), "configuration_reading".to_string()))
    }

    /// Validate current range
    ///
    /// Provides centralized validation logic for current values
    /// used across all parameter operations.
    ///
    /// # Arguments
    /// * `current` - Current value to validate
    ///
    /// # Returns
    /// * `bool` - True if current is within valid range, false otherwise
    fn validate_current_range(current: u16) -> bool {
        current <= Self::get_max_current_range()
    }

    /// Get maximum current range
    ///
    /// Provides the maximum allowed current value for validation.
    ///
    /// # Returns
    /// * `u16` - Maximum current in mA
    fn get_max_current_range() -> u16 {
        5000 // 5A maximum current
    }
}
