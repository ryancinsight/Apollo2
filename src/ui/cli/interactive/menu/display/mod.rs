//! Menu display sub-module for interactive CLI
//!
//! This module organizes menu display functionality into specialized components:
//! - `stage_options`: Stage firing and custom current option display
//! - `status_options`: Device status, information, and control option display
//!
//! The display system provides:
//! - Organized menu option display with proper formatting
//! - Separation of stage options from status/control options
//! - Validation utilities for menu choice categorization
//! - Comprehensive menu display coordination

pub mod stage_options;
pub mod status_options;

// Re-export commonly used items for convenience
pub use stage_options::StageOptionsDisplay;
pub use status_options::StatusOptionsDisplay;

use crate::core::Result;
use crate::device::LumidoxDevice;

/// Menu display coordination utilities and functionality
pub struct MenuDisplay;

impl MenuDisplay {
    /// Display the complete interactive menu
    /// 
    /// Shows all menu options in organized sections including stage options,
    /// control options, status options, parameter options, and current control.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for dynamic information display
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if device information cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// MenuDisplay::display_complete_menu(&device)?;
    /// ```
    pub fn display_complete_menu(device: &mut LumidoxDevice) -> Result<()> {
        println!("-- Select an action --");
        println!();
        
        // Display stage options
        StageOptionsDisplay::display_all_stage_options(device)?;
        
        // Display status and control options
        StatusOptionsDisplay::display_all_status_options()?;
        
        println!();
        Ok(())
    }
    
    /// Display menu header
    /// 
    /// Shows the main menu header with action selection prompt.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// MenuDisplay::display_header()?;
    /// ```
    pub fn display_header() -> Result<()> {
        println!("-- Select an action --");
        Ok(())
    }
    
    /// Display input prompt
    /// 
    /// Shows the input prompt asking user to enter their choice.
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error from stdout flush
    /// 
    /// # Example
    /// ```
    /// MenuDisplay::display_input_prompt()?;
    /// ```
    pub fn display_input_prompt() -> Result<()> {
        use std::io::{self, Write};
        print!("Please enter choice number, then press ENTER: ");
        io::stdout().flush()?;
        Ok(())
    }
    
    /// Validate menu choice
    /// 
    /// Checks if a user input choice is valid for any menu option.
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is valid for any menu option
    /// 
    /// # Example
    /// ```
    /// if MenuDisplay::is_valid_choice("5") {
    ///     println!("Valid menu choice");
    /// }
    /// ```
    pub fn is_valid_choice(choice: &str) -> bool {
        StageOptionsDisplay::is_stage_option(choice) || 
        StatusOptionsDisplay::is_valid_status_choice(choice)
    }
    
    /// Get choice category
    /// 
    /// Returns the category of a menu choice for routing to appropriate handlers.
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `Option<&'static str>` - Category name if choice is valid
    /// 
    /// # Example
    /// ```
    /// match MenuDisplay::get_choice_category("3") {
    ///     Some("stage") => println!("Stage option selected"),
    ///     Some(category) => println!("Other category: {}", category),
    ///     None => println!("Invalid choice"),
    /// }
    /// ```
    pub fn get_choice_category(choice: &str) -> Option<&'static str> {
        if StageOptionsDisplay::is_stage_option(choice) {
            Some("stage")
        } else {
            StatusOptionsDisplay::get_choice_category(choice)
        }
    }
    
    /// Display invalid choice message
    /// 
    /// Shows an error message for invalid menu choices.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// MenuDisplay::display_invalid_choice_message()?;
    /// ```
    pub fn display_invalid_choice_message() -> Result<()> {
        println!();
        println!("Not a valid choice. Please try again.");
        println!();
        Ok(())
    }
    
    /// Get option description
    /// 
    /// Returns a description for any valid menu option.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string
    /// * `device` - Reference to device for dynamic descriptions
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - Description if choice is valid, None otherwise
    /// 
    /// # Example
    /// ```
    /// if let Ok(Some(desc)) = MenuDisplay::get_option_description("1", &device) {
    ///     println!("Option 1: {}", desc);
    /// }
    /// ```
    pub fn get_option_description(choice: &str, device: &mut LumidoxDevice) -> Result<Option<String>> {
        if let Some(stage) = StageOptionsDisplay::parse_stage_number(choice) {
            Ok(Some(StageOptionsDisplay::get_stage_description(device, stage)?))
        } else if StageOptionsDisplay::is_custom_current_option(choice) {
            Ok(Some(StageOptionsDisplay::get_custom_current_description(device)?))
        } else {
            Ok(StatusOptionsDisplay::get_option_description(choice))
        }
    }
    
    /// Display menu sections separately
    /// 
    /// Allows displaying individual menu sections for more granular control.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for dynamic information
    /// * `sections` - List of section names to display
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if device information cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// MenuDisplay::display_sections(&device, &["stage", "control"])?;
    /// ```
    pub fn display_sections(device: &mut LumidoxDevice, sections: &[&str]) -> Result<()> {
        for section in sections {
            match *section {
                "header" => Self::display_header()?,
                "stage" => StageOptionsDisplay::display_all_stage_options(device)?,
                "control" => StatusOptionsDisplay::display_control_options()?,
                "status" => StatusOptionsDisplay::display_status_options()?,
                "parameter" => StatusOptionsDisplay::display_parameter_options()?,
                "current_control" => StatusOptionsDisplay::display_current_control_options()?,
                _ => {
                    println!("Unknown section: {}", section);
                }
            }
        }
        Ok(())
    }
    
    /// Get all available menu choices
    /// 
    /// Returns a list of all valid menu choice strings.
    /// 
    /// # Returns
    /// * `Vec<&'static str>` - List of all valid menu choices
    /// 
    /// # Example
    /// ```
    /// let choices = MenuDisplay::get_all_choices();
    /// println!("Available choices: {:?}", choices);
    /// ```
    pub fn get_all_choices() -> Vec<&'static str> {
        vec![
            "1", "2", "3", "4", "5", "6",  // Stage options
            "7", "8", "9",                  // Control options
            "10", "11", "12",               // Status options
            "13", "14", "15",               // Parameter options
            "16"                            // Current control options
        ]
    }
    
    /// Count total menu options
    /// 
    /// Returns the total number of available menu options.
    /// 
    /// # Returns
    /// * `usize` - Total number of menu options
    /// 
    /// # Example
    /// ```
    /// let count = MenuDisplay::count_options();
    /// println!("Total menu options: {}", count);
    /// ```
    pub fn count_options() -> usize {
        Self::get_all_choices().len()
    }
}
