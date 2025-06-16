//! Power monitoring operations for unified power system
//!
//! This module provides real-time power monitoring capabilities including
//! continuous measurement tracking, status updates, and change detection
//! for responsive GUI updates and CLI monitoring features.

use crate::core::{LumidoxError, Result};
use crate::device::LumidoxDevice;
use super::measurement::PowerMeasurementData;
use super::validation::PowerValidationResult;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Power status data for monitoring
/// 
/// Contains comprehensive power status information for real-time monitoring
/// and display across CLI and GUI interfaces.
#[derive(Debug, Clone)]
pub struct PowerStatusData {
    /// Current power measurements for all stages
    pub stage_measurements: HashMap<u8, PowerMeasurementData>,
    /// Overall system status
    pub system_status: PowerSystemStatus,
    /// Last update timestamp
    pub last_update: Instant,
    /// Update frequency in seconds
    pub update_interval: Duration,
    /// Validation results for current measurements
    pub validation_results: HashMap<u8, PowerValidationResult>,
}

impl PowerStatusData {
    /// Create new power status data
    pub fn new(update_interval: Duration) -> Self {
        Self {
            stage_measurements: HashMap::new(),
            system_status: PowerSystemStatus::Unknown,
            last_update: Instant::now(),
            update_interval,
            validation_results: HashMap::new(),
        }
    }
    
    /// Check if status data needs updating
    pub fn needs_update(&self) -> bool {
        self.last_update.elapsed() >= self.update_interval
    }
    
    /// Get measurement for specific stage
    pub fn get_stage_measurement(&self, stage: u8) -> Option<&PowerMeasurementData> {
        self.stage_measurements.get(&stage)
    }
    
    /// Get all stage measurements sorted by stage number
    pub fn get_all_measurements_sorted(&self) -> Vec<&PowerMeasurementData> {
        let mut measurements: Vec<_> = self.stage_measurements.values().collect();
        measurements.sort_by_key(|m| m.stage_number);
        measurements
    }
    
    /// Check if all stages have recent measurements
    pub fn has_complete_data(&self) -> bool {
        (1..=5).all(|stage| {
            self.stage_measurements.get(&stage)
                .map(|m| m.is_recent(60)) // 1 minute freshness
                .unwrap_or(false)
        })
    }
    
    /// Get summary status string
    pub fn get_status_summary(&self) -> String {
        match self.system_status {
            PowerSystemStatus::Healthy => {
                format!("✓ All {} stages operational", self.stage_measurements.len())
            }
            PowerSystemStatus::Warning(ref msg) => {
                format!("⚠ Warning: {}", msg)
            }
            PowerSystemStatus::Error(ref msg) => {
                format!("✗ Error: {}", msg)
            }
            PowerSystemStatus::Unknown => {
                "? Status unknown".to_string()
            }
        }
    }
}

/// Power system status enumeration
/// 
/// Represents the overall health and operational status of the power system.
#[derive(Debug, Clone)]
pub enum PowerSystemStatus {
    /// All power measurements are healthy and consistent
    Healthy,
    /// Some issues detected but system is operational
    Warning(String),
    /// Significant issues detected that may affect operation
    Error(String),
    /// Status cannot be determined
    Unknown,
}

/// Power monitoring operations
/// 
/// Provides real-time monitoring capabilities for power measurements with
/// automatic status updates and change detection for responsive interfaces.
pub struct PowerMonitoringOperations;

impl PowerMonitoringOperations {
    /// Start continuous power monitoring
    /// 
    /// Initiates continuous monitoring of power measurements with automatic
    /// status updates and validation for real-time interface updates.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `update_interval` - How often to update measurements
    /// 
    /// # Returns
    /// * `Result<PowerStatusData>` - Initial power status data
    pub fn start_monitoring(
        device: &mut LumidoxDevice,
        update_interval: Duration,
    ) -> Result<PowerStatusData> {
        let mut status_data = PowerStatusData::new(update_interval);
        
        // Perform initial measurement update
        Self::update_power_status(device, &mut status_data)?;
        
        Ok(status_data)
    }
    
