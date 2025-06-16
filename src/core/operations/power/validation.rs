//! Power validation operations for unified power system
//!
//! This module provides comprehensive validation operations for power measurements
//! including device readiness checks, measurement validation, and consistency
//! verification to ensure reliable power data across CLI and GUI interfaces.

use crate::core::{LumidoxError, Result};
use crate::device::LumidoxDevice;
use super::measurement::PowerMeasurementData;

/// Power validation result
/// 
/// Contains the results of power validation operations including
/// validation status, detected issues, and recommendations.
#[derive(Debug, Clone)]
pub struct PowerValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// List of validation issues found
    pub issues: Vec<String>,
    /// Recommendations for resolving issues
    pub recommendations: Vec<String>,
    /// Validation confidence level (0.0 - 1.0)
    pub confidence: f32,
}

impl PowerValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
            recommendations: Vec::new(),
            confidence: 1.0,
        }
    }
    
    /// Create a failed validation result with issues
    pub fn failure(issues: Vec<String>, recommendations: Vec<String>) -> Self {
        Self {
            is_valid: false,
            issues,
            recommendations,
            confidence: 0.0,
        }
    }
    
    /// Add an issue to the validation result
    pub fn add_issue(&mut self, issue: String, recommendation: Option<String>) {
        self.issues.push(issue);
        if let Some(rec) = recommendation {
            self.recommendations.push(rec);
        }
        self.is_valid = false;
        self.confidence = 0.0;
    }
    
    /// Get formatted validation report
    pub fn get_report(&self) -> String {
        let mut report = String::new();
        
        if self.is_valid {
            report.push_str("✓ Power validation PASSED\n");
        } else {
            report.push_str("✗ Power validation FAILED\n");
        }
        
        report.push_str(&format!("Confidence: {:.1}%\n", self.confidence * 100.0));
        
        if !self.issues.is_empty() {
            report.push_str("\nIssues found:\n");
            for (i, issue) in self.issues.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", i + 1, issue));
            }
        }
        
        if !self.recommendations.is_empty() {
            report.push_str("\nRecommendations:\n");
            for (i, rec) in self.recommendations.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", i + 1, rec));
            }
        }
        
        report
    }
}

/// Power validation operations
/// 
/// Provides comprehensive validation operations for power measurements
/// and device readiness with detailed error reporting and recommendations.
pub struct PowerValidationOperations;

