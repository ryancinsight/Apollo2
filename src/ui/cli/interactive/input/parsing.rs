//! Input parsing utilities for interactive CLI
//!
//! This module provides specialized input parsing functionality for interactive
//! CLI operations including menu choice parsing, numeric value extraction,
//! and command parsing. It handles the conversion of validated user input
//! into appropriate data types for action handlers.
//!
//! The input parsing system provides:
//! - Menu choice parsing with category detection
//! - Numeric value parsing with type conversion
//! - Command argument parsing and extraction
//! - Input normalization and standardization
//! - Error handling for parsing failures

use crate::core::{LumidoxError, Result};
use super::validation::InputValidator;

/// Input parsing utilities and functionality
pub struct InputParser;

impl InputParser {
    /// Parse menu choice from user input
    /// 
    /// Parses and validates a menu choice from user input, returning both
    /// the choice number and its category for routing to appropriate handlers.
    /// 
    /// # Arguments
    /// * `input` - Raw user input string
    /// 
    /// # Returns
    /// * `Result<MenuChoice>` - Parsed menu choice or parsing error
    /// 
    /// # Example
    /// ```
    /// let choice = InputParser::parse_menu_choice("  5  ")?;
    /// assert_eq!(choice.number, 5);
    /// assert_eq!(choice.category, MenuCategory::Stage);
    /// ```
    pub fn parse_menu_choice(input: &str) -> Result<MenuChoice> {
        let choice_str = InputValidator::validate_choice_format(input)?;
        let choice_num = InputValidator::validate_choice_range(&choice_str, 1, 16)?;
        
        let category = Self::determine_choice_category(choice_num);
        let action = Self::determine_choice_action(choice_num);
        
        Ok(MenuChoice {
            number: choice_num,
            category,
            action,
            raw_input: input.to_string(),
        })
    }
    
    /// Parse stage number from user input
    /// 
    /// Parses and validates a stage number from user input.
    /// 
    /// # Arguments
    /// * `input` - Raw user input string
    /// 
    /// # Returns
    /// * `Result<u8>` - Stage number (1-5) or parsing error
    /// 
    /// # Example
    /// ```
    /// let stage = InputParser::parse_stage_number("3")?;
    /// assert_eq!(stage, 3);
    /// ```
    pub fn parse_stage_number(input: &str) -> Result<u8> {
        InputValidator::validate_stage_number(input)
    }
    
    /// Parse current value from user input
    /// 
    /// Parses and validates a current value from user input with optional
    /// range checking against maximum allowed current.
    /// 
    /// # Arguments
    /// * `input` - Raw user input string
    /// * `max_current` - Optional maximum allowed current for validation
    /// 
    /// # Returns
    /// * `Result<u16>` - Current value in mA or parsing error
    /// 
    /// # Example
    /// ```
    /// let current = InputParser::parse_current_value("500", Some(1000))?;
    /// assert_eq!(current, 500);
    /// ```
    pub fn parse_current_value(input: &str, max_current: Option<u16>) -> Result<u16> {
        match max_current {
            Some(max) => InputValidator::validate_current_with_range(input, max),
            None => InputValidator::validate_current_value(input),
        }
    }
    
    /// Parse yes/no response from user input
    /// 
    /// Parses user input for yes/no questions with flexible acceptance.
    /// 
    /// # Arguments
    /// * `input` - Raw user input string
    /// 
    /// # Returns
    /// * `Result<bool>` - True for yes, false for no, or parsing error
    /// 
    /// # Example
    /// ```
    /// let response = InputParser::parse_yes_no("y")?;
    /// assert_eq!(response, true);
    /// ```
    pub fn parse_yes_no(input: &str) -> Result<bool> {
        InputValidator::validate_yes_no(input)
    }
    
    /// Determine menu choice category
    /// 
    /// Determines the category of a menu choice number for routing purposes.
    /// 
    /// # Arguments
    /// * `choice_num` - Menu choice number
    /// 
    /// # Returns
    /// * `MenuCategory` - Category of the menu choice
    /// 
    /// # Example
    /// ```
    /// let category = InputParser::determine_choice_category(3);
    /// assert_eq!(category, MenuCategory::Stage);
    /// ```
    pub fn determine_choice_category(choice_num: u8) -> MenuCategory {
        match choice_num {
            1..=6 => MenuCategory::Stage,
            7..=9 => MenuCategory::Device,
            10..=16 => MenuCategory::Information,
            _ => MenuCategory::Invalid,
        }
    }
    
    /// Determine menu choice action
    /// 
    /// Determines the specific action for a menu choice number.
    /// 
    /// # Arguments
    /// * `choice_num` - Menu choice number
    /// 
    /// # Returns
    /// * `MenuAction` - Specific action for the menu choice
    /// 
    /// # Example
    /// ```
    /// let action = InputParser::determine_choice_action(3);
    /// assert_eq!(action, MenuAction::FireStage(3));
    /// ```
    pub fn determine_choice_action(choice_num: u8) -> MenuAction {
        match choice_num {
            1 => MenuAction::FireStage(1),
            2 => MenuAction::FireStage(2),
            3 => MenuAction::FireStage(3),
            4 => MenuAction::FireStage(4),
            5 => MenuAction::FireStage(5),
            6 => MenuAction::FireCustomCurrent,
            7 => MenuAction::ArmDevice,
            8 => MenuAction::TurnOffDevice,
            9 => MenuAction::ShutdownAndQuit,
            10 => MenuAction::ShowDeviceStatus,
            11 => MenuAction::ReadRemoteMode,
            12 => MenuAction::ReadCurrentSettings,
            13 => MenuAction::ShowStageParameters,
            14 => MenuAction::ReadStageArmCurrent,
            15 => MenuAction::ReadStageVoltageParameters,
            16 => MenuAction::SetArmCurrent,
            _ => MenuAction::Invalid,
        }
    }
    
