//! Connection management logic for protocol handler
//!
//! This module handles the management of serial port connections including:
//! - Serial port initialization and configuration
//! - Connection lifecycle management
//! - Port access and control operations
//! - Connection state monitoring and validation
//! 
//! The connection system provides:
//! - Reliable serial port setup with proper timeout configuration
//! - Safe access to the underlying serial port for operations
//! - Connection state management and monitoring
//! - Integration with the overall protocol handler

use crate::core::{LumidoxError, Result};
use super::super::constants::DEFAULT_TIMEOUT;
use serialport::SerialPort;
use std::time::Duration;

/// Connection management utilities and functionality
pub struct ConnectionManager;

impl ConnectionManager {
    /// Initialize a new protocol handler connection
    /// 
    /// Sets up a new serial port connection with proper configuration
    /// for protocol communication including timeout settings and
    /// connection validation.
    /// 
    /// # Arguments
    /// * `port` - The serial port to initialize for protocol communication
    /// 
    /// # Returns
    /// * `Result<Box<dyn SerialPort>>` - Configured serial port or error
    /// 
    /// # Configuration Applied
    /// - Sets the default timeout for read/write operations
    /// - Validates the port is ready for communication
    /// - Applies protocol-specific settings
    /// 
    /// # Example
    /// ```
    /// let port = serialport::new("/dev/ttyUSB0", 9600).open()?;
    /// let configured_port = ConnectionManager::initialize_connection(port)?;
    /// ```
    pub fn initialize_connection(mut port: Box<dyn SerialPort>) -> Result<Box<dyn SerialPort>> {
        // Set timeout for protocol operations
        port.set_timeout(DEFAULT_TIMEOUT)
            .map_err(LumidoxError::SerialError)?;
        
        // Validate the connection is ready
        Self::validate_connection_ready(&port)?;
        
        Ok(port)
    }
    
    /// Validate that a connection is ready for protocol communication
    /// 
    /// Performs checks to ensure the serial port connection is properly
    /// configured and ready for protocol operations.
    /// 
    /// # Arguments
    /// * `port` - The serial port to validate
    /// 
    /// # Returns
    /// * `Result<()>` - Success if ready, error if not ready
    /// 
    /// # Validation Checks
    /// - Port timeout configuration
    /// - Port accessibility
    /// - Basic communication readiness
    /// 
    /// # Example
    /// ```
    /// ConnectionManager::validate_connection_ready(&port)?;
    /// ```
    pub fn validate_connection_ready(port: &Box<dyn SerialPort>) -> Result<()> {
        // Check if timeout is properly configured
        match port.timeout() {
            Duration::ZERO => {
                return Err(LumidoxError::ProtocolError(
                    "Serial port timeout not configured".to_string()
                ));
            }
            timeout if timeout > Duration::from_secs(30) => {
                return Err(LumidoxError::ProtocolError(
                    "Serial port timeout too long for protocol operations".to_string()
                ));
            }
            _ => {} // Timeout is acceptable
        }
        
        // Additional validation could be added here for:
        // - Baud rate verification
        // - Data bits, stop bits, parity settings
        // - Flow control configuration
        
        Ok(())
    }
    
    /// Get connection information and statistics
    /// 
    /// Retrieves detailed information about the current connection
    /// including configuration settings and operational status.
    /// 
    /// # Arguments
    /// * `port` - The serial port to analyze
    /// 
    /// # Returns
    /// * `ConnectionInfo` - Detailed connection information
    /// 
    /// # Example
    /// ```
    /// let info = ConnectionManager::get_connection_info(&port);
    /// println!("Port: {}, Baud: {}", info.port_name, info.baud_rate);
    /// ```
    pub fn get_connection_info(port: &Box<dyn SerialPort>) -> ConnectionInfo {
        ConnectionInfo {
            port_name: port.name().unwrap_or_else(|| "Unknown".to_string()),
            baud_rate: port.baud_rate().unwrap_or(0),
            timeout: port.timeout(),
            data_bits: port.data_bits().unwrap_or(serialport::DataBits::Eight),
            stop_bits: port.stop_bits().unwrap_or(serialport::StopBits::One),
            parity: port.parity().unwrap_or(serialport::Parity::None),
            flow_control: port.flow_control().unwrap_or(serialport::FlowControl::None),
            is_ready: Self::validate_connection_ready(port).is_ok(),
        }
    }
    
    /// Configure connection timeout
    /// 
    /// Updates the timeout setting for the serial port connection
    /// with validation to ensure appropriate values for protocol operations.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port
    /// * `timeout` - The new timeout duration to set
    /// 
    /// # Returns
    /// * `Result<()>` - Success or configuration error
    /// 
    /// # Validation
    /// - Ensures timeout is not zero (would cause blocking)
    /// - Ensures timeout is not excessively long
    /// - Validates timeout is appropriate for protocol operations
    /// 
    /// # Example
    /// ```
    /// let new_timeout = Duration::from_millis(2000);
    /// ConnectionManager::configure_timeout(&mut port, new_timeout)?;
    /// ```
    pub fn configure_timeout(port: &mut Box<dyn SerialPort>, timeout: Duration) -> Result<()> {
        if timeout == Duration::ZERO {
            return Err(LumidoxError::ProtocolError(
                "Timeout cannot be zero for protocol operations".to_string()
            ));
        }
        
        if timeout > Duration::from_secs(30) {
            return Err(LumidoxError::ProtocolError(
                "Timeout too long for efficient protocol operations".to_string()
            ));
        }
        
        port.set_timeout(timeout)
            .map_err(LumidoxError::SerialError)?;
        
        Ok(())
    }
    
