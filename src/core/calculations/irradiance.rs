//! Irradiance and plate geometry calculations
//!
//! This module provides functionality for calculating mW/cm² irradiance values
//! based on device power readings and plate geometry specifications.

use crate::core::Result;
use crate::device::models::PowerInfo;

/// Plate geometry specifications
/// 
/// Contains physical dimensions and specifications for the plate used
/// in irradiance calculations, based on the device schematic.
#[derive(Debug, Clone)]
pub struct PlateGeometry {
    /// Plate length in centimeters
    pub plate_length_cm: f32,
    /// Plate width in centimeters
    pub plate_width_cm: f32,
    /// Total plate area in cm²
    pub total_area_cm2: f32,
    /// Number of wells in the plate
    pub well_count: u32,
    /// Individual well area in cm²
    pub well_area_cm2: f32,
    /// Well spacing in millimeters
    pub well_spacing_mm: f32,
    /// Well diameter in millimeters
    pub well_diameter_mm: f32,
}

/// Irradiance calculation results
/// 
/// Contains calculated irradiance values and related measurements
/// for display and analysis purposes.
#[derive(Debug, Clone)]
pub struct IrradianceData {
    /// Total irradiance in mW/cm²
    pub total_irradiance_mw_cm2: f32,
    /// Total power in mW
    pub total_power_mw: f32,
    /// Total plate area in cm²
    pub total_area_cm2: f32,
    /// Number of wells
    pub well_count: u32,
    /// Per-well irradiance in mW/cm² (if applicable)
    pub per_well_irradiance_mw_cm2: Option<f32>,
}

/// Irradiance calculation utilities
pub struct IrradianceCalculator;

impl IrradianceCalculator {
    /// Get standard plate geometry
    /// 
    /// Returns the plate geometry specifications based on the device schematic.
    /// This corresponds to the get_plate_geometry() function in the Python implementation.
    /// 
    /// # Returns
    /// * `PlateGeometry` - Standard plate geometry specifications
    /// 
    /// # Example
    /// ```
    /// let geometry = IrradianceCalculator::get_plate_geometry();
    /// println!("Plate area: {:.2} cm²", geometry.total_area_cm2);
    /// ```
    pub fn get_plate_geometry() -> PlateGeometry {
        // Based on the schematic dimensions (from Python implementation)
        let plate_length_mm = 127.75;
        let plate_width_mm = 105.5;
        
        // Convert to cm
        let plate_length_cm = plate_length_mm / 10.0;
        let plate_width_cm = plate_width_mm / 10.0;
        
        // Calculate total area
        let total_area_cm2 = plate_length_cm * plate_width_cm;
        
        // From schematic: appears to be 96-well plate (8x12 grid)
        let well_count = 96;
        
        // Estimate well spacing and area (typical 96-well plate)
        let well_spacing_mm = 9.0; // Typical 96-well spacing
        let well_diameter_mm = 6.5; // Typical well diameter
        
        let well_area_cm2 = std::f32::consts::PI * (well_diameter_mm as f32 / 20.0_f32).powi(2); // Convert to cm² and calculate circle area
        
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
    /// Calculates mW/cm² irradiance values based on power information and plate geometry.
    /// This corresponds to the calculate_mw_cm2_for_stage() function in the Python implementation.
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
    /// println!("Total irradiance: {:.3} mW/cm²", irradiance.total_irradiance_mw_cm2);
    /// ```
    pub fn calculate_irradiance(power_info: &PowerInfo) -> Result<IrradianceData> {
        let geometry = Self::get_plate_geometry();
        
        // Convert total power to mW if needed
        let total_power_mw = if power_info.total_units.contains("W TOTAL") && !power_info.total_units.contains("mW") {
            power_info.total_power * 1000.0 // Convert W to mW
        } else if power_info.total_units.contains("mW TOTAL") {
            power_info.total_power
        } else {
            power_info.total_power // Assume mW if unclear
        };
        
        // Calculate irradiance in mW/cm²
        let total_irradiance_mw_cm2 = if total_power_mw > 0.0 {
            total_power_mw / geometry.total_area_cm2
        } else {
            0.0
        };
        
        // Calculate per-well irradiance if per-power is available
        let per_well_irradiance_mw_cm2 = if power_info.per_power > 0.0 {
            let per_power_mw = if power_info.per_units.contains("W PER") && !power_info.per_units.contains("mW") {
                power_info.per_power * 1000.0 // Convert W to mW
            } else {
                power_info.per_power
            };
            Some(per_power_mw / geometry.well_area_cm2)
        } else {
            None
        };
        
        Ok(IrradianceData {
            total_irradiance_mw_cm2,
            total_power_mw,
            total_area_cm2: geometry.total_area_cm2,
            well_count: geometry.well_count,
            per_well_irradiance_mw_cm2,
        })
    }
    
    /// Format irradiance value for menu display
    /// 
    /// Creates a formatted string for displaying irradiance in menu options.
    /// This corresponds to the get_stage_mw_cm2_display() function in the Python implementation.
    /// 
    /// # Arguments
    /// * `irradiance_data` - Calculated irradiance data
    /// 
    /// # Returns
    /// * `String` - Formatted irradiance string for menu display
    /// 
    /// # Example
    /// ```
    /// let display_text = IrradianceCalculator::format_irradiance_for_menu(&irradiance_data);
    /// println!("Menu text: {}", display_text);
    /// ```
    pub fn format_irradiance_for_menu(irradiance_data: &IrradianceData) -> String {
        format!(", {:.3} mW/cm² total irradiance", irradiance_data.total_irradiance_mw_cm2)
    }
    
    /// Get irradiance display text for a given power info
    /// 
    /// Convenience method that combines calculation and formatting for menu display.
    /// Returns a fallback string if calculation fails.
    /// 
    /// # Arguments
    /// * `power_info` - Power information from the device
    /// 
    /// # Returns
    /// * `String` - Formatted irradiance string, or fallback if calculation fails
    /// 
    /// # Example
    /// ```
    /// let display_text = IrradianceCalculator::get_irradiance_display(&power_info);
    /// ```
    pub fn get_irradiance_display(power_info: &PowerInfo) -> String {
        match Self::calculate_irradiance(power_info) {
            Ok(irradiance_data) => Self::format_irradiance_for_menu(&irradiance_data),
            Err(_) => ", -- mW/cm² total irradiance".to_string(),
        }
    }
}
