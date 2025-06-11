//! Device information operations for Lumidox II Controller
//!
//! This module handles device information retrieval operations including
//! device status reading, current settings queries, power information,
//! and other informational operations that don't modify device state.
//! 
//! The information operations system provides:
//! - Device status and state information retrieval
//! - Current settings and parameter queries
//! - Power information and capability queries
//! - Remote mode state verification
//! - Comprehensive device information access

use crate::core::Result;
use crate::device::models::{DeviceInfo, PowerInfo};
use crate::device::operations::{readback, power};

/// Device information operations utilities and functionality
pub struct DeviceInformationOperations;

impl DeviceInformationOperations {
    /// Get cached device information
    /// 
    /// Retrieves the cached device information that was loaded during
    /// device initialization. This includes firmware version, model number,
    /// serial number, and other static device characteristics.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// 
    /// # Returns
    /// * `Option<&DeviceInfo>` - Reference to cached device info, if available
    /// 
    /// # Example
    /// ```
    /// if let Some(info) = DeviceInformationOperations::get_device_info(&device) {
    ///     println!("Firmware: {}", info.firmware_version);
    /// }
    /// ```
    pub fn get_device_info(device: &super::super::LumidoxDevice) -> Option<&DeviceInfo> {
        device.info.as_ref()
    }
    
    /// Read current remote mode state from device
    ///
    /// Queries the device to determine its current remote mode state,
    /// which indicates the device's operational mode and readiness status.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    ///
    /// # Returns
    /// * `Result<DeviceMode>` - Remote mode state or query error
    ///
    /// # Mode Values
    /// - Local: Device controlled locally (manual operation)
    /// - Standby: Device on but output disabled (safe state)
    /// - Armed: Device ready for firing operations
    /// - Remote: Device actively firing or in remote control mode
    ///
    /// # Example
    /// ```
    /// let mode_state = DeviceInformationOperations::read_remote_mode_state(&mut device)?;
    /// println!("Remote mode state: {:?}", mode_state);
    /// ```
    pub fn read_remote_mode_state(device: &mut super::super::LumidoxDevice) -> Result<crate::device::models::DeviceMode> {
        readback::read_remote_mode_state(&mut device.protocol)
    }
    
    /// Read ARM current setting from device
    /// 
    /// Queries the device to retrieve the current ARM current setting,
    /// which is the current value used when the device is in armed mode.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<u16>` - ARM current in milliamps or query error
    /// 
    /// # Example
    /// ```
    /// let arm_current = DeviceInformationOperations::read_arm_current(&mut device)?;
    /// println!("ARM current: {}mA", arm_current);
    /// ```
    pub fn read_arm_current(device: &mut super::super::LumidoxDevice) -> Result<u16> {
        readback::read_arm_current(&mut device.protocol)
    }
    
    /// Read FIRE current setting from device
    /// 
    /// Queries the device to retrieve the current FIRE current setting,
    /// which is the current value used during active firing operations.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<u16>` - FIRE current in milliamps or query error
    /// 
    /// # Example
    /// ```
    /// let fire_current = DeviceInformationOperations::read_fire_current(&mut device)?;
    /// println!("FIRE current: {}mA", fire_current);
    /// ```
    pub fn read_fire_current(device: &mut super::super::LumidoxDevice) -> Result<u16> {
        readback::read_fire_current(&mut device.protocol)
    }
    
    /// Get power information for a specific stage
    /// 
    /// Retrieves power information for the specified stage including
    /// total power, per-LED power, and power units for display purposes.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage_num` - The stage number to query (1-5)
    /// 
    /// # Returns
    /// * `Result<PowerInfo>` - Power information or query error
    /// 
    /// # Example
    /// ```
    /// let power_info = DeviceInformationOperations::get_power_info(&mut device, 3)?;
    /// println!("Stage 3: {} {}, {} {}", 
    ///     power_info.total_power, power_info.total_units,
    ///     power_info.per_power, power_info.per_units);
    /// ```
    pub fn get_power_info(device: &mut super::super::LumidoxDevice, stage_num: u8) -> Result<PowerInfo> {
        power::get_power_info(&mut device.protocol, stage_num)
    }
    
