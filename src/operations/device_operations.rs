//! Unified device operations for Lumidox II Controller
//!
//! This module provides unified device operation management that can be used
//! by both CLI and GUI interfaces. It extracts common device operations from
//! CLI handlers and provides a consistent interface for device interaction
//! with proper error handling and result types.
//!
//! The device operations module includes:
//! - Unified device operation manager with consistent interfaces
//! - Integration with existing device operation functions
//! - Support for both synchronous (CLI) and asynchronous (GUI) patterns
//! - Proper error handling and result propagation
//! - Configuration management for operation customization

use crate::device::{LumidoxDevice, models::{DeviceMode, DeviceInfo}};
use crate::ui::cli::device::create_device_controller_with_fallback;
use crate::core::{LumidoxError, Result};
use super::{OperationResult, OperationConfig};
use std::time::{Duration, Instant};

/// Device operation configuration
/// 
/// Configuration parameters specific to device operations that can be
/// customized for different operation modes and requirements.
#[derive(Debug, Clone)]
pub struct DeviceOperationConfig {
    /// Base operation configuration
    pub base: OperationConfig,
    /// Connection timeout in seconds
    pub connection_timeout: u32,
    /// Operation retry delay in milliseconds
    pub retry_delay_ms: u64,
    /// Whether to validate parameters before operations
    pub validate_parameters: bool,
    /// Whether to read device status after operations
    pub read_status_after_operation: bool,
}

impl DeviceOperationConfig {
    /// Create default device operation configuration
    /// 
    /// # Returns
    /// * `DeviceOperationConfig` - Default configuration
    /// 
    /// # Example
    /// ```
    /// let config = DeviceOperationConfig::default();
    /// ```
    pub fn default() -> Self {
        Self {
            base: OperationConfig::default(),
            connection_timeout: 30,
            retry_delay_ms: 1000,
            validate_parameters: true,
            read_status_after_operation: false,
        }
    }
    
    /// Create configuration from base operation config
    /// 
    /// # Arguments
    /// * `base` - Base operation configuration
    /// 
    /// # Returns
    /// * `DeviceOperationConfig` - Device operation configuration
    /// 
    /// # Example
    /// ```
    /// let config = DeviceOperationConfig::from_base(base_config);
    /// ```
    pub fn from_base(base: OperationConfig) -> Self {
        Self {
            base,
            connection_timeout: 30,
            retry_delay_ms: 1000,
            validate_parameters: true,
            read_status_after_operation: false,
        }
    }
}

/// Unified device operation manager
/// 
/// Provides a consistent interface for device operations that can be used
/// by both CLI and GUI components with proper error handling and result types.
pub struct DeviceOperationManager {
    /// Operation configuration
    config: DeviceOperationConfig,
}

impl DeviceOperationManager {
    /// Create new device operation manager
    /// 
    /// # Arguments
    /// * `config` - Device operation configuration
    /// 
    /// # Returns
    /// * `DeviceOperationManager` - New operation manager
    /// 
    /// # Example
    /// ```
    /// let manager = DeviceOperationManager::new(config);
    /// ```
    pub fn new(config: DeviceOperationConfig) -> Self {
        Self { config }
    }
    
    /// Create device operation manager with default configuration
    /// 
    /// # Returns
    /// * `DeviceOperationManager` - Operation manager with default config
    /// 
    /// # Example
    /// ```
    /// let manager = DeviceOperationManager::default();
    /// ```
    pub fn default() -> Self {
        Self::new(DeviceOperationConfig::default())
    }
    
