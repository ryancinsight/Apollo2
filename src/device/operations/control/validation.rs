//! Input validation operations for Lumidox II Controller
//!
//! This module provides functions for validating device inputs
//! including current limits, stage numbers, and parameter ranges.

use crate::core::{LumidoxError, Result};

/// Validate stage number is within valid range (1-5)
pub fn validate_stage_number(stage_num: u8) -> Result<()> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}. Must be 1-5", stage_num)
        ));
    }
    Ok(())
}

/// Validate current value against maximum allowed
pub fn validate_current(current_ma: u16, max_current_ma: u16) -> Result<()> {
    if current_ma > max_current_ma {
        return Err(LumidoxError::InvalidInput(
            format!("Cannot set current above {}mA (requested: {}mA)", max_current_ma, current_ma)
        ));
    }
    Ok(())
}

/// Validate current value is not zero for firing operations
pub fn validate_non_zero_current(current_ma: u16) -> Result<()> {
    if current_ma == 0 {
        return Err(LumidoxError::InvalidInput(
            "Cannot fire with zero current".to_string()
        ));
    }
    Ok(())
}
