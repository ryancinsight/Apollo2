//! Irradiance and plate geometry calculations
//!
//! This module provides functionality for calculating mW/cm² irradiance values
//! based on device power readings and plate geometry specifications from the
//! Lumidox II schematic and validated against real measurements.
//!
//! Key validation: User measured 5.5 mW/cm² at 405mA through lid at well bottom.

use crate::core::Result;
use crate::device::models::PowerInfo;

/// Plate geometry specifications
/// 
/// Contains physical dimensions from the Lumidox II schematic for accurate
/// irradiance calculations.
#[derive(Debug, Clone)]
pub struct PlateGeometry {
    /// Plate length in centimeters
    pub plate_length_cm: f32,
    /// Plate width in centimeters  
    pub plate_width_cm: f32,
    /// Total plate area in cm²
    pub total_area_cm2: f32,
    /// Number of wells in the plate (96-well standard)
    pub well_count: u32,
    /// Individual well area in cm²
    pub well_area_cm2: f32,
    /// Well spacing in millimeters
    pub well_spacing_mm: f32,
    /// Well diameter in millimeters (from schematic)
    pub well_diameter_mm: f32,
}

/// Irradiance calculation results
/// 
/// Contains calculated irradiance values for both surface and well-bottom
/// measurements with proper validation against real data.
#[derive(Debug, Clone)]
pub struct IrradianceData {
    /// Surface irradiance in mW/cm²
    pub surface_irradiance_mw_cm2: f32,
    /// Well-bottom irradiance in mW/cm² (with geometry and lid losses)
    pub well_bottom_irradiance_mw_cm2: f32,
    /// Total power used for calculation in mW
    pub total_power_mw: f32,
    /// Plate area in cm²
    pub total_area_cm2: f32,
    /// Number of wells
    pub well_count: u32,
    /// Per-well power in mW (if available)
    pub per_well_power_mw: Option<f32>,
    /// Per-well irradiance in mW/cm² (if available)  
    pub per_well_irradiance_mw_cm2: Option<f32>,
    /// Whether calculation includes lid transmission losses
    pub includes_lid_losses: bool,
}

/// Irradiance calculation utilities
pub struct IrradianceCalculator;

impl IrradianceCalculator {
    /// Get standard plate geometry from schematic
    /// 
    /// Returns the plate geometry specifications based on the actual Lumidox II schematic.
    /// All values are from the device documentation, not estimates.
    /// 
    /// # Returns
    /// * `PlateGeometry` - Schematic-based plate geometry specifications
    /// 
    /// # Example
    /// ```
    /// let geometry = IrradianceCalculator::get_plate_geometry();
    /// println!("Plate area: {:.2} cm²", geometry.total_area_cm2);
    /// ```
    pub fn get_plate_geometry() -> PlateGeometry {
        // Based on the actual Lumidox II schematic dimensions
        let plate_length_mm = 127.75;  // From schematic
        let plate_width_mm = 105.5;    // From schematic
        
        // Convert to cm
        let plate_length_cm = plate_length_mm / 10.0;
        let plate_width_cm = plate_width_mm / 10.0;
        
        // Calculate total area
        let total_area_cm2 = plate_length_cm * plate_width_cm;
        
        // From schematic: 96-well plate (8x12 grid)
        let well_count = 96;
        
        // Actual values from schematic: "∅5.0 (96 PLACES)"
        let well_spacing_mm = 9.0; // Standard 96-well spacing
        let well_diameter_mm = 5.0; // From schematic (not estimated)
        
        // Calculate well area: π × (diameter/2)² converted to cm²
        let well_area_cm2 = std::f32::consts::PI * (well_diameter_mm / 20.0_f32).powi(2);
        
        PlateGeometry {
            plate_length_cm,
            plate_width_cm,
            total_area_cm2,
            well_count,
            well_area_cm2,
            well_spacing_mm,
            well_diameter_mm,
        }
    }
    
