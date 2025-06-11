//! Menu option validation for interactive CLI
//!
//! This module handles validation of user menu selections and input
//! processing to ensure valid menu navigation and option selection.
//! 
//! The validation system provides:
//! - Menu option range validation
//! - Input format validation
//! - User choice categorization
//! - Error handling for invalid selections

/// Menu option categories for better organization
/// 
/// This enum categorizes menu options by their functional purpose
/// to enable specialized handling and validation logic.
#[derive(Debug, Clone, PartialEq)]
pub enum MenuOptionCategory {
    /// Stage firing options (1-5)
    StageFiring,
    /// Device control options (6-9)
    DeviceControl,
    /// Device status options (10-12)
    DeviceStatus,
    /// Stage parameter options (13-15)
    StageParameters,
    /// Current control options (16)
    CurrentControl,
    /// Invalid or unrecognized option
    Invalid,
}

/// Menu validation utilities and helper functions
pub struct MenuValidation;

impl MenuValidation {
    /// Validate a menu choice string and return the parsed option
    /// 
    /// This function validates user input for menu selection and
    /// returns the parsed choice as a u8 if valid.
    /// 
    /// # Arguments
    /// * `choice` - The user input string to validate
    /// 
    /// # Returns
    /// * `Some(u8)` - Valid menu option number
    /// * `None` - Invalid input that cannot be parsed
    /// 
    /// # Example
    /// ```
    /// let choice = MenuValidation::validate_choice("5");
    /// assert_eq!(choice, Some(5));
    /// 
    /// let invalid = MenuValidation::validate_choice("abc");
    /// assert_eq!(invalid, None);
    /// ```
    pub fn validate_choice(choice: &str) -> Option<u8> {
        choice.trim().parse::<u8>().ok()
    }
    
    /// Categorize a menu option by its number
    /// 
    /// Determines which functional category a menu option belongs to
    /// based on its numeric value.
    /// 
    /// # Arguments
    /// * `option` - The menu option number to categorize
    /// 
    /// # Returns
    /// * `MenuOptionCategory` - The category of the menu option
    /// 
    /// # Example
    /// ```
    /// let category = MenuValidation::categorize_option(3);
    /// assert_eq!(category, MenuOptionCategory::StageFiring);
    /// 
    /// let category = MenuValidation::categorize_option(10);
    /// assert_eq!(category, MenuOptionCategory::DeviceStatus);
    /// ```
    pub fn categorize_option(option: u8) -> MenuOptionCategory {
        match option {
            1..=5 => MenuOptionCategory::StageFiring,
            6..=9 => MenuOptionCategory::DeviceControl,
            10..=12 => MenuOptionCategory::DeviceStatus,
            13..=15 => MenuOptionCategory::StageParameters,
            16 => MenuOptionCategory::CurrentControl,
            _ => MenuOptionCategory::Invalid,
        }
    }
    
    /// Validate if a menu option is within the valid range
    /// 
    /// Checks if the provided option number is within the valid
    /// range of available menu options (1-16).
    /// 
    /// # Arguments
    /// * `option` - The menu option number to validate
    /// 
    /// # Returns
    /// * `bool` - True if the option is valid, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(MenuValidation::is_valid_option(5));
    /// assert!(MenuValidation::is_valid_option(16));
    /// assert!(!MenuValidation::is_valid_option(0));
    /// assert!(!MenuValidation::is_valid_option(17));
    /// ```
    pub fn is_valid_option(option: u8) -> bool {
        (1..=16).contains(&option)
    }
    
    /// Validate stage number input
    /// 
    /// Validates that a stage number is within the valid range (1-5)
    /// for stage-specific operations.
    /// 
    /// # Arguments
    /// * `stage` - The stage number to validate
    /// 
    /// # Returns
    /// * `bool` - True if the stage number is valid, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(MenuValidation::is_valid_stage(3));
    /// assert!(!MenuValidation::is_valid_stage(0));
    /// assert!(!MenuValidation::is_valid_stage(6));
    /// ```
    pub fn is_valid_stage(stage: u8) -> bool {
        (1..=5).contains(&stage)
    }
    
    /// Validate current value input
    /// 
    /// Validates that a current value is within reasonable bounds
    /// for device operation (non-zero and within typical limits).
    /// 
    /// # Arguments
    /// * `current_ma` - The current value in milliamps to validate
    /// 
    /// # Returns
    /// * `bool` - True if the current value is valid, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(MenuValidation::is_valid_current(1000));
    /// assert!(!MenuValidation::is_valid_current(0));
    /// assert!(!MenuValidation::is_valid_current(100000));
    /// ```
    pub fn is_valid_current(current_ma: u16) -> bool {
        current_ma > 0 && current_ma <= 10000 // Reasonable upper limit
    }
    
    /// Get a description of a menu option category
    /// 
    /// Returns a human-readable description of what a menu option
    /// category represents for user feedback and help text.
    /// 
    /// # Arguments
    /// * `category` - The menu option category to describe
    /// 
    /// # Returns
    /// * `&'static str` - Description of the category
    /// 
    /// # Example
    /// ```
    /// let desc = MenuValidation::category_description(MenuOptionCategory::StageFiring);
    /// assert_eq!(desc, "Stage firing operations");
    /// ```
    pub fn category_description(category: MenuOptionCategory) -> &'static str {
        match category {
            MenuOptionCategory::StageFiring => "Stage firing operations",
            MenuOptionCategory::DeviceControl => "Device control operations",
            MenuOptionCategory::DeviceStatus => "Device status and information",
            MenuOptionCategory::StageParameters => "Stage parameter information",
            MenuOptionCategory::CurrentControl => "Current control operations",
            MenuOptionCategory::Invalid => "Invalid option",
        }
    }
    
    /// Check if an option requires additional user input
    /// 
    /// Determines whether a menu option will require additional
    /// input from the user (such as stage numbers or current values).
    /// 
    /// # Arguments
    /// * `option` - The menu option number to check
    /// 
    /// # Returns
    /// * `bool` - True if additional input is required, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(MenuValidation::requires_additional_input(6)); // Current input
    /// assert!(MenuValidation::requires_additional_input(13)); // Stage input
    /// assert!(!MenuValidation::requires_additional_input(7)); // Direct action
    /// ```
    pub fn requires_additional_input(option: u8) -> bool {
        matches!(option, 6 | 13 | 14 | 15 | 16)
    }
}
