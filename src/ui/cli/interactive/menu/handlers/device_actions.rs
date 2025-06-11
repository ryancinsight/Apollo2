//! Device control action handlers for interactive CLI menu
//!
//! This module handles the execution of device control actions including
//! arming, turning off, and shutdown operations. It provides specialized
//! handlers for device control menu selections with proper error handling
//! and user feedback.
//!
//! The device action handlers system provides:
//! - Device arming with status feedback
//! - Device turn off operations
//! - Device shutdown with proper cleanup
//! - Error handling and user-friendly messages
//! - Integration with device control operations

use crate::core::{Result, DeviceControlOperations, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Duration;
use std::thread;

/// Device control action handlers utilities and functionality
pub struct DeviceActionHandlers;

impl DeviceActionHandlers {
    /// Handle device arming action
    /// 
    /// Executes device arming operation with appropriate user feedback
    /// and status confirmation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for arming operations
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = DeviceActionHandlers::handle_arm_device(&mut device)?;
    /// ```
    pub fn handle_arm_device(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        println!("Arming device...");

        // Use unified operation layer
        match DeviceControlOperations::arm_device(device) {
            Ok(response) => {
                // CLI-specific presentation of the unified result
                println!("{}", response.message);
                if let DeviceOperationData::DeviceControl { new_state, .. } = &response.data {
                    if let Some(state) = new_state {
                        println!("Device state: {}", state);
                    }
                }
                println!("The device is prepared to execute firing commands.");
                println!();
            }
            Err(e) => {
                println!("Error arming device: {}", e);
                println!("Please check device status and try again.");
                println!();
            }
        }

        Ok(true)
    }
    
    /// Handle device turn off action
    /// 
    /// Executes device turn off operation with appropriate user feedback.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for turn off operations
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = DeviceActionHandlers::handle_turn_off_device(&mut device)?;
    /// ```
    pub fn handle_turn_off_device(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        println!("Turning off device...");

        // Use unified operation layer
        match DeviceControlOperations::turn_off_device(device) {
            Ok(response) => {
                // CLI-specific presentation of the unified result
                println!("{}", response.message);
                if let DeviceOperationData::DeviceControl { new_state, .. } = &response.data {
                    if let Some(state) = new_state {
                        println!("Device state: {}", state);
                    }
                }
                println!("The device is now in a safe, non-armed state.");
                println!();
            }
            Err(e) => {
                println!("Error turning off device: {}", e);
                println!("Device may still be in an active state.");
                println!();
            }
        }

        Ok(true)
    }
    
    /// Handle device shutdown and program quit action
    /// 
    /// Executes device shutdown operation and prepares for program exit
    /// with appropriate user feedback and cleanup procedures.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for shutdown operations
    /// 
    /// # Returns
    /// * `Result<bool>` - False to exit menu loop
    /// 
    /// # Example
    /// ```
    /// let continue_menu = DeviceActionHandlers::handle_shutdown_and_quit(&mut device)?;
    /// ```
    pub fn handle_shutdown_and_quit(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        println!("Shutting down device...");

        // Use unified operation layer
        match DeviceControlOperations::shutdown_device(device) {
            Ok(response) => {
                // CLI-specific presentation of the unified result
                println!("{}", response.message);
                if let DeviceOperationData::DeviceControl { new_state, .. } = &response.data {
                    if let Some(state) = new_state {
                        println!("Device state: {}", state);
                    }
                }
                println!("To resume using the controller in local mode, please cycle the power with on/off switch.");
            }
            Err(e) => {
                println!("Error during device shutdown: {}", e);
                println!("Device may not have shutdown properly.");
            }
        }

        thread::sleep(Duration::from_millis(1000));
        println!("Quitting program...");
        thread::sleep(Duration::from_millis(1000));
        println!();

        Ok(false) // Exit menu loop
    }
    
    /// Handle device control action based on choice
    /// 
    /// Routes device control menu choices to appropriate handlers.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<Option<bool>>` - Some(bool) if handled, None if not a device control choice
    /// 
    /// # Example
    /// ```
    /// if let Some(continue_menu) = DeviceActionHandlers::handle_device_choice(&mut device, "7")? {
    ///     // Device control choice was handled
    /// }
    /// ```
    pub fn handle_device_choice(device: &mut LumidoxDevice, choice: &str) -> Result<Option<bool>> {
        match choice {
            "7" => Ok(Some(Self::handle_arm_device(device)?)),
            "8" => Ok(Some(Self::handle_turn_off_device(device)?)),
            "9" => Ok(Some(Self::handle_shutdown_and_quit(device)?)),
            _ => Ok(None)
        }
    }
    
    /// Display device arming confirmation
    /// 
    /// Shows confirmation and safety information before arming the device.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// DeviceActionHandlers::display_arm_confirmation()?;
    /// ```
    pub fn display_arm_confirmation() -> Result<()> {
        println!();
        println!("Preparing to arm device...");
        println!("Once armed, the device will be ready to execute firing commands.");
        println!("Ensure all safety precautions are in place.");
        Ok(())
    }
    
    /// Display device turn off confirmation
    /// 
    /// Shows confirmation information before turning off the device.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// DeviceActionHandlers::display_turn_off_confirmation()?;
    /// ```
    pub fn display_turn_off_confirmation() -> Result<()> {
        println!();
        println!("Preparing to turn off device...");
        println!("This will put the device in a safe, non-armed state.");
        Ok(())
    }
    
    /// Display shutdown confirmation
    /// 
    /// Shows confirmation and cleanup information before shutting down.
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds for display operations
    /// 
    /// # Example
    /// ```
    /// DeviceActionHandlers::display_shutdown_confirmation()?;
    /// ```
    pub fn display_shutdown_confirmation() -> Result<()> {
        println!();
        println!("Preparing to shutdown device and quit program...");
        println!("This will return the device to local mode and exit the application.");
        println!("To resume remote control, restart the application.");
        Ok(())
    }
    
    /// Check device readiness for arming
    /// 
    /// Validates that the device is in a suitable state for arming.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for status checks
    /// 
    /// # Returns
    /// * `Result<bool>` - True if device is ready for arming
    /// 
    /// # Example
    /// ```
    /// if DeviceActionHandlers::check_arm_readiness(&device)? {
    ///     println!("Device is ready for arming");
    /// }
    /// ```
    pub fn check_arm_readiness(device: &mut LumidoxDevice) -> Result<bool> {
        // Check if device is in remote mode
        match device.read_remote_mode() {
            Ok(mode) => {
                match mode {
                    crate::device::models::DeviceMode::Standby |
                    crate::device::models::DeviceMode::Armed |
                    crate::device::models::DeviceMode::Remote => {
                        Ok(true)
                    }
                    crate::device::models::DeviceMode::Local => {
                        println!("Warning: Device is not in remote mode.");
                        println!("Arming may not be possible until device is in remote mode.");
                        Ok(false)
                    }
                }
            }
            Err(e) => {
                println!("Warning: Error checking device remote mode: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Get device status summary for control operations
    /// 
    /// Retrieves and formats device status information relevant to control operations.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for status queries
    /// 
    /// # Returns
    /// * `Result<String>` - Formatted status summary
    /// 
    /// # Example
    /// ```
    /// let status = DeviceActionHandlers::get_device_status_summary(&device)?;
    /// println!("Device Status: {}", status);
    /// ```
    pub fn get_device_status_summary(device: &mut LumidoxDevice) -> Result<String> {
        let mut status_parts = Vec::new();
        
        // Get remote mode status
        if let Ok(mode) = device.read_remote_mode() {
            match mode {
                crate::device::models::DeviceMode::Local => {
                    status_parts.push("Remote Mode: Inactive (Local)".to_string());
                }
                crate::device::models::DeviceMode::Standby => {
                    status_parts.push("Remote Mode: Active (Standby)".to_string());
                }
                crate::device::models::DeviceMode::Armed => {
                    status_parts.push("Remote Mode: Active (Armed)".to_string());
                }
                crate::device::models::DeviceMode::Remote => {
                    status_parts.push("Remote Mode: Active (Remote)".to_string());
                }
            }
        } else {
            status_parts.push("Remote Mode: Unknown".to_string());
        }
        
        // Get device state
        if let Ok(state_desc) = device.read_device_state() {
            status_parts.push(format!("State: {}", state_desc));
        } else {
            status_parts.push("State: Unknown".to_string());
        }
        
        // Get current settings
        if let Ok(current_summary) = device.read_current_settings() {
            status_parts.push(format!("Current: {}", current_summary));
        } else {
            status_parts.push("Current: Unknown".to_string());
        }
        
        Ok(status_parts.join(" | "))
    }
    
    /// Handle device control with status check
    /// 
    /// Enhanced device control handler that includes status checks before operations.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<Option<bool>>` - Some(bool) if handled, None if not a device control choice
    /// 
    /// # Example
    /// ```
    /// if let Some(continue_menu) = DeviceActionHandlers::handle_device_choice_with_status(&mut device, "7")? {
    ///     // Device control choice was handled with status check
    /// }
    /// ```
    pub fn handle_device_choice_with_status(device: &mut LumidoxDevice, choice: &str) -> Result<Option<bool>> {
        match choice {
            "7" => {
                Self::display_arm_confirmation()?;
                if Self::check_arm_readiness(device)? {
                    Ok(Some(Self::handle_arm_device(device)?))
                } else {
                    println!("Arming operation cancelled due to device status.");
                    println!();
                    Ok(Some(true))
                }
            }
            "8" => {
                Self::display_turn_off_confirmation()?;
                Ok(Some(Self::handle_turn_off_device(device)?))
            }
            "9" => {
                Self::display_shutdown_confirmation()?;
                Ok(Some(Self::handle_shutdown_and_quit(device)?))
            }
            _ => Ok(None)
        }
    }
}
