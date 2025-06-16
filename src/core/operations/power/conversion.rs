//! Power unit conversion operations for unified power system
//!
//! This module provides comprehensive power unit conversion capabilities including
//! mathematical conversions between different power units, validation of conversion
//! factors, and structured conversion results for display purposes.

use crate::core::{LumidoxError, Result};
use crate::device::models::PowerInfo;

/// Supported power units for conversion
///
/// Represents all power units supported by the Lumidox II device and
/// conversion system, matching the Python reference implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerUnit {
    /// Watts total radiant power
    Watts,
    /// Milliwatts total radiant power
    MilliWatts,
    /// Watts per square centimeter total irradiance
    WattsPerCm2,
    /// Milliwatts per square centimeter total irradiance
    MilliWattsPerCm2,
    /// Amperes total current
    Amperes,
    /// Milliamperes total current
    MilliAmperes,
    /// Watts per well
    WattsPerWell,
    /// Milliwatts per well
    MilliWattsPerWell,
    /// Milliwatts per square centimeter per well
    MilliWattsPerCm2PerWell,
    /// Joules per second
    JoulesPerSecond,
    /// Amperes per well
    AmperesPerWell,
    /// Milliamperes per well
    MilliAmperesPerWell,
}

impl PowerUnit {
    /// Get the display string for the power unit
    pub fn display_string(&self) -> String {
        match self {
            PowerUnit::Watts => "W TOTAL RADIANT POWER".to_string(),
            PowerUnit::MilliWatts => "mW TOTAL RADIANT POWER".to_string(),
            PowerUnit::WattsPerCm2 => "W/cm² TOTAL IRRADIANCE".to_string(),
            PowerUnit::MilliWattsPerCm2 => "mW/cm² TOTAL IRRADIANCE".to_string(),
            PowerUnit::Amperes => "A TOTAL CURRENT".to_string(),
            PowerUnit::MilliAmperes => "mA TOTAL CURRENT".to_string(),
            PowerUnit::WattsPerWell => "W PER WELL".to_string(),
            PowerUnit::MilliWattsPerWell => "mW PER WELL".to_string(),
            PowerUnit::MilliWattsPerCm2PerWell => "mW/cm² PER WELL".to_string(),
            PowerUnit::JoulesPerSecond => "J/s".to_string(),
            PowerUnit::AmperesPerWell => "A PER WELL".to_string(),
            PowerUnit::MilliAmperesPerWell => "mA PER WELL".to_string(),
        }
    }
    
    /// Parse power unit from device unit string
    /// 
    /// Converts device unit strings to PowerUnit enum values for conversion operations.
    /// 
    /// # Arguments
    /// * `unit_string` - Unit string from device
    /// 
    /// # Returns
    /// * `Option<PowerUnit>` - Parsed power unit or None if not recognized
    pub fn from_device_string(unit_string: &str) -> Option<PowerUnit> {
        match unit_string {
            "W TOTAL RADIANT POWER" => Some(PowerUnit::Watts),
            "mW TOTAL RADIANT POWER" => Some(PowerUnit::MilliWatts),
            "W/cm² TOTAL IRRADIANCE" => Some(PowerUnit::WattsPerCm2),
            "mW/cm² TOTAL IRRADIANCE" => Some(PowerUnit::MilliWattsPerCm2),
            "A TOTAL CURRENT" => Some(PowerUnit::Amperes),
            "mA TOTAL CURRENT" => Some(PowerUnit::MilliAmperes),
            "W PER WELL" => Some(PowerUnit::WattsPerWell),
            "mW PER WELL" => Some(PowerUnit::MilliWattsPerWell),
            "mW/cm² PER WELL" => Some(PowerUnit::MilliWattsPerCm2PerWell),
            "J/s" => Some(PowerUnit::JoulesPerSecond),
            "A PER WELL" => Some(PowerUnit::AmperesPerWell),
            "mA PER WELL" => Some(PowerUnit::MilliAmperesPerWell),
            _ => None,
        }
    }
}

/// Power conversion result
/// 
/// Contains the results of power unit conversion operations including
/// converted values and their associated unit strings.
#[derive(Debug, Clone)]
pub struct ConversionResult {
    /// Converted total power value
    pub total_power: f32,
    /// Converted total power units
    pub total_units: String,
    /// Converted per-well power value
    pub per_power: f32,
    /// Converted per-well power units
    pub per_units: String,
    /// Original power info for reference
    pub original_power_info: PowerInfo,
    /// Target unit that was converted to
    pub target_unit: Option<PowerUnit>,
}

impl ConversionResult {
    /// Create conversion result from raw power info (no conversion)
    pub fn from_raw_power_info(power_info: PowerInfo) -> Self {
        Self {
            total_power: power_info.total_power,
            total_units: power_info.total_units.clone(),
            per_power: power_info.per_power,
            per_units: power_info.per_units.clone(),
            original_power_info: power_info,
            target_unit: None,
        }
    }
    
