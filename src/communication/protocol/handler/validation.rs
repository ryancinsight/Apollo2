//! Protocol validation logic for protocol handler
//!
//! This module handles validation of protocol operations including:
//! - Command format validation and integrity checking
//! - Response validation and format verification
//! - Protocol compliance verification
//! - Data integrity and checksum validation
//! 
//! The validation system provides:
//! - Comprehensive protocol compliance checking
//! - Data integrity verification through checksums
//! - Format validation for commands and responses
//! - Error detection and reporting for protocol violations

use crate::core::{LumidoxError, Result};
use super::super::constants::{CMD_START, CMD_TERMINATOR, RESPONSE_END};

/// Protocol validation utilities and functionality
pub struct ProtocolValidator;

impl ProtocolValidator {
    /// Validate a complete protocol command
    /// 
    /// Performs comprehensive validation of a protocol command to ensure
    /// it meets all format requirements and protocol specifications.
    /// 
    /// # Arguments
    /// * `command` - The formatted command to validate
    /// 
    /// # Returns
    /// * `Result<ValidationReport>` - Validation results or error
    /// 
    /// # Validation Checks
    /// - Command length requirements
    /// - Start and termination markers
    /// - Checksum verification
    /// - Format compliance
    /// 
    /// # Example
    /// ```
    /// let command = vec![0x3E, 0x02, 0x30, 0x30, 0x30, 0x30, 0x34, 0x30, 0x0D];
    /// let report = ProtocolValidator::validate_command(&command)?;
    /// if report.is_valid {
    ///     println!("Command is valid for transmission");
    /// }
    /// ```
    pub fn validate_command(command: &[u8]) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();
        
        // Check minimum length
        if command.len() < 8 {
            report.add_error("Command too short for valid protocol format".to_string());
            return Ok(report);
        }
        
        // Validate start marker
        if command[0] != CMD_START {
            report.add_error("Command missing start marker".to_string());
        } else {
            report.add_success("Start marker valid".to_string());
        }
        
        // Validate terminator
        if command[command.len() - 1] != CMD_TERMINATOR {
            report.add_error("Command missing terminator".to_string());
        } else {
            report.add_success("Terminator valid".to_string());
        }
        
        // Validate checksum
        if let Err(e) = Self::validate_command_checksum(command) {
            report.add_error(format!("Checksum validation failed: {}", e));
        } else {
            report.add_success("Checksum valid".to_string());
        }
        
        // Validate hex format in value section
        if command.len() >= 8 {
            let value_section = &command[command.len() - 6..command.len() - 3];
            if Self::validate_hex_format(value_section) {
                report.add_success("Value format valid".to_string());
            } else {
                report.add_error("Invalid hex format in value section".to_string());
            }
        }
        