    /// Get comprehensive device state description
    /// 
    /// Provides a comprehensive description of the current device state
    /// including mode, current settings, and operational status.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<String>` - Device state description or query error
    /// 
    /// # Example
    /// ```
    /// let description = DeviceInformationOperations::get_device_state_description(&mut device)?;
    /// println!("Device state: {}", description);
    /// ```
    pub fn get_device_state_description(device: &mut super::super::LumidoxDevice) -> Result<String> {
        readback::get_device_state_description(&mut device.protocol)
    }
    
    /// Get current settings summary
    /// 
    /// Provides a summary of current device settings including ARM and
    /// FIRE current values and their relationship.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<String>` - Current settings summary or query error
    /// 
    /// # Example
    /// ```
    /// let summary = DeviceInformationOperations::get_current_settings_summary(&mut device)?;
    /// println!("Current settings: {}", summary);
    /// ```
    pub fn get_current_settings_summary(device: &mut super::super::LumidoxDevice) -> Result<String> {
        readback::get_current_settings_summary(&mut device.protocol)
    }
    
    /// Check if device is remote controlled
    /// 
    /// Determines whether the device is currently under remote control
    /// based on its mode state and operational status.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<bool>` - True if remote controlled, false otherwise
    /// 
    /// # Example
    /// ```
    /// if DeviceInformationOperations::is_remote_controlled(&mut device)? {
    ///     println!("Device is under remote control");
    /// }
    /// ```
    pub fn is_remote_controlled(device: &mut super::super::LumidoxDevice) -> Result<bool> {
        readback::is_remote_controlled(&mut device.protocol)
    }
    
    /// Check if device is ready for firing
    /// 
    /// Determines whether the device is in a state ready for firing
    /// operations based on mode and current settings.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<bool>` - True if ready for firing, false otherwise
    /// 
    /// # Example
    /// ```
    /// if DeviceInformationOperations::is_ready_for_firing(&mut device)? {
    ///     println!("Device is ready for firing operations");
    /// }
    /// ```
    pub fn is_ready_for_firing(device: &mut super::super::LumidoxDevice) -> Result<bool> {
        readback::is_ready_for_firing(&mut device.protocol)
    }
    
    /// Get comprehensive device status report
    /// 
    /// Generates a comprehensive status report including all available
    /// device information, current settings, and operational status.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<DeviceStatusReport>` - Comprehensive status report
    /// 
    /// # Example
    /// ```
    /// let report = DeviceInformationOperations::get_comprehensive_status(&mut device)?;
    /// println!("Device Status Report:\n{}", report.summary());
    /// ```
    pub fn get_comprehensive_status(
        device: &mut super::super::LumidoxDevice
    ) -> Result<DeviceStatusReport> {
        let device_info = device.info.clone();
        let current_mode = device.current_mode;
        let optimization_enabled = device.optimize_transitions;
        
        let remote_mode_state = Self::read_remote_mode_state(device).ok().map(|mode| mode as u16);
        let arm_current = Self::read_arm_current(device).ok();
        let fire_current = Self::read_fire_current(device).ok();
        let is_remote_controlled = Self::is_remote_controlled(device).ok();
        let is_ready_for_firing = Self::is_ready_for_firing(device).ok();
        
        Ok(DeviceStatusReport {
            device_info,
            current_mode,
            optimization_enabled,
            remote_mode_state,
            arm_current,
            fire_current,
            is_remote_controlled,
            is_ready_for_firing,
        })
    }
    
    /// Get stage-specific information summary
    /// 
    /// Provides information specific to a particular stage including
    /// power characteristics and operational parameters.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage_num` - The stage number to query (1-5)
    /// 
    /// # Returns
    /// * `Result<StageInformation>` - Stage-specific information
    /// 
    /// # Example
    /// ```
    /// let stage_info = DeviceInformationOperations::get_stage_information(&mut device, 2)?;
    /// println!("Stage 2 power: {} {}", stage_info.power_info.total_power, stage_info.power_info.total_units);
    /// ```
    pub fn get_stage_information(
        device: &mut super::super::LumidoxDevice,
        stage_num: u8
    ) -> Result<StageInformation> {
        let power_info = Self::get_power_info(device, stage_num)?;
        let is_valid_stage = (1..=5).contains(&stage_num);
        
        Ok(StageInformation {
            stage_number: stage_num,
            is_valid_stage,
            power_info,
        })
    }
    
