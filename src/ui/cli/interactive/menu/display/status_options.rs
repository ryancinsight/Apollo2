//! Status and information menu options display for interactive CLI
//!
//! This module handles the display of status, information, and control
//! menu options including device status, parameter information, and
//! current control options. It provides specialized display functions
//! for non-stage menu items.
//!
//! The status options display system provides:
//! - Device control options (arm, turn off, quit)
//! - Device status and information options
//! - Stage parameter information options
//! - Current control options
//! - Formatted output for menu organization

use crate::core::Result;

/// Status and information options display utilities and functionality
pub struct StatusOptionsDisplay;

impl StatusOptionsDisplay {
    /// Display device control options
    /// 
    /// Shows basic device control options including arm and turn off.
    /// Quit option is moved to the end after all other options.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// StatusOptionsDisplay::display_control_options()?;
    /// ```
    pub fn display_control_options() -> Result<()> {
        println!("7) Arm device (prepare for firing).");
        println!("8) Turn off device.");
        Ok(())
    }
      /// Display device status and information options
    /// 
    /// Shows options for reading device status, remote mode state,
    /// and current settings information.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// StatusOptionsDisplay::display_status_options()?;
    /// ```
    pub fn display_status_options() -> Result<()> {
        println!("--- Device Status & Information ---");
        println!("9) Show device status.");
        println!("10) Read remote mode state.");
        println!("11) Read ARM/FIRE current settings.");
        Ok(())
    }
      /// Display stage parameter information options
    /// 
    /// Shows options for reading detailed stage parameter information
    /// including complete parameters, ARM current, and voltage parameters.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// StatusOptionsDisplay::display_parameter_options()?;
    /// ```
    pub fn display_parameter_options() -> Result<()> {
        println!("--- Stage Parameter Information ---");
        println!("12) Show complete stage parameters.");
        println!("13) Read stage ARM current.");
        println!("14) Read stage voltage parameters.");
        Ok(())
    }    /// Display current control options
    /// 
    /// Shows options for controlling device current settings
    /// including ARM current configuration.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// StatusOptionsDisplay::display_current_control_options()?;
    /// ```
    pub fn display_current_control_options() -> Result<()> {
        println!("--- Current Control ---");
        println!("15) Set ARM current.");
        Ok(())
    }
    
    /// Display quit option
    /// 
    /// Shows the quit program option at the end of the menu.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// StatusOptionsDisplay::display_quit_option()?;
    /// ```
    pub fn display_quit_option() -> Result<()> {
        println!();
        println!("16) Quit program.");
        Ok(())
    }
      /// Display all status and information options
    /// 
    /// Combines all status, information, and control options into
    /// a complete non-stage options display section.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// StatusOptionsDisplay::display_all_status_options()?;
    /// ```
    pub fn display_all_status_options() -> Result<()> {
        println!();
        Self::display_control_options()?;
        println!();
        Self::display_status_options()?;
        Self::display_parameter_options()?;
        Self::display_current_control_options()?;
        Self::display_quit_option()?;
        Ok(())
    }
    
    /// Get option description for a specific choice
    /// 
    /// Returns a formatted description for a specific menu choice,
    /// providing detailed information about what each option does.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string
    /// 
    /// # Returns
    /// * `Option<String>` - Description if choice is valid, None otherwise
    /// 
    /// # Example
    /// ```
    /// if let Some(desc) = StatusOptionsDisplay::get_option_description("10") {
    ///     println!("Option 10: {}", desc);
    /// }
    /// ```
    pub fn get_option_description(choice: &str) -> Option<String> {
        match choice {
            "7" => Some("Arm device (prepare for firing)".to_string()),
            "8" => Some("Turn off device".to_string()),
            "9" => Some("Show device status".to_string()),
            "10" => Some("Read remote mode state".to_string()),
            "11" => Some("Read ARM/FIRE current settings".to_string()),
            "12" => Some("Show complete stage parameters".to_string()),
            "13" => Some("Read stage ARM current".to_string()),
            "14" => Some("Read stage voltage parameters".to_string()),
            "15" => Some("Set ARM current".to_string()),
            "16" => Some("Quit program".to_string()),
            _ => None,
        }
    }
    
    /// Check if a choice is a valid control option
    /// 
    /// Validates whether a user input choice corresponds to a valid
    /// device control option (7-9).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is a valid control option
    /// 
    /// # Example
    /// ```
    /// if StatusOptionsDisplay::is_control_option("7") {
    ///     println!("Control option selected");
    /// }
    /// ```
    pub fn is_control_option(choice: &str) -> bool {
        matches!(choice, "7" | "8" | "9")
    }
    
    /// Check if a choice is a valid status option
    /// 
    /// Validates whether a user input choice corresponds to a valid
    /// device status or information option (10-12).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is a valid status option
    /// 
    /// # Example
    /// ```
    /// if StatusOptionsDisplay::is_status_option("10") {
    ///     println!("Status option selected");
    /// }
    /// ```
    pub fn is_status_option(choice: &str) -> bool {
        matches!(choice, "10" | "11" | "12")
    }
    
    /// Check if a choice is a valid parameter option
    /// 
    /// Validates whether a user input choice corresponds to a valid
    /// stage parameter information option (13-15).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is a valid parameter option
    /// 
    /// # Example
    /// ```
    /// if StatusOptionsDisplay::is_parameter_option("13") {
    ///     println!("Parameter option selected");
    /// }
    /// ```
    pub fn is_parameter_option(choice: &str) -> bool {
        matches!(choice, "13" | "14" | "15")
    }
    
    /// Check if a choice is a valid current control option
    /// 
    /// Validates whether a user input choice corresponds to a valid
    /// current control option (16).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is a valid current control option
    /// 
    /// # Example
    /// ```
    /// if StatusOptionsDisplay::is_current_control_option("16") {
    ///     println!("Current control option selected");
    /// }
    /// ```
    pub fn is_current_control_option(choice: &str) -> bool {
        choice == "16"
    }
    
    /// Check if a choice is any valid status-related option
    /// 
    /// Validates whether a user input choice corresponds to any
    /// valid status, information, or control option (7-16).
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is any valid status-related option
    /// 
    /// # Example
    /// ```
    /// if StatusOptionsDisplay::is_valid_status_choice("12") {
    ///     println!("Valid status-related choice");
    /// }
    /// ```
    pub fn is_valid_status_choice(choice: &str) -> bool {
        Self::is_control_option(choice) || 
        Self::is_status_option(choice) || 
        Self::is_parameter_option(choice) || 
        Self::is_current_control_option(choice)
    }
    
    /// Get category for a status choice
    /// 
    /// Returns the category name for a given status choice,
    /// helping to organize and route menu selections.
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `Option<&'static str>` - Category name if choice is valid
    /// 
    /// # Example
    /// ```
    /// if let Some(category) = StatusOptionsDisplay::get_choice_category("10") {
    ///     println!("Choice category: {}", category);
    /// }
    /// ```
    pub fn get_choice_category(choice: &str) -> Option<&'static str> {
        if Self::is_control_option(choice) {
            Some("control")
        } else if Self::is_status_option(choice) {
            Some("status")
        } else if Self::is_parameter_option(choice) {
            Some("parameter")
        } else if Self::is_current_control_option(choice) {
            Some("current_control")
        } else {
            None
        }
    }
}