impl PowerValidationOperations {
    /// Validate device readiness for power operations
    /// 
    /// Performs comprehensive checks to ensure the device is ready for
    /// reliable power measurement operations.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if device is ready, error with details if not
    pub fn validate_device_ready_for_power_operations(device: &LumidoxDevice) -> Result<()> {
        // Check device connection state
        if device.info().is_none() {
            return Err(LumidoxError::DeviceError(
                "Device not properly initialized. Call device.initialize() first.".to_string()
            ));
        }
        
        // Check device mode compatibility
        match device.current_mode() {
            Some(mode) => {
                use crate::device::models::DeviceMode;
                match mode {
                    DeviceMode::Local => {
                        return Err(LumidoxError::InvalidInput(
                            "Device is in local mode. Switch to remote mode for power operations.".to_string()
                        ));
                    }
                    DeviceMode::Standby | DeviceMode::Armed | DeviceMode::Remote => {
                        // These modes are acceptable for power operations
                    }
                }
            }
            None => {
                return Err(LumidoxError::DeviceError(
                    "Device mode is unknown. Cannot determine readiness for power operations.".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Validate power readings for consistency and accuracy
    /// 
    /// Performs comprehensive validation of power measurement data to detect
    /// potential issues with device communication or hardware problems.
    /// 
    /// # Arguments
    /// * `power_data` - Power measurement data to validate
    /// 
    /// # Returns
    /// * `Result<PowerValidationResult>` - Validation result with detailed analysis
    pub fn validate_power_readings(power_data: &PowerMeasurementData) -> Result<PowerValidationResult> {
        let mut result = PowerValidationResult::success();
        
        // Validate basic measurement data
        if let Err(e) = super::measurement::PowerMeasurementOperations::validate_measurement_data(power_data) {
            result.add_issue(
                format!("Basic measurement validation failed: {}", e),
                Some("Check device connection and retry measurement".to_string())
            );
        }
        
        // Validate power values against expected ranges for each stage
        let expected_ranges = Self::get_expected_power_ranges();
        if let Some((min_total, max_total, min_per, max_per)) = expected_ranges.get(&power_data.stage_number) {
            let total_power = power_data.raw_power_info.total_power;
            let per_power = power_data.raw_power_info.per_power;
            
            if total_power < *min_total || total_power > *max_total {
                result.add_issue(
                    format!(
                        "Stage {} total power ({:.3}) outside expected range {:.3}-{:.3}",
                        power_data.stage_number, total_power, min_total, max_total
                    ),
                    Some("Verify device calibration and smart card configuration".to_string())
                );
            }
            
            if per_power < *min_per || per_power > *max_per {
                result.add_issue(
                    format!(
                        "Stage {} per-well power ({:.3}) outside expected range {:.3}-{:.3}",
                        power_data.stage_number, per_power, min_per, max_per
                    ),
                    Some("Check LED configuration and device calibration".to_string())
                );
            }
        }
        
        // Validate current values
        let (arm_ma, fire_ma) = power_data.current_ma;
        if arm_ma == 0 && fire_ma == 0 {
            result.add_issue(
                "Both ARM and FIRE currents are zero".to_string(),
                Some("Check device current settings and smart card configuration".to_string())
            );
        }
        
        if arm_ma > 10000 || fire_ma > 10000 {
            result.add_issue(
                format!("Current values seem unreasonably high: ARM {}mA, FIRE {}mA", arm_ma, fire_ma),
                Some("Verify device specifications and current settings".to_string())
            );
        }
        
        // Validate measurement freshness
        if !power_data.is_recent(60) { // 1 minute
            result.add_issue(
                "Power measurement is not recent".to_string(),
                Some("Refresh power measurements for current data".to_string())
            );
        }
        
        // Calculate confidence based on issues found
        if result.is_valid {
            result.confidence = 1.0;
        } else {
            // Reduce confidence based on number of issues
            let issue_penalty = 0.2 * result.issues.len() as f32;
            result.confidence = (1.0 - issue_penalty).max(0.0);
        }
        
        Ok(result)
    }
    
    /// Get expected power ranges for each stage
    /// 
    /// Returns the expected power ranges for each stage based on factory
    /// calibration values from the Python reference implementation.
    /// 
    /// # Returns
    /// * `std::collections::HashMap<u8, (f32, f32, f32, f32)>` - Stage -> (min_total, max_total, min_per, max_per)
    fn get_expected_power_ranges() -> std::collections::HashMap<u8, (f32, f32, f32, f32)> {
        let mut ranges = std::collections::HashMap::new();
        
        // Based on factory-calibrated power levels from lumidox_II_console_interface_rev1b.py:
        // Stage 1: 0.5W total, 5mW per well
        // Stage 2: 1W total, 10mW per well
        // Stage 3: 2.4W total, 25mW per well
        // Stage 4: 4.8W total, 50mW per well
        // Stage 5: 9.6W total, 100mW per well
        
        // Allow ±20% tolerance for device variations
        ranges.insert(1, (400.0, 600.0, 4.0, 6.0));     // Stage 1: 500mW ±20%, 5mW ±20%
        ranges.insert(2, (800.0, 1200.0, 8.0, 12.0));   // Stage 2: 1000mW ±20%, 10mW ±20%
        ranges.insert(3, (1920.0, 2880.0, 20.0, 30.0)); // Stage 3: 2400mW ±20%, 25mW ±20%
        ranges.insert(4, (3840.0, 5760.0, 40.0, 60.0)); // Stage 4: 4800mW ±20%, 50mW ±20%
        ranges.insert(5, (7680.0, 11520.0, 80.0, 120.0)); // Stage 5: 9600mW ±20%, 100mW ±20%
        
        ranges
    }
    
    /// Validate multiple power measurements for consistency
    /// 
    /// Analyzes multiple power measurements to detect patterns that might
    /// indicate device communication issues or hardcoded values.
    /// 
    /// # Arguments
    /// * `measurements` - Vector of power measurements to analyze
    /// 
    /// # Returns
    /// * `Result<PowerValidationResult>` - Consistency validation result
    pub fn validate_measurement_consistency(measurements: &[PowerMeasurementData]) -> Result<PowerValidationResult> {
        let mut result = PowerValidationResult::success();
        
        if measurements.len() < 2 {
            result.add_issue(
                "Insufficient measurements for consistency analysis".to_string(),
                Some("Collect measurements from multiple stages for analysis".to_string())
            );
            return Ok(result);
        }
        
        // Check for identical values (hardcoding detection)
        if let Err(e) = super::measurement::PowerMeasurementOperations::validate_measurement_consistency(measurements) {
            result.add_issue(
                format!("Consistency check failed: {}", e),
                Some("This indicates potential hardcoded values or device communication issues. Check device connection and protocol implementation.".to_string())
            );
        }
        
        // Check for reasonable progression between stages
        let mut total_powers: Vec<(u8, f32)> = measurements.iter()
            .map(|m| (m.stage_number, m.raw_power_info.total_power))
            .collect();
        total_powers.sort_by_key(|&(stage, _)| stage);
        
        // Verify that higher stages generally have higher power (with some tolerance)
        for i in 1..total_powers.len() {
            let (prev_stage, prev_power) = total_powers[i-1];
            let (curr_stage, curr_power) = total_powers[i];
            
            // Allow some variation, but higher stages should generally have higher power
            if curr_power < prev_power * 0.8 {
                result.add_issue(
                    format!(
                        "Stage {} power ({:.1}mW) is significantly lower than Stage {} power ({:.1}mW)",
                        curr_stage, curr_power, prev_stage, prev_power
                    ),
                    Some("Verify stage configuration and smart card calibration".to_string())
                );
            }
        }
        
        // Calculate confidence based on consistency
        if result.is_valid {
            result.confidence = 1.0;
        } else {
            result.confidence = 0.3; // Low confidence for consistency issues
        }
        
        Ok(result)
    }
    
    /// Create comprehensive validation report
    /// 
    /// Generates a detailed validation report for debugging power measurement
    /// issues and providing actionable recommendations.
    /// 
    /// # Arguments
    /// * `measurements` - Vector of power measurements to analyze
    /// * `device` - Reference to device for additional context
    /// 
    /// # Returns
    /// * `String` - Comprehensive validation report
    pub fn create_validation_report(
        measurements: &[PowerMeasurementData],
        device: &LumidoxDevice,
    ) -> String {
        let mut report = String::new();
        
        report.push_str("=== POWER VALIDATION REPORT ===\n\n");
        
        // Device readiness check
        report.push_str("Device Readiness:\n");
        match Self::validate_device_ready_for_power_operations(device) {
            Ok(()) => report.push_str("✓ Device is ready for power operations\n"),
            Err(e) => report.push_str(&format!("✗ Device readiness issue: {}\n", e)),
        }
        
        // Individual measurement validation
        report.push_str("\nIndividual Measurement Validation:\n");
        for measurement in measurements {
            match Self::validate_power_readings(measurement) {
                Ok(validation_result) => {
                    report.push_str(&format!("Stage {}: ", measurement.stage_number));
                    if validation_result.is_valid {
                        report.push_str("✓ VALID\n");
                    } else {
                        report.push_str("✗ ISSUES FOUND\n");
                        for issue in &validation_result.issues {
                            report.push_str(&format!("  - {}\n", issue));
                        }
                    }
                }
                Err(e) => {
                    report.push_str(&format!("Stage {}: ✗ VALIDATION ERROR: {}\n", measurement.stage_number, e));
                }
            }
        }
        
        // Consistency validation
        report.push_str("\nConsistency Analysis:\n");
        match Self::validate_measurement_consistency(measurements) {
            Ok(consistency_result) => {
                if consistency_result.is_valid {
                    report.push_str("✓ Measurements are consistent\n");
                } else {
                    report.push_str("✗ Consistency issues detected:\n");
                    for issue in &consistency_result.issues {
                        report.push_str(&format!("  - {}\n", issue));
                    }
                }
            }
            Err(e) => {
                report.push_str(&format!("✗ Consistency analysis error: {}\n", e));
            }
        }
        
        report.push_str("\n=== END VALIDATION REPORT ===\n");
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::models::{PowerInfo, DeviceMode, DeviceInfo};
    use super::super::measurement::PowerMeasurementData;
    use super::super::conversion::ConversionResult;
    
    fn create_test_measurement(stage: u8, total_power: f32, per_power: f32) -> PowerMeasurementData {
        let power_info = PowerInfo {
            total_power,
            total_units: "mW TOTAL RADIANT POWER".to_string(),
            per_power,
            per_units: "mW PER WELL".to_string(),
        };
        let conversion_result = ConversionResult::from_raw_power_info(power_info.clone());
        
        PowerMeasurementData::new(
            stage,
            power_info,
            conversion_result,
            (1500, 2000),
        )
    }
    
    #[test]
    fn test_validation_result_creation() {
        let success = PowerValidationResult::success();
        assert!(success.is_valid);
        assert!(success.issues.is_empty());
        assert_eq!(success.confidence, 1.0);
        
        let failure = PowerValidationResult::failure(
            vec!["Test issue".to_string()],
            vec!["Test recommendation".to_string()]
        );
        assert!(!failure.is_valid);
        assert_eq!(failure.issues.len(), 1);
        assert_eq!(failure.confidence, 0.0);
    }
    
    #[test]
    fn test_validation_result_add_issue() {
        let mut result = PowerValidationResult::success();
        result.add_issue("Test issue".to_string(), Some("Test recommendation".to_string()));
        
        assert!(!result.is_valid);
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.recommendations.len(), 1);
        assert_eq!(result.confidence, 0.0);
    }
    
    #[test]
    fn test_power_ranges() {
        let ranges = PowerValidationOperations::get_expected_power_ranges();
        
        // Check that all stages have ranges defined
        for stage in 1..=5 {
            assert!(ranges.contains_key(&stage));
        }
        
        // Check that Stage 5 has higher ranges than Stage 1
        let (stage1_min_total, _, _, _) = ranges[&1];
        let (stage5_min_total, _, _, _) = ranges[&5];
        assert!(stage5_min_total > stage1_min_total);
    }
    
    #[test]
    fn test_power_reading_validation() {
        // Test valid measurement
        let valid_measurement = create_test_measurement(1, 500.0, 5.0);
        let result = PowerValidationOperations::validate_power_readings(&valid_measurement).unwrap();
        assert!(result.is_valid);
        
        // Test measurement outside expected range
        let invalid_measurement = create_test_measurement(1, 10000.0, 5.0);
        let result = PowerValidationOperations::validate_power_readings(&invalid_measurement).unwrap();
        assert!(!result.is_valid);
        assert!(!result.issues.is_empty());
    }
    
    #[test]
    fn test_consistency_validation() {
        // Test consistent measurements
        let consistent_measurements = vec![
            create_test_measurement(1, 500.0, 5.0),
            create_test_measurement(2, 1000.0, 10.0),
            create_test_measurement(3, 2400.0, 25.0),
        ];
        let result = PowerValidationOperations::validate_measurement_consistency(&consistent_measurements).unwrap();
        assert!(result.is_valid);
        
        // Test identical measurements (should fail)
        let identical_measurements = vec![
            create_test_measurement(1, 500.0, 5.0),
            create_test_measurement(2, 500.0, 5.0),
            create_test_measurement(3, 500.0, 5.0),
        ];
        let result = PowerValidationOperations::validate_measurement_consistency(&identical_measurements).unwrap();
        assert!(!result.is_valid);
    }
    
    #[test]
    fn test_validation_report_formatting() {
        let result = PowerValidationResult::success();
        let report = result.get_report();
        assert!(report.contains("PASSED"));
        assert!(report.contains("100.0%"));
        
        let mut failure_result = PowerValidationResult::failure(
            vec!["Test issue".to_string()],
            vec!["Test recommendation".to_string()]
        );
        let report = failure_result.get_report();
        assert!(report.contains("FAILED"));
        assert!(report.contains("Test issue"));
        assert!(report.contains("Test recommendation"));
    }
}
