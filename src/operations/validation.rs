//! Unified validation layer for Lumidox II Controller
//!
//! This module provides unified validation functionality that can be used
//! by both CLI and GUI interfaces. It extracts validation logic from CLI
//! input handlers and GUI message validation to provide consistent validation
//! with proper error handling and result types.
//!
//! The validation module includes:
//! - Unified validation manager with consistent interfaces
//! - Validation results that work with both CLI and GUI feedback systems
//! - Support for both immediate validation (GUI) and interactive retry (CLI)
//! - Common validation functions for all input types
//! - Proper error messages and user feedback

use crate::core::{LumidoxError, Result};
use std::fmt;

/// Validation error types
/// 
/// Represents different types of validation errors that can occur during
/// input validation, with appropriate error messages and retry information.
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Invalid input format
    InvalidFormat {
        /// The invalid input value
        input: String,
        /// Expected format description
        expected: String,
        /// Additional context information
        context: Option<String>,
    },
    /// Input value out of valid range
    OutOfRange {
        /// The input value
        input: String,
        /// Minimum valid value
        min: Option<i64>,
        /// Maximum valid value
        max: Option<i64>,
        /// Value type description
        value_type: String,
    },
    /// Required input is missing
    Missing {
        /// Field name that is missing
        field_name: String,
        /// Description of required field
        description: String,
    },
    /// Input length validation failed
    InvalidLength {
        /// The input value
        input: String,
        /// Minimum length
        min_length: Option<usize>,
        /// Maximum length
        max_length: Option<usize>,
    },
    /// Custom validation error
    Custom {
        /// Error message
        message: String,
        /// Whether the error is retryable
        retryable: bool,
    },
}

impl ValidationError {
    /// Check if the validation error is retryable
    /// 
    /// # Returns
    /// * `bool` - True if the error is retryable
    /// 
    /// # Example
    /// ```
    /// if error.is_retryable() {
    ///     println!("Please try again");
    /// }
    /// ```
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Custom { retryable, .. } => *retryable,
            _ => true, // Most validation errors are retryable
        }
    }
    
    /// Get user-friendly error message
    /// 
    /// # Returns
    /// * `String` - User-friendly error message
    /// 
    /// # Example
    /// ```
    /// println!("Error: {}", error.message());
    /// ```
    pub fn message(&self) -> String {
        match self {
            Self::InvalidFormat { input, expected, context } => {
                let base_msg = format!("Invalid format for '{}'. Expected: {}", input, expected);
                if let Some(ctx) = context {
                    format!("{} ({})", base_msg, ctx)
                } else {
                    base_msg
                }
            }
            Self::OutOfRange { input, min, max, value_type } => {
                let range_str = match (min, max) {
                    (Some(min_val), Some(max_val)) => format!("{}-{}", min_val, max_val),
                    (Some(min_val), None) => format!(">= {}", min_val),
                    (None, Some(max_val)) => format!("<= {}", max_val),
                    (None, None) => "valid range".to_string(),
                };
                format!("Value '{}' is out of range for {}. Valid range: {}", input, value_type, range_str)
            }
            Self::Missing { field_name, description } => {
                format!("Missing required field '{}': {}", field_name, description)
            }
            Self::InvalidLength { input, min_length, max_length } => {
                let length_str = match (min_length, max_length) {
                    (Some(min_len), Some(max_len)) => format!("{}-{} characters", min_len, max_len),
                    (Some(min_len), None) => format!("at least {} characters", min_len),
                    (None, Some(max_len)) => format!("at most {} characters", max_len),
                    (None, None) => "valid length".to_string(),
                };
                format!("Input '{}' has invalid length. Expected: {}", input, length_str)
            }
            Self::Custom { message, .. } => message.clone(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ValidationError {}

/// Validation result type
/// 
/// Represents the result of a validation operation with success data
/// or validation error information.
#[derive(Debug, Clone)]
pub enum ValidationResult<T> {
    /// Validation succeeded
    Valid(T),
    /// Validation failed
    Invalid(ValidationError),
}

impl<T> ValidationResult<T> {
    /// Check if validation was successful
    /// 
    /// # Returns
    /// * `bool` - True if validation succeeded
    /// 
    /// # Example
    /// ```
    /// if result.is_valid() {
    ///     println!("Validation passed");
    /// }
    /// ```
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid(_))
    }
    
    /// Check if validation failed
    /// 
    /// # Returns
    /// * `bool` - True if validation failed
    /// 
    /// # Example
    /// ```
    /// if result.is_invalid() {
    ///     println!("Validation failed");
    /// }
    /// ```
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
    
    /// Get the valid value if available
    /// 
    /// # Returns
    /// * `Option<&T>` - Valid value if available
    /// 
    /// # Example
    /// ```
    /// if let Some(value) = result.value() {
    ///     println!("Valid value: {:?}", value);
    /// }
    /// ```
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Valid(value) => Some(value),
            Self::Invalid(_) => None,
        }
    }
    
    /// Get the validation error if available
    /// 
    /// # Returns
    /// * `Option<&ValidationError>` - Validation error if available
    /// 
    /// # Example
    /// ```
    /// if let Some(error) = result.error() {
    ///     eprintln!("Validation error: {}", error);
    /// }
    /// ```
    pub fn error(&self) -> Option<&ValidationError> {
        match self {
            Self::Valid(_) => None,
            Self::Invalid(error) => Some(error),
        }
    }
    
    /// Convert to standard Result type
    /// 
    /// # Returns
    /// * `Result<T>` - Standard result type
    /// 
    /// # Example
    /// ```
    /// let std_result = validation_result.to_result()?;
    /// ```
    pub fn to_result(self) -> Result<T> {
        match self {
            Self::Valid(value) => Ok(value),
            Self::Invalid(error) => Err(LumidoxError::ValidationError(error.message())),
        }
    }
}

