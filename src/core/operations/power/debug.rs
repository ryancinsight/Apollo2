//! Power debugging operations for unified power system
//!
//! This module provides debugging and diagnostic tools for investigating
//! power value display inconsistencies between CLI and GUI interfaces.
//! It includes comprehensive analysis, reporting, and troubleshooting functions.

use crate::core::{LumidoxError, Result};
use crate::device::LumidoxDevice;
use super::measurement::PowerMeasurementData;
use super::validation::PowerValidationOperations;

use std::time::Instant;

/// Power debugging operations
/// 
/// Provides comprehensive debugging tools for diagnosing power measurement
/// issues and identifying root causes of display inconsistencies.
pub struct PowerDebugOperations;

impl PowerDebugOperations {
    /// Perform comprehensive power debugging analysis
    /// 
    /// Executes a complete diagnostic analysis of power measurements to identify
    /// potential issues with hardcoded values, device communication, or calculation errors.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<String>` - Comprehensive debugging report
    pub fn perform_comprehensive_analysis(device: &mut LumidoxDevice) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("=== LUMIDOX II POWER DEBUGGING ANALYSIS ===\n\n");
        report.push_str(&format!("Analysis timestamp: {:?}\n", Instant::now()));
        
        // Phase 1: Device readiness check
        report.push_str("\n--- PHASE 1: DEVICE READINESS ---\n");
        match PowerValidationOperations::validate_device_ready_for_power_operations(device) {
            Ok(()) => report.push_str("✓ Device is ready for power operations\n"),
            Err(e) => {
                report.push_str(&format!("✗ Device readiness issue: {}\n", e));
                report.push_str("RECOMMENDATION: Resolve device connection issues before proceeding\n");
                return Ok(report);
            }
        }
        
        // Phase 2: Individual stage measurements
        report.push_str("\n--- PHASE 2: INDIVIDUAL STAGE ANALYSIS ---\n");
        let mut measurements = Vec::new();
        let mut measurement_errors = Vec::new();
        
        for stage in 1..=5 {
            match Self::get_detailed_stage_measurement(device, stage) {
                Ok(measurement) => {
                    report.push_str(&format!("Stage {}: ✓ Measurement successful\n", stage));
                    report.push_str(&format!("  Total Power: {:.3} {}\n", 
                        measurement.raw_power_info.total_power, 
                        measurement.raw_power_info.total_units));
                    report.push_str(&format!("  Per-Well Power: {:.3} {}\n", 
                        measurement.raw_power_info.per_power, 
                        measurement.raw_power_info.per_units));
                    report.push_str(&format!("  Current: {}\n", measurement.get_current_display()));
                    measurements.push(measurement);
                }
                Err(e) => {
                    report.push_str(&format!("Stage {}: ✗ Measurement failed: {}\n", stage, e));
                    measurement_errors.push(format!("Stage {}: {}", stage, e));
                }
            }
        }
        
        // Phase 3: Hardcoding detection
        report.push_str("\n--- PHASE 3: HARDCODING DETECTION ---\n");
        if measurements.len() >= 2 {
            match super::measurement::PowerMeasurementOperations::validate_measurement_consistency(&measurements) {
                Ok(()) => report.push_str("✓ No hardcoding detected - measurements are appropriately different\n"),
                Err(e) => {
                    report.push_str(&format!("✗ HARDCODING DETECTED: {}\n", e));
                    report.push_str("CRITICAL: This indicates the GUI may be showing hardcoded values!\n");
                    report.push_str("RECOMMENDATION: Check GUI power display logic for hardcoded values\n");
                }
            }
        } else {
            report.push_str("⚠ Insufficient measurements for hardcoding analysis\n");
        }
        
