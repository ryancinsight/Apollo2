//! Automated connection management for Lumidox II Controller
//!
//! This module provides unified automated connection management that combines
//! port detection and baud rate detection to automatically establish
//! communication with Lumidox II Controller devices.
//!
//! The auto-connection system provides:
//! - Fully automated device detection and connection
//! - Fallback to manual selection when auto-detection fails
//! - Connection caching for faster reconnection
//! - Comprehensive error reporting and user guidance

use crate::core::{LumidoxError, Result};
use crate::communication::{ProtocolHandler, port_detection::*, baud_detection::*};
use crate::device::LumidoxDevice;
use std::time::Duration;

/// Auto-connection configuration and settings
#[derive(Debug, Clone)]
pub struct AutoConnectConfig {
    /// Port detection configuration
    pub port_config: PortDetectionConfig,
    /// Baud rate detection configuration
    pub baud_config: BaudDetectionConfig,
    /// Whether to enable verbose output during detection
    pub verbose: bool,
    /// Whether to cache successful connections
    pub enable_caching: bool,
    /// Maximum time to spend on auto-detection
    pub max_detection_time: Duration,
}

impl Default for AutoConnectConfig {
    fn default() -> Self {
        Self {
            port_config: PortDetectionConfig::default(),
            baud_config: BaudDetectionConfig::default(),
            verbose: false,
            enable_caching: true,
            max_detection_time: Duration::from_secs(30),
        }
    }
}

/// Result of auto-connection attempt
#[derive(Debug, Clone)]
pub struct AutoConnectResult {
    /// Whether connection was successful
    pub success: bool,
    /// Port name that was connected to
    pub port_name: Option<String>,
    /// Baud rate that was used
    pub baud_rate: Option<u32>,
    /// Method used for connection (auto-detected, cached, manual)
    pub connection_method: ConnectionMethod,
    /// Time taken for connection process
    pub connection_time: Duration,
    /// Detailed log of connection attempts
    pub connection_log: Vec<String>,
    /// Device information if connection was successful
    pub device_info: Option<crate::device::models::DeviceInfo>,
}

/// Method used to establish connection
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionMethod {
    /// Fully automated detection
    AutoDetected,
    /// Used cached connection parameters
    Cached,
    /// Manual port/baud rate specification
    Manual,
    /// Fallback after auto-detection failed
    Fallback,
}

/// Cached connection information
#[derive(Debug, Clone)]
pub struct ConnectionCache {
    /// Port name
    pub port_name: String,
    /// Baud rate
    pub baud_rate: u32,
    /// Timestamp of last successful connection
    pub last_used: std::time::SystemTime,
    /// Device serial number for validation
    pub device_serial: Option<String>,
}

/// Auto-connection utilities and functionality
pub struct AutoConnector;

