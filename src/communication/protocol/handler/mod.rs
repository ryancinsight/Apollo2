//! Protocol handler module for Lumidox II Controller communication
//!
//! This module provides the core ProtocolHandler struct that manages
//! low-level serial communication through specialized sub-modules:
//! 
//! - transmission: Command formatting, transmission, and flow control
//! - response: Response reading, parsing, and data interpretation
//! - connection: Serial port connection management and configuration
//! - validation: Protocol validation, integrity checking, and error detection
//! 
//! The handler system provides:
//! - Reliable command transmission with proper protocol formatting
//! - Accurate response processing and data conversion
//! - Robust connection management with health monitoring
//! - Comprehensive protocol validation and error detection
//! - Seamless integration maintaining the existing public API

use crate::core::{LumidoxError, Result};
use super::constants::DEFAULT_TIMEOUT;
use serialport::SerialPort;

// Import specialized sub-modules
pub mod transmission;
pub mod response;
pub mod connection;
pub mod validation;

// Re-export commonly used items for convenience
pub use transmission::{CommandTransmission, CommandTransmissionStats};
pub use response::{ResponseProcessor, ResponseData, ResponseDataType, ResponseProcessingStats};
pub use connection::{ConnectionManager, ConnectionInfo, ConnectionHealth, HealthStatus};
pub use validation::{ProtocolValidator, ValidationReport, TimingType};

/// Low-level protocol handler with enhanced modular architecture
/// 
/// The ProtocolHandler provides a unified interface for device communication
/// while internally using specialized modules for different aspects of
/// protocol handling. This maintains the existing public API while
/// providing improved internal organization and maintainability.
pub struct ProtocolHandler {
    port: Box<dyn SerialPort>,
}

impl ProtocolHandler {
    /// Create a new protocol handler with the given serial port
    /// 
    /// Initializes a new protocol handler using the connection management
    /// module to properly configure the serial port for protocol operations.
    /// 
    /// # Arguments
    /// * `port` - The serial port to use for communication
    /// 
    /// # Returns
    /// * `Result<Self>` - The configured protocol handler or error
    /// 
    /// # Example
    /// ```
    /// let port = serialport::new("/dev/ttyUSB0", 9600).open()?;
    /// let handler = ProtocolHandler::new(port)?;
    /// ```
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self> {
        let configured_port = ConnectionManager::initialize_connection(port)?;
        Ok(ProtocolHandler { port: configured_port })
    }
    
    /// Send a command and receive response
    /// 
    /// This is the main public interface for protocol communication.
    /// It uses the transmission and response modules internally while
    /// maintaining the exact same API as the original implementation.
    /// 
    /// # Arguments
    /// * `command` - The command bytes to send
    /// * `value` - The value parameter for the command
    /// 
    /// # Returns
    /// * `Result<i32>` - The response value or error
    /// 
    /// # Example
    /// ```
    /// let result = handler.send_command(&[0x02], 1000)?;
    /// println!("Device returned: {}", result);
    /// ```
    pub fn send_command(&mut self, command: &[u8], value: u16) -> Result<i32> {
        // Use transmission module to send the command
        CommandTransmission::send_formatted_command(&mut self.port, command, value)?;
        
        // Use response module to read and process the response
        ResponseProcessor::read_and_process_response(&mut self.port)
    }
    
    /// Calculate checksum for command data
    /// 
    /// Delegates to the validation module while maintaining the original
    /// public API for backward compatibility.
    /// 
    /// # Arguments
    /// * `data` - The data to calculate checksum for
    /// 
    /// # Returns
    /// * `Vec<u8>` - The calculated checksum bytes
    /// 
    /// # Example
    /// ```
    /// let checksum = ProtocolHandler::calculate_checksum(&data);
    /// ```
    pub fn calculate_checksum(data: &[u8]) -> Vec<u8> {
        ProtocolValidator::calculate_checksum(data)
    }
    
