//! Stage menu options display for interactive CLI
//!
//! This module handles the display of stage-related menu options including
//! stage firing options with power information and custom current options.
//! It provides specialized display functions for stage-related menu items.
//!
//! The stage options display system provides:
//! - Stage firing options with power information
//! - Custom current firing option with maximum current display
//! - Formatted output for stage selection menu
//! - Integration with device power information queries

use crate::core::Result;
use crate::device::LumidoxDevice;

/// Stage options display utilities and functionality
pub struct StageOptionsDisplay;

impl StageOptionsDisplay {
    /// Display stage firing options with power information
    /// 
    /// Shows stages 1-5 with their power information if available,
    /// providing users with detailed information about each stage's
    /// power characteristics before selection.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for power information queries
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if power information cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// StageOptionsDisplay::display_stage_options(&device)?;
    /// ```
    pub fn display_stage_options(device: &mut LumidoxDevice) -> Result<()> {
        // Display stage options with power info
        for stage in 1..=5 {
            if let Ok(power_info) = device.get_power_info(stage) {
                println!("{}) Turn on stage {}: {} {}, {} {}", 
                    stage, stage, power_info.total_power, power_info.total_units, 
                    power_info.per_power, power_info.per_units);
            } else {
                println!("{}) Turn on stage {}", stage, stage);
            }
        }
        
        Ok(())
    }
    
    /// Display custom current firing option
    /// 
    /// Shows the custom current firing option with maximum current
    /// information if available, helping users understand the current
    /// limits for custom firing operations.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for maximum current query
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if maximum current cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// StageOptionsDisplay::display_custom_current_option(&device)?;
    /// ```
    pub fn display_custom_current_option(device: &mut LumidoxDevice) -> Result<()> {
        if let Ok(max_current) = device.get_max_current() {
            println!("6) Turn on stage with specific current (up to {}mA).", max_current);
        } else {
            println!("6) Turn on stage with specific current.");
        }
        
        Ok(())
    }
    
    /// Display all stage-related menu options
    /// 
    /// Combines stage firing options and custom current option into
    /// a complete stage options display section.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for information queries
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if device information cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// StageOptionsDisplay::display_all_stage_options(&device)?;
    /// ```
    pub fn display_all_stage_options(device: &mut LumidoxDevice) -> Result<()> {
        Self::display_stage_options(device)?;
        Self::display_custom_current_option(device)?;
        Ok(())
    }
    
    /// Get stage option description for a specific stage
    /// 
    /// Returns a formatted description for a specific stage option,
    /// including power information if available.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for power information
    /// * `stage` - Stage number (1-5)
    /// 
    /// # Returns
    /// * `Result<String>` - Formatted stage description or error
    /// 
    /// # Example
    /// ```
    /// let description = StageOptionsDisplay::get_stage_description(&device, 1)?;
    /// println!("{}", description);
    /// ```
    pub fn get_stage_description(device: &mut LumidoxDevice, stage: u8) -> Result<String> {
        if !(1..=5).contains(&stage) {
            return Err(crate::core::LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5.", stage)
            ));
        }
        
        if let Ok(power_info) = device.get_power_info(stage) {
            Ok(format!("{}) Turn on stage {}: {} {}, {} {}", 
                stage, stage, power_info.total_power, power_info.total_units, 
                power_info.per_power, power_info.per_units))
        } else {
            Ok(format!("{}) Turn on stage {}", stage, stage))
        }
    }
    
    /// Get custom current option description
    /// 
    /// Returns a formatted description for the custom current option,
    /// including maximum current information if available.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for maximum current information
    /// 
    /// # Returns
    /// * `Result<String>` - Formatted custom current description or error
    /// 
    /// # Example
    /// ```
    /// let description = StageOptionsDisplay::get_custom_current_description(&device)?;
    /// println!("{}", description);
    /// ```
    pub fn get_custom_current_description(device: &mut LumidoxDevice) -> Result<String> {
        if let Ok(max_current) = device.get_max_current() {
            Ok(format!("6) Turn on stage with specific current (up to {}mA).", max_current))
        } else {
            Ok("6) Turn on stage with specific current.".to_string())
        }
    }
    
    /// Check if a choice is a valid stage option
    /// 
    /// Validates whether a user input choice corresponds to a valid
    /// stage firing option (1-5) or custom current option (6).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is a valid stage option
    /// 
    /// # Example
    /// ```
    /// if StageOptionsDisplay::is_stage_option("3") {
    ///     println!("Valid stage option");
    /// }
    /// ```
    pub fn is_stage_option(choice: &str) -> bool {
        matches!(choice, "1" | "2" | "3" | "4" | "5" | "6")
    }
    
    /// Parse stage number from choice
    /// 
    /// Converts a valid stage choice string to a stage number.
    /// Only works for choices 1-5 (not custom current option 6).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `Option<u8>` - Stage number if valid, None otherwise
    /// 
    /// # Example
    /// ```
    /// if let Some(stage) = StageOptionsDisplay::parse_stage_number("3") {
    ///     println!("Selected stage: {}", stage);
    /// }
    /// ```
    pub fn parse_stage_number(choice: &str) -> Option<u8> {
        match choice {
            "1" | "2" | "3" | "4" | "5" => choice.parse().ok(),
            _ => None,
        }
    }
    
    /// Check if choice is custom current option
    /// 
    /// Determines whether the user choice corresponds to the
    /// custom current firing option.
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is custom current option
    /// 
    /// # Example
    /// ```
    /// if StageOptionsDisplay::is_custom_current_option("6") {
    ///     println!("Custom current option selected");
    /// }
    /// ```
    pub fn is_custom_current_option(choice: &str) -> bool {
        choice == "6"
    }
}