    /// Calculate irradiance for power information
    /// 
    /// Calculates both surface and well-bottom irradiance based on power information
    /// and validated geometry. Uses simplified model calibrated against real measurements.
    /// 
    /// # Arguments
    /// * `power_info` - Power information from the device
    /// 
    /// # Returns
    /// * `Result<IrradianceData>` - Calculated irradiance data or error
    /// 
    /// # Example
    /// ```
    /// let irradiance = IrradianceCalculator::calculate_irradiance(&power_info)?;
    /// println!("Surface: {:.1} mW/cm², Wells: {:.1} mW/cm²", 
    ///          irradiance.surface_irradiance_mw_cm2,
    ///          irradiance.well_bottom_irradiance_mw_cm2);
    /// ```
    pub fn calculate_irradiance(power_info: &PowerInfo) -> Result<IrradianceData> {
        Self::calculate_irradiance_with_lid(power_info, true) // Default includes lid losses
    }

    /// Calculate irradiance with configurable lid losses
    /// 
    /// # Arguments
    /// * `power_info` - Power information from the device
    /// * `include_lid_losses` - Whether to include lid transmission losses in well calculations
    /// 
    /// # Returns
    /// * `Result<IrradianceData>` - Calculated irradiance data or error
    pub fn calculate_irradiance_with_lid(
        power_info: &PowerInfo, 
        include_lid_losses: bool
    ) -> Result<IrradianceData> {
        let geometry = Self::get_plate_geometry();
        
        // Convert total power to mW if needed
        let total_power_mw = if power_info.total_units.contains("W TOTAL") && !power_info.total_units.contains("mW") {
            power_info.total_power * 1000.0 // Convert W to mW
        } else {
            power_info.total_power // Assume already in mW
        };
        
        // Calculate surface irradiance
        let surface_irradiance_mw_cm2 = if total_power_mw > 0.0 {
            total_power_mw / geometry.total_area_cm2
        } else {
            0.0
        };
        
        // Calculate per-well power if available
        let per_well_power_mw = if power_info.per_power > 0.0 {
            let per_power_mw = if power_info.per_units.contains("W PER") && !power_info.per_units.contains("mW") {
                power_info.per_power * 1000.0 // Convert W to mW
            } else {
                power_info.per_power
            };
            Some(per_power_mw)
        } else {
            None
        };
        
        // Calculate per-well irradiance if per-well power is available
        let per_well_irradiance_mw_cm2 = per_well_power_mw
            .map(|power| power / geometry.well_area_cm2);
        
        // Calculate well-bottom irradiance using calibrated attenuation
        let well_attenuation_factor = Self::calculate_well_attenuation_factor(include_lid_losses);
        let well_bottom_irradiance_mw_cm2 = surface_irradiance_mw_cm2 * well_attenuation_factor;
        
        Ok(IrradianceData {
            surface_irradiance_mw_cm2,
            well_bottom_irradiance_mw_cm2,
            total_power_mw,
            total_area_cm2: geometry.total_area_cm2,
            well_count: geometry.well_count,
            per_well_power_mw,
            per_well_irradiance_mw_cm2,
            includes_lid_losses: include_lid_losses,
        })
    }
    
    /// Format irradiance for menu display
    /// 
    /// Creates a formatted string showing both surface and well-bottom irradiance.
    /// 
    /// # Arguments
    /// * `irradiance_data` - Calculated irradiance data
    /// 
    /// # Returns
    /// * `String` - Formatted string for menu display
    /// 
    /// # Example
    /// ```
    /// let display = IrradianceCalculator::format_irradiance_display(&irradiance_data);
    /// ```
    pub fn format_irradiance_display(irradiance_data: &IrradianceData) -> String {
        let lid_indicator = if irradiance_data.includes_lid_losses { 
            " (with lid)" 
        } else { 
            "" 
        };
        
        format!(", {:.1} mW/cm² surface, {:.1} mW/cm² wells{}", 
            irradiance_data.surface_irradiance_mw_cm2,
            irradiance_data.well_bottom_irradiance_mw_cm2,
            lid_indicator)
    }
    
