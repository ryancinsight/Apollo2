//! Menu action handlers sub-module for interactive CLI
//!
//! This module organizes menu action handlers into specialized components:
//! - `stage_actions`: Stage firing and custom current action handlers
//! - `device_actions`: Device control action handlers (arm, turn off, shutdown)
//! - `info_actions`: Information retrieval and status display handlers
//!
//! The handlers system provides:
//! - Specialized action execution for different menu categories
//! - Consistent error handling and user feedback
//! - Input validation and retry logic
//! - Integration with device operations and status queries

pub mod stage_actions;
pub mod device_actions;
pub mod info_actions;

// Re-export commonly used items for convenience
pub use stage_actions::StageActionHandlers;
pub use device_actions::DeviceActionHandlers;
pub use info_actions::InfoActionHandlers;

use crate::core::Result;
use crate::device::LumidoxDevice;

/// Menu action handlers coordination utilities and functionality
pub struct MenuActionHandlers;

impl MenuActionHandlers {
    /// Handle menu choice with appropriate action handler
    /// 
    /// Routes menu choices to the appropriate specialized handler based
    /// on the choice category and executes the corresponding action.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<Option<bool>>` - Some(bool) if handled, None if invalid choice
    /// 
    /// # Example
    /// ```
    /// if let Some(continue_menu) = MenuActionHandlers::handle_choice(&mut device, "3")? {
    ///     // Choice was handled successfully
    /// }
    /// ```
    pub fn handle_choice(device: &mut LumidoxDevice, choice: &str) -> Result<Option<bool>> {
        // Try stage actions first (choices 1-6)
        if let Some(result) = StageActionHandlers::handle_stage_choice(device, choice)? {
            return Ok(Some(result));
        }
          // Try device control actions (choices 7-8)
        if let Some(result) = DeviceActionHandlers::handle_device_choice(device, choice)? {
            return Ok(Some(result));
        }
        
        // Try information actions (choices 9-16)
        if let Some(result) = InfoActionHandlers::handle_info_choice(device, choice)? {
            return Ok(Some(result));
        }
        
        // Choice not handled by any handler
        Ok(None)
    }
    
    /// Handle menu choice with enhanced status checking
    /// 
    /// Enhanced version that includes device status checks before executing
    /// certain operations for improved safety and user feedback.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<Option<bool>>` - Some(bool) if handled, None if invalid choice
    /// 
    /// # Example
    /// ```
    /// if let Some(continue_menu) = MenuActionHandlers::handle_choice_with_status(&mut device, "7")? {
    ///     // Choice was handled with status checking
    /// }
    /// ```
    pub fn handle_choice_with_status(device: &mut LumidoxDevice, choice: &str) -> Result<Option<bool>> {
        // Try stage actions first (choices 1-6)
        if let Some(result) = StageActionHandlers::handle_stage_choice(device, choice)? {
            return Ok(Some(result));
        }
          // Try device control actions with status checking (choices 7-8)
        if let Some(result) = DeviceActionHandlers::handle_device_choice_with_status(device, choice)? {
            return Ok(Some(result));
        }
        
        // Try information actions (choices 9-16)
        if let Some(result) = InfoActionHandlers::handle_info_choice(device, choice)? {
            return Ok(Some(result));
        }
        
        // Choice not handled by any handler
        Ok(None)
    }
    