    /// Create conversion result with converted values
    pub fn with_conversion(
        original_power_info: PowerInfo,
        total_power: f32,
        total_units: String,
        per_power: f32,
        per_units: String,
        target_unit: PowerUnit,
    ) -> Self {
        Self {
            total_power,
            total_units,
            per_power,
            per_units,
            original_power_info,
            target_unit: Some(target_unit),
        }
    }
}

/// Power unit converter
/// 
/// Provides mathematical unit conversion operations with proper validation
/// and error handling for reliable power unit transformations.
pub struct PowerUnitConverter;

impl PowerUnitConverter {
    /// Convert power information to target unit
    /// 
    /// Performs mathematical unit conversion from device units to target units
    /// using proper conversion factors based on the Python reference implementation.
    /// 
    /// # Arguments
    /// * `power_info` - Original power information from device
    /// * `target_unit` - Target unit for conversion
    /// 
    /// # Returns
    /// * `Result<ConversionResult>` - Converted power values or conversion error
    pub fn convert_power_info(
        power_info: &PowerInfo,
        target_unit: PowerUnit,
    ) -> Result<ConversionResult> {
        // Parse original units
        let original_total_unit = PowerUnit::from_device_string(&power_info.total_units);
        let original_per_unit = PowerUnit::from_device_string(&power_info.per_units);
        
        // Convert total power
        let converted_total_power = if let Some(orig_unit) = original_total_unit {
            Self::convert_value(power_info.total_power, orig_unit, target_unit.clone())?
        } else {
            // If we can't parse the original unit, return the original value
            power_info.total_power
        };
        
        // Convert per-well power
        let converted_per_power = if let Some(orig_unit) = original_per_unit {
            Self::convert_value(power_info.per_power, orig_unit, target_unit.clone())?
        } else {
            // If we can't parse the original unit, return the original value
            power_info.per_power
        };
        
        Ok(ConversionResult::with_conversion(
            power_info.clone(),
            converted_total_power,
            target_unit.display_string(),
            converted_per_power,
            target_unit.display_string(),
            target_unit,
        ))
    }
    
    /// Convert a single power value between units
    /// 
    /// Performs mathematical conversion of a single power value using
    /// conversion factors derived from the Python reference implementation.
    /// 
    /// # Arguments
    /// * `value` - Original power value
    /// * `from_unit` - Original unit
    /// * `to_unit` - Target unit
    /// 
    /// # Returns
    /// * `Result<f32>` - Converted value or conversion error
    fn convert_value(value: f32, from_unit: PowerUnit, to_unit: PowerUnit) -> Result<f32> {
        // If units are the same, no conversion needed
        if from_unit == to_unit {
            return Ok(value);
        }
        
        // Get conversion factor from reference implementation
        let conversion_factor = Self::get_conversion_factor(from_unit, to_unit)?;
        
        Ok(value * conversion_factor)
    }
    
    /// Get conversion factor between two units
    /// 
    /// Returns the mathematical conversion factor to convert from one unit to another,
    /// based on the Lumidox II device specifications and Python reference implementation.
    /// 
    /// # Arguments
    /// * `from_unit` - Source unit
    /// * `to_unit` - Target unit
    /// 
    /// # Returns
    /// * `Result<f32>` - Conversion factor or error if conversion not supported
    fn get_conversion_factor(from_unit: PowerUnit, to_unit: PowerUnit) -> Result<f32> {
        use PowerUnit::*;
        
        match (from_unit, to_unit) {
            // Same unit conversions
            (a, b) if a == b => Ok(1.0),
            
            // Watts <-> MilliWatts conversions
            (Watts, MilliWatts) => Ok(1000.0),
            (MilliWatts, Watts) => Ok(0.001),
            
            // Amperes <-> MilliAmperes conversions
            (Amperes, MilliAmperes) => Ok(1000.0),
            (MilliAmperes, Amperes) => Ok(0.001),
            
            // Per-well conversions
            (WattsPerWell, MilliWattsPerWell) => Ok(1000.0),
            (MilliWattsPerWell, WattsPerWell) => Ok(0.001),
            (AmperesPerWell, MilliAmperesPerWell) => Ok(1000.0),
            (MilliAmperesPerWell, AmperesPerWell) => Ok(0.001),
            
            // Irradiance conversions (W/cm² <-> mW/cm²)
            (WattsPerCm2, MilliWattsPerCm2) => Ok(1000.0),
            (MilliWattsPerCm2, WattsPerCm2) => Ok(0.001),
            
            // Power to current conversions (would require device-specific factors)
            // These would need actual device calibration data
            (MilliWatts, MilliAmperes) => {
                // This is a placeholder - actual conversion would require voltage data
                Err(LumidoxError::InvalidInput(
                    "Power to current conversion requires voltage information".to_string()
                ))
            }
            
            // Unsupported conversions
            _ => Err(LumidoxError::InvalidInput(
                format!("Conversion from {:?} to {:?} is not supported", from_unit, to_unit)
            )),
        }
    }
    
