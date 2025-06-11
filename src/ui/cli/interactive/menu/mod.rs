//! Menu sub-module for interactive CLI
//!
//! This module organizes menu functionality into logical components:
//! - `display`: Menu display and formatting utilities
//!   - `stage_options`: Stage firing and custom current option display
//!   - `status_options`: Device status, information, and control option display
//! - `handlers`: Menu action handlers for different categories
//!   - `stage_actions`: Stage firing and custom current action handlers
//!   - `device_actions`: Device control action handlers (arm, turn off, shutdown)
//!   - `info_actions`: Information retrieval and status display handlers
//!
//! The menu system provides:
//! - Organized menu display with proper categorization
//! - Specialized action handlers for different menu types
//! - Consistent error handling and user feedback
//! - Integration with device operations and input processing

pub mod display;
pub mod handlers;

// Re-export commonly used items for convenience
pub use display::{MenuDisplay, StageOptionsDisplay, StatusOptionsDisplay};
pub use handlers::{MenuActionHandlers, StageActionHandlers, DeviceActionHandlers, InfoActionHandlers};

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::input::{InputProcessor, MenuChoice};

/// Menu system coordination utilities and functionality
pub struct MenuSystem;

impl MenuSystem {
    /// Display menu and get user choice
    ///
    /// Shows the complete interactive menu and gets a validated user choice.
    ///
    /// # Arguments
    /// * `device` - Reference to the device for dynamic menu information
    ///
    /// # Returns
    /// * `Result<MenuChoice>` - Validated menu choice or input error
    ///
    /// # Example
    /// ```
    /// let choice = MenuSystem::display_and_get_choice(&device)?;
    /// println!("User selected: {}", choice.number);
    /// ```
    pub fn display_and_get_choice(device: &mut LumidoxDevice) -> Result<MenuChoice> {
        MenuDisplay::display_complete_menu(device)?;
        InputProcessor::get_menu_choice()
    }

    /// Execute menu choice
    ///
    /// Executes a menu choice using the appropriate action handler.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - Menu choice to execute
    ///
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    ///
    /// # Example
    /// ```
    /// let continue_menu = MenuSystem::execute_choice(&mut device, choice)?;
    /// ```
    pub fn execute_choice(device: &mut LumidoxDevice, choice: MenuChoice) -> Result<bool> {
        match MenuActionHandlers::execute_choice_safely(device, &choice.number.to_string())? {
            true => Ok(true),
            false => Ok(false),
        }
    }

    /// Run interactive menu loop
    ///
    /// Runs the main interactive menu loop with display, input, and execution.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    ///
    /// # Returns
    /// * `Result<()>` - Success or error during menu operation
    ///
    /// # Example
    /// ```
    /// MenuSystem::run_menu_loop(&mut device)?;
    /// ```
    pub fn run_menu_loop(device: &mut LumidoxDevice) -> Result<()> {
        let mut continue_loop = true;

        while continue_loop {
            match Self::display_and_get_choice(device) {
                Ok(choice) => {
                    continue_loop = Self::execute_choice(device, choice)?;
                }
                Err(e) => {
                    InputProcessor::display_input_error(&e);
                    continue_loop = true; // Continue on input errors
                }
            }
        }

        Ok(())
    }

    /// Run enhanced menu loop with retry logic
    ///
    /// Enhanced menu loop with retry logic for input errors and menu display failures.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `max_input_attempts` - Maximum attempts for invalid input
    ///
    /// # Returns
    /// * `Result<()>` - Success or error during menu operation
    ///
    /// # Example
    /// ```
    /// MenuSystem::run_enhanced_menu_loop(&mut device, 3)?;
    /// ```
    pub fn run_enhanced_menu_loop(device: &mut LumidoxDevice, max_input_attempts: u8) -> Result<()> {
        let mut continue_loop = true;

        while continue_loop {
            let mut attempts = 0;
            let mut choice_obtained = false;

            while attempts < max_input_attempts && !choice_obtained {
                match Self::display_and_get_choice(device) {
                    Ok(choice) => {
                        continue_loop = Self::execute_choice(device, choice)?;
                        choice_obtained = true;
                    }
                    Err(e) => {
                        InputProcessor::display_input_error(&e);
                        attempts += 1;
                        if attempts < max_input_attempts {
                            println!("Please try again. ({}/{} attempts)", attempts, max_input_attempts);
                        }
                    }
                }
            }

            if !choice_obtained {
                println!("Too many invalid inputs. Exiting menu.");
                break;
            }
        }

        Ok(())
    }

    /// Validate menu system integrity
    ///
    /// Validates that all menu components are properly configured and accessible.
    ///
    /// # Returns
    /// * `Result<()>` - Success if menu system is valid
    ///
    /// # Example
    /// ```
    /// MenuSystem::validate_system()?;
    /// ```
    pub fn validate_system() -> Result<()> {
        // Check that all menu choices have valid handlers
        let all_choices = MenuDisplay::get_all_choices();

        for choice in all_choices {
            if MenuDisplay::get_choice_category(choice).is_none() {
                return Err(crate::core::LumidoxError::ConfigError(
                    format!("Menu choice '{}' has no valid category", choice)
                ));
            }

            if MenuActionHandlers::get_action_description(choice).is_none() {
                return Err(crate::core::LumidoxError::ConfigError(
                    format!("Menu choice '{}' has no action description", choice)
                ));
            }
        }

        Ok(())
    }

    /// Get menu statistics
    ///
    /// Returns statistics about the menu system configuration.
    ///
    /// # Returns
    /// * `MenuStatistics` - Menu system statistics
    ///
    /// # Example
    /// ```
    /// let stats = MenuSystem::get_statistics();
    /// println!("Total options: {}", stats.total_options);
    /// ```
    pub fn get_statistics() -> MenuStatistics {
        let all_choices = MenuDisplay::get_all_choices();
        let mut stage_options = 0;
        let mut device_options = 0;
        let mut info_options = 0;

        for choice in &all_choices {
            match MenuDisplay::get_choice_category(choice) {
                Some("stage") => stage_options += 1,
                Some("device") => device_options += 1,
                Some("info") => info_options += 1,
                _ => {}
            }
        }

        MenuStatistics {
            total_options: all_choices.len(),
            stage_options,
            device_options,
            info_options,
        }
    }
}

/// Menu system statistics
#[derive(Debug, Clone)]
pub struct MenuStatistics {
    /// Total number of menu options
    pub total_options: usize,
    /// Number of stage-related options
    pub stage_options: usize,
    /// Number of device control options
    pub device_options: usize,
    /// Number of information options
    pub info_options: usize,
}
