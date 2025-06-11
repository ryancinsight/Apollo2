//! Menu display logic for interactive CLI
//!
//! This module handles the visual presentation of menu options, including
//! dynamic content generation based on device state and capabilities.
//! 
//! The menu display system provides:
//! - Organized menu sections with clear categorization
//! - Dynamic power information display for stages
//! - Device capability-aware option presentation
//! - Consistent formatting and user experience

use std::io::{self, Write};
use crate::core::Result;
use crate::device::LumidoxDevice;

/// Menu display utilities and formatting functions
pub struct MenuDisplay;

impl MenuDisplay {
    /// Display the complete interactive menu with all options
    /// 
    /// This function renders the full menu interface including:
    /// - Stage firing options with power information
    /// - Device control options
    /// - Device status and information options
    /// - Stage parameter information options
    /// - Current control options
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller for dynamic content
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error
    /// 
    /// # Example
    /// ```
    /// MenuDisplay::show_main_menu(&device)?;
    /// ```
    pub fn show_main_menu(device: &mut LumidoxDevice) -> Result<()> {
        println!("-- Select an action --");
        
        // Display stage firing options
        Self::show_stage_options(device)?;
        
        // Display device control options
        Self::show_device_control_options(device)?;
        
        // Display device status and information options
        Self::show_device_status_options();
        
        // Display stage parameter information options
        Self::show_stage_parameter_options();
        
        // Display current control options
        Self::show_current_control_options();
        
        println!();
        Ok(())
    }
    
    /// Display stage firing options with power information
    /// 
    /// Shows options 1-5 for firing individual stages, with dynamic
    /// power information when available from the device.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller for power info
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error
    pub fn show_stage_options(device: &mut LumidoxDevice) -> Result<()> {
        for stage in 1..=5 {
            if let Ok(power_info) = device.get_power_info(stage) {
                println!("{}) Turn on stage {}: {} {}, {} {}", 
                    stage, stage, power_info.total_power, power_info.total_units, 
                    power_info.per_power, power_info.per_units);
            } else {
                println!("{}) Turn on stage {}", stage, stage);
            }
        }
        Ok(())
    }
    
    /// Display device control options
    /// 
    /// Shows options 6-9 for device control operations including
    /// custom current firing, arming, turning off, and quitting.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller for capability info
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error
    pub fn show_device_control_options(device: &mut LumidoxDevice) -> Result<()> {
        if let Ok(max_current) = device.get_max_current() {
            println!("6) Turn on stage with specific current (up to {}mA).", max_current);
        } else {
            println!("6) Turn on stage with specific current.");
        }

        println!("7) Arm device (prepare for firing).");
        println!("8) Turn off device.");
        println!("9) Quit program.");
        println!();
        Ok(())
    }
    
    /// Display device status and information options
    /// 
    /// Shows options 10-12 for reading device status, remote mode state,
    /// and current settings.
    pub fn show_device_status_options() {
        println!("--- Device Status & Information ---");
        println!("10) Show device status.");
        println!("11) Read remote mode state.");
        println!("12) Read ARM/FIRE current settings.");
    }
    
    /// Display stage parameter information options
    /// 
    /// Shows options 13-15 for reading stage-specific parameter information
    /// including complete parameters, ARM currents, and voltage settings.
    pub fn show_stage_parameter_options() {
        println!("--- Stage Parameter Information ---");
        println!("13) Show complete stage parameters.");
        println!("14) Read stage ARM current.");
        println!("15) Read stage voltage parameters.");
    }
    
    /// Display current control options
    /// 
    /// Shows option 16 for setting ARM current values.
    pub fn show_current_control_options() {
        println!("--- Current Control ---");
        println!("16) Set ARM current.");
    }
    
    /// Display the input prompt for menu selection
    /// 
    /// Shows a consistent prompt for user input and flushes stdout
    /// to ensure immediate display.
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error
    pub fn show_input_prompt() -> Result<()> {
        print!("Please enter choice number, then press ENTER: ");
        io::stdout().flush()?;
        Ok(())
    }
    
    /// Display device information header
    /// 
    /// Shows device connection confirmation and device information
    /// including firmware version, model, serial number, and wavelength.
    /// 
    /// # Arguments
    /// * `port_name` - The name of the connected port
    /// * `device` - Reference to the device controller for info
    /// 
    /// # Returns
    /// * `Result<()>` - Success or I/O error
    pub fn show_device_info_header(port_name: &str, device: &mut LumidoxDevice) -> Result<()> {
        println!("{} has been connected!", port_name);
        println!("--------------------------------------");
        
        if let Some(info) = device.info() {
            println!("Controller Firmware Version: {}", info.firmware_version);
            println!("Device Model Number: {}", info.model_number);
            println!("Device Serial Number: {}", info.serial_number);
            println!("Device Wavelength: {}", info.wavelength);
        }
        
        println!();
        Ok(())
    }
}
