//! Power measurement operations for unified power system
//!
//! This module provides comprehensive power measurement operations including
//! raw device data retrieval, measurement validation, and structured data
//! organization for both CLI and GUI interfaces.

use crate::core::{LumidoxError, Result};
use crate::device::models::PowerInfo;
use super::conversion::ConversionResult;
use std::time::Instant;

/// Comprehensive power measurement data
/// 
/// Contains all power measurement information for a specific stage including
/// raw device data, converted values, current settings, and metadata.
#[derive(Debug, Clone)]
pub struct PowerMeasurementData {
    /// Stage number (1-5)
    pub stage_number: u8,
    /// Raw power information from device
    pub raw_power_info: PowerInfo,
    /// Converted power data (if conversion was applied)
    pub converted_data: ConversionResult,
    /// Current settings (ARM mA, FIRE mA)
    pub current_ma: (u16, u16),
    /// Timestamp when measurement was taken
    pub measurement_timestamp: Instant,
}

impl PowerMeasurementData {
    /// Create new power measurement data
    pub fn new(
        stage_number: u8,
        raw_power_info: PowerInfo,
        converted_data: ConversionResult,
        current_ma: (u16, u16),
    ) -> Self {
        Self {
            stage_number,
            raw_power_info,
            converted_data,
            current_ma,
            measurement_timestamp: Instant::now(),
        }
    }
    
    /// Get display-ready power values
    /// 
    /// Returns the appropriate power values for display, preferring converted
    /// values if available, falling back to raw values.
    /// 
    /// # Returns
    /// * `(f32, String, f32, String)` - (total_power, total_units, per_power, per_units)
    pub fn get_display_values(&self) -> (f32, String, f32, String) {
        (
            self.converted_data.total_power,
            self.converted_data.total_units.clone(),
            self.converted_data.per_power,
            self.converted_data.per_units.clone(),
        )
    }
    
    /// Get current values for display
    /// 
    /// Returns formatted current values for display purposes.
    /// 
    /// # Returns
    /// * `String` - Formatted current information
    pub fn get_current_display(&self) -> String {
        format!("ARM: {}mA, FIRE: {}mA", self.current_ma.0, self.current_ma.1)
    }
    
    /// Check if measurement is recent
    /// 
    /// Determines if the measurement was taken recently enough to be considered
    /// current for real-time display purposes.
    /// 
    /// # Arguments
    /// * `max_age_seconds` - Maximum age in seconds to consider recent
    /// 
    /// # Returns
    /// * `bool` - True if measurement is recent enough
    pub fn is_recent(&self, max_age_seconds: u64) -> bool {
        self.measurement_timestamp.elapsed().as_secs() <= max_age_seconds
    }
}

/// Power measurement operations
/// 
/// Provides centralized power measurement operations with comprehensive
/// error handling and validation for reliable power data retrieval.
pub struct PowerMeasurementOperations;