impl AutoConnector {
    /// Automatically connect to a Lumidox II Controller
    /// 
    /// Performs fully automated detection and connection to a Lumidox II
    /// device by scanning ports and testing baud rates.
    /// 
    /// # Arguments
    /// * `config` - Auto-connection configuration
    /// 
    /// # Returns
    /// * `Result<(LumidoxDevice, AutoConnectResult)>` - Connected device and connection details
    /// 
    /// # Example
    /// ```
    /// let config = AutoConnectConfig::default();
    /// let (device, result) = AutoConnector::auto_connect(&config)?;
    /// println!("Connected to {} at {} baud", 
    ///     result.port_name.unwrap(), result.baud_rate.unwrap());
    /// ```
    pub fn auto_connect(config: &AutoConnectConfig) -> Result<(LumidoxDevice, AutoConnectResult)> {
        let start_time = std::time::Instant::now();
        let mut connection_log = Vec::new();
        
        if config.verbose {
            println!("Starting automated Lumidox II Controller detection...");
        }
        
        connection_log.push("Starting auto-connection process".to_string());
        
        // Step 1: Try cached connection if enabled
        if config.enable_caching {
            if let Ok(Some((device, cache))) = Self::try_cached_connection(config) {
                let connection_time = start_time.elapsed();
                connection_log.push(format!("Used cached connection: {} at {} baud", cache.port_name, cache.baud_rate));
                
                let device_info = device.info().cloned();
                let result = AutoConnectResult {
                    success: true,
                    port_name: Some(cache.port_name),
                    baud_rate: Some(cache.baud_rate),
                    connection_method: ConnectionMethod::Cached,
                    connection_time,
                    connection_log,
                    device_info,
                };
                
                if config.verbose {
                    println!("Connected using cached settings: {} at {} baud", 
                        result.port_name.as_ref().unwrap(), result.baud_rate.unwrap());
                }
                
                return Ok((device, result));
            }
        }
        
        // Step 2: Auto-detect ports
        connection_log.push("Scanning for compatible ports".to_string());
        if config.verbose {
            println!("Scanning for compatible serial ports...");
        }
        
        let port_candidates = PortDetector::detect_ports(&config.port_config)?;
        connection_log.push(format!("Found {} port candidates", port_candidates.len()));
        
        if port_candidates.is_empty() {
            let connection_time = start_time.elapsed();
            connection_log.push("No compatible ports found".to_string());
            
            let _result = AutoConnectResult {
                success: false,
                port_name: None,
                baud_rate: None,
                connection_method: ConnectionMethod::AutoDetected,
                connection_time,
                connection_log,
                device_info: None,
            };
            
            return Err(LumidoxError::DeviceError("No compatible serial ports found".to_string()));
        }
        
        // Step 3: Test each port candidate
        for (index, candidate) in port_candidates.iter().enumerate() {
            if start_time.elapsed() > config.max_detection_time {
                connection_log.push("Detection timeout reached".to_string());
                break;
            }
            
            if config.verbose {
                println!("Testing port {} ({}/{}): {} (score: {})", 
                    candidate.port_info.port_name, 
                    index + 1, 
                    port_candidates.len(),
                    candidate.score_reason,
                    candidate.compatibility_score);
            }
            
            connection_log.push(format!("Testing port {}: {}", candidate.port_info.port_name, candidate.score_reason));
            
            // If device was already identified during port detection, try default baud rate first
            if candidate.device_identified {
                if let Ok(device) = Self::try_connect_with_baud(&candidate.port_info.port_name, BaudDetector::get_recommended_baud_rate()) {
                    let connection_time = start_time.elapsed();
                    connection_log.push(format!("Connected successfully: {} at {} baud", 
                        candidate.port_info.port_name, BaudDetector::get_recommended_baud_rate()));
                    
                    let device_info = device.info().cloned();
                    
                    // Cache this successful connection
                    if config.enable_caching {
                        Self::cache_connection(&candidate.port_info.port_name, BaudDetector::get_recommended_baud_rate(), &device);
                    }
                    
                    let result = AutoConnectResult {
                        success: true,
                        port_name: Some(candidate.port_info.port_name.clone()),
                        baud_rate: Some(BaudDetector::get_recommended_baud_rate()),
                        connection_method: ConnectionMethod::AutoDetected,
                        connection_time,
                        connection_log,
                        device_info,
                    };
                    
                    if config.verbose {
                        println!("Successfully connected to {} at {} baud", 
                            result.port_name.as_ref().unwrap(), result.baud_rate.unwrap());
                    }
                    
                    return Ok((device, result));
                }
            }
            
            // Try baud rate detection
            connection_log.push(format!("Testing baud rates for {}", candidate.port_info.port_name));
            if config.verbose {
                println!("  Testing baud rates...");
            }
            
            if let Ok(Some(baud_rate)) = BaudDetector::detect_baud_rate(&candidate.port_info.port_name, &config.baud_config) {
                if let Ok(device) = Self::try_connect_with_baud(&candidate.port_info.port_name, baud_rate) {
                    let connection_time = start_time.elapsed();
                    connection_log.push(format!("Connected successfully: {} at {} baud", 
                        candidate.port_info.port_name, baud_rate));
                    
                    let device_info = device.info().cloned();
                    
                    // Cache this successful connection
                    if config.enable_caching {
                        Self::cache_connection(&candidate.port_info.port_name, baud_rate, &device);
                    }
                    
                    let result = AutoConnectResult {
                        success: true,
                        port_name: Some(candidate.port_info.port_name.clone()),
                        baud_rate: Some(baud_rate),
                        connection_method: ConnectionMethod::AutoDetected,
                        connection_time,
                        connection_log,
                        device_info,
                    };
                    
                    if config.verbose {
                        println!("Successfully connected to {} at {} baud", 
                            result.port_name.as_ref().unwrap(), result.baud_rate.unwrap());
                    }
                    
                    return Ok((device, result));
                }
            }
            
            connection_log.push(format!("No working baud rate found for {}", candidate.port_info.port_name));
        }
        
        // Step 4: Auto-detection failed
        let connection_time = start_time.elapsed();
        connection_log.push("Auto-detection failed for all candidates".to_string());
        
        let _result = AutoConnectResult {
            success: false,
            port_name: None,
            baud_rate: None,
            connection_method: ConnectionMethod::AutoDetected,
            connection_time,
            connection_log,
            device_info: None,
        };
        
        Err(LumidoxError::DeviceError("Auto-detection failed to find a working Lumidox II Controller".to_string()))
    }
    
    /// Try to connect using cached connection parameters
    /// 
    /// Attempts to use previously successful connection parameters
    /// to quickly reconnect to the same device.
    /// 
    /// # Arguments
    /// * `config` - Auto-connection configuration
    /// 
    /// # Returns
    /// * `Result<Option<(LumidoxDevice, ConnectionCache)>>` - Device and cache info if successful
    fn try_cached_connection(_config: &AutoConnectConfig) -> Result<Option<(LumidoxDevice, ConnectionCache)>> {
        // For now, return None (no cache implementation)
        // In a full implementation, this would check a cache file or registry
        Ok(None)
    }
    
