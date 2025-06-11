//! Input validation utilities for interactive CLI
//!
//! This module provides comprehensive input validation for interactive CLI
//! operations including menu choice validation, numeric input validation,
//! and range checking. It ensures user inputs are safe and valid before
//! processing by action handlers.
//!
//! The input validation system provides:
//! - Menu choice format and range validation
//! - Numeric input validation with type checking
//! - Range validation for stage numbers and current values
//! - Error message generation for invalid inputs
//! - Input sanitization and normalization

use crate::core::{LumidoxError, Result};

/// Input validation utilities and functionality
pub struct InputValidator;

impl InputValidator {
    /// Validate menu choice format
    /// 
    /// Checks if a menu choice string is in the correct format for processing.
    /// Valid choices are numeric strings representing menu options.
    /// 
    /// # Arguments
    /// * `choice` - User input choice string
    /// 
    /// # Returns
    /// * `Result<String>` - Normalized choice string or validation error
    /// 
    /// # Example
    /// ```
    /// let choice = InputValidator::validate_choice_format("  3  ")?;
    /// assert_eq!(choice, "3");
    /// ```
    pub fn validate_choice_format(choice: &str) -> Result<String> {
        let trimmed = choice.trim();
        
        if trimmed.is_empty() {
            return Err(LumidoxError::InvalidInput(
                "Choice cannot be empty. Please enter a menu option number.".to_string()
            ));
        }
        
        if !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid choice format: '{}'. Please enter a number.", trimmed)
            ));
        }
        
        Ok(trimmed.to_string())
    }
    
    /// Validate menu choice range
    /// 
    /// Checks if a menu choice is within the valid range of menu options.
    /// 
    /// # Arguments
    /// * `choice` - Menu choice string (must be numeric)
    /// * `min_choice` - Minimum valid choice number
    /// * `max_choice` - Maximum valid choice number
    /// 
    /// # Returns
    /// * `Result<u8>` - Choice number or validation error
    /// 
    /// # Example
    /// ```
    /// let choice_num = InputValidator::validate_choice_range("5", 1, 16)?;
    /// assert_eq!(choice_num, 5);
    /// ```
    pub fn validate_choice_range(choice: &str, min_choice: u8, max_choice: u8) -> Result<u8> {
        let choice_num = choice.parse::<u8>()
            .map_err(|_| LumidoxError::InvalidInput(
                format!("Invalid choice: '{}'. Must be a number between {} and {}.", 
                    choice, min_choice, max_choice)
            ))?;
        
        if choice_num < min_choice || choice_num > max_choice {
            return Err(LumidoxError::InvalidInput(
                format!("Choice {} is out of range. Must be between {} and {}.", 
                    choice_num, min_choice, max_choice)
            ));
        }
        
        Ok(choice_num)
    }
    
    /// Validate stage number input
    /// 
    /// Validates that a stage number is within the valid range (1-5).
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `Result<u8>` - Stage number or validation error
    /// 
    /// # Example
    /// ```
    /// let stage = InputValidator::validate_stage_number("3")?;
    /// assert_eq!(stage, 3);
    /// ```
    pub fn validate_stage_number(input: &str) -> Result<u8> {
        let trimmed = input.trim();
        
        let stage = trimmed.parse::<u8>()
            .map_err(|_| LumidoxError::InvalidInput(
                format!("Invalid stage number: '{}'. Must be a number between 1 and 5.", trimmed)
            ))?;
        
        if !(1..=5).contains(&stage) {
            return Err(LumidoxError::InvalidInput(
                format!("Stage number {} is out of range. Must be between 1 and 5.", stage)
            ));
        }
        
        Ok(stage)
    }
    
    /// Validate current value input
    /// 
    /// Validates that a current value is a valid positive integer.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `Result<u16>` - Current value in mA or validation error
    /// 
    /// # Example
    /// ```
    /// let current = InputValidator::validate_current_value("500")?;
    /// assert_eq!(current, 500);
    /// ```
    pub fn validate_current_value(input: &str) -> Result<u16> {
        let trimmed = input.trim();
        
        let current = trimmed.parse::<u16>()
            .map_err(|_| LumidoxError::InvalidInput(
                format!("Invalid current value: '{}'. Must be a whole number (no decimals).", trimmed)
            ))?;
        
        Ok(current)
    }
    
    /// Validate current value with range checking
    /// 
    /// Validates current value and checks it against maximum allowed current.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// * `max_current` - Maximum allowed current in mA
    /// 
    /// # Returns
    /// * `Result<u16>` - Current value in mA or validation error
    /// 
    /// # Example
    /// ```
    /// let current = InputValidator::validate_current_with_range("500", 1000)?;
    /// assert_eq!(current, 500);
    /// ```
    pub fn validate_current_with_range(input: &str, max_current: u16) -> Result<u16> {
        let current = Self::validate_current_value(input)?;
        
        if current > max_current {
            return Err(LumidoxError::InvalidInput(
                format!("Current value {}mA exceeds maximum allowed current of {}mA.", 
                    current, max_current)
            ));
        }
        
        Ok(current)
    }
    
    /// Validate non-zero current value
    /// 
    /// Validates current value and ensures it is not zero.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `Result<u16>` - Non-zero current value in mA or validation error
    /// 
    /// # Example
    /// ```
    /// let current = InputValidator::validate_non_zero_current("500")?;
    /// assert_eq!(current, 500);
    /// ```
    pub fn validate_non_zero_current(input: &str) -> Result<u16> {
        let current = Self::validate_current_value(input)?;
        
        if current == 0 {
            return Err(LumidoxError::InvalidInput(
                "Current value cannot be zero. Please enter a positive current value.".to_string()
            ));
        }
        
        Ok(current)
    }
    
    /// Sanitize user input
    /// 
    /// Removes leading/trailing whitespace and normalizes input for processing.
    /// 
    /// # Arguments
    /// * `input` - Raw user input string
    /// 
    /// # Returns
    /// * `String` - Sanitized input string
    /// 
    /// # Example
    /// ```
    /// let sanitized = InputValidator::sanitize_input("  hello world  ");
    /// assert_eq!(sanitized, "hello world");
    /// ```
    pub fn sanitize_input(input: &str) -> String {
        input.trim().to_string()
    }
    
    /// Check if input is empty after sanitization
    /// 
    /// Determines if input is effectively empty after removing whitespace.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `bool` - True if input is empty after sanitization
    /// 
    /// # Example
    /// ```
    /// assert!(InputValidator::is_empty_input("   "));
    /// assert!(!InputValidator::is_empty_input("  hello  "));
    /// ```
    pub fn is_empty_input(input: &str) -> bool {
        input.trim().is_empty()
    }
    
    /// Validate yes/no input
    /// 
    /// Validates user input for yes/no questions with flexible acceptance.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `Result<bool>` - True for yes, false for no, or validation error
    /// 
    /// # Example
    /// ```
    /// assert_eq!(InputValidator::validate_yes_no("y")?, true);
    /// assert_eq!(InputValidator::validate_yes_no("no")?, false);
    /// ```
    pub fn validate_yes_no(input: &str) -> Result<bool> {
        let trimmed = input.trim().to_lowercase();
        
        match trimmed.as_str() {
            "y" | "yes" | "true" | "1" => Ok(true),
            "n" | "no" | "false" | "0" => Ok(false),
            _ => Err(LumidoxError::InvalidInput(
                format!("Invalid yes/no input: '{}'. Please enter 'y' or 'n'.", input.trim())
            )),
        }
    }
    
    /// Generate validation error message
    /// 
    /// Creates a standardized error message for validation failures.
    /// 
    /// # Arguments
    /// * `input` - The invalid input that caused the error
    /// * `expected` - Description of what was expected
    /// * `context` - Additional context about the validation
    /// 
    /// # Returns
    /// * `String` - Formatted error message
    /// 
    /// # Example
    /// ```
    /// let msg = InputValidator::generate_error_message("abc", "a number", "menu choice");
    /// ```
    pub fn generate_error_message(input: &str, expected: &str, context: &str) -> String {
        format!("Invalid {} input: '{}'. Expected {}.", context, input.trim(), expected)
    }
    
    /// Validate input length
    /// 
    /// Checks if input length is within acceptable bounds.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// * `max_length` - Maximum allowed length
    /// 
    /// # Returns
    /// * `Result<String>` - Input string or validation error
    /// 
    /// # Example
    /// ```
    /// let input = InputValidator::validate_input_length("hello", 10)?;
    /// assert_eq!(input, "hello");
    /// ```
    pub fn validate_input_length(input: &str, max_length: usize) -> Result<String> {
        let trimmed = input.trim();
        
        if trimmed.len() > max_length {
            return Err(LumidoxError::InvalidInput(
                format!("Input too long: {} characters. Maximum allowed: {} characters.", 
                    trimmed.len(), max_length)
            ));
        }
        
        Ok(trimmed.to_string())
    }
    
    /// Validate numeric input with custom range
    /// 
    /// Generic numeric validation with custom minimum and maximum values.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// * `min_value` - Minimum allowed value
    /// * `max_value` - Maximum allowed value
    /// * `value_name` - Name of the value being validated (for error messages)
    /// 
    /// # Returns
    /// * `Result<u16>` - Validated numeric value or validation error
    /// 
    /// # Example
    /// ```
    /// let value = InputValidator::validate_numeric_range("50", 1, 100, "percentage")?;
    /// assert_eq!(value, 50);
    /// ```
    pub fn validate_numeric_range(
        input: &str, 
        min_value: u16, 
        max_value: u16, 
        value_name: &str
    ) -> Result<u16> {
        let trimmed = input.trim();
        
        let value = trimmed.parse::<u16>()
            .map_err(|_| LumidoxError::InvalidInput(
                format!("Invalid {} value: '{}'. Must be a whole number.", value_name, trimmed)
            ))?;
        
        if value < min_value || value > max_value {
            return Err(LumidoxError::InvalidInput(
                format!("{} value {} is out of range. Must be between {} and {}.", 
                    value_name, value, min_value, max_value)
            ));
        }
        
        Ok(value)
    }
    
    /// Check if input contains only allowed characters
    /// 
    /// Validates that input contains only characters from an allowed set.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// * `allowed_chars` - String containing all allowed characters
    /// 
    /// # Returns
    /// * `Result<String>` - Input string or validation error
    /// 
    /// # Example
    /// ```
    /// let input = InputValidator::validate_allowed_chars("123", "0123456789")?;
    /// assert_eq!(input, "123");
    /// ```
    pub fn validate_allowed_chars(input: &str, allowed_chars: &str) -> Result<String> {
        let trimmed = input.trim();
        
        for ch in trimmed.chars() {
            if !allowed_chars.contains(ch) {
                return Err(LumidoxError::InvalidInput(
                    format!("Invalid character '{}' in input '{}'. Allowed characters: {}", 
                        ch, trimmed, allowed_chars)
                ));
            }
        }
        
        Ok(trimmed.to_string())
    }
}
