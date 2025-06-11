//! Automated COM port detection for Lumidox II Controller
//!
//! This module provides automated detection of Lumidox II Controller devices
//! by scanning available serial ports and testing for device compatibility.
//! It filters ports based on device characteristics and validates connectivity.
//!
//! The port detection system provides:
//! - Automatic scanning of available serial ports
//! - Filtering for USB Serial Port devices (FTDI-based)
//! - Device identification through protocol commands
//! - Ranking of candidate ports by compatibility score

use crate::core::{LumidoxError, Result};
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use std::time::Duration;

/// Port detection configuration and settings
#[derive(Debug, Clone)]
pub struct PortDetectionConfig {
    /// Timeout for device identification attempts
    pub identification_timeout: Duration,
    /// Whether to include all port types or only USB ports
    pub usb_ports_only: bool,
    /// Whether to perform device identification tests
    pub test_device_identification: bool,
    /// Vendor IDs to prioritize (FTDI: 0x0403)
    pub preferred_vendor_ids: Vec<u16>,
    /// Product IDs to prioritize
    pub preferred_product_ids: Vec<u16>,
}

impl Default for PortDetectionConfig {
    fn default() -> Self {
        Self {
            identification_timeout: Duration::from_millis(2000),
            usb_ports_only: true,
            test_device_identification: true,
            preferred_vendor_ids: vec![0x0403], // FTDI
            preferred_product_ids: vec![],
        }
    }
}

/// Information about a detected port candidate
#[derive(Debug, Clone)]
pub struct PortCandidate {
    /// Serial port information
    pub port_info: SerialPortInfo,
    /// Compatibility score (0-100, higher is better)
    pub compatibility_score: u8,
    /// Whether device identification was successful
    pub device_identified: bool,
    /// Device identification details if available
    pub device_details: Option<DeviceIdentification>,
    /// Reason for compatibility score
    pub score_reason: String,
}

/// Device identification information
#[derive(Debug, Clone)]
pub struct DeviceIdentification {
    /// Firmware version if detected
    pub firmware_version: Option<String>,
    /// Model number if detected
    pub model_number: Option<String>,
    /// Serial number if detected
    pub serial_number: Option<String>,
    /// Whether the device responds to Lumidox II protocol
    pub protocol_compatible: bool,
}

/// Port detection utilities and functionality
pub struct PortDetector;

impl PortDetector {
    /// Detect Lumidox II Controller ports automatically
    /// 
    /// Scans all available serial ports and returns a list of candidates
    /// ranked by compatibility score. Higher scores indicate better matches.
    /// 
    /// # Arguments
    /// * `config` - Detection configuration settings
    /// 
    /// # Returns
    /// * `Result<Vec<PortCandidate>>` - List of port candidates sorted by score
    /// 
    /// # Example
    /// ```
    /// let config = PortDetectionConfig::default();
    /// let candidates = PortDetector::detect_ports(&config)?;
    /// if let Some(best) = candidates.first() {
    ///     println!("Best candidate: {}", best.port_info.port_name);
    /// }
    /// ```
    pub fn detect_ports(config: &PortDetectionConfig) -> Result<Vec<PortCandidate>> {
        let available_ports = serialport::available_ports()
            .map_err(|e| LumidoxError::SerialError(e))?;
        
        let mut candidates = Vec::new();
        
        for port_info in available_ports {
            // Apply initial filtering
            if config.usb_ports_only && !Self::is_usb_port(&port_info) {
                continue;
            }
            
            // Calculate compatibility score
            let compatibility_score = Self::calculate_compatibility_score(&port_info, config);
            
            // Test device identification if enabled
            let (device_identified, device_details) = if config.test_device_identification {
                Self::test_device_identification(&port_info, config)
            } else {
                (false, None)
            };
            
            let score_reason = Self::generate_score_reason(&port_info, compatibility_score, device_identified);
            
            candidates.push(PortCandidate {
                port_info,
                compatibility_score,
                device_identified,
                device_details,
                score_reason,
            });
        }
        
        // Sort by compatibility score (highest first)
        candidates.sort_by(|a, b| b.compatibility_score.cmp(&a.compatibility_score));
        
        Ok(candidates)
    }
    