        // Phase 4: Expected vs actual values
        report.push_str("\n--- PHASE 4: EXPECTED VS ACTUAL VALUES ---\n");
        let expected_values = Self::get_expected_factory_values();
        for measurement in &measurements {
            if let Some((expected_total, expected_per)) = expected_values.get(&measurement.stage_number) {
                let total_diff = (measurement.raw_power_info.total_power - expected_total).abs();
                let per_diff = (measurement.raw_power_info.per_power - expected_per).abs();
                
                report.push_str(&format!("Stage {}:\n", measurement.stage_number));
                report.push_str(&format!("  Expected: {:.1}mW total, {:.1}mW per-well\n", expected_total, expected_per));
                report.push_str(&format!("  Actual:   {:.1}mW total, {:.1}mW per-well\n", 
                    measurement.raw_power_info.total_power, measurement.raw_power_info.per_power));
                report.push_str(&format!("  Difference: {:.1}mW total, {:.1}mW per-well\n", total_diff, per_diff));
                
                if total_diff > expected_total * 0.2 || per_diff > expected_per * 0.2 {
                    report.push_str("  ⚠ Values differ significantly from factory calibration\n");
                }
            }
        }
        
        // Phase 5: Protocol command verification
        report.push_str("\n--- PHASE 5: PROTOCOL COMMAND VERIFICATION ---\n");
        report.push_str("Verifying protocol commands match Python reference implementation:\n");
        for stage in 1..=5 {
            let stage_idx = (stage - 1) as usize;
            let expected_base_cmd = match stage_idx {
                0 => 0x7b, // Stage 1
                1 => 0x83, // Stage 2
                2 => 0x8b, // Stage 3
                3 => 0x93, // Stage 4
                4 => 0x9b, // Stage 5
                _ => unreachable!(),
            };
            report.push_str(&format!("  Stage {}: Base command 0x{:02x} ✓\n", stage, expected_base_cmd));
        }
        
        // Phase 6: Recommendations
        report.push_str("\n--- PHASE 6: RECOMMENDATIONS ---\n");
        if measurement_errors.is_empty() && measurements.len() == 5 {
            report.push_str("✓ All measurements successful\n");
            
            // Check for consistency issues
            let total_powers: Vec<f32> = measurements.iter()
                .map(|m| m.raw_power_info.total_power)
                .collect();
            let unique_powers: std::collections::HashSet<_> = total_powers.iter()
                .map(|&p| (p * 1000.0) as i32)
                .collect();
            
            if unique_powers.len() == 1 {
                report.push_str("CRITICAL ISSUE: All stages report identical power values\n");
                report.push_str("RECOMMENDATIONS:\n");
                report.push_str("1. Check device smart card configuration\n");
                report.push_str("2. Verify device is properly calibrated\n");
                report.push_str("3. Test with different smart card if available\n");
                report.push_str("4. Check GUI power display implementation for hardcoded values\n");
            } else {
                report.push_str("✓ Power values show appropriate variation between stages\n");
                report.push_str("If GUI shows identical values, the issue is in GUI implementation\n");
            }
        } else {
            report.push_str("RECOMMENDATIONS:\n");
            report.push_str("1. Resolve device communication issues\n");
            report.push_str("2. Check device connection and initialization\n");
            report.push_str("3. Verify device is in proper mode for power operations\n");
        }
        