    /// Validate conversion factor accuracy
    /// 
    /// Validates that conversion factors are mathematically correct and
    /// within expected ranges for the Lumidox II device specifications.
    /// 
    /// # Arguments
    /// * `from_unit` - Source unit
    /// * `to_unit` - Target unit
    /// * `factor` - Conversion factor to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if factor is valid, error if invalid
    pub fn validate_conversion_factor(
        from_unit: PowerUnit,
        to_unit: PowerUnit,
        factor: f32,
    ) -> Result<()> {
        // Check for reasonable factor ranges
        if factor <= 0.0 {
            return Err(LumidoxError::InvalidInput(
                "Conversion factor must be positive".to_string()
            ));
        }
        
        if factor > 1_000_000.0 || factor < 0.000001 {
            return Err(LumidoxError::InvalidInput(
                format!("Conversion factor {} is outside reasonable range", factor)
            ));
        }
        
        // Validate specific known conversions
        use PowerUnit::*;
        match (from_unit, to_unit) {
            (Watts, MilliWatts) => {
                if (factor - 1000.0).abs() > 0.001 {
                    return Err(LumidoxError::InvalidInput(
                        format!("Expected W->mW factor of 1000.0, got {}", factor)
                    ));
                }
            }
            (MilliWatts, Watts) => {
                if (factor - 0.001).abs() > 0.000001 {
                    return Err(LumidoxError::InvalidInput(
                        format!("Expected mW->W factor of 0.001, got {}", factor)
                    ));
                }
            }
            _ => {} // Other conversions not validated here
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_power_info() -> PowerInfo {
        PowerInfo {
            total_power: 10.0,
            total_units: "mW TOTAL RADIANT POWER".to_string(),
            per_power: 5.0,
            per_units: "mW PER WELL".to_string(),
        }
    }
    
    #[test]
    fn test_power_unit_display_strings() {
        assert_eq!(PowerUnit::Watts.display_string(), "W TOTAL RADIANT POWER");
        assert_eq!(PowerUnit::MilliWatts.display_string(), "mW TOTAL RADIANT POWER");
        assert_eq!(PowerUnit::WattsPerCm2.display_string(), "W/cm² TOTAL IRRADIANCE");
    }
    
    #[test]
    fn test_power_unit_parsing() {
        assert_eq!(
            PowerUnit::from_device_string("mW TOTAL RADIANT POWER"),
            Some(PowerUnit::MilliWatts)
        );
        assert_eq!(
            PowerUnit::from_device_string("W PER WELL"),
            Some(PowerUnit::WattsPerWell)
        );
        assert_eq!(PowerUnit::from_device_string("UNKNOWN UNIT"), None);
    }
    
    #[test]
    fn test_conversion_factors() {
        assert_eq!(
            PowerUnitConverter::get_conversion_factor(PowerUnit::Watts, PowerUnit::MilliWatts).unwrap(),
            1000.0
        );
        assert_eq!(
            PowerUnitConverter::get_conversion_factor(PowerUnit::MilliWatts, PowerUnit::Watts).unwrap(),
            0.001
        );
        assert_eq!(
            PowerUnitConverter::get_conversion_factor(PowerUnit::MilliWatts, PowerUnit::MilliWatts).unwrap(),
            1.0
        );
    }
    
    #[test]
    fn test_value_conversion() {
        let result = PowerUnitConverter::convert_value(
            10.0,
            PowerUnit::MilliWatts,
            PowerUnit::Watts
        ).unwrap();
        assert!((result - 0.01).abs() < 0.0001);
        
        let result = PowerUnitConverter::convert_value(
            1.0,
            PowerUnit::Watts,
            PowerUnit::MilliWatts
        ).unwrap();
        assert!((result - 1000.0).abs() < 0.001);
    }
    
    #[test]
    fn test_power_info_conversion() {
        let power_info = create_test_power_info();
        let result = PowerUnitConverter::convert_power_info(&power_info, PowerUnit::Watts).unwrap();
        
        assert!((result.total_power - 0.01).abs() < 0.0001);
        assert!((result.per_power - 0.005).abs() < 0.0001);
        assert_eq!(result.total_units, "W TOTAL RADIANT POWER");
        assert_eq!(result.target_unit, Some(PowerUnit::Watts));
    }
    
    #[test]
    fn test_conversion_result_from_raw() {
        let power_info = create_test_power_info();
        let result = ConversionResult::from_raw_power_info(power_info.clone());
        
        assert_eq!(result.total_power, power_info.total_power);
        assert_eq!(result.per_power, power_info.per_power);
        assert_eq!(result.target_unit, None);
    }
    
    #[test]
    fn test_conversion_factor_validation() {
        assert!(PowerUnitConverter::validate_conversion_factor(
            PowerUnit::Watts,
            PowerUnit::MilliWatts,
            1000.0
        ).is_ok());
        
        assert!(PowerUnitConverter::validate_conversion_factor(
            PowerUnit::Watts,
            PowerUnit::MilliWatts,
            -1.0
        ).is_err());
        
        assert!(PowerUnitConverter::validate_conversion_factor(
            PowerUnit::Watts,
            PowerUnit::MilliWatts,
            999.0
        ).is_err());
    }
    
    #[test]
    fn test_unsupported_conversions() {
        assert!(PowerUnitConverter::get_conversion_factor(
            PowerUnit::MilliWatts,
            PowerUnit::MilliAmperes
        ).is_err());
    }
}