    /// Get irradiance display text for power info
    /// 
    /// Convenience method that calculates and formats irradiance for menu display.
    /// 
    /// # Arguments
    /// * `power_info` - Power information from the device
    /// 
    /// # Returns
    /// * `String` - Formatted irradiance string or fallback if calculation fails
    pub fn get_irradiance_display(power_info: &PowerInfo) -> String {
        match Self::calculate_irradiance(power_info) {
            Ok(irradiance_data) => Self::format_irradiance_display(&irradiance_data),
            Err(_) => ", -- mW/cm² irradiance".to_string(),
        }
    }

    /// Estimate power from current using device-calibrated data
    /// 
    /// Uses interpolation between known device stage calibration points to estimate
    /// power for custom current values. Falls back to realistic defaults if no
    /// device data is available.
    /// 
    /// # Arguments
    /// * `current_ma` - Current in milliamps
    /// * `device_calibration_data` - Optional device stage data as (current_mA, total_mW, per_mW) tuples
    /// 
    /// # Returns
    /// * `PowerInfo` - Estimated power information
    /// 
    /// # Example
    /// ```
    /// let power = IrradianceCalculator::estimate_power_from_current(405, None);
    /// ```
    pub fn estimate_power_from_current(
        current_ma: u16, 
        device_calibration_data: Option<&[(u16, f32, f32)]>
    ) -> PowerInfo {
        let calibration_points = if let Some(data) = device_calibration_data {
            // Use actual device calibration data
            let mut points = data.to_vec();
            points.sort_by_key(|&(current, _, _)| current);
            points
        } else {
            // Use realistic calibration values validated against device behavior
            // These values match the working Python implementation and user measurements
            vec![
                (60, 500.0, 5.0),      // Stage 1: 60mA → 500mW total, 5mW per-well
                (110, 1900.0, 19.0),   // Stage 2: 110mA → 1900mW total, 19mW per-well
                (230, 2400.0, 24.0),   // Stage 3: 230mA → 2400mW total, 24mW per-well
                (420, 4800.0, 48.0),   // Stage 4: 420mA → 4800mW total, 48mW per-well
                (795, 9600.0, 96.0),   // Stage 5: 795mA → 9600mW total, 96mW per-well
            ]
        };
        
        let current_f32 = current_ma as f32;
        
        // Handle edge cases
        if calibration_points.is_empty() {
            let efficiency = 2.0; // Conservative 2.0 mW per mA based on typical device behavior
            let total_power = current_f32 * efficiency;
            let per_power = total_power / 96.0;
            
            return PowerInfo {
                total_power,
                total_units: "mW TOTAL RADIANT POWER (ESTIMATED)".to_string(),
                per_power,
                per_units: "mW PER WELL (ESTIMATED)".to_string(),
            };
        }
        
        // Interpolate or extrapolate based on current
        let (estimated_total_power, estimated_per_power) = 
            if current_f32 <= calibration_points[0].0 as f32 {
                // Below lowest point - extrapolate downward
                let (low_current, low_total, low_per) = calibration_points[0];
                let ratio = current_f32 / low_current as f32;
                (low_total * ratio, low_per * ratio)
            } else if current_f32 >= calibration_points.last().unwrap().0 as f32 {
                // Above highest point - extrapolate upward
                let (high_current, high_total, high_per) = *calibration_points.last().unwrap();
                let ratio = current_f32 / high_current as f32;
                (high_total * ratio, high_per * ratio)
            } else {
                // Interpolate between two points
                let mut lower_idx = 0;
                for (i, &(cal_current, _, _)) in calibration_points.iter().enumerate() {
                    if current_f32 > cal_current as f32 {
                        lower_idx = i;
                    } else {
                        break;
                    }
                }
                
                let upper_idx = (lower_idx + 1).min(calibration_points.len() - 1);
                let (low_current, low_total, low_per) = calibration_points[lower_idx];
                let (high_current, high_total, high_per) = calibration_points[upper_idx];
                
                if low_current == high_current {
                    (low_total, low_per)
                } else {
                    // Linear interpolation
                    let current_range = high_current as f32 - low_current as f32;
                    let current_offset = current_f32 - low_current as f32;
                    let factor = current_offset / current_range;
                    
                    let interpolated_total = low_total + (high_total - low_total) * factor;
                    let interpolated_per = low_per + (high_per - low_per) * factor;
                    
                    (interpolated_total, interpolated_per)
                }
            };
        
        PowerInfo {
            total_power: estimated_total_power,
            total_units: "mW TOTAL RADIANT POWER (ESTIMATED)".to_string(),
            per_power: estimated_per_power,
            per_units: "mW PER WELL (ESTIMATED)".to_string(),
        }
    }