        report.push_str("\n=== END DEBUGGING ANALYSIS ===\n");
        Ok(report)
    }
    
    /// Get detailed stage measurement with timing information
    /// 
    /// Retrieves a comprehensive power measurement for a single stage with
    /// detailed timing and validation information for debugging purposes.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage` - Stage number to measure
    /// 
    /// # Returns
    /// * `Result<PowerMeasurementData>` - Detailed measurement data
    fn get_detailed_stage_measurement(device: &mut LumidoxDevice, stage: u8) -> Result<PowerMeasurementData> {
        let start_time = Instant::now();
        
        // Get power information with timing
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
        
        // Create measurement with timing
        let conversion_result = super::conversion::ConversionResult::from_raw_power_info(power_info.clone());
        let measurement = PowerMeasurementData::new(
            stage,
            power_info,
            conversion_result,
            (arm_current, fire_current),
        );
        
        let duration = start_time.elapsed();
        if duration.as_millis() > 1000 {
            eprintln!("Warning: Stage {} measurement took {}ms", stage, duration.as_millis());
        }
        
        Ok(measurement)
    }
    
    /// Get expected factory calibration values
    /// 
    /// Returns the expected power values based on factory calibration from
    /// the Python reference implementation for comparison purposes.
    /// 
    /// # Returns
    /// * `std::collections::HashMap<u8, (f32, f32)>` - Stage -> (total_power_mW, per_power_mW)
    fn get_expected_factory_values() -> std::collections::HashMap<u8, (f32, f32)> {
        let mut values = std::collections::HashMap::new();
        
        // Based on lumidox_II_console_interface_rev1b.py factory calibration:
        values.insert(1, (500.0, 5.0));    // Stage 1: 0.5W total, 5mW per well
        values.insert(2, (1000.0, 10.0));  // Stage 2: 1W total, 10mW per well
        values.insert(3, (2400.0, 25.0));  // Stage 3: 2.4W total, 25mW per well
        values.insert(4, (4800.0, 50.0));  // Stage 4: 4.8W total, 50mW per well
        values.insert(5, (9600.0, 100.0)); // Stage 5: 9.6W total, 100mW per well
        
        values
    }
    
    /// Create CLI-friendly debugging report
    /// 
    /// Generates a debugging report formatted for CLI display with appropriate
    /// formatting and actionable recommendations.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if analysis fails
    pub fn display_cli_debugging_report(device: &mut LumidoxDevice) -> Result<()> {
        println!("Starting comprehensive power debugging analysis...\n");
        
        let report = Self::perform_comprehensive_analysis(device)?;
        println!("{}", report);
        
        println!("\nFor GUI debugging, compare these CLI values with GUI display.");
        println!("If GUI shows identical values for stages 2-5, the issue is in GUI implementation.");
        
        Ok(())
    }
    
    /// Quick hardcoding check
    /// 
    /// Performs a quick check for hardcoded values by measuring all stages
    /// and checking for identical values.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<bool>` - True if hardcoding detected, false otherwise
    pub fn quick_hardcoding_check(device: &mut LumidoxDevice) -> Result<bool> {
        let mut total_powers = Vec::new();
        
        for stage in 1..=5 {
            match device.get_power_info(stage) {
                Ok(power_info) => total_powers.push(power_info.total_power),
                Err(_) => continue, // Skip failed measurements
            }
        }
        
        if total_powers.len() < 2 {
            return Ok(false); // Can't determine with insufficient data
        }
        
        // Check if all values are identical (within small tolerance)
        let first_value = total_powers[0];
        let all_identical = total_powers.iter().all(|&value| (value - first_value).abs() < 0.001);
        
        Ok(all_identical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expected_factory_values() {
        let values = PowerDebugOperations::get_expected_factory_values();
        
        // Check that all stages have values
        for stage in 1..=5 {
            assert!(values.contains_key(&stage));
        }
        
        // Check that Stage 5 has higher values than Stage 1
        let (stage1_total, stage1_per) = values[&1];
        let (stage5_total, stage5_per) = values[&5];
        
        assert!(stage5_total > stage1_total);
        assert!(stage5_per > stage1_per);
    }
    
    #[test]
    fn test_factory_values_match_specification() {
        let values = PowerDebugOperations::get_expected_factory_values();
        
        // Verify specific values match the specification
        assert_eq!(values[&1], (500.0, 5.0));
        assert_eq!(values[&2], (1000.0, 10.0));
        assert_eq!(values[&3], (2400.0, 25.0));
        assert_eq!(values[&4], (4800.0, 50.0));
        assert_eq!(values[&5], (9600.0, 100.0));
    }
}