    /// Update power status data
    /// 
    /// Updates all power measurements and validates the results for
    /// comprehensive status monitoring.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `status_data` - Mutable reference to status data to update
    /// 
    /// # Returns
    /// * `Result<bool>` - True if significant changes detected, false otherwise
    pub fn update_power_status(
        device: &mut LumidoxDevice,
        status_data: &mut PowerStatusData,
    ) -> Result<bool> {
        let start_time = Instant::now();
        let mut significant_changes = false;
        let mut new_measurements = HashMap::new();
        let mut new_validations = HashMap::new();
        let mut errors = Vec::new();
        
        // Update measurements for all stages
        for stage in 1..=5 {
            match Self::get_stage_measurement_with_validation(device, stage) {
                Ok((measurement, validation)) => {
                    // Check for significant changes
                    if let Some(old_measurement) = status_data.stage_measurements.get(&stage) {
                        if Self::has_significant_change(old_measurement, &measurement) {
                            significant_changes = true;
                        }
                    } else {
                        significant_changes = true; // New measurement
                    }
                    
                    new_measurements.insert(stage, measurement);
                    new_validations.insert(stage, validation);
                }
                Err(e) => {
                    errors.push(format!("Stage {}: {}", stage, e));
                }
            }
        }
        
        // Update status data
        status_data.stage_measurements = new_measurements;
        status_data.validation_results = new_validations;
        status_data.last_update = start_time;
        
        // Determine overall system status
        status_data.system_status = Self::determine_system_status(&status_data.validation_results, &errors);
        
        Ok(significant_changes)
    }
    
    /// Get single stage measurement with validation
    /// 
    /// Retrieves power measurement for a single stage and performs validation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage` - Stage number to measure
    /// 
    /// # Returns
    /// * `Result<(PowerMeasurementData, PowerValidationResult)>` - Measurement and validation
    fn get_stage_measurement_with_validation(
        device: &mut LumidoxDevice,
        stage: u8,
    ) -> Result<(PowerMeasurementData, PowerValidationResult)> {
        // Get raw power information
        let power_info = device.get_power_info(stage)
            .map_err(|e| LumidoxError::DeviceError(
                format!("Failed to get power info for stage {}: {}", stage, e)
            ))?;
        
        // Get current values
        let arm_current = device.get_stage_arm_current(stage).unwrap_or(0);
        let stage_params = device.get_stage_parameters(stage)
            .map_err(|e| LumidoxError::DeviceError(
                format!("Failed to get stage {} parameters: {}", stage, e)
            ))?;
        let fire_current = stage_params.fire_current_ma;
        
        // Create measurement data
        let conversion_result = super::conversion::ConversionResult::from_raw_power_info(power_info.clone());
        let measurement = PowerMeasurementData::new(
            stage,
            power_info,
            conversion_result,
            (arm_current, fire_current),
        );
        
        // Validate measurement
        let validation = super::validation::PowerValidationOperations::validate_power_readings(&measurement)
            .unwrap_or_else(|_| super::validation::PowerValidationResult::failure(
                vec!["Validation failed".to_string()],
                vec!["Check device connection".to_string()]
            ));
        
        Ok((measurement, validation))
    }
    
    /// Check for significant changes between measurements
    /// 
    /// Determines if there are significant changes between two measurements
    /// that warrant interface updates or notifications.
    /// 
    /// # Arguments
    /// * `old` - Previous measurement
    /// * `new` - New measurement
    /// 
    /// # Returns
    /// * `bool` - True if significant changes detected
    fn has_significant_change(old: &PowerMeasurementData, new: &PowerMeasurementData) -> bool {
        // Define thresholds for significant changes
        const POWER_THRESHOLD: f32 = 0.1; // 10% change
        const CURRENT_THRESHOLD: u16 = 50; // 50mA change
        
        // Check power changes
        let total_power_change = (new.raw_power_info.total_power - old.raw_power_info.total_power).abs() 
            / old.raw_power_info.total_power.max(0.001);
        let per_power_change = (new.raw_power_info.per_power - old.raw_power_info.per_power).abs() 
            / old.raw_power_info.per_power.max(0.001);
        
        if total_power_change > POWER_THRESHOLD || per_power_change > POWER_THRESHOLD {
            return true;
        }
        
        // Check current changes
        let arm_change = (new.current_ma.0 as i32 - old.current_ma.0 as i32).abs() as u16;
        let fire_change = (new.current_ma.1 as i32 - old.current_ma.1 as i32).abs() as u16;
        
        if arm_change > CURRENT_THRESHOLD || fire_change > CURRENT_THRESHOLD {
            return true;
        }
        
        false
    }
    