/// Unified validation manager
/// 
/// Provides consistent validation functionality that can be used by both
/// CLI and GUI components with proper error handling and result types.
pub struct ValidationManager;

impl ValidationManager {
    /// Validate stage number
    /// 
    /// Validates that a stage number is within the valid range (1-5).
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// 
    /// # Returns
    /// * `ValidationResult<u8>` - Validation result with stage number
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_stage_number("3");
    /// ```
    pub fn validate_stage_number(input: &str) -> ValidationResult<u8> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return ValidationResult::Invalid(ValidationError::Missing {
                field_name: "stage_number".to_string(),
                description: "Stage number is required".to_string(),
            });
        }
        
        match trimmed.parse::<u8>() {
            Ok(stage) => {
                if (1..=5).contains(&stage) {
                    ValidationResult::Valid(stage)
                } else {
                    ValidationResult::Invalid(ValidationError::OutOfRange {
                        input: input.to_string(),
                        min: Some(1),
                        max: Some(5),
                        value_type: "stage number".to_string(),
                    })
                }
            }
            Err(_) => ValidationResult::Invalid(ValidationError::InvalidFormat {
                input: input.to_string(),
                expected: "integer between 1 and 5".to_string(),
                context: Some("Stage numbers must be 1, 2, 3, 4, or 5".to_string()),
            }),
        }
    }
    
    /// Validate current value
    /// 
    /// Validates that a current value is within the valid range (1-3000mA).
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// 
    /// # Returns
    /// * `ValidationResult<u16>` - Validation result with current value
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_current_value("1500");
    /// ```
    pub fn validate_current_value(input: &str) -> ValidationResult<u16> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return ValidationResult::Invalid(ValidationError::Missing {
                field_name: "current_value".to_string(),
                description: "Current value is required".to_string(),
            });
        }
        
        // Remove 'mA' suffix if present
        let cleaned = trimmed.to_lowercase().replace("ma", "").replace("m", "");
        
        match cleaned.parse::<u16>() {
            Ok(current) => {
                if current == 0 {
                    ValidationResult::Invalid(ValidationError::OutOfRange {
                        input: input.to_string(),
                        min: Some(1),
                        max: Some(3000),
                        value_type: "current value".to_string(),
                    })
                } else if current > 3000 {
                    ValidationResult::Invalid(ValidationError::OutOfRange {
                        input: input.to_string(),
                        min: Some(1),
                        max: Some(3000),
                        value_type: "current value".to_string(),
                    })
                } else {
                    ValidationResult::Valid(current)
                }
            }
            Err(_) => ValidationResult::Invalid(ValidationError::InvalidFormat {
                input: input.to_string(),
                expected: "integer between 1 and 3000".to_string(),
                context: Some("Current values must be in milliamps (mA)".to_string()),
            }),
        }
    }
    
    /// Validate port name
    /// 
    /// Validates that a port name has a valid format for serial communication.
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// 
    /// # Returns
    /// * `ValidationResult<String>` - Validation result with port name
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_port_name("COM3");
    /// ```
    pub fn validate_port_name(input: &str) -> ValidationResult<String> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return ValidationResult::Invalid(ValidationError::Missing {
                field_name: "port_name".to_string(),
                description: "Port name is required".to_string(),
            });
        }
        
        // Basic port name validation
        if trimmed.len() < 3 {
            return ValidationResult::Invalid(ValidationError::InvalidLength {
                input: input.to_string(),
                min_length: Some(3),
                max_length: None,
            });
        }
        
        // Check for common port name patterns
        let is_valid_pattern = trimmed.starts_with("COM") ||  // Windows
                              trimmed.starts_with("/dev/tty") ||  // Unix/Linux
                              trimmed.starts_with("/dev/cu.") ||  // macOS
                              trimmed.contains("USB") ||  // Generic USB
                              trimmed.contains("ACM");  // Linux ACM devices
        
        if is_valid_pattern {
            ValidationResult::Valid(trimmed.to_string())
        } else {
            ValidationResult::Invalid(ValidationError::InvalidFormat {
                input: input.to_string(),
                expected: "valid serial port name".to_string(),
                context: Some("Examples: COM3, /dev/ttyUSB0, /dev/cu.usbserial".to_string()),
            })
        }
    }
    
    /// Validate boolean input
    /// 
    /// Validates that input represents a boolean value (yes/no, true/false, etc.).
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// 
    /// # Returns
    /// * `ValidationResult<bool>` - Validation result with boolean value
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_boolean("yes");
    /// ```
    pub fn validate_boolean(input: &str) -> ValidationResult<bool> {
        let trimmed = input.trim().to_lowercase();
        
        if trimmed.is_empty() {
            return ValidationResult::Invalid(ValidationError::Missing {
                field_name: "boolean_value".to_string(),
                description: "Boolean value is required".to_string(),
            });
        }
        
        match trimmed.as_str() {
            "true" | "yes" | "y" | "1" | "on" | "enable" | "enabled" => {
                ValidationResult::Valid(true)
            }
            "false" | "no" | "n" | "0" | "off" | "disable" | "disabled" => {
                ValidationResult::Valid(false)
            }
            _ => ValidationResult::Invalid(ValidationError::InvalidFormat {
                input: input.to_string(),
                expected: "boolean value".to_string(),
                context: Some("Valid values: true/false, yes/no, y/n, 1/0, on/off".to_string()),
            }),
        }
    }
    
    /// Validate timeout value
    /// 
    /// Validates that a timeout value is within reasonable range (1-300 seconds).
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// 
    /// # Returns
    /// * `ValidationResult<u32>` - Validation result with timeout value
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_timeout("30");
    /// ```
    pub fn validate_timeout(input: &str) -> ValidationResult<u32> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return ValidationResult::Invalid(ValidationError::Missing {
                field_name: "timeout".to_string(),
                description: "Timeout value is required".to_string(),
            });
        }
        
        // Remove 's' or 'sec' suffix if present
        let cleaned = trimmed.to_lowercase()
            .replace("sec", "")
            .replace("s", "")
            .trim()
            .to_string();
        
        match cleaned.parse::<u32>() {
            Ok(timeout) => {
                if timeout == 0 {
                    ValidationResult::Invalid(ValidationError::OutOfRange {
                        input: input.to_string(),
                        min: Some(1),
                        max: Some(300),
                        value_type: "timeout".to_string(),
                    })
                } else if timeout > 300 {
                    ValidationResult::Invalid(ValidationError::OutOfRange {
                        input: input.to_string(),
                        min: Some(1),
                        max: Some(300),
                        value_type: "timeout".to_string(),
                    })
                } else {
                    ValidationResult::Valid(timeout)
                }
            }
            Err(_) => ValidationResult::Invalid(ValidationError::InvalidFormat {
                input: input.to_string(),
                expected: "integer between 1 and 300".to_string(),
                context: Some("Timeout values must be in seconds".to_string()),
            }),
        }
    }
    
    /// Validate non-empty string
    /// 
    /// Validates that a string is not empty or whitespace-only.
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// * `field_name` - Name of the field being validated
    /// 
    /// # Returns
    /// * `ValidationResult<String>` - Validation result with trimmed string
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_non_empty("test", "username");
    /// ```
    pub fn validate_non_empty(input: &str, field_name: &str) -> ValidationResult<String> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            ValidationResult::Invalid(ValidationError::Missing {
                field_name: field_name.to_string(),
                description: format!("{} cannot be empty", field_name),
            })
        } else {
            ValidationResult::Valid(trimmed.to_string())
        }
    }
    
    /// Validate string length
    /// 
    /// Validates that a string length is within specified bounds.
    /// 
    /// # Arguments
    /// * `input` - Input string to validate
    /// * `min_length` - Minimum length (optional)
    /// * `max_length` - Maximum length (optional)
    /// 
    /// # Returns
    /// * `ValidationResult<String>` - Validation result with string
    /// 
    /// # Example
    /// ```
    /// let result = ValidationManager::validate_string_length("test", Some(2), Some(10));
    /// ```
    pub fn validate_string_length(
        input: &str,
        min_length: Option<usize>,
        max_length: Option<usize>,
    ) -> ValidationResult<String> {
        let length = input.len();
        
        if let Some(min_len) = min_length {
            if length < min_len {
                return ValidationResult::Invalid(ValidationError::InvalidLength {
                    input: input.to_string(),
                    min_length,
                    max_length,
                });
            }
        }
        
        if let Some(max_len) = max_length {
            if length > max_len {
                return ValidationResult::Invalid(ValidationError::InvalidLength {
                    input: input.to_string(),
                    min_length,
                    max_length,
                });
            }
        }
        
        ValidationResult::Valid(input.to_string())
    }
    
    /// Create custom validation error
    /// 
    /// Creates a custom validation error with specified message and retry flag.
    /// 
    /// # Arguments
    /// * `message` - Error message
    /// * `retryable` - Whether the error is retryable
    /// 
    /// # Returns
    /// * `ValidationError` - Custom validation error
    /// 
    /// # Example
    /// ```
    /// let error = ValidationManager::custom_error("Invalid configuration", true);
    /// ```
    pub fn custom_error(message: impl Into<String>, retryable: bool) -> ValidationError {
        ValidationError::Custom {
            message: message.into(),
            retryable,
        }
    }
}