    /// Validate information consistency
    /// 
    /// Checks the consistency of device information by comparing cached
    /// values with current device state where possible.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<InformationConsistencyReport>` - Consistency validation results
    /// 
    /// # Example
    /// ```
    /// let consistency = DeviceInformationOperations::validate_information_consistency(&mut device)?;
    /// if !consistency.is_consistent {
    ///     println!("Information inconsistencies detected");
    /// }
    /// ```
    pub fn validate_information_consistency(
        device: &mut super::super::LumidoxDevice
    ) -> Result<InformationConsistencyReport> {
        let has_cached_info = device.info.is_some();
        let has_current_mode = device.current_mode.is_some();
        
        let remote_mode_readable = Self::read_remote_mode_state(device).is_ok();
        let current_settings_readable = Self::read_arm_current(device).is_ok() && 
                                       Self::read_fire_current(device).is_ok();
        
        let is_consistent = has_cached_info && has_current_mode && 
                           remote_mode_readable && current_settings_readable;
        
        let mut issues = Vec::new();
        if !has_cached_info {
            issues.push("Device information not cached".to_string());
        }
        if !has_current_mode {
            issues.push("Current mode not set".to_string());
        }
        if !remote_mode_readable {
            issues.push("Cannot read remote mode state".to_string());
        }
        if !current_settings_readable {
            issues.push("Cannot read current settings".to_string());
        }
        
        Ok(InformationConsistencyReport {
            is_consistent,
            has_cached_info,
            has_current_mode,
            remote_mode_readable,
            current_settings_readable,
            consistency_issues: issues,
        })
    }
}

/// Comprehensive device status report
#[derive(Debug, Clone)]
pub struct DeviceStatusReport {
    /// Cached device information
    pub device_info: Option<DeviceInfo>,
    /// Current device mode from internal tracking
    pub current_mode: Option<crate::device::models::DeviceMode>,
    /// Whether optimization is enabled
    pub optimization_enabled: bool,
    /// Remote mode state from device
    pub remote_mode_state: Option<u16>,
    /// ARM current setting
    pub arm_current: Option<u16>,
    /// FIRE current setting
    pub fire_current: Option<u16>,
    /// Whether device is remote controlled
    pub is_remote_controlled: Option<bool>,
    /// Whether device is ready for firing
    pub is_ready_for_firing: Option<bool>,
}

impl DeviceStatusReport {
    /// Generate a summary string of the device status
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        
        if let Some(info) = &self.device_info {
            summary.push_str(&format!("Device: {} v{}\n", info.model_number, info.firmware_version));
            summary.push_str(&format!("Serial: {}, Wavelength: {}\n", info.serial_number, info.wavelength));
        }
        
        summary.push_str(&format!("Mode: {:?}\n", self.current_mode));
        summary.push_str(&format!("Optimization: {}\n", self.optimization_enabled));
        
        if let Some(state) = self.remote_mode_state {
            summary.push_str(&format!("Remote State: 0x{:04x}\n", state));
        }
        
        if let (Some(arm), Some(fire)) = (self.arm_current, self.fire_current) {
            summary.push_str(&format!("Currents: ARM {}mA, FIRE {}mA\n", arm, fire));
        }
        
        summary
    }
}

/// Stage-specific information
#[derive(Debug, Clone)]
pub struct StageInformation {
    /// The stage number
    pub stage_number: u8,
    /// Whether the stage number is valid (1-5)
    pub is_valid_stage: bool,
    /// Power information for the stage
    pub power_info: PowerInfo,
}

/// Information consistency validation report
#[derive(Debug, Clone)]
pub struct InformationConsistencyReport {
    /// Whether all information is consistent
    pub is_consistent: bool,
    /// Whether device information is cached
    pub has_cached_info: bool,
    /// Whether current mode is set
    pub has_current_mode: bool,
    /// Whether remote mode state is readable
    pub remote_mode_readable: bool,
    /// Whether current settings are readable
    pub current_settings_readable: bool,
    /// List of consistency issues found
    pub consistency_issues: Vec<String>,
}