    /// Determine overall system status
    /// 
    /// Analyzes validation results to determine the overall health status
    /// of the power measurement system.
    /// 
    /// # Arguments
    /// * `validations` - Validation results for all stages
    /// * `errors` - List of errors encountered during measurement
    /// 
    /// # Returns
    /// * `PowerSystemStatus` - Overall system status
    fn determine_system_status(
        validations: &HashMap<u8, PowerValidationResult>,
        errors: &[String],
    ) -> PowerSystemStatus {
        if !errors.is_empty() {
            return PowerSystemStatus::Error(
                format!("Measurement errors: {}", errors.join(", "))
            );
        }
        
        let mut warning_count = 0;
        let mut error_count = 0;
        
        for validation in validations.values() {
            if !validation.is_valid {
                if validation.confidence < 0.5 {
                    error_count += 1;
                } else {
                    warning_count += 1;
                }
            }
        }
        
        if error_count > 0 {
            PowerSystemStatus::Error(
                format!("{} stages have validation errors", error_count)
            )
        } else if warning_count > 0 {
            PowerSystemStatus::Warning(
                format!("{} stages have validation warnings", warning_count)
            )
        } else if validations.len() == 5 {
            PowerSystemStatus::Healthy
        } else {
            PowerSystemStatus::Unknown
        }
    }
    
    /// Create monitoring report
    /// 
    /// Generates a comprehensive monitoring report for debugging and
    /// status analysis purposes.
    /// 
    /// # Arguments
    /// * `status_data` - Current power status data
    /// 
    /// # Returns
    /// * `String` - Formatted monitoring report
    pub fn create_monitoring_report(status_data: &PowerStatusData) -> String {
        let mut report = String::new();
        
        report.push_str("=== POWER MONITORING REPORT ===\n\n");
        
        report.push_str(&format!("System Status: {}\n", status_data.get_status_summary()));
        report.push_str(&format!("Last Update: {:?} ago\n", status_data.last_update.elapsed()));
        report.push_str(&format!("Update Interval: {:?}\n", status_data.update_interval));
        report.push_str(&format!("Complete Data: {}\n", 
            if status_data.has_complete_data() { "Yes" } else { "No" }));
        
        report.push_str("\nStage Measurements:\n");
        for stage in 1..=5 {
            if let Some(measurement) = status_data.get_stage_measurement(stage) {
                report.push_str(&format!("  Stage {}: {:.1} {} ({:.1} {})\n",
                    stage,
                    measurement.raw_power_info.total_power,
                    measurement.raw_power_info.total_units,
                    measurement.raw_power_info.per_power,
                    measurement.raw_power_info.per_units
                ));
                
                if let Some(validation) = status_data.validation_results.get(&stage) {
                    if !validation.is_valid {
                        report.push_str(&format!("    ⚠ {} issues detected\n", validation.issues.len()));
                    }
                }
            } else {
                report.push_str(&format!("  Stage {}: No data\n", stage));
            }
        }
        
        report.push_str("\n=== END MONITORING REPORT ===\n");
        report
    }
    
