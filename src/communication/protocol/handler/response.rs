//! Response processing logic for protocol handler
//!
//! This module handles the processing of responses from the device including:
//! - Response reading from serial port
//! - Data parsing and interpretation
//! - Hex to decimal conversion
//! - Response validation and error handling
//! 
//! The response system provides:
//! - Reliable response reading with proper termination detection
//! - Accurate data conversion from device format to application format
//! - Error handling for malformed or missing responses
//! - Integration with the overall protocol handler

use crate::core::{LumidoxError, Result};
use super::super::constants::RESPONSE_END;
use serialport::SerialPort;
use std::io::Read;

/// Response processing utilities and functionality
pub struct ResponseProcessor;

impl ResponseProcessor {
    /// Read and process a complete response from the device
    /// 
    /// This function handles the complete response processing workflow including
    /// reading the raw response from the serial port, validating it, and
    /// converting it to the appropriate data format.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port for reading
    /// 
    /// # Returns
    /// * `Result<i32>` - The processed response value or error
    /// 
    /// # Example
    /// ```
    /// let value = ResponseProcessor::read_and_process_response(&mut port)?;
    /// println!("Device returned: {}", value);
    /// ```
    pub fn read_and_process_response(port: &mut Box<dyn SerialPort>) -> Result<i32> {
        let response = Self::read_raw_response(port)?;
        Self::validate_response_format(&response)?;
        Ok(Self::convert_hex_response_to_decimal(&response))
    }
    
    /// Read raw response from serial port
    /// 
    /// Reads bytes from the serial port until the response end marker is found
    /// or an error occurs. Handles various read scenarios including partial reads
    /// and timeout conditions.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port
    /// 
    /// # Returns
    /// * `Result<Vec<u8>>` - The raw response bytes or error
    /// 
    /// # Protocol Behavior
    /// - Reads byte-by-byte until RESPONSE_END marker is found
    /// - Handles partial reads and continues until complete response
    /// - Returns error if no data is received
    /// - Includes the end marker in the returned response
    /// 
    /// # Example
    /// ```
    /// let response = ResponseProcessor::read_raw_response(&mut port)?;
    /// // Response might be: [0x31, 0x32, 0x33, 0x34, 0x0A] for "1234\n"
    /// ```
    pub fn read_raw_response(port: &mut Box<dyn SerialPort>) -> Result<Vec<u8>> {
        let mut response = Vec::new();
        let mut buffer = [0u8; 1];
        
        loop {
            match port.read(&mut buffer) {
                Ok(1) => {
                    response.push(buffer[0]);
                    if buffer[0] == RESPONSE_END {
                        break;
                    }
                }
                Ok(0) => break, // No more data
                Ok(_) => {
                    // Handle unexpected read size
                    response.push(buffer[0]);
                    if buffer[0] == RESPONSE_END {
                        break;
                    }
                }
                Err(e) => return Err(LumidoxError::IoError(e)),
            }
        }
        
        if response.is_empty() {
            return Err(LumidoxError::ProtocolError(
                "No response received from device".to_string()
            ));
        }
        
        Ok(response)
    }
    
    /// Convert hex response to decimal value
    /// 
    /// Converts a hex-encoded response from the device to a decimal integer value.
    /// Handles the specific format used by the Lumidox II protocol including
    /// signed 16-bit value representation.
    /// 
    /// # Arguments
    /// * `buffer` - The response buffer containing hex digits
    /// 
    /// # Returns
    /// * `i32` - The converted decimal value
    /// 
    /// # Protocol Format
    /// The response is expected to contain 4 hex digits starting at position 1:
    /// [MARKER][HEX_DIGIT_1][HEX_DIGIT_2][HEX_DIGIT_3][HEX_DIGIT_4][...][END]
    /// 
    /// # Conversion Algorithm
    /// 1. Extract 4 hex digits from positions 1-4
    /// 2. Convert each hex digit to its numeric value
    /// 3. Combine using positional notation (base 16)
    /// 4. Handle signed 16-bit representation (two's complement)
    /// 
    /// # Example
    /// ```
    /// let response = vec![0x3E, 0x31, 0x32, 0x33, 0x34, 0x0A]; // ">1234\n"
    /// let value = ResponseProcessor::convert_hex_response_to_decimal(&response);
    /// assert_eq!(value, 0x1234); // 4660 in decimal
    /// ```
    pub fn convert_hex_response_to_decimal(buffer: &[u8]) -> i32 {
        if buffer.len() < 5 {
            return 0;
        }
        
        let mut value = 0i32;
        let mut multiplier = 4096i32; // 16^3
        
        // Process 4 hex digits starting from position 1
        for pos in 1..5 {
            let byte_val = buffer[pos];
            let digit_val = if byte_val < 97 { 
                byte_val - 48  // '0'-'9' (ASCII 48-57)
            } else { 
                byte_val - 87  // 'a'-'f' (ASCII 97-102)
            };
            value += (digit_val as i32) * multiplier;
            multiplier /= 16;
        }
        
        // Handle signed 16-bit values (two's complement)
        if value > 32767 {
            value -= 65536;
        }
        
        value
    }
    