    /// Get the best port candidate automatically
    /// 
    /// Returns the highest-scoring port candidate, or None if no suitable
    /// candidates are found.
    /// 
    /// # Arguments
    /// * `config` - Detection configuration settings
    /// 
    /// # Returns
    /// * `Result<Option<PortCandidate>>` - Best candidate or None
    /// 
    /// # Example
    /// ```
    /// let config = PortDetectionConfig::default();
    /// if let Some(best) = PortDetector::get_best_port(&config)? {
    ///     println!("Auto-detected port: {}", best.port_info.port_name);
    /// }
    /// ```
    pub fn get_best_port(config: &PortDetectionConfig) -> Result<Option<PortCandidate>> {
        let candidates = Self::detect_ports(config)?;
        Ok(candidates.into_iter().next())
    }
    
    /// Check if a port is a USB serial port
    /// 
    /// Determines whether the given port is a USB-based serial port,
    /// which is the expected connection type for Lumidox II devices.
    /// 
    /// # Arguments
    /// * `port_info` - Serial port information to check
    /// 
    /// # Returns
    /// * `bool` - True if the port is USB-based
    fn is_usb_port(port_info: &SerialPortInfo) -> bool {
        matches!(port_info.port_type, SerialPortType::UsbPort(_))
    }
    
    /// Calculate compatibility score for a port
    /// 
    /// Assigns a compatibility score (0-100) based on port characteristics
    /// such as port type, vendor ID, product ID, and description.
    /// 
    /// # Arguments
    /// * `port_info` - Serial port information
    /// * `config` - Detection configuration
    /// 
    /// # Returns
    /// * `u8` - Compatibility score (0-100)
    fn calculate_compatibility_score(port_info: &SerialPortInfo, config: &PortDetectionConfig) -> u8 {
        let mut score = 0u8;
        
        match &port_info.port_type {
            SerialPortType::UsbPort(usb_info) => {
                score += 40; // Base score for USB port
                
                // Check vendor ID
                if config.preferred_vendor_ids.contains(&usb_info.vid) {
                    score += 30; // FTDI or other preferred vendor
                }
                
                // Check product ID if specified
                if !config.preferred_product_ids.is_empty() && 
                   config.preferred_product_ids.contains(&usb_info.pid) {
                    score += 20;
                }
                
                // Check product description
                if let Some(product) = &usb_info.product {
                    if product.to_lowercase().contains("serial") {
                        score += 10;
                    }
                    if product.to_lowercase().contains("ftdi") {
                        score += 10;
                    }
                }
                
                // Check manufacturer
                if let Some(manufacturer) = &usb_info.manufacturer {
                    if manufacturer.to_lowercase().contains("ftdi") {
                        score += 10;
                    }
                }
            }
            _ => {
                score += 10; // Lower score for non-USB ports
            }
        }
        
        score.min(100)
    }
    
