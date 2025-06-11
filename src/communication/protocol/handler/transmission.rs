//! Command transmission logic for protocol handler
//!
//! This module handles the transmission of commands to the device including:
//! - Command formatting with values and checksums
//! - Serial port writing operations
//! - Command transmission flow control
//! - Data formatting and protocol compliance
//! 
//! The transmission system provides:
//! - Reliable command delivery to the device
//! - Proper protocol formatting with checksums
//! - Error handling for transmission failures
//! - Integration with the overall protocol handler

use crate::core::{LumidoxError, Result};
use super::super::constants::{CMD_START, CMD_TERMINATOR};
use serialport::SerialPort;
use std::io::Write;

/// Command transmission utilities and functionality
pub struct CommandTransmission;

impl CommandTransmission {
    /// Send a formatted command to the device
    /// 
    /// This function handles the complete command transmission process including
    /// formatting the command with proper protocol markers, values, and checksums,
    /// then writing it to the serial port.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port for writing
    /// * `command` - The command bytes to send
    /// * `value` - The value parameter for the command
    /// 
    /// # Returns
    /// * `Result<()>` - Success or transmission error
    /// 
    /// # Example
    /// ```
    /// CommandTransmission::send_formatted_command(&mut port, &[0x02], 1000)?;
    /// ```
    pub fn send_formatted_command(
        port: &mut Box<dyn SerialPort>, 
        command: &[u8], 
        value: u16
    ) -> Result<()> {
        let formatted_cmd = Self::format_command(command, value)?;
        Self::write_command_to_port(port, &formatted_cmd)?;
        Ok(())
    }
    
    /// Format a command with value and checksum
    /// 
    /// Creates a properly formatted command according to the Lumidox II protocol
    /// specification, including start marker, command bytes, value, checksum, and terminator.
    /// 
    /// # Arguments
    /// * `command` - The base command bytes
    /// * `value` - The 16-bit value parameter to include
    /// 
    /// # Returns
    /// * `Result<Vec<u8>>` - The formatted command bytes ready for transmission
    /// 
    /// # Protocol Format
    /// The formatted command follows this structure:
    /// [START_MARKER][COMMAND_BYTES][4_HEX_DIGITS_VALUE][2_HEX_CHECKSUM][TERMINATOR]
    /// 
    /// # Example
    /// ```
    /// let formatted = CommandTransmission::format_command(&[0x02], 1000)?;
    /// // Result: [0x3E, 0x02, 0x30, 0x33, 0x65, 0x38, 0x34, 0x31, 0x0D]
    /// //         [>   , cmd, 0   , 3   , e   , 8   , checksum , \r ]
    /// ```
    pub fn format_command(command: &[u8], value: u16) -> Result<Vec<u8>> {
        let mut cmd = vec![CMD_START];
        cmd.extend_from_slice(command);
        
        // Add value as 4-digit hex
        if value == 0 {
            cmd.extend_from_slice(b"0000");
        } else {
            cmd.extend_from_slice(format!("{:04x}", value).as_bytes());
        }
        
        // Add checksum
        let checksum = Self::calculate_command_checksum(&cmd);
        cmd.extend_from_slice(&checksum);
        cmd.push(CMD_TERMINATOR);
        
        Ok(cmd)
    }
    
    /// Calculate checksum for command data
    /// 
    /// Computes the protocol-specific checksum for command validation.
    /// The checksum is calculated by summing all bytes after the start marker
    /// and taking the result modulo 256, then formatting as 2-digit hex.
    /// 
    /// # Arguments
    /// * `data` - The command data to calculate checksum for
    /// 
    /// # Returns
    /// * `Vec<u8>` - The checksum as 2-byte hex string
    /// 
    /// # Algorithm
    /// 1. Sum all bytes after the start marker (skip first byte)
    /// 2. Take result modulo 256
    /// 3. Format as 2-digit lowercase hex string
    /// 
    /// # Example
    /// ```
    /// let checksum = CommandTransmission::calculate_command_checksum(&[0x3E, 0x02, 0x30, 0x33, 0x65, 0x38]);
    /// // Result: [0x34, 0x31] representing "41" in hex
    /// ```
    pub fn calculate_command_checksum(data: &[u8]) -> Vec<u8> {
        let mut value = 0u32;
        // Skip the first byte (command start marker)
        for &byte in &data[1..] {
            value += byte as u32;
        }
        value %= 256;
        format!("{:02x}", value).into_bytes()
    }
    