    /// Connect to device
    /// 
    /// Establishes connection to the Lumidox II device using the configured
    /// connection parameters. Uses existing CLI device controller creation.
    /// 
    /// # Returns
    /// * `OperationResult<LumidoxDevice>` - Connection result with device
    /// 
    /// # Example
    /// ```
    /// let result = manager.connect_device().await;
    /// ```
    pub fn connect_device(&self) -> OperationResult<LumidoxDevice> {
        let start_time = Instant::now();
        
        if self.config.base.verbose {
            println!("Attempting to connect to device...");
        }
        
        // Use existing CLI device controller creation
        let result = if self.config.base.auto_detect {
            create_device_controller_with_fallback(
                self.config.base.port_name.clone(),
                self.config.base.auto_detect,
                self.config.base.optimize_transitions,
                self.config.base.verbose,
            )
        } else if let Some(ref port) = self.config.base.port_name {
            create_device_controller_with_fallback(
                Some(port.clone()),
                false,
                self.config.base.optimize_transitions,
                self.config.base.verbose,
            )
        } else {
            Err(LumidoxError::InvalidInput(
                "Port name required when auto-detection is disabled".to_string()
            ))
        };
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(device) => {
                let message = if self.config.base.verbose {
                    "Device connected successfully".to_string()
                } else {
                    "Device connected successfully".to_string()
                };
                
                OperationResult::Success {
                    data: device,
                    message: Some(message),
                    duration_ms: Some(duration.as_millis() as u64),
                }
            }
            Err(error) => {
                if self.config.base.verbose {
                    eprintln!("Connection failed: {}", error);
                }
                
                OperationResult::retryable_error(error, Some("Device connection failed".to_string()))
            }
        }
    }
    
    /// Fire specific stage
    /// 
    /// Fires a specific stage using existing device operation functions
    /// with proper validation and error handling.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to device
    /// * `stage` - Stage number to fire (1-5)
    /// 
    /// # Returns
    /// * `OperationResult<()>` - Firing operation result
    /// 
    /// # Example
    /// ```
    /// let result = manager.fire_stage(&mut device, 3);
    /// ```
    pub fn fire_stage(&self, device: &mut LumidoxDevice, stage: u8) -> OperationResult<()> {
        let start_time = Instant::now();
        
        // Validate stage number if configured
        if self.config.validate_parameters {
            if !(1..=5).contains(&stage) {
                return OperationResult::error(LumidoxError::InvalidInput(
                    format!("Invalid stage number: {}. Must be 1-5", stage)
                ));
            }
        }
        
        if self.config.base.verbose {
            println!("Firing stage {}...", stage);
        }
        
        // Use existing device operation
        let result = device.fire_stage(stage);
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(_) => {
                let message = format!("Stage {} fired successfully", stage);
                
                if self.config.base.verbose {
                    println!("{}", message);
                }
                
                OperationResult::Success {
                    data: (),
                    message: Some(message),
                    duration_ms: Some(duration.as_millis() as u64),
                }
            }
            Err(error) => {
                if self.config.base.verbose {
                    eprintln!("Stage {} firing failed: {}", stage, error);
                }
                
                OperationResult::error(error)
            }
        }
    }
    
    /// Fire with custom current
    /// 
    /// Fires the device with a custom current value using existing
    /// device operation functions.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to device
    /// * `current` - Current value in milliamps
    /// 
    /// # Returns
    /// * `OperationResult<()>` - Firing operation result
    /// 
    /// # Example
    /// ```
    /// let result = manager.fire_with_current(&mut device, 1500);
    /// ```
    pub fn fire_with_current(&self, device: &mut LumidoxDevice, current: u16) -> OperationResult<()> {
        let start_time = Instant::now();
        
        // Validate current value if configured
        if self.config.validate_parameters {
            if current == 0 {
                return OperationResult::error(LumidoxError::InvalidInput(
                    "Current value must be greater than 0".to_string()
                ));
            }
            if current > 3000 {
                return OperationResult::error(LumidoxError::InvalidInput(
                    format!("Current value {} exceeds maximum (3000mA)", current)
                ));
            }
        }
        
        if self.config.base.verbose {
            println!("Firing with custom current {}mA...", current);
        }
        
        // Use existing device operation
        let result = device.fire_with_current(current);
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(_) => {
                let message = format!("Fired with {}mA successfully", current);
                
                if self.config.base.verbose {
                    println!("{}", message);
                }
                
                OperationResult::Success {
                    data: (),
                    message: Some(message),
                    duration_ms: Some(duration.as_millis() as u64),
                }
            }
            Err(error) => {
                if self.config.base.verbose {
                    eprintln!("Custom current firing failed: {}", error);
                }
                
                OperationResult::error(error)
            }
        }
    }
    
    /// Set ARM current
    /// 
    /// Sets the ARM current value using existing device operation functions.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to device
    /// * `current` - ARM current value in milliamps
    /// 
    /// # Returns
    /// * `OperationResult<()>` - ARM current setting result
    /// 
    /// # Example
    /// ```
    /// let result = manager.set_arm_current(&mut device, 1000);
    /// ```
    pub fn set_arm_current(&self, device: &mut LumidoxDevice, current: u16) -> OperationResult<()> {
        let start_time = Instant::now();
        
        // Validate current value if configured
        if self.config.validate_parameters {
            if current == 0 {
                return OperationResult::error(LumidoxError::InvalidInput(
                    "ARM current must be greater than 0".to_string()
                ));
            }
            if current > 3000 {
                return OperationResult::error(LumidoxError::InvalidInput(
                    format!("ARM current {} exceeds maximum (3000mA)", current)
                ));
            }
        }
        
        if self.config.base.verbose {
            println!("Setting ARM current to {}mA...", current);
        }
        
        // Use existing device operation
        let result = device.set_arm_current(current);
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(_) => {
                let message = format!("ARM current set to {}mA successfully", current);
                
                if self.config.base.verbose {
                    println!("{}", message);
                }
                
                OperationResult::Success {
                    data: (),
                    message: Some(message),
                    duration_ms: Some(duration.as_millis() as u64),
                }
            }
            Err(error) => {
                if self.config.base.verbose {
                    eprintln!("ARM current setting failed: {}", error);
                }
                
                OperationResult::error(error)
            }
        }
    }
    
    /// Arm device
    /// 
    /// Arms the device for firing operations using existing device functions.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to device
    /// 
    /// # Returns
    /// * `OperationResult<()>` - Device arming result
    /// 
    /// # Example
    /// ```
    /// let result = manager.arm_device(&mut device);
    /// ```
    pub fn arm_device(&self, device: &mut LumidoxDevice) -> OperationResult<()> {
        let start_time = Instant::now();
        
        if self.config.base.verbose {
            println!("Arming device...");
        }
        
        // Use existing device operation
        let result = device.arm();
        
        let duration = start_time.elapsed();
        
        match result {
            Ok(_) => {
                let message = "Device armed successfully".to_string();
                
                if self.config.base.verbose {
                    println!("{}", message);
                }
                
                OperationResult::Success {
                    data: (),
                    message: Some(message),
                    duration_ms: Some(duration.as_millis() as u64),
                }
            }
            Err(error) => {
                if self.config.base.verbose {
                    eprintln!("Device arming failed: {}", error);
                }
                
                OperationResult::error(error)
            }
        }
    }
    
    /// Read device status
    /// 
    /// Reads current device status including ARM current, FIRE current,
    /// and remote mode state using existing device functions.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to device
    /// 
    /// # Returns
    /// * `OperationResult<DeviceStatus>` - Device status reading result
    /// 
    /// # Example
    /// ```
    /// let result = manager.read_device_status(&mut device);
    /// ```
    pub fn read_device_status(&self, device: &mut LumidoxDevice) -> OperationResult<DeviceStatus> {
        let start_time = Instant::now();
        
        if self.config.base.verbose {
            println!("Reading device status...");
        }
        
        // Read device parameters using existing operations
        let arm_current = device.read_arm_current().ok();
        let fire_current = device.read_fire_current().ok();
        let remote_mode = device.read_remote_mode().ok();
        
        let duration = start_time.elapsed();
        
        let status = DeviceStatus {
            arm_current,
            fire_current,
            remote_mode,
            connection_active: true,
        };
        
        let message = if self.config.base.verbose {
            format!("Device status: ARM={}mA, FIRE={}mA, Mode={:?}",
                arm_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string()),
                fire_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string()),
                remote_mode.unwrap_or(DeviceMode::Local)
            )
        } else {
            "Device status read successfully".to_string()
        };
        
        OperationResult::Success {
            data: status,
            message: Some(message),
            duration_ms: Some(duration.as_millis() as u64),
        }
    }
    
    /// Get operation configuration
    /// 
    /// Returns the current operation configuration.
    /// 
    /// # Returns
    /// * `&DeviceOperationConfig` - Current configuration
    /// 
    /// # Example
    /// ```
    /// let config = manager.get_config();
    /// ```
    pub fn get_config(&self) -> &DeviceOperationConfig {
        &self.config
    }
    
    /// Update operation configuration
    /// 
    /// Updates the operation configuration with new settings.
    /// 
    /// # Arguments
    /// * `config` - New operation configuration
    /// 
    /// # Example
    /// ```
    /// manager.update_config(new_config);
    /// ```
    pub fn update_config(&mut self, config: DeviceOperationConfig) {
        self.config = config;
    }
}