        Ok(report)
    }
    
    /// Validate a protocol response
    /// 
    /// Performs validation of a response received from the device to ensure
    /// it meets protocol requirements and contains valid data.
    /// 
    /// # Arguments
    /// * `response` - The response bytes to validate
    /// 
    /// # Returns
    /// * `Result<ValidationReport>` - Validation results or error
    /// 
    /// # Validation Checks
    /// - Response length requirements
    /// - Proper termination
    /// - Hex data format validation
    /// - Response structure compliance
    /// 
    /// # Example
    /// ```
    /// let response = vec![0x3E, 0x31, 0x32, 0x33, 0x34, 0x0A];
    /// let report = ProtocolValidator::validate_response(&response)?;
    /// ```
    pub fn validate_response(response: &[u8]) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();
        
        // Check minimum length
        if response.len() < 5 {
            report.add_error("Response too short for valid hex data".to_string());
            return Ok(report);
        }
        
        // Validate termination
        if response[response.len() - 1] != RESPONSE_END {
            report.add_error("Response missing proper termination".to_string());
        } else {
            report.add_success("Response termination valid".to_string());
        }
        
        // Validate hex digits in data section
        let hex_section = &response[1..5.min(response.len())];
        if Self::validate_hex_format(hex_section) {
            report.add_success("Hex data format valid".to_string());
        } else {
            report.add_error("Invalid hex format in response data".to_string());
        }
        
        // Check for reasonable data range
        if let Ok(value) = Self::extract_response_value(response) {
            if Self::validate_response_value_range(value) {
                report.add_success("Response value within reasonable range".to_string());
            } else {
                report.add_warning(format!("Response value {} may be out of expected range", value));
            }
        }
        
        Ok(report)
    }
    
    /// Validate command checksum
    /// 
    /// Verifies that the checksum in a command matches the calculated
    /// checksum for the command data.
    /// 
    /// # Arguments
    /// * `command` - The command with embedded checksum to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if checksum is valid, error if invalid
    /// 
    /// # Algorithm
    /// 1. Extract the embedded checksum from the command
    /// 2. Calculate the expected checksum for the command data
    /// 3. Compare embedded and calculated checksums
    /// 
    /// # Example
    /// ```
    /// ProtocolValidator::validate_command_checksum(&command)?;
    /// ```
    pub fn validate_command_checksum(command: &[u8]) -> Result<()> {
        if command.len() < 8 {
            return Err(LumidoxError::ProtocolError(
                "Command too short for checksum validation".to_string()
            ));
        }
        
        // Extract embedded checksum (last 2 bytes before terminator)
        let embedded_checksum = &command[command.len() - 3..command.len() - 1];
        
        // Calculate expected checksum for data before checksum
        let data_for_checksum = &command[..command.len() - 3];
        let calculated_checksum = Self::calculate_checksum(data_for_checksum);
        
        if embedded_checksum == calculated_checksum.as_slice() {
            Ok(())
        } else {
            Err(LumidoxError::ProtocolError(
                format!("Checksum mismatch: embedded {:?}, calculated {:?}", 
                       embedded_checksum, calculated_checksum)
            ))
        }
    }
    
    /// Calculate checksum for data
    /// 
    /// Computes the protocol-specific checksum for the given data.
    /// 
    /// # Arguments
    /// * `data` - The data to calculate checksum for
    /// 
    /// # Returns
    /// * `Vec<u8>` - The calculated checksum bytes
    /// 
    /// # Example
    /// ```
    /// let checksum = ProtocolValidator::calculate_checksum(&data);
    /// ```
    pub fn calculate_checksum(data: &[u8]) -> Vec<u8> {
        let mut value = 0u32;
        // Skip the first byte (command start marker)
        for &byte in &data[1..] {
            value += byte as u32;
        }
        value %= 256;
        format!("{:02x}", value).into_bytes()
    }
    
    /// Validate hex format of data
    /// 
    /// Checks that all bytes in the data represent valid hexadecimal digits.
    /// 
    /// # Arguments
    /// * `data` - The data to validate as hex format
    /// 
    /// # Returns
    /// * `bool` - True if all bytes are valid hex digits, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(ProtocolValidator::validate_hex_format(b"1234"));
    /// assert!(ProtocolValidator::validate_hex_format(b"abcd"));
    /// assert!(!ProtocolValidator::validate_hex_format(b"xyz"));
    /// ```
    pub fn validate_hex_format(data: &[u8]) -> bool {
        data.iter().all(|&byte| Self::is_valid_hex_digit(byte))
    }
    
    /// Check if a byte represents a valid hex digit
    /// 
    /// Validates that a byte value represents a valid hexadecimal digit.
    /// 
    /// # Arguments
    /// * `byte` - The byte to check
    /// 
    /// # Returns
    /// * `bool` - True if valid hex digit, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(ProtocolValidator::is_valid_hex_digit(b'5'));
    /// assert!(ProtocolValidator::is_valid_hex_digit(b'a'));
    /// assert!(!ProtocolValidator::is_valid_hex_digit(b'g'));
    /// ```
    pub fn is_valid_hex_digit(byte: u8) -> bool {
        matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
    }
    
    /// Extract numeric value from response
    /// 
    /// Extracts and converts the numeric value from a response.
    /// 
    /// # Arguments
    /// * `response` - The response to extract value from
    /// 
    /// # Returns
    /// * `Result<i32>` - The extracted value or error
    /// 
    /// # Example
    /// ```
    /// let value = ProtocolValidator::extract_response_value(&response)?;
    /// ```
    pub fn extract_response_value(response: &[u8]) -> Result<i32> {
        if response.len() < 5 {
            return Err(LumidoxError::ProtocolError(
                "Response too short to extract value".to_string()
            ));
        }
        
        let mut value = 0i32;
        let mut multiplier = 4096i32; // 16^3
        
        // Process 4 hex digits starting from position 1
        for pos in 1..5 {
            let byte_val = response[pos];
            if !Self::is_valid_hex_digit(byte_val) {
                return Err(LumidoxError::ProtocolError(
                    format!("Invalid hex digit at position {}: 0x{:02x}", pos, byte_val)
                ));
            }
            
            let digit_val = if byte_val < 97 { 
                byte_val - 48  // '0'-'9'
            } else { 
                byte_val - 87  // 'a'-'f'
            };
            value += (digit_val as i32) * multiplier;
            multiplier /= 16;
        }
        
        // Handle signed 16-bit values
        if value > 32767 {
            value -= 65536;
        }
        
        Ok(value)
    }
    
    /// Validate response value range
    /// 
    /// Checks if a response value is within reasonable expected ranges
    /// for typical device operations.
    /// 
    /// # Arguments
    /// * `value` - The value to validate
    /// 
    /// # Returns
    /// * `bool` - True if value is within reasonable range, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(ProtocolValidator::validate_response_value_range(1000));
    /// assert!(!ProtocolValidator::validate_response_value_range(-50000));
    /// ```
    pub fn validate_response_value_range(value: i32) -> bool {
        // Define reasonable ranges for different types of values
        // This is a general validation - specific validation should be done
        // based on the expected data type
        value >= -32768 && value <= 32767
    }
    
    /// Validate protocol timing constraints
    /// 
    /// Validates that protocol operations meet timing requirements
    /// for reliable communication.
    /// 
    /// # Arguments
    /// * `operation_duration` - Duration of the protocol operation
    /// * `operation_type` - Type of operation for specific timing requirements
    /// 
    /// # Returns
    /// * `Result<()>` - Success if timing is acceptable, error if not
    /// 
    /// # Example
    /// ```
    /// let duration = std::time::Duration::from_millis(500);
    /// ProtocolValidator::validate_timing_constraints(duration, TimingType::Command)?;
    /// ```
    pub fn validate_timing_constraints(
        operation_duration: std::time::Duration, 
        operation_type: TimingType
    ) -> Result<()> {
        let max_duration = match operation_type {
            TimingType::Command => std::time::Duration::from_secs(5),
            TimingType::Response => std::time::Duration::from_secs(10),
            TimingType::Connection => std::time::Duration::from_secs(30),
        };
        
        if operation_duration > max_duration {
            Err(LumidoxError::ProtocolError(
                format!("Operation took too long: {:?} > {:?}", operation_duration, max_duration)
            ))
        } else {
            Ok(())
        }
    }
}

/// Types of protocol operations for timing validation
#[derive(Debug, Clone, PartialEq)]
pub enum TimingType {
    /// Command transmission operation
    Command,
    /// Response reception operation
    Response,
    /// Connection establishment operation
    Connection,
}

/// Validation report containing results of protocol validation
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether the overall validation passed
    pub is_valid: bool,
    /// List of validation errors found
    pub errors: Vec<String>,
    /// List of validation warnings
    pub warnings: Vec<String>,
    /// List of successful validation checks
    pub successes: Vec<String>,
}

impl ValidationReport {
    /// Create a new empty validation report
    pub fn new() -> Self {
        ValidationReport {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            successes: Vec::new(),
        }
    }
    
    /// Add an error to the validation report
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }
    
    /// Add a warning to the validation report
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    /// Add a success to the validation report
    pub fn add_success(&mut self, success: String) {
        self.successes.push(success);
    }
    
    /// Get a summary of the validation results
    pub fn summary(&self) -> String {
        format!(
            "Validation: {} (Errors: {}, Warnings: {}, Successes: {})",
            if self.is_valid { "PASSED" } else { "FAILED" },
            self.errors.len(),
            self.warnings.len(),
            self.successes.len()
        )
    }
}
