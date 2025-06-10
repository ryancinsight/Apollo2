//! Protocol handler for Lumidox II Controller communication
//!
//! This module provides the core ProtocolHandler struct that manages
//! low-level serial communication, command formatting, and response parsing.

use crate::core::{LumidoxError, Result};
use super::constants::{CMD_START, RESPONSE_END, CMD_TERMINATOR, DEFAULT_TIMEOUT};
use serialport::SerialPort;
use std::io::{Read, Write};

/// Low-level protocol handler
pub struct ProtocolHandler {
    port: Box<dyn SerialPort>,
}

impl ProtocolHandler {
    /// Create a new protocol handler with the given serial port
    pub fn new(mut port: Box<dyn SerialPort>) -> Result<Self> {
        // Set timeout
        port.set_timeout(DEFAULT_TIMEOUT)
            .map_err(LumidoxError::SerialError)?;
        
        Ok(ProtocolHandler { port })
    }
    
    /// Calculate checksum for command data
    pub fn calculate_checksum(data: &[u8]) -> Vec<u8> {
        let mut value = 0u32;
        // Skip the first byte (command start marker)
        for &byte in &data[1..] {
            value += byte as u32;
        }
        value %= 256;
        format!("{:02x}", value).into_bytes()
    }
    
    /// Convert hex response to decimal value
    pub fn hex_to_decimal(buffer: &[u8]) -> i32 {
        if buffer.len() < 5 {
            return 0;
        }
        
        let mut value = 0i32;
        let mut multiplier = 4096i32;
        
        // Process 4 hex digits starting from position 1
        for pos in 1..5 {
            let byte_val = buffer[pos];
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
        
        value
    }
    
    /// Send a command and receive response
    pub fn send_command(&mut self, command: &[u8], value: u16) -> Result<i32> {
        let formatted_cmd = self.format_command(command, value)?;
        self.write_command(&formatted_cmd)?;
        let response = self.read_response()?;
        Ok(Self::hex_to_decimal(&response))
    }
    
    /// Format a command with value and checksum
    fn format_command(&self, command: &[u8], value: u16) -> Result<Vec<u8>> {
        let mut cmd = vec![CMD_START];
        cmd.extend_from_slice(command);
        
        // Add value as 4-digit hex
        if value == 0 {
            cmd.extend_from_slice(b"0000");
        } else {
            cmd.extend_from_slice(format!("{:04x}", value).as_bytes());
        }
        
        // Add checksum
        let checksum = Self::calculate_checksum(&cmd);
        cmd.extend_from_slice(&checksum);
        cmd.push(CMD_TERMINATOR);
        
        Ok(cmd)
    }
    
    /// Write command to serial port
    fn write_command(&mut self, command: &[u8]) -> Result<()> {
        self.port.write_all(command)
            .map_err(LumidoxError::IoError)?;
        Ok(())
    }
    
    /// Read response from serial port
    fn read_response(&mut self) -> Result<Vec<u8>> {
        let mut response = Vec::new();
        let mut buffer = [0u8; 1];
        
        loop {
            match self.port.read(&mut buffer) {
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
    
    /// Get the underlying serial port (for advanced operations)
    pub fn port_mut(&mut self) -> &mut Box<dyn SerialPort> {
        &mut self.port
    }
}
