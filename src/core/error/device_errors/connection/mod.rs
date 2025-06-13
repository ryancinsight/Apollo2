//! Device connection error handling utilities
//!
//! This module provides specialized error handling for device connection-related
//! errors in the Lumidox II Controller. It handles various connection scenarios
//! including timeouts, initialization failures, and network issues.
//!
//! ## Module Structure (8+ levels deep)
//! ```
//! src/core/error/device_errors/connection/  (Level 5)
//! ├── timeout/                              (Level 6)
//! │   ├── network/                          (Level 7)
//! │   │   └── mod.rs                        (Level 8) - Network timeout errors
//! │   ├── serial/                           (Level 7)
//! │   │   └── mod.rs                        (Level 8) - Serial timeout errors
//! │   └── mod.rs                            (Level 7) - Timeout coordination
//! ├── initialization/                       (Level 6)
//! │   ├── port_setup/                       (Level 7)
//! │   │   └── mod.rs                        (Level 8) - Port setup errors
//! │   └── mod.rs                            (Level 7) - Init error coordination
//! └── mod.rs                                (Level 6) - Connection coordination
//! ```

// Import specialized sub-modules
pub mod timeout;
pub mod initialization;

// Re-export commonly used items for convenience
// Note: Utilities are available but not currently used in the codebase
// pub use timeout::{TimeoutErrorUtils, TimeoutErrorCategory};
// pub use initialization::{InitializationErrorUtils, InitializationErrorCategory};

use crate::core::error::types::LumidoxError;

/// Connection error categories for better error classification
/// 
/// This enum helps categorize different types of connection errors
/// for more specific error handling and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionErrorCategory {
    /// Serial port connection errors
    SerialPort,
    /// Network connection errors
    Network,
    /// Device initialization errors
    Initialization,
    /// Connection timeout errors
    Timeout,
    /// Port configuration errors
    Configuration,
}

/// Connection error utilities and helper functions
pub struct ConnectionErrorUtils;

impl ConnectionErrorUtils {
    /// Create a general device connection error
    /// 
    /// Used when the device cannot be connected to or initialized.
    /// 
    /// # Arguments
    /// * `details` - Specific details about the connection failure
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted device connection error
    /// 
    /// # Example
    /// ```
    /// let error = ConnectionErrorUtils::connection_error("Failed to open serial port COM3");
    /// ```
    pub fn connection_error(details: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!("Device connection failed: {}", details))
    }
    
    /// Create a serial port connection error
    /// 
    /// Used when serial port connection fails.
    /// 
    /// # Arguments
    /// * `port_name` - The name of the port that failed to connect
    /// * `reason` - The reason for the connection failure
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted serial port connection error
    pub fn serial_port_error(port_name: &str, reason: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Serial port connection failed on {}: {}", 
            port_name, reason
        ))
    }
    
    /// Create a network connection error
    /// 
    /// Used when network-based device connection fails.
    /// 
    /// # Arguments
    /// * `address` - The network address that failed to connect
    /// * `reason` - The reason for the connection failure
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted network connection error
    pub fn network_error(address: &str, reason: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Network connection failed to {}: {}", 
            address, reason
        ))
    }
    
    /// Create a port configuration error
    /// 
    /// Used when port configuration fails.
    /// 
    /// # Arguments
    /// * `port_name` - The name of the port with configuration issues
    /// * `config_issue` - Description of the configuration problem
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted port configuration error
    pub fn configuration_error(port_name: &str, config_issue: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Port configuration failed on {}: {}", 
            port_name, config_issue
        ))
    }
    
    /// Categorize a connection error for better handling
    /// 
    /// Analyzes a connection error message to determine its category.
    /// 
    /// # Arguments
    /// * `error_message` - The connection error message to categorize
    /// 
    /// # Returns
    /// * `ConnectionErrorCategory` - The determined error category
    pub fn categorize_error(error_message: &str) -> ConnectionErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("serial") || message_lower.contains("com") || message_lower.contains("tty") {
            ConnectionErrorCategory::SerialPort
        } else if message_lower.contains("network") || message_lower.contains("tcp") || message_lower.contains("ip") {
            ConnectionErrorCategory::Network
        } else if message_lower.contains("timeout") || message_lower.contains("timed out") {
            ConnectionErrorCategory::Timeout
        } else if message_lower.contains("init") || message_lower.contains("setup") {
            ConnectionErrorCategory::Initialization
        } else {
            ConnectionErrorCategory::Configuration
        }
    }
    
    /// Check if an error is recoverable
    /// 
    /// Determines if a connection error can potentially be recovered from
    /// by retrying the connection or adjusting parameters.
    /// 
    /// # Arguments
    /// * `category` - The connection error category
    /// 
    /// # Returns
    /// * `bool` - True if the error is potentially recoverable
    pub fn is_recoverable(category: &ConnectionErrorCategory) -> bool {
        match category {
            ConnectionErrorCategory::Timeout => true,
            ConnectionErrorCategory::Network => true,
            ConnectionErrorCategory::Configuration => false,
            ConnectionErrorCategory::SerialPort => true,
            ConnectionErrorCategory::Initialization => true,
        }
    }
    
    /// Get suggested recovery action for a connection error
    /// 
    /// Provides a human-readable suggestion for recovering from the error.
    /// 
    /// # Arguments
    /// * `category` - The connection error category
    /// 
    /// # Returns
    /// * `&'static str` - Suggested recovery action
    pub fn get_recovery_suggestion(category: &ConnectionErrorCategory) -> &'static str {
        match category {
            ConnectionErrorCategory::Timeout => "Try increasing timeout duration or check device responsiveness",
            ConnectionErrorCategory::Network => "Check network connectivity and device IP address",
            ConnectionErrorCategory::Configuration => "Verify port configuration settings",
            ConnectionErrorCategory::SerialPort => "Check serial port availability and permissions",
            ConnectionErrorCategory::Initialization => "Ensure device is powered on and ready",
        }
    }
}