impl PowerMeasurementOperations {
    /// Validate power measurement data
    /// 
    /// Performs comprehensive validation of power measurement data to ensure
    /// values are within expected ranges and consistent with device specifications.
    /// 
    /// # Arguments
    /// * `data` - Power measurement data to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if validation passes, error if issues found
    pub fn validate_measurement_data(data: &PowerMeasurementData) -> Result<()> {
        // Validate stage number
        if !(1..=5).contains(&data.stage_number) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}", data.stage_number)
            ));
        }
        
        // Validate power values are non-negative
        if data.raw_power_info.total_power < 0.0 || data.raw_power_info.per_power < 0.0 {
            return Err(LumidoxError::DeviceError(
                "Power values cannot be negative".to_string()
            ));
        }
        
        // Validate current values are reasonable
        let (arm_ma, fire_ma) = data.current_ma;
        if arm_ma > 10000 || fire_ma > 10000 {
            return Err(LumidoxError::DeviceError(
                format!("Current values seem unreasonable: ARM {}mA, FIRE {}mA", arm_ma, fire_ma)
            ));
        }
        
        // Validate measurement is not too old
        if !data.is_recent(300) { // 5 minutes
            return Err(LumidoxError::DeviceError(
                "Power measurement data is too old".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Compare power measurements for consistency
    /// 
    /// Compares multiple power measurements to detect inconsistencies that
    /// might indicate device communication issues or hardware problems.
    /// 
    /// # Arguments
    /// * `measurements` - Vector of power measurements to compare
    /// 
    /// # Returns
    /// * `Result<()>` - Success if measurements are consistent, error if issues found
    pub fn validate_measurement_consistency(measurements: &[PowerMeasurementData]) -> Result<()> {
        if measurements.len() < 2 {
            return Ok(()); // Cannot compare single measurement
        }
        
        // Check for identical values across different stages (potential hardcoding issue)
        let mut unique_total_powers = std::collections::HashSet::new();
        let mut unique_per_powers = std::collections::HashSet::new();
        
        for measurement in measurements {
            // Use integer representation to avoid floating-point comparison issues
            let total_power_int = (measurement.raw_power_info.total_power * 1000.0) as i32;
            let per_power_int = (measurement.raw_power_info.per_power * 1000.0) as i32;
            
            unique_total_powers.insert(total_power_int);
            unique_per_powers.insert(per_power_int);
        }
        
        // If all stages have identical power values, this indicates a problem
        if unique_total_powers.len() == 1 && measurements.len() > 1 {
            return Err(LumidoxError::DeviceError(
                format!(
                    "All {} stages report identical total power values ({}), indicating potential hardcoding or device communication issue",
                    measurements.len(),
                    measurements[0].raw_power_info.total_power
                )
            ));
        }
        
        if unique_per_powers.len() == 1 && measurements.len() > 1 {
            return Err(LumidoxError::DeviceError(
                format!(
                    "All {} stages report identical per-well power values ({}), indicating potential hardcoding or device communication issue",
                    measurements.len(),
                    measurements[0].raw_power_info.per_power
                )
            ));
        }
        
        Ok(())
    }
    
    /// Create debug report for power measurements
    /// 
    /// Generates a comprehensive debug report for power measurements to help
    /// diagnose issues with power value display inconsistencies.
    /// 
    /// # Arguments
    /// * `measurements` - Vector of power measurements to analyze
    /// 
    /// # Returns
    /// * `String` - Detailed debug report
    pub fn create_debug_report(measurements: &[PowerMeasurementData]) -> String {
        let mut report = String::new();
        
        report.push_str("=== POWER MEASUREMENT DEBUG REPORT ===\n\n");
        
        report.push_str(&format!("Total measurements: {}\n", measurements.len()));
        report.push_str(&format!("Timestamp: {:?}\n\n", Instant::now()));
        
        // Individual stage analysis
        for measurement in measurements {
            report.push_str(&format!("Stage {}:\n", measurement.stage_number));
            report.push_str(&format!("  Total Power: {} {}\n", 
                measurement.raw_power_info.total_power, 
                measurement.raw_power_info.total_units));
            report.push_str(&format!("  Per-Well Power: {} {}\n", 
                measurement.raw_power_info.per_power, 
                measurement.raw_power_info.per_units));
            report.push_str(&format!("  Current: {}\n", measurement.get_current_display()));
            report.push_str(&format!("  Age: {:?}\n\n", measurement.measurement_timestamp.elapsed()));
        }
        
        // Consistency analysis
        report.push_str("=== CONSISTENCY ANALYSIS ===\n");
        match Self::validate_measurement_consistency(measurements) {
            Ok(()) => report.push_str("✓ Measurements appear consistent\n"),
            Err(e) => report.push_str(&format!("✗ Consistency issue detected: {}\n", e)),
        }
        
        // Value distribution analysis
        if measurements.len() > 1 {
            let total_powers: Vec<f32> = measurements.iter()
                .map(|m| m.raw_power_info.total_power)
                .collect();
            let per_powers: Vec<f32> = measurements.iter()
                .map(|m| m.raw_power_info.per_power)
                .collect();
            
            report.push_str(&format!("Total power range: {:.3} - {:.3}\n", 
                total_powers.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
                total_powers.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))));
            report.push_str(&format!("Per-well power range: {:.3} - {:.3}\n", 
                per_powers.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
                per_powers.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b))));
        }
        
        report.push_str("\n=== END REPORT ===\n");
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::models::PowerInfo;
    use super::super::conversion::ConversionResult;
    
    fn create_test_power_info(total_power: f32, per_power: f32) -> PowerInfo {
        PowerInfo {
            total_power,
            total_units: "mW TOTAL RADIANT POWER".to_string(),
            per_power,
            per_units: "mW PER WELL".to_string(),
        }
    }
    
    fn create_test_measurement_data(stage: u8, total_power: f32, per_power: f32) -> PowerMeasurementData {
        let power_info = create_test_power_info(total_power, per_power);
        let conversion_result = ConversionResult::from_raw_power_info(power_info.clone());
        
        PowerMeasurementData::new(
            stage,
            power_info,
            conversion_result,
            (1500, 2000), // ARM, FIRE current
        )
    }
    
    #[test]
    fn test_measurement_data_validation() {
        let data = create_test_measurement_data(1, 10.0, 5.0);
        assert!(PowerMeasurementOperations::validate_measurement_data(&data).is_ok());
        
        // Test invalid stage number
        let invalid_data = create_test_measurement_data(0, 10.0, 5.0);
        assert!(PowerMeasurementOperations::validate_measurement_data(&invalid_data).is_err());
        
        // Test negative power values
        let negative_data = create_test_measurement_data(1, -10.0, 5.0);
        assert!(PowerMeasurementOperations::validate_measurement_data(&negative_data).is_err());
    }
    
    #[test]
    fn test_consistency_validation() {
        // Test with different values (should pass)
        let measurements = vec![
            create_test_measurement_data(1, 5.0, 2.5),
            create_test_measurement_data(2, 10.0, 5.0),
            create_test_measurement_data(3, 25.0, 12.5),
        ];
        assert!(PowerMeasurementOperations::validate_measurement_consistency(&measurements).is_ok());
        
        // Test with identical values (should fail)
        let identical_measurements = vec![
            create_test_measurement_data(1, 10.0, 5.0),
            create_test_measurement_data(2, 10.0, 5.0),
            create_test_measurement_data(3, 10.0, 5.0),
        ];
        assert!(PowerMeasurementOperations::validate_measurement_consistency(&identical_measurements).is_err());
    }
    
    #[test]
    fn test_debug_report_generation() {
        let measurements = vec![
            create_test_measurement_data(1, 5.0, 2.5),
            create_test_measurement_data(2, 10.0, 5.0),
        ];
        
        let report = PowerMeasurementOperations::create_debug_report(&measurements);
        assert!(report.contains("POWER MEASUREMENT DEBUG REPORT"));
        assert!(report.contains("Stage 1:"));
        assert!(report.contains("Stage 2:"));
        assert!(report.contains("CONSISTENCY ANALYSIS"));
    }
    
    #[test]
    fn test_display_values() {
        let data = create_test_measurement_data(1, 10.0, 5.0);
        let (total, total_units, per, per_units) = data.get_display_values();
        
        assert_eq!(total, 10.0);
        assert_eq!(per, 5.0);
        assert!(total_units.contains("mW"));
        assert!(per_units.contains("mW"));
    }
    
    #[test]
    fn test_current_display() {
        let data = create_test_measurement_data(1, 10.0, 5.0);
        let current_display = data.get_current_display();
        
        assert!(current_display.contains("ARM: 1500mA"));
        assert!(current_display.contains("FIRE: 2000mA"));
    }
    
    #[test]
    fn test_measurement_age() {
        let data = create_test_measurement_data(1, 10.0, 5.0);
        
        // Should be recent immediately after creation
        assert!(data.is_recent(60));
        
        // Should not be recent with very short max age
        assert!(!data.is_recent(0));
    }
}