/// Device status information
/// 
/// Contains current device status information that can be used by both
/// CLI and GUI interfaces for display and decision making.
#[derive(Debug, Clone)]
pub struct DeviceStatus {
    /// Current ARM current setting in milliamps
    pub arm_current: Option<u16>,
    /// Current FIRE current setting in milliamps
    pub fire_current: Option<u16>,
    /// Current remote mode state
    pub remote_mode: Option<DeviceMode>,
    /// Whether device connection is active
    pub connection_active: bool,
}

impl DeviceStatus {
    /// Check if device is ready for firing
    /// 
    /// # Returns
    /// * `bool` - True if device is ready for firing
    /// 
    /// # Example
    /// ```
    /// if status.is_ready_for_firing() {
    ///     println!("Device ready for firing");
    /// }
    /// ```
    pub fn is_ready_for_firing(&self) -> bool {
        self.connection_active && 
        self.arm_current.is_some() && 
        matches!(self.remote_mode, Some(DeviceMode::Armed) | Some(DeviceMode::Remote))
    }
    
    /// Get status summary string
    /// 
    /// # Returns
    /// * `String` - Human-readable status summary
    /// 
    /// # Example
    /// ```
    /// println!("Status: {}", status.get_summary());
    /// ```
    pub fn get_summary(&self) -> String {
        if !self.connection_active {
            return "Device not connected".to_string();
        }
        
        let arm_str = self.arm_current
            .map(|c| format!("{}mA", c))
            .unwrap_or_else(|| "N/A".to_string());
        
        let fire_str = self.fire_current
            .map(|c| format!("{}mA", c))
            .unwrap_or_else(|| "N/A".to_string());
        
        let mode_str = self.remote_mode
            .map(|m| format!("{:?}", m))
            .unwrap_or_else(|| "Unknown".to_string());
        
        format!("ARM: {}, FIRE: {}, Mode: {}", arm_str, fire_str, mode_str)
    }
}