    /// Calculate irradiance from current
    /// 
    /// Convenience method that estimates power from current and calculates irradiance.
    /// 
    /// # Arguments
    /// * `current_ma` - Current in milliamps
    /// 
    /// # Returns
    /// * `Result<IrradianceData>` - Calculated irradiance data
    pub fn calculate_irradiance_from_current(current_ma: u16) -> Result<IrradianceData> {
        let power_info = Self::estimate_power_from_current(current_ma, None);
        Self::calculate_irradiance(&power_info)
    }

    /// Extract calibration data from device stage information
    /// 
    /// Converts device stage information into calibration data points for interpolation.
    /// 
    /// # Arguments
    /// * `stage_info_map` - Map of stage information from device
    /// 
    /// # Returns
    /// * `Vec<(u16, f32, f32)>` - Calibration points as (current_mA, total_mW, per_mW)
    pub fn extract_device_calibration_data(
        stage_info_map: &std::collections::HashMap<u8, crate::ui::gui::StageInfo>
    ) -> Vec<(u16, f32, f32)> {
        let mut calibration_points = Vec::new();
        
        for stage_info in stage_info_map.values() {
            if let (Some(current), Some(mut total_power), Some(mut per_power)) = (
                stage_info.fire_current_ma,
                stage_info.total_power,
                stage_info.per_power,
            ) {
                // Convert to mW if needed
                if let Some(ref total_units) = stage_info.total_units {
                    if total_units.contains(" W ") && !total_units.contains("mW") {
                        total_power *= 1000.0;
                    }
                }
                
                if let Some(ref per_units) = stage_info.per_units {
                    if per_units.contains(" W ") && !per_units.contains("mW") {
                        per_power *= 1000.0;
                    }
                }
                
                calibration_points.push((current, total_power, per_power));
            }
        }
        
        // Sort by current for proper interpolation
        calibration_points.sort_by_key(|&(current, _, _)| current);
        calibration_points
    }

    /// Estimate power using device data when available
    /// 
    /// Uses actual device stage data if available, otherwise falls back to defaults.
    /// 
    /// # Arguments
    /// * `current_ma` - Current in milliamps
    /// * `stage_info_map` - Optional device stage information
    /// 
    /// # Returns  
    /// * `PowerInfo` - Estimated power information
    pub fn estimate_power_with_device_data(
        current_ma: u16,
        stage_info_map: Option<&std::collections::HashMap<u8, crate::ui::gui::StageInfo>>
    ) -> PowerInfo {
        if let Some(stage_map) = stage_info_map {
            let calibration_data = Self::extract_device_calibration_data(stage_map);
            if !calibration_data.is_empty() {
                return Self::estimate_power_from_current(current_ma, Some(&calibration_data));
            }
        }
        
        // Fall back to default calibration
        Self::estimate_power_from_current(current_ma, None)
    }

    /// Calculate well attenuation factor
    /// 
    /// Uses a simplified attenuation model calibrated against real measurements.
    /// The user measured 5.5 mW/cm² at 405mA through lid at well bottom.
    /// 
    /// # Arguments
    /// * `include_lid_losses` - Whether to include lid transmission losses
    /// 
    /// # Returns
    /// * `f32` - Attenuation factor (0.0 to 1.0)
    fn calculate_well_attenuation_factor(include_lid_losses: bool) -> f32 {
        // Calibrated attenuation factors based on user measurement validation
        // User measured 5.5 mW/cm² at 405mA through lid at well bottom
        // This corresponds to ~16% of surface irradiance reaching well bottom through lid
        
        let base_attenuation = 0.20; // 20% transmission to well bottom (no lid)
        let lid_transmission = 0.75;  // 75% transmission through lid
        
        if include_lid_losses {
            base_attenuation * lid_transmission // ~15% total transmission
        } else {
            base_attenuation // 20% transmission without lid
        }
    }

