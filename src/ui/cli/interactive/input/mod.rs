//! Input processing sub-module for interactive CLI
//!
//! This module organizes input processing functionality into specialized components:
//! - `validation`: Input validation utilities and error checking
//! - `parsing`: Input parsing and data type conversion utilities
//!
//! The input processing system provides:
//! - Comprehensive input validation with detailed error messages
//! - Specialized parsing for different input types (menu choices, numbers, commands)
//! - Input sanitization and normalization
//! - Type-safe conversion of user input to application data types

pub mod validation;
pub mod parsing;

// Re-export commonly used items for convenience
pub use validation::InputValidator;
pub use parsing::{InputParser, MenuChoice, MenuCategory, MenuAction, ParsedCommand};

use crate::core::Result;
use std::io::{self, Write};

/// Input processing coordination utilities and functionality
pub struct InputProcessor;

impl InputProcessor {
    /// Get user input with prompt
    /// 
    /// Displays a prompt and reads user input from stdin with proper flushing.
    /// 
    /// # Arguments
    /// * `prompt` - Prompt message to display to user
    /// 
    /// # Returns
    /// * `Result<String>` - User input string or I/O error
    /// 
    /// # Example
    /// ```
    /// let input = InputProcessor::get_user_input("Enter choice: ")?;
    /// ```
    pub fn get_user_input(prompt: &str) -> Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input)
    }
    
    /// Get validated menu choice from user
    /// 
    /// Prompts user for menu choice and validates it before returning.
    /// 
    /// # Returns
    /// * `Result<MenuChoice>` - Validated menu choice or input error
    /// 
    /// # Example
    /// ```
    /// let choice = InputProcessor::get_menu_choice()?;
    /// println!("Selected: {}", choice.number);
    /// ```
    pub fn get_menu_choice() -> Result<MenuChoice> {
        let input = Self::get_user_input("Please enter choice number, then press ENTER: ")?;
        InputParser::parse_menu_choice(&input)
    }
    
    /// Get validated stage number from user
    /// 
    /// Prompts user for stage number and validates it before returning.
    /// 
    /// # Returns
    /// * `Result<u8>` - Validated stage number (1-5) or input error
    /// 
    /// # Example
    /// ```
    /// let stage = InputProcessor::get_stage_number()?;
    /// println!("Selected stage: {}", stage);
    /// ```
    pub fn get_stage_number() -> Result<u8> {
        let input = Self::get_user_input("Enter stage number (1-5): ")?;
        InputParser::parse_stage_number(&input)
    }
    
    /// Get validated current value from user
    /// 
    /// Prompts user for current value and validates it before returning.
    /// 
    /// # Arguments
    /// * `max_current` - Optional maximum allowed current for validation
    /// 
    /// # Returns
    /// * `Result<u16>` - Validated current value in mA or input error
    /// 
    /// # Example
    /// ```
    /// let current = InputProcessor::get_current_value(Some(1000))?;
    /// println!("Selected current: {}mA", current);
    /// ```
    pub fn get_current_value(max_current: Option<u16>) -> Result<u16> {
        let prompt = match max_current {
            Some(max) => format!("Enter current in mA (max {}mA): ", max),
            None => "Enter current in mA: ".to_string(),
        };
        
        let input = Self::get_user_input(&prompt)?;
        InputParser::parse_current_value(&input, max_current)
    }
    
    /// Get yes/no confirmation from user
    /// 
    /// Prompts user for yes/no confirmation and validates response.
    /// 
    /// # Arguments
    /// * `prompt` - Confirmation prompt message
    /// 
    /// # Returns
    /// * `Result<bool>` - True for yes, false for no, or input error
    /// 
    /// # Example
    /// ```
    /// let confirmed = InputProcessor::get_confirmation("Continue? (y/n): ")?;
    /// if confirmed {
    ///     println!("User confirmed");
    /// }
    /// ```
    pub fn get_confirmation(prompt: &str) -> Result<bool> {
        let input = Self::get_user_input(prompt)?;
        InputParser::parse_yes_no(&input)
    }
    
    /// Get user input with retry logic
    /// 
    /// Prompts user for input with validation and retry on invalid input.
    /// 
    /// # Arguments
    /// * `prompt` - Prompt message to display
    /// * `validator` - Validation function that returns Result<T>
    /// * `max_attempts` - Maximum number of retry attempts
    /// 
    /// # Returns
    /// * `Result<Option<T>>` - Validated input or None if max attempts reached
    /// 
    /// # Example
    /// ```
    /// let stage = InputProcessor::get_input_with_retry(
    ///     "Enter stage (1-5): ",
    ///     |input| InputParser::parse_stage_number(input),
    ///     3
    /// )?;
    /// ```
    pub fn get_input_with_retry<T, F>(
        prompt: &str,
        validator: F,
        max_attempts: u8,
    ) -> Result<Option<T>>
    where
        F: Fn(&str) -> Result<T>,
    {
        for attempt in 1..=max_attempts {
            let input = Self::get_user_input(prompt)?;
            
            match validator(&input) {
                Ok(value) => return Ok(Some(value)),
                Err(e) => {
                    println!("Error: {}", e);
                    if attempt < max_attempts {
                        println!("Please try again. ({}/{} attempts)", attempt, max_attempts);
                    }
                }
            }
        }
        
        println!("Maximum attempts reached.");
        Ok(None)
    }
    
    /// Get menu choice with retry logic
    /// 
    /// Enhanced menu choice input with retry on invalid choices.
    /// 
    /// # Arguments
    /// * `max_attempts` - Maximum number of retry attempts
    /// 
    /// # Returns
    /// * `Result<Option<MenuChoice>>` - Validated menu choice or None if max attempts reached
    /// 
    /// # Example
    /// ```
    /// if let Some(choice) = InputProcessor::get_menu_choice_with_retry(3)? {
    ///     println!("Valid choice: {}", choice.number);
    /// }
    /// ```
    pub fn get_menu_choice_with_retry(max_attempts: u8) -> Result<Option<MenuChoice>> {
        Self::get_input_with_retry(
            "Please enter choice number, then press ENTER: ",
            |input| InputParser::parse_menu_choice(input),
            max_attempts,
        )
    }
    
    /// Get stage number with retry logic
    /// 
    /// Enhanced stage number input with retry on invalid numbers.
    /// 
    /// # Arguments
    /// * `max_attempts` - Maximum number of retry attempts
    /// 
    /// # Returns
    /// * `Result<Option<u8>>` - Validated stage number or None if max attempts reached
    /// 
    /// # Example
    /// ```
    /// if let Some(stage) = InputProcessor::get_stage_number_with_retry(3)? {
    ///     println!("Valid stage: {}", stage);
    /// }
    /// ```
    pub fn get_stage_number_with_retry(max_attempts: u8) -> Result<Option<u8>> {
        Self::get_input_with_retry(
            "Enter stage number (1-5): ",
            |input| InputParser::parse_stage_number(input),
            max_attempts,
        )
    }
    
    /// Get current value with retry logic
    /// 
    /// Enhanced current value input with retry on invalid values.
    /// 
    /// # Arguments
    /// * `max_current` - Optional maximum allowed current for validation
    /// * `max_attempts` - Maximum number of retry attempts
    /// 
    /// # Returns
    /// * `Result<Option<u16>>` - Validated current value or None if max attempts reached
    /// 
    /// # Example
    /// ```
    /// if let Some(current) = InputProcessor::get_current_value_with_retry(Some(1000), 3)? {
    ///     println!("Valid current: {}mA", current);
    /// }
    /// ```
    pub fn get_current_value_with_retry(
        max_current: Option<u16>,
        max_attempts: u8,
    ) -> Result<Option<u16>> {
        let prompt = match max_current {
            Some(max) => format!("Enter current in mA (max {}mA): ", max),
            None => "Enter current in mA: ".to_string(),
        };
        
        Self::get_input_with_retry(
            &prompt,
            |input| InputParser::parse_current_value(input, max_current),
            max_attempts,
        )
    }
    
    /// Display input error message
    /// 
    /// Shows a formatted error message for input validation failures.
    /// 
    /// # Arguments
    /// * `error` - The input validation error
    /// 
    /// # Example
    /// ```
    /// InputProcessor::display_input_error(&error);
    /// ```
    pub fn display_input_error(error: &crate::core::LumidoxError) {
        println!();
        println!("Input Error: {}", error);
        println!();
    }
    
    /// Check if input indicates user wants to quit
    /// 
    /// Checks if user input indicates they want to quit or cancel.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `bool` - True if input indicates quit/cancel intent
    /// 
    /// # Example
    /// ```
    /// if InputProcessor::is_quit_input("quit") {
    ///     println!("User wants to quit");
    /// }
    /// ```
    pub fn is_quit_input(input: &str) -> bool {
        let normalized = InputParser::normalize_input(input);
        matches!(normalized.as_str(), "quit" | "exit" | "q" | "cancel" | "abort")
    }
    
    /// Get input with quit option
    /// 
    /// Gets user input with the option to quit/cancel the operation.
    /// 
    /// # Arguments
    /// * `prompt` - Prompt message to display
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - Input string or None if user wants to quit
    /// 
    /// # Example
    /// ```
    /// if let Some(input) = InputProcessor::get_input_with_quit("Enter value (or 'quit'): ")? {
    ///     println!("User entered: {}", input);
    /// } else {
    ///     println!("User cancelled");
    /// }
    /// ```
    pub fn get_input_with_quit(prompt: &str) -> Result<Option<String>> {
        let input = Self::get_user_input(prompt)?;
        
        if Self::is_quit_input(&input) {
            Ok(None)
        } else {
            Ok(Some(InputValidator::sanitize_input(&input)))
        }
    }
}