    /// Convert hex response to decimal value
    /// 
    /// Delegates to the response module while maintaining the original
    /// public API for backward compatibility.
    /// 
    /// # Arguments
    /// * `buffer` - The response buffer containing hex digits
    /// 
    /// # Returns
    /// * `i32` - The converted decimal value
    /// 
    /// # Example
    /// ```
    /// let value = ProtocolHandler::hex_to_decimal(&response);
    /// ```
    pub fn hex_to_decimal(buffer: &[u8]) -> i32 {
        ResponseProcessor::convert_hex_response_to_decimal(buffer)
    }
    
    /// Get the underlying serial port (for advanced operations)
    /// 
    /// Provides access to the underlying serial port through the
    /// connection manager for advanced operations while maintaining
    /// the original public API.
    /// 
    /// # Returns
    /// * `&mut Box<dyn SerialPort>` - Mutable reference to the serial port
    /// 
    /// # Example
    /// ```
    /// let port = handler.port_mut();
    /// // Perform advanced port operations
    /// ```
    pub fn port_mut(&mut self) -> &mut Box<dyn SerialPort> {
        ConnectionManager::get_port_access(&mut self.port)
    }
    
    /// Get connection information and health status
    /// 
    /// Provides detailed information about the current connection
    /// using the connection management module.
    /// 
    /// # Returns
    /// * `ConnectionInfo` - Detailed connection information
    /// 
    /// # Example
    /// ```
    /// let info = handler.get_connection_info();
    /// println!("Connected to: {}", info.port_name);
    /// ```
    pub fn get_connection_info(&self) -> ConnectionInfo {
        ConnectionManager::get_connection_info(&self.port)
    }
    
    /// Monitor connection health
    /// 
    /// Provides ongoing monitoring of connection health using the
    /// connection management module.
    /// 
    /// # Returns
    /// * `ConnectionHealth` - Current health status
    /// 
    /// # Example
    /// ```
    /// let health = handler.monitor_health();
    /// if health.status == HealthStatus::Good {
    ///     println!("Connection is healthy");
    /// }
    /// ```
    pub fn monitor_health(&self) -> ConnectionHealth {
        ConnectionManager::monitor_connection_health(&self.port)
    }
    
    /// Validate a command before transmission
    /// 
    /// Uses the validation module to perform comprehensive validation
    /// of a command before transmission.
    /// 
    /// # Arguments
    /// * `command` - The command bytes to validate
    /// * `value` - The value parameter for the command
    /// 
    /// # Returns
    /// * `Result<ValidationReport>` - Validation results or error
    /// 
    /// # Example
    /// ```
    /// let report = handler.validate_command(&[0x02], 1000)?;
    /// if !report.is_valid {
    ///     println!("Command validation failed: {:?}", report.errors);
    /// }
    /// ```
    pub fn validate_command(&self, command: &[u8], value: u16) -> Result<ValidationReport> {
        let formatted_command = CommandTransmission::format_command(command, value)?;
        ProtocolValidator::validate_command(&formatted_command)
    }
    
    /// Get transmission statistics
    /// 
    /// Provides statistics about command transmission using the
    /// transmission module.
    /// 
    /// # Arguments
    /// * `command` - The command to analyze
    /// * `value` - The value parameter
    /// 
    /// # Returns
    /// * `CommandTransmissionStats` - Transmission statistics
    /// 
    /// # Example
    /// ```
    /// let stats = handler.get_transmission_stats(&[0x02], 1000);
    /// println!("Command length: {}", stats.formatted_length);
    /// ```
    pub fn get_transmission_stats(&self, command: &[u8], value: u16) -> CommandTransmissionStats {
        CommandTransmission::get_transmission_stats(command, value)
    }
    
    /// Test connection responsiveness
    /// 
    /// Uses the connection management module to test connection
    /// responsiveness and readiness.
    /// 
    /// # Returns
    /// * `Result<connection::ConnectionTestResult>` - Test results or error
    /// 
    /// # Example
    /// ```
    /// let test_result = handler.test_connection()?;
    /// if test_result.is_responsive {
    ///     println!("Connection is responsive");
    /// }
    /// ```
    pub fn test_connection(&mut self) -> Result<connection::ConnectionTestResult> {
        ConnectionManager::test_connection(&mut self.port)
    }
}