    /// Validate response format
    /// 
    /// Performs validation checks on a response to ensure it meets protocol
    /// requirements and contains valid data.
    /// 
    /// # Arguments
    /// * `response` - The response bytes to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if valid, error if invalid
    /// 
    /// # Validation Checks
    /// - Minimum length requirements for hex data
    /// - Proper response termination
    /// - Valid hex digit format
    /// - Response structure compliance
    /// 
    /// # Example
    /// ```
    /// let response = vec![0x3E, 0x31, 0x32, 0x33, 0x34, 0x0A];
    /// ResponseProcessor::validate_response_format(&response)?;
    /// ```
    pub fn validate_response_format(response: &[u8]) -> Result<()> {
        if response.len() < 5 {
            return Err(LumidoxError::ProtocolError(
                "Response too short for valid hex data".to_string()
            ));
        }
        
        if response[response.len() - 1] != RESPONSE_END {
            return Err(LumidoxError::ProtocolError(
                "Response missing proper termination".to_string()
            ));
        }
        
        // Validate hex digits in positions 1-4
        for pos in 1..5.min(response.len()) {
            let byte_val = response[pos];
            if !Self::is_valid_hex_digit(byte_val) {
                return Err(LumidoxError::ProtocolError(
                    format!("Invalid hex digit at position {}: 0x{:02x}", pos, byte_val)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check if a byte represents a valid hex digit
    /// 
    /// Validates that a byte value represents a valid hexadecimal digit
    /// in ASCII encoding ('0'-'9', 'a'-'f', 'A'-'F').
    /// 
    /// # Arguments
    /// * `byte` - The byte value to check
    /// 
    /// # Returns
    /// * `bool` - True if valid hex digit, false otherwise
    /// 
    /// # Example
    /// ```
    /// assert!(ResponseProcessor::is_valid_hex_digit(b'5'));
    /// assert!(ResponseProcessor::is_valid_hex_digit(b'a'));
    /// assert!(ResponseProcessor::is_valid_hex_digit(b'F'));
    /// assert!(!ResponseProcessor::is_valid_hex_digit(b'g'));
    /// ```
    pub fn is_valid_hex_digit(byte: u8) -> bool {
        matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
    }
    
    /// Parse response for specific data types
    /// 
    /// Provides specialized parsing for different types of device responses
    /// based on the expected data format and range.
    /// 
    /// # Arguments
    /// * `response` - The response bytes to parse
    /// * `data_type` - The expected data type for validation
    /// 
    /// # Returns
    /// * `Result<ResponseData>` - Parsed response data or error
    /// 
    /// # Example
    /// ```
    /// let response = vec![0x3E, 0x31, 0x32, 0x33, 0x34, 0x0A];
    /// let data = ResponseProcessor::parse_typed_response(&response, ResponseDataType::Current)?;
    /// ```
    pub fn parse_typed_response(response: &[u8], data_type: ResponseDataType) -> Result<ResponseData> {
        Self::validate_response_format(response)?;
        let value = Self::convert_hex_response_to_decimal(response);
        
        match data_type {
            ResponseDataType::Current => {
                if value < 0 || value > 10000 {
                    return Err(LumidoxError::ProtocolError(
                        format!("Current value out of range: {}", value)
                    ));
                }
                Ok(ResponseData::Current(value as u16))
            }
            ResponseDataType::Voltage => {
                if value < 0 || value > 5000 {
                    return Err(LumidoxError::ProtocolError(
                        format!("Voltage value out of range: {}", value)
                    ));
                }
                Ok(ResponseData::Voltage(value as u16))
            }
            ResponseDataType::Status => {
                Ok(ResponseData::Status(value as u16))
            }
            ResponseDataType::Raw => {
                Ok(ResponseData::Raw(value))
            }
        }
    }
    
    /// Get response processing statistics
    /// 
    /// Provides information about response processing for debugging
    /// and monitoring purposes.
    /// 
    /// # Arguments
    /// * `response` - The response to analyze
    /// 
    /// # Returns
    /// * `ResponseProcessingStats` - Statistics about the response
    /// 
    /// # Example
    /// ```
    /// let stats = ResponseProcessor::get_processing_stats(&response);
    /// println!("Response length: {}", stats.total_length);
    /// ```
    pub fn get_processing_stats(response: &[u8]) -> ResponseProcessingStats {
        let hex_digits = if response.len() >= 5 {
            response[1..5].to_vec()
        } else {
            Vec::new()
        };
        
        ResponseProcessingStats {
            total_length: response.len(),
            hex_data_length: hex_digits.len(),
            has_terminator: response.last() == Some(&RESPONSE_END),
            converted_value: if response.len() >= 5 {
                Self::convert_hex_response_to_decimal(response)
            } else {
                0
            },
            is_valid_format: Self::validate_response_format(response).is_ok(),
        }
    }
}

/// Types of response data for specialized parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseDataType {
    /// Current measurement data (0-10000 mA)
    Current,
    /// Voltage measurement data (0-5000 mV)
    Voltage,
    /// Status or mode data
    Status,
    /// Raw unvalidated data
    Raw,
}

/// Parsed response data with type information
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseData {
    /// Current value in milliamps
    Current(u16),
    /// Voltage value in millivolts
    Voltage(u16),
    /// Status value
    Status(u16),
    /// Raw integer value
    Raw(i32),
}

/// Statistics about response processing
#[derive(Debug, Clone)]
pub struct ResponseProcessingStats {
    /// Total length of the response in bytes
    pub total_length: usize,
    /// Length of the hex data portion
    pub hex_data_length: usize,
    /// Whether the response has proper termination
    pub has_terminator: bool,
    /// The converted decimal value
    pub converted_value: i32,
    /// Whether the response format is valid
    pub is_valid_format: bool,
}