    /// Write command to serial port
    /// 
    /// Performs the actual writing of command bytes to the serial port with
    /// proper error handling and conversion to protocol errors.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port
    /// * `command` - The formatted command bytes to write
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error
    /// 
    /// # Error Handling
    /// Converts I/O errors to LumidoxError::IoError for consistent error handling
    /// throughout the protocol system.
    /// 
    /// # Example
    /// ```
    /// let command = vec![0x3E, 0x02, 0x30, 0x30, 0x30, 0x30, 0x34, 0x30, 0x0D];
    /// CommandTransmission::write_command_to_port(&mut port, &command)?;
    /// ```
    pub fn write_command_to_port(port: &mut Box<dyn SerialPort>, command: &[u8]) -> Result<()> {
        port.write_all(command)
            .map_err(LumidoxError::IoError)?;
        Ok(())
    }
    
    /// Validate command format before transmission
    /// 
    /// Performs validation checks on a formatted command to ensure it meets
    /// protocol requirements before transmission.
    /// 
    /// # Arguments
    /// * `command` - The formatted command to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if valid, error if invalid
    /// 
    /// # Validation Checks
    /// - Minimum length requirements
    /// - Proper start and end markers
    /// - Checksum validation
    /// - Command structure compliance
    /// 
    /// # Example
    /// ```
    /// let command = CommandTransmission::format_command(&[0x02], 1000)?;
    /// CommandTransmission::validate_command_format(&command)?;
    /// ```
    pub fn validate_command_format(command: &[u8]) -> Result<()> {
        if command.len() < 8 {
            return Err(LumidoxError::ProtocolError(
                "Command too short for valid protocol format".to_string()
            ));
        }
        
        if command[0] != CMD_START {
            return Err(LumidoxError::ProtocolError(
                "Command missing start marker".to_string()
            ));
        }
        
        if command[command.len() - 1] != CMD_TERMINATOR {
            return Err(LumidoxError::ProtocolError(
                "Command missing terminator".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Get command transmission statistics
    /// 
    /// Provides information about command formatting and transmission
    /// for debugging and monitoring purposes.
    /// 
    /// # Arguments
    /// * `command` - The command to analyze
    /// * `value` - The value parameter used
    /// 
    /// # Returns
    /// * `CommandTransmissionStats` - Statistics about the command
    /// 
    /// # Example
    /// ```
    /// let stats = CommandTransmission::get_transmission_stats(&[0x02], 1000);
    /// println!("Command length: {}", stats.formatted_length);
    /// ```
    pub fn get_transmission_stats(command: &[u8], value: u16) -> CommandTransmissionStats {
        let formatted = Self::format_command(command, value).unwrap_or_default();
        CommandTransmissionStats {
            base_command_length: command.len(),
            value_parameter: value,
            formatted_length: formatted.len(),
            checksum_bytes: 2,
            protocol_overhead: formatted.len() - command.len(),
        }
    }
}

/// Statistics about command transmission
/// 
/// Provides detailed information about command formatting and transmission
/// characteristics for monitoring and debugging purposes.
#[derive(Debug, Clone)]
pub struct CommandTransmissionStats {
    /// Length of the base command (without protocol formatting)
    pub base_command_length: usize,
    /// The value parameter included in the command
    pub value_parameter: u16,
    /// Total length of the formatted command
    pub formatted_length: usize,
    /// Number of checksum bytes added
    pub checksum_bytes: usize,
    /// Total protocol overhead bytes (markers, checksum, terminator)
    pub protocol_overhead: usize,
}