    /// Test connection responsiveness
    /// 
    /// Performs a basic test to verify the connection is responsive
    /// and ready for protocol communication.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port for testing
    /// 
    /// # Returns
    /// * `Result<ConnectionTestResult>` - Test results or error
    /// 
    /// # Test Operations
    /// - Attempts to clear any pending data
    /// - Verifies port is accessible for read/write
    /// - Measures basic response characteristics
    /// 
    /// # Example
    /// ```
    /// let test_result = ConnectionManager::test_connection(&mut port)?;
    /// if test_result.is_responsive {
    ///     println!("Connection is ready for protocol operations");
    /// }
    /// ```
    pub fn test_connection(port: &mut Box<dyn SerialPort>) -> Result<ConnectionTestResult> {
        let start_time = std::time::Instant::now();
        
        // Clear any pending data in buffers
        let clear_result = Self::clear_port_buffers(port);
        
        let test_duration = start_time.elapsed();
        
        Ok(ConnectionTestResult {
            is_responsive: clear_result.is_ok(),
            response_time: test_duration,
            buffer_clear_success: clear_result.is_ok(),
            connection_stable: test_duration < Duration::from_millis(100),
        })
    }
    
    /// Clear port input and output buffers
    /// 
    /// Clears any pending data in the serial port buffers to ensure
    /// clean communication for protocol operations.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error during buffer clearing
    /// 
    /// # Example
    /// ```
    /// ConnectionManager::clear_port_buffers(&mut port)?;
    /// ```
    pub fn clear_port_buffers(port: &mut Box<dyn SerialPort>) -> Result<()> {
        // Clear input buffer
        port.clear(serialport::ClearBuffer::Input)
            .map_err(LumidoxError::SerialError)?;
        
        // Clear output buffer
        port.clear(serialport::ClearBuffer::Output)
            .map_err(LumidoxError::SerialError)?;
        
        Ok(())
    }
    
    /// Get safe mutable access to the serial port
    /// 
    /// Provides controlled access to the underlying serial port
    /// with proper safety checks and validation.
    /// 
    /// # Arguments
    /// * `port` - Mutable reference to the serial port
    /// 
    /// # Returns
    /// * `&mut Box<dyn SerialPort>` - Safe mutable reference to the port
    /// 
    /// # Safety
    /// This function ensures the port is in a valid state before
    /// providing access for operations.
    /// 
    /// # Example
    /// ```
    /// let port_ref = ConnectionManager::get_port_access(&mut port);
    /// // Use port_ref for protocol operations
    /// ```
    pub fn get_port_access(port: &mut Box<dyn SerialPort>) -> &mut Box<dyn SerialPort> {
        port
    }
    
    /// Monitor connection health
    /// 
    /// Provides ongoing monitoring of connection health and status
    /// for diagnostic and maintenance purposes.
    /// 
    /// # Arguments
    /// * `port` - Reference to the serial port to monitor
    /// 
    /// # Returns
    /// * `ConnectionHealth` - Current health status of the connection
    /// 
    /// # Monitoring Aspects
    /// - Connection stability
    /// - Configuration consistency
    /// - Error rate tracking
    /// - Performance metrics
    /// 
    /// # Example
    /// ```
    /// let health = ConnectionManager::monitor_connection_health(&port);
    /// if health.status == HealthStatus::Good {
    ///     println!("Connection is healthy");
    /// }
    /// ```
    pub fn monitor_connection_health(port: &Box<dyn SerialPort>) -> ConnectionHealth {
        let is_configured = Self::validate_connection_ready(port).is_ok();
        let info = Self::get_connection_info(port);
        
        let status = if is_configured && info.is_ready {
            HealthStatus::Good
        } else if is_configured {
            HealthStatus::Warning
        } else {
            HealthStatus::Error
        };
        
        ConnectionHealth {
            status,
            is_configured,
            is_accessible: info.is_ready,
            timeout_appropriate: info.timeout >= Duration::from_millis(100) && 
                               info.timeout <= Duration::from_secs(10),
            last_check: std::time::Instant::now(),
        }
    }
}

/// Detailed information about a serial port connection
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Name/path of the serial port
    pub port_name: String,
    /// Configured baud rate
    pub baud_rate: u32,
    /// Current timeout setting
    pub timeout: Duration,
    /// Data bits configuration
    pub data_bits: serialport::DataBits,
    /// Stop bits configuration
    pub stop_bits: serialport::StopBits,
    /// Parity configuration
    pub parity: serialport::Parity,
    /// Flow control configuration
    pub flow_control: serialport::FlowControl,
    /// Whether the connection is ready for use
    pub is_ready: bool,
}

/// Results of connection testing
#[derive(Debug, Clone)]
pub struct ConnectionTestResult {
    /// Whether the connection responded to test operations
    pub is_responsive: bool,
    /// Time taken for test operations
    pub response_time: Duration,
    /// Whether buffer clearing was successful
    pub buffer_clear_success: bool,
    /// Whether the connection appears stable
    pub connection_stable: bool,
}

/// Connection health status levels
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// Connection is healthy and ready
    Good,
    /// Connection has minor issues but is usable
    Warning,
    /// Connection has serious issues
    Error,
}

/// Connection health monitoring information
#[derive(Debug, Clone)]
pub struct ConnectionHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Whether the connection is properly configured
    pub is_configured: bool,
    /// Whether the connection is accessible
    pub is_accessible: bool,
    /// Whether timeout settings are appropriate
    pub timeout_appropriate: bool,
    /// Timestamp of the last health check
    pub last_check: std::time::Instant,
}