    /// Test device identification on a port
    /// 
    /// Attempts to communicate with a device on the given port to determine
    /// if it's a Lumidox II Controller by sending identification commands.
    /// 
    /// # Arguments
    /// * `port_info` - Serial port to test
    /// * `config` - Detection configuration
    /// 
    /// # Returns
    /// * `(bool, Option<DeviceIdentification>)` - Success flag and device details
    fn test_device_identification(
        port_info: &SerialPortInfo, 
        config: &PortDetectionConfig
    ) -> (bool, Option<DeviceIdentification>) {
        // Try to open the port with default baud rate
        let port_result = serialport::new(&port_info.port_name, crate::communication::protocol::constants::DEFAULT_BAUD_RATE)
            .timeout(config.identification_timeout)
            .open();
        
        let mut port = match port_result {
            Ok(p) => p,
            Err(_) => return (false, None),
        };
        
        // Try to create a protocol handler and test basic communication
        match crate::communication::ProtocolHandler::new(port) {
            Ok(mut protocol) => {
                // Test if we can read device info
                let device_info_test = crate::device::info::read_device_info(&mut protocol);

                if device_info_test.is_ok() {
                    // Device responds to protocol, extract details
                    let device_info = device_info_test.ok();
                    let firmware_version = device_info.as_ref().map(|info| info.firmware_version.clone());
                    let model_number = device_info.as_ref().map(|info| info.model_number.clone());
                    let serial_number = device_info.as_ref().map(|info| info.serial_number.clone());
                    
                    let device_details = DeviceIdentification {
                        firmware_version,
                        model_number,
                        serial_number,
                        protocol_compatible: true,
                    };
                    
                    (true, Some(device_details))
                } else {
                    (false, None)
                }
            }
            Err(_) => (false, None),
        }
    }
    
    /// Generate human-readable reason for compatibility score
    /// 
    /// Creates a descriptive explanation of why a port received its
    /// compatibility score for user feedback and debugging.
    /// 
    /// # Arguments
    /// * `port_info` - Serial port information
    /// * `score` - Calculated compatibility score
    /// * `device_identified` - Whether device identification succeeded
    /// 
    /// # Returns
    /// * `String` - Human-readable score explanation
    fn generate_score_reason(
        port_info: &SerialPortInfo, 
        score: u8, 
        device_identified: bool
    ) -> String {
        let mut reasons = Vec::new();
        
        match &port_info.port_type {
            SerialPortType::UsbPort(usb_info) => {
                reasons.push("USB Serial Port".to_string());
                
                if usb_info.vid == 0x0403 {
                    reasons.push("FTDI device".to_string());
                }
                
                if let Some(product) = &usb_info.product {
                    if product.to_lowercase().contains("serial") {
                        reasons.push("Serial device".to_string());
                    }
                }
            }
            _ => {
                reasons.push("Non-USB port".to_string());
            }
        }
        
        if device_identified {
            reasons.push("Device responds to Lumidox II protocol".to_string());
        }
        
        format!("Score {}: {}", score, reasons.join(", "))
    }
    
    /// Get detailed port information for debugging
    /// 
    /// Returns comprehensive information about all available ports
    /// for troubleshooting and debugging purposes.
    /// 
    /// # Returns
    /// * `Result<Vec<String>>` - List of detailed port descriptions
    /// 
    /// # Example
    /// ```
    /// let details = PortDetector::get_detailed_port_info()?;
    /// for detail in details {
    ///     println!("{}", detail);
    /// }
    /// ```
    pub fn get_detailed_port_info() -> Result<Vec<String>> {
        let ports = serialport::available_ports()
            .map_err(|e| LumidoxError::SerialError(e))?;
        
        let mut details = Vec::new();
        
        for port in ports {
            let detail = match &port.port_type {
                SerialPortType::UsbPort(usb_info) => {
                    format!(
                        "{}: USB Serial Port (VID: 0x{:04x}, PID: 0x{:04x}) - {} by {}",
                        port.port_name,
                        usb_info.vid,
                        usb_info.pid,
                        usb_info.product.as_ref().unwrap_or(&"Unknown".to_string()),
                        usb_info.manufacturer.as_ref().unwrap_or(&"Unknown".to_string())
                    )
                }
                SerialPortType::PciPort => {
                    format!("{}: PCI Serial Port", port.port_name)
                }
                SerialPortType::BluetoothPort => {
                    format!("{}: Bluetooth Serial Port", port.port_name)
                }
                SerialPortType::Unknown => {
                    format!("{}: Unknown Port Type", port.port_name)
                }
            };
            details.push(detail);
        }
        
        Ok(details)
    }
}
