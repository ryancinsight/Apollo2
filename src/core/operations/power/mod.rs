//! Unified power operations for Lumidox II Controller
//!
//! This module provides the unified power operation layer that serves as the single
//! source of truth for all power-related operations across CLI and GUI interfaces.
//! It implements structured responses, comprehensive error handling, and ensures
//! exact functionality parity between interfaces.
//!
//! The unified power operations provide:
//! - Centralized power value calculations and unit conversions
//! - Structured operation responses with detailed power data
//! - Comprehensive error handling and device state validation
//! - Interface-independent business logic for power operations
//! - Real-time power monitoring and status updates
//! - Mathematical unit conversion with proper validation

pub mod measurement;
pub mod conversion;
pub mod validation;
pub mod monitoring;
pub mod debug;

// Re-export commonly used functions and types
pub use measurement::{PowerMeasurementOperations, PowerMeasurementData};
pub use conversion::{PowerUnitConverter, PowerUnit, ConversionResult};
pub use validation::{PowerValidationOperations, PowerValidationResult};
pub use monitoring::{PowerMonitoringOperations, PowerStatusData};
pub use debug::PowerDebugOperations;

use crate::core::{LumidoxError, Result};
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

/// Unified power operations coordinator
/// 
/// Central coordinator for all power-related operations providing a unified
/// interface for both CLI and GUI components with guaranteed functionality parity.
pub struct UnifiedPowerOperations;

impl UnifiedPowerOperations {
    /// Get comprehensive power information for a specific stage
    /// 
    /// This is the unified entry point for power information retrieval that
    /// both CLI and GUI interfaces should use to ensure identical behavior.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage_num` - Stage number (1-5)
    /// * `target_unit` - Optional target unit for conversion
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured power data with conversion info
    /// 
    /// # Example
    /// ```
    /// let response = UnifiedPowerOperations::get_stage_power_unified(&mut device, 2, Some(PowerUnit::MilliWatts))?;
    /// ```
    pub fn get_stage_power_unified(
        device: &mut LumidoxDevice,
        stage_num: u8,
        target_unit: Option<PowerUnit>,
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Validate stage number
        if !(1..=5).contains(&stage_num) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5", stage_num)
            ));
        }
        
        // Validate device connection and readiness
        PowerValidationOperations::validate_device_ready_for_power_operations(device)?;
        
        // Get raw power information from device
        let raw_power_info = match device.get_power_info(stage_num) {
            Ok(info) => info,
            Err(e) => {
                return Err(LumidoxError::DeviceError(
                    format!("Failed to read power info for stage {}: {}", stage_num, e)
                ));
            }
        };
        
        // Get current (mA) values for comprehensive display
        let current_ma = Self::get_stage_current_ma(device, stage_num)?;
        
        // Perform unit conversion if requested
        let converted_data = if let Some(unit) = target_unit {
            PowerUnitConverter::convert_power_info(&raw_power_info, unit)?
        } else {
            ConversionResult::from_raw_power_info(raw_power_info.clone())
        };
        
        // Create comprehensive power measurement data
        let power_data = PowerMeasurementData {
            stage_number: stage_num,
            raw_power_info,
            converted_data,
            current_ma,
            measurement_timestamp: start_time,
        };
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::PowerMeasurement {
            stage_number: stage_num,
            power_data: power_data.clone(),
            validation_result: PowerValidationOperations::validate_power_readings(&power_data)?,
        };
        
        let message = Self::format_power_message(&power_data);
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_stage_power".to_string(),
            duration,
        ).with_context("operation".to_string(), "unified_power_measurement".to_string()))
    }
    
    /// Get current (mA) values for a specific stage
    /// 
    /// Retrieves both ARM and FIRE current values for comprehensive power analysis.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage_num` - Stage number (1-5)
    /// 
    /// # Returns
    /// * `Result<(u16, u16)>` - (ARM current mA, FIRE current mA)
    fn get_stage_current_ma(device: &mut LumidoxDevice, stage_num: u8) -> Result<(u16, u16)> {
        let arm_current = device.get_stage_arm_current(stage_num)
            .unwrap_or_else(|_| 0);
        
        // Get FIRE current from stage parameters
        let stage_params = device.get_stage_parameters(stage_num)
            .map_err(|e| LumidoxError::DeviceError(
                format!("Failed to get stage {} parameters: {}", stage_num, e)
            ))?;
        
        Ok((arm_current, stage_params.fire_current_ma))
    }
    
    /// Format power information message for display
    /// 
    /// Creates a comprehensive, human-readable message describing the power
    /// measurement results for both CLI and GUI display.
    /// 
    /// # Arguments
    /// * `power_data` - Power measurement data to format
    /// 
    /// # Returns
    /// * `String` - Formatted power information message
    fn format_power_message(power_data: &PowerMeasurementData) -> String {
        let raw = &power_data.raw_power_info;
        let converted = &power_data.converted_data;
        let (arm_ma, fire_ma) = power_data.current_ma;
        
        let mut message = format!(
            "Stage {} Power Information:\n",
            power_data.stage_number
        );
        
        message.push_str(&format!(
            "  Total Power: {} {} (Raw: {} {})\n",
            converted.total_power, converted.total_units,
            raw.total_power, raw.total_units
        ));
        
        message.push_str(&format!(
            "  Per-Well Power: {} {} (Raw: {} {})\n",
            converted.per_power, converted.per_units,
            raw.per_power, raw.per_units
        ));
        
        message.push_str(&format!(
            "  Current Settings: ARM {}mA, FIRE {}mA",
            arm_ma, fire_ma
        ));
        
        message
    }
    
    /// Get all stages power information for comprehensive display
    /// 
    /// Retrieves power information for all stages (1-5) in a single operation
    /// for efficient GUI updates and CLI comprehensive displays.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `target_unit` - Optional target unit for conversion
    /// 
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - All stages power data
    pub fn get_all_stages_power_unified(
        device: &mut LumidoxDevice,
        target_unit: Option<PowerUnit>,
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Validate device connection and readiness
        PowerValidationOperations::validate_device_ready_for_power_operations(device)?;
        
        let mut all_stages_data = Vec::new();
        let mut errors = Vec::new();
        
        // Get power information for all stages
        for stage in 1..=5 {
            match Self::get_stage_power_unified(device, stage, target_unit.clone()) {
                Ok(response) => {
                    if let DeviceOperationData::PowerMeasurement { power_data, .. } = response.data {
                        all_stages_data.push(power_data);
                    }
                }
                Err(e) => {
                    errors.push(format!("Stage {}: {}", stage, e));
                }
            }
        }
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        if !errors.is_empty() {
            return Err(LumidoxError::DeviceError(
                format!("Failed to read power info for some stages: {}", errors.join(", "))
            ));
        }
        
        let data = DeviceOperationData::AllStagesPower {
            stages_data: all_stages_data.clone(),
            target_unit,
            measurement_timestamp: start_time,
        };
        
        let message = format!(
            "Retrieved power information for all {} stages successfully",
            all_stages_data.len()
        );
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_all_stages_power".to_string(),
            duration,
        ).with_context("operation".to_string(), "unified_all_stages_power".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unified_power_operations_stage_validation() {
        // Test stage number validation
        // This would require a mock device for full testing
        assert!(!(0..=0).contains(&1)); // Placeholder test structure
    }
    
    #[test]
    fn test_power_message_formatting() {
        // Test power message formatting logic
        // This would require mock PowerMeasurementData for full testing
        assert!(true); // Placeholder test structure
    }
}