    /// Get choice category for routing
    /// 
    /// Determines the category of a menu choice for routing to appropriate handlers.
    /// 
    /// # Arguments
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Option<&'static str>` - Category name if choice is valid
    /// 
    /// # Example
    /// ```
    /// match MenuActionHandlers::get_choice_category("5") {
    ///     Some("stage") => println!("Stage action"),
    ///     Some("device") => println!("Device control action"),
    ///     Some("info") => println!("Information action"),
    ///     None => println!("Invalid choice"),
    /// }
    /// ```
    pub fn get_choice_category(choice: &str) -> Option<&'static str> {
        match choice {
            "1" | "2" | "3" | "4" | "5" | "6" => Some("stage"),
            "7" | "8" | "9" => Some("device"),
            "10" | "11" | "12" | "13" | "14" | "15" | "16" => Some("info"),
            _ => None,
        }
    }
    
    /// Validate choice format
    /// 
    /// Validates that a choice string is in the correct format for menu processing.
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice format is valid
    /// 
    /// # Example
    /// ```
    /// if MenuActionHandlers::is_valid_choice_format("10") {
    ///     println!("Valid choice format");
    /// }
    /// ```
    pub fn is_valid_choice_format(choice: &str) -> bool {
        !choice.trim().is_empty() && choice.trim().chars().all(|c| c.is_ascii_digit())
    }
    
    /// Get action description for a choice
    /// 
    /// Returns a human-readable description of what action will be performed
    /// for a given menu choice.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string
    /// 
    /// # Returns
    /// * `Option<&'static str>` - Action description if choice is valid
    /// 
    /// # Example
    /// ```
    /// if let Some(desc) = MenuActionHandlers::get_action_description("7") {
    ///     println!("Action: {}", desc);
    /// }
    /// ```
    pub fn get_action_description(choice: &str) -> Option<&'static str> {
        match choice {
            "1" => Some("Fire stage 1"),
            "2" => Some("Fire stage 2"),
            "3" => Some("Fire stage 3"),
            "4" => Some("Fire stage 4"),
            "5" => Some("Fire stage 5"),
            "6" => Some("Fire with custom current"),
            "7" => Some("Arm device"),
            "8" => Some("Turn off device"),
            "9" => Some("Shutdown and quit"),
            "10" => Some("Show device status"),
            "11" => Some("Read remote mode state"),
            "12" => Some("Read current settings"),
            "13" => Some("Show stage parameters"),
            "14" => Some("Read stage ARM current"),
            "15" => Some("Read stage voltage parameters"),
            "16" => Some("Set ARM current"),
            _ => None,
        }
    }
    
    /// Check if choice requires device interaction
    /// 
    /// Determines whether a menu choice requires active device communication.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice requires device interaction
    /// 
    /// # Example
    /// ```
    /// if MenuActionHandlers::requires_device_interaction("7") {
    ///     println!("This action will communicate with the device");
    /// }
    /// ```
    pub fn requires_device_interaction(choice: &str) -> bool {
        // All valid choices require device interaction
        Self::get_choice_category(choice).is_some()
    }
    
    /// Check if choice is potentially destructive
    /// 
    /// Determines whether a menu choice could potentially cause device state changes
    /// that might be considered destructive or require extra caution.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string
    /// 
    /// # Returns
    /// * `bool` - True if choice is potentially destructive
    /// 
    /// # Example
    /// ```
    /// if MenuActionHandlers::is_potentially_destructive("1") {
    ///     println!("This action may change device state");
    /// }
    /// ```
    pub fn is_potentially_destructive(choice: &str) -> bool {
        match choice {
            "1" | "2" | "3" | "4" | "5" | "6" => true, // Firing operations
            "7" => true,  // Arming
            "8" => true,  // Turn off
            "9" => true,  // Shutdown
            "16" => true, // Set ARM current
            _ => false,   // Information reading operations
        }
    }
    
    /// Get safety level for a choice
    /// 
    /// Returns a safety level indicator for menu choices to help users
    /// understand the potential impact of their selections.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string
    /// 
    /// # Returns
    /// * `Option<&'static str>` - Safety level if choice is valid
    /// 
    /// # Example
    /// ```
    /// match MenuActionHandlers::get_safety_level("1") {
    ///     Some("high_impact") => println!("High impact operation"),
    ///     Some("medium_impact") => println!("Medium impact operation"),
    ///     Some("low_impact") => println!("Low impact operation"),
    ///     None => println!("Invalid choice"),
    /// }
    /// ```
    pub fn get_safety_level(choice: &str) -> Option<&'static str> {
        match choice {
            "1" | "2" | "3" | "4" | "5" | "6" => Some("high_impact"), // Firing operations
            "7" | "8" | "9" | "16" => Some("medium_impact"), // Control and configuration
            "10" | "11" | "12" | "13" | "14" | "15" => Some("low_impact"), // Information reading
            _ => None,
        }
    }
    
    /// Execute choice with error handling
    /// 
    /// Executes a menu choice with comprehensive error handling and user feedback.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = MenuActionHandlers::execute_choice_safely(&mut device, "3")?;
    /// ```
    pub fn execute_choice_safely(device: &mut LumidoxDevice, choice: &str) -> Result<bool> {
        if !Self::is_valid_choice_format(choice) {
            println!();
            println!("Invalid choice format. Please enter a number.");
            println!();
            return Ok(true);
        }
        
        match Self::handle_choice_with_status(device, choice)? {
            Some(result) => Ok(result),
            None => {
                println!();
                println!("Not a valid choice. Please try again.");
                println!();
                Ok(true)
            }
        }
    }
}