    /// Check if monitoring data indicates hardcoding issue
    /// 
    /// Analyzes monitoring data to detect patterns that suggest hardcoded
    /// values or device communication issues.
    /// 
    /// # Arguments
    /// * `status_data` - Current power status data
    /// 
    /// # Returns
    /// * `Option<String>` - Description of hardcoding issue if detected
    pub fn detect_hardcoding_issue(status_data: &PowerStatusData) -> Option<String> {
        let measurements = status_data.get_all_measurements_sorted();
        
        if measurements.len() < 2 {
            return None;
        }
        
        // Check for identical total power values
        let total_powers: Vec<f32> = measurements.iter()
            .map(|m| m.raw_power_info.total_power)
            .collect();
        
        let unique_total_powers: std::collections::HashSet<_> = total_powers.iter()
            .map(|&p| (p * 1000.0) as i32) // Convert to integer for comparison
            .collect();
        
        if unique_total_powers.len() == 1 && measurements.len() > 1 {
            return Some(format!(
                "All {} stages report identical total power ({:.3}), indicating hardcoded values or device communication failure",
                measurements.len(),
                total_powers[0]
            ));
        }
        
        // Check for identical per-well power values
        let per_powers: Vec<f32> = measurements.iter()
            .map(|m| m.raw_power_info.per_power)
            .collect();
        
        let unique_per_powers: std::collections::HashSet<_> = per_powers.iter()
            .map(|&p| (p * 1000.0) as i32)
            .collect();
        
        if unique_per_powers.len() == 1 && measurements.len() > 1 {
            return Some(format!(
                "All {} stages report identical per-well power ({:.3}), indicating hardcoded values or device communication failure",
                measurements.len(),
                per_powers[0]
            ));
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::models::PowerInfo;
    use super::super::measurement::PowerMeasurementData;
    use super::super::conversion::ConversionResult;
    use super::super::validation::PowerValidationResult;
    
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
    fn test_power_status_data_creation() {
        let status_data = PowerStatusData::new(Duration::from_secs(5));
        assert_eq!(status_data.update_interval, Duration::from_secs(5));
        assert!(status_data.stage_measurements.is_empty());
    }
    
    #[test]
    fn test_needs_update() {
        let mut status_data = PowerStatusData::new(Duration::from_millis(1));
        assert!(!status_data.needs_update()); // Should not need update immediately
        
        std::thread::sleep(Duration::from_millis(2));
        assert!(status_data.needs_update()); // Should need update after interval
    }
    
    #[test]
    fn test_has_complete_data() {
        let mut status_data = PowerStatusData::new(Duration::from_secs(5));
        assert!(!status_data.has_complete_data());
        
        // Add measurements for all stages
        for stage in 1..=5 {
            let measurement = create_test_measurement(stage, stage as f32 * 100.0, stage as f32 * 10.0);
            status_data.stage_measurements.insert(stage, measurement);
        }
        
        assert!(status_data.has_complete_data());
    }
    
    #[test]
    fn test_significant_change_detection() {
        let old_measurement = create_test_measurement(1, 100.0, 10.0);
        
        // Small change - should not be significant
        let small_change = create_test_measurement(1, 105.0, 10.5);
        assert!(!PowerMonitoringOperations::has_significant_change(&old_measurement, &small_change));
        
        // Large change - should be significant
        let large_change = create_test_measurement(1, 150.0, 15.0);
        assert!(PowerMonitoringOperations::has_significant_change(&old_measurement, &large_change));
    }
    
    #[test]
    fn test_system_status_determination() {
        let mut validations = HashMap::new();
        let errors = Vec::new();
        
        // All valid - should be healthy
        for stage in 1..=5 {
            validations.insert(stage, PowerValidationResult::success());
        }
        
        let status = PowerMonitoringOperations::determine_system_status(&validations, &errors);
        assert!(matches!(status, PowerSystemStatus::Healthy));
        
        // Add an error - should be error status
        let errors = vec!["Test error".to_string()];
        let status = PowerMonitoringOperations::determine_system_status(&validations, &errors);
        assert!(matches!(status, PowerSystemStatus::Error(_)));
    }
    
    #[test]
    fn test_hardcoding_detection() {
        let mut status_data = PowerStatusData::new(Duration::from_secs(5));
        
        // Add identical measurements (should detect hardcoding)
        for stage in 1..=3 {
            let measurement = create_test_measurement(stage, 100.0, 10.0); // Same values
            status_data.stage_measurements.insert(stage, measurement);
        }
        
        let issue = PowerMonitoringOperations::detect_hardcoding_issue(&status_data);
        assert!(issue.is_some());
        assert!(issue.unwrap().contains("identical"));
        
        // Add different measurements (should not detect hardcoding)
        status_data.stage_measurements.clear();
        for stage in 1..=3 {
            let measurement = create_test_measurement(stage, stage as f32 * 100.0, stage as f32 * 10.0);
            status_data.stage_measurements.insert(stage, measurement);
        }
        
        let issue = PowerMonitoringOperations::detect_hardcoding_issue(&status_data);
        assert!(issue.is_none());
    }
    
    #[test]
    fn test_monitoring_report_generation() {
        let mut status_data = PowerStatusData::new(Duration::from_secs(5));
        status_data.system_status = PowerSystemStatus::Healthy;
        
        // Add some test measurements
        for stage in 1..=2 {
            let measurement = create_test_measurement(stage, stage as f32 * 100.0, stage as f32 * 10.0);
            status_data.stage_measurements.insert(stage, measurement);
            status_data.validation_results.insert(stage, PowerValidationResult::success());
        }
        
        let report = PowerMonitoringOperations::create_monitoring_report(&status_data);
        assert!(report.contains("POWER MONITORING REPORT"));
        assert!(report.contains("System Status"));
        assert!(report.contains("Stage 1:"));
        assert!(report.contains("Stage 2:"));
    }
}