    /// Try to connect to a specific port with a specific baud rate
    /// 
    /// Attempts to establish a connection to the given port using the
    /// specified baud rate and validates the connection.
    /// 
    /// # Arguments
    /// * `port_name` - Name of the serial port
    /// * `baud_rate` - Baud rate to use
    /// 
    /// # Returns
    /// * `Result<LumidoxDevice>` - Connected device if successful
    fn try_connect_with_baud(port_name: &str, baud_rate: u32) -> Result<LumidoxDevice> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()
            .map_err(LumidoxError::SerialError)?;
        
        let protocol = ProtocolHandler::new(port)?;
        let mut device = LumidoxDevice::new(protocol);
        device.initialize()?;
        
        Ok(device)
    }
    
    /// Cache a successful connection for future use
    /// 
    /// Stores connection parameters for a successful connection to enable
    /// faster reconnection in the future.
    /// 
    /// # Arguments
    /// * `port_name` - Name of the serial port
    /// * `baud_rate` - Baud rate used
    /// * `device` - Connected device for additional info
    fn cache_connection(port_name: &str, baud_rate: u32, device: &LumidoxDevice) {
        // For now, this is a no-op
        // In a full implementation, this would save to a cache file or registry
        let _cache = ConnectionCache {
            port_name: port_name.to_string(),
            baud_rate,
            last_used: std::time::SystemTime::now(),
            device_serial: device.info().map(|info| info.serial_number.clone()),
        };
    }
    
    /// Create a quick auto-connect configuration
    /// 
    /// Returns a configuration optimized for speed, suitable for
    /// interactive applications where fast connection is preferred.
    /// 
    /// # Returns
    /// * `AutoConnectConfig` - Quick connection configuration
    /// 
    /// # Example
    /// ```
    /// let config = AutoConnector::quick_config();
    /// let (device, result) = AutoConnector::auto_connect(&config)?;
    /// ```
    pub fn quick_config() -> AutoConnectConfig {
        AutoConnectConfig {
            port_config: PortDetectionConfig::default(),
            baud_config: BaudDetector::quick_detection_config(),
            verbose: false,
            enable_caching: true,
            max_detection_time: Duration::from_secs(10),
        }
    }
    
    /// Create a thorough auto-connect configuration
    /// 
    /// Returns a configuration that performs comprehensive testing,
    /// suitable for situations where reliability is more important than speed.
    /// 
    /// # Returns
    /// * `AutoConnectConfig` - Thorough connection configuration
    /// 
    /// # Example
    /// ```
    /// let config = AutoConnector::thorough_config();
    /// let (device, result) = AutoConnector::auto_connect(&config)?;
    /// ```
    pub fn thorough_config() -> AutoConnectConfig {
        AutoConnectConfig {
            port_config: PortDetectionConfig::default(),
            baud_config: BaudDetector::thorough_detection_config(),
            verbose: true,
            enable_caching: true,
            max_detection_time: Duration::from_secs(60),
        }
    }
    
    /// Get detailed information about available ports and their compatibility
    /// 
    /// Returns comprehensive information about all available ports and their
    /// compatibility with Lumidox II devices for troubleshooting.
    /// 
    /// # Returns
    /// * `Result<Vec<String>>` - Detailed port information
    /// 
    /// # Example
    /// ```
    /// let info = AutoConnector::get_port_diagnostics()?;
    /// for line in info {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn get_port_diagnostics() -> Result<Vec<String>> {
        let mut diagnostics = Vec::new();
        
        diagnostics.push("=== Port Diagnostics ===".to_string());
        
        let config = PortDetectionConfig::default();
        let candidates = PortDetector::detect_ports(&config)?;
        
        if candidates.is_empty() {
            diagnostics.push("No compatible ports found".to_string());
        } else {
            diagnostics.push(format!("Found {} port candidates:", candidates.len()));
            
            for (index, candidate) in candidates.iter().enumerate() {
                diagnostics.push(format!("{}. {} - {}", 
                    index + 1, 
                    candidate.port_info.port_name, 
                    candidate.score_reason));
                
                if let Some(details) = &candidate.device_details {
                    if let Some(fw) = &details.firmware_version {
                        diagnostics.push(format!("   Firmware: {}", fw));
                    }
                    if let Some(model) = &details.model_number {
                        diagnostics.push(format!("   Model: {}", model));
                    }
                }
            }
        }
        
        diagnostics.push("".to_string());
        diagnostics.push("=== Detailed Port Information ===".to_string());
        let detailed_info = PortDetector::get_detailed_port_info()?;
        diagnostics.extend(detailed_info);
        
        Ok(diagnostics)
    }
}