    /// Parse command with arguments
    /// 
    /// Parses a command string that may contain arguments separated by spaces.
    /// 
    /// # Arguments
    /// * `input` - Raw command input string
    /// 
    /// # Returns
    /// * `Result<ParsedCommand>` - Parsed command with arguments or parsing error
    /// 
    /// # Example
    /// ```
    /// let cmd = InputParser::parse_command("fire 3")?;
    /// assert_eq!(cmd.command, "fire");
    /// assert_eq!(cmd.arguments, vec!["3"]);
    /// ```
    pub fn parse_command(input: &str) -> Result<ParsedCommand> {
        let sanitized = InputValidator::sanitize_input(input);
        
        if sanitized.is_empty() {
            return Err(LumidoxError::InvalidInput(
                "Command cannot be empty.".to_string()
            ));
        }
        
        let parts: Vec<&str> = sanitized.split_whitespace().collect();
        let command = parts[0].to_lowercase();
        let arguments: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        Ok(ParsedCommand {
            command,
            arguments,
            raw_input: input.to_string(),
        })
    }
    
    /// Extract numeric value from input
    /// 
    /// Extracts the first numeric value found in the input string.
    /// 
    /// # Arguments
    /// * `input` - Input string that may contain numeric values
    /// 
    /// # Returns
    /// * `Result<Option<u16>>` - First numeric value found or None if no numbers
    /// 
    /// # Example
    /// ```
    /// let value = InputParser::extract_numeric_value("stage 3 current")?;
    /// assert_eq!(value, Some(3));
    /// ```
    pub fn extract_numeric_value(input: &str) -> Result<Option<u16>> {
        let sanitized = InputValidator::sanitize_input(input);
        
        for word in sanitized.split_whitespace() {
            if let Ok(value) = word.parse::<u16>() {
                return Ok(Some(value));
            }
        }
        
        Ok(None)
    }
    
    /// Parse multiple numeric values
    /// 
    /// Extracts all numeric values from the input string.
    /// 
    /// # Arguments
    /// * `input` - Input string that may contain multiple numeric values
    /// 
    /// # Returns
    /// * `Result<Vec<u16>>` - All numeric values found in order
    /// 
    /// # Example
    /// ```
    /// let values = InputParser::parse_multiple_numeric_values("stage 3 current 500")?;
    /// assert_eq!(values, vec![3, 500]);
    /// ```
    pub fn parse_multiple_numeric_values(input: &str) -> Result<Vec<u16>> {
        let sanitized = InputValidator::sanitize_input(input);
        let mut values = Vec::new();
        
        for word in sanitized.split_whitespace() {
            if let Ok(value) = word.parse::<u16>() {
                values.push(value);
            }
        }
        
        Ok(values)
    }
    
    /// Normalize input for comparison
    /// 
    /// Normalizes input for case-insensitive comparison and matching.
    /// 
    /// # Arguments
    /// * `input` - Raw input string
    /// 
    /// # Returns
    /// * `String` - Normalized input string
    /// 
    /// # Example
    /// ```
    /// let normalized = InputParser::normalize_input("  Hello World  ");
    /// assert_eq!(normalized, "hello world");
    /// ```
    pub fn normalize_input(input: &str) -> String {
        InputValidator::sanitize_input(input).to_lowercase()
    }
    
    /// Check if input matches pattern
    /// 
    /// Checks if normalized input matches a specific pattern.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// * `pattern` - Pattern to match against
    /// 
    /// # Returns
    /// * `bool` - True if input matches pattern
    /// 
    /// # Example
    /// ```
    /// assert!(InputParser::matches_pattern("YES", "yes"));
    /// assert!(InputParser::matches_pattern("  quit  ", "quit"));
    /// ```
    pub fn matches_pattern(input: &str, pattern: &str) -> bool {
        Self::normalize_input(input) == pattern.to_lowercase()
    }
}

/// Represents a parsed menu choice with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct MenuChoice {
    /// The numeric choice (1-16)
    pub number: u8,
    /// Category of the choice for routing
    pub category: MenuCategory,
    /// Specific action to be performed
    pub action: MenuAction,
    /// Original raw input from user
    pub raw_input: String,
}

/// Categories of menu choices for routing
#[derive(Debug, Clone, PartialEq)]
pub enum MenuCategory {
    /// Stage firing operations (1-6)
    Stage,
    /// Device control operations (7-9)
    Device,
    /// Information and status operations (10-16)
    Information,
    /// Invalid choice
    Invalid,
}

/// Specific actions for menu choices
#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    /// Fire a specific stage
    FireStage(u8),
    /// Fire with custom current
    FireCustomCurrent,
    /// Arm the device
    ArmDevice,
    /// Turn off the device
    TurnOffDevice,
    /// Shutdown and quit program
    ShutdownAndQuit,
    /// Show device status
    ShowDeviceStatus,
    /// Read remote mode state
    ReadRemoteMode,
    /// Read current settings
    ReadCurrentSettings,
    /// Show stage parameters
    ShowStageParameters,
    /// Read stage ARM current
    ReadStageArmCurrent,
    /// Read stage voltage parameters
    ReadStageVoltageParameters,
    /// Set ARM current
    SetArmCurrent,
    /// Invalid action
    Invalid,
}

/// Represents a parsed command with arguments
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// The main command
    pub command: String,
    /// Command arguments
    pub arguments: Vec<String>,
    /// Original raw input from user
    pub raw_input: String,
}