    /// Validate model against user measurement
    /// 
    /// Tests the calculation model against the user's real measurement:
    /// 5.5 mW/cm² at 405mA through lid at well bottom.
    /// 
    /// # Returns
    /// * `Result<String>` - Validation report
    pub fn validate_against_measurement() -> Result<String> {
        let mut report = String::new();
        
        report.push_str("=== Model Validation Against User Measurement ===\n");
        report.push_str("Reference: 5.5 mW/cm² at 405mA through lid at well bottom\n\n");
        
        // Calculate for 405mA
        let irradiance_data = Self::calculate_irradiance_from_current(405)?;
        
        report.push_str(&format!("Calculated Results for 405mA:\n"));
        report.push_str(&format!("  Surface irradiance: {:.1} mW/cm²\n", 
                                 irradiance_data.surface_irradiance_mw_cm2));
        report.push_str(&format!("  Well-bottom irradiance: {:.1} mW/cm²\n", 
                                 irradiance_data.well_bottom_irradiance_mw_cm2));
        report.push_str(&format!("  Target measurement: 5.5 mW/cm²\n"));
        
        let error = (irradiance_data.well_bottom_irradiance_mw_cm2 - 5.5).abs();
        let error_percent = (error / 5.5) * 100.0;
        
        report.push_str(&format!("  Error: ±{:.1} mW/cm² ({:.1}%)\n\n", error, error_percent));
        
        if error_percent < 10.0 {
            report.push_str("✓ EXCELLENT: Model matches measurement within 10%\n");
        } else if error_percent < 20.0 {
            report.push_str("✓ GOOD: Model matches measurement within 20%\n");
        } else {
            report.push_str("⚠ FAIR: Model has >20% error, may need calibration\n");
        }
        
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_estimation_405ma() {
        println!("=== Testing Power Calculation for 405mA ===");
        
        // Test fallback calibration
        let power_info = IrradianceCalculator::estimate_power_from_current(405, None);
        println!("Total power: {} {}", power_info.total_power, power_info.total_units);
        println!("Per power: {} {}", power_info.per_power, power_info.per_units);
        
        // Should not be 0
        assert!(power_info.total_power > 0.0, "Total power should not be 0");
        assert!(power_info.per_power > 0.0, "Per power should not be 0");
        
        // Test irradiance calculation
        let irradiance_result = IrradianceCalculator::calculate_irradiance_from_current(405);
        assert!(irradiance_result.is_ok(), "Irradiance calculation should succeed");
        
        let irradiance_data = irradiance_result.unwrap();
        println!("Surface irradiance: {:.1} mW/cm²", irradiance_data.surface_irradiance_mw_cm2);
        println!("Well-bottom irradiance: {:.1} mW/cm²", irradiance_data.well_bottom_irradiance_mw_cm2);
        
        assert!(irradiance_data.surface_irradiance_mw_cm2 > 0.0, "Surface irradiance should not be 0");
        assert!(irradiance_data.well_bottom_irradiance_mw_cm2 > 0.0, "Well-bottom irradiance should not be 0");
    }
    
    #[test]
    fn test_empty_stage_info() {
        println!("=== Testing with Empty Stage Info ===");
        
        let empty_stage_info = std::collections::HashMap::new();
        let power_info = IrradianceCalculator::estimate_power_with_device_data(405, Some(&empty_stage_info));
        
        println!("Total power: {} {}", power_info.total_power, power_info.total_units);
        println!("Per power: {} {}", power_info.per_power, power_info.per_units);
        
        // Should fall back to default calibration and not be 0
        assert!(power_info.total_power > 0.0, "Total power should not be 0 with empty stage info");
        assert!(power_info.per_power > 0.0, "Per power should not be 0 with empty stage info");
    }
}
