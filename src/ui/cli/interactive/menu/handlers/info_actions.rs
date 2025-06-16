//! Information and status action handlers for interactive CLI menu
//!
//! This module handles the execution of information retrieval and status
//! display actions including device status, parameter information, and
//! current control operations. It provides specialized handlers for
//! information menu selections with proper formatting and error handling.
//!
//! The information action handlers system provides:
//! - Device status and state information display
//! - Stage parameter information retrieval
//! - Current settings display and modification
//! - Formatted output for user-friendly information presentation
//! - Error handling and user-friendly messages

use crate::core::Result;
use crate::device::LumidoxDevice;
use std::io::{self, Write};

/// Information and status action handlers utilities and functionality
pub struct InfoActionHandlers;

impl InfoActionHandlers {
    /// Handle device status display action
    /// 
    /// Retrieves and displays comprehensive device status information
    /// including device state and current settings.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for status queries
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_device_status(&mut device)?;
    /// ```
    pub fn handle_device_status(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        println!("Reading device status...");
        
        // Display device state
        match device.read_device_state() {
            Ok(state_desc) => println!("Device State: {}", state_desc),
            Err(e) => println!("Error reading device state: {}", e),
        }
        
        // Display current settings
        match device.read_current_settings() {
            Ok(current_summary) => println!("Current Settings: {}", current_summary),
            Err(e) => println!("Error reading current settings: {}", e),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle remote mode state display action
    /// 
    /// Retrieves and displays device remote mode state information.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for remote mode queries
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_remote_mode_state(&mut device)?;
    /// ```
    pub fn handle_remote_mode_state(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        println!("Reading remote mode state...");
        
        match device.read_remote_mode() {
            Ok(mode) => {
                println!("Remote Mode State: {:?}", mode);
                match mode {
                    crate::device::models::DeviceMode::Local => {
                        println!("Device is currently in local mode.");
                    }
                    crate::device::models::DeviceMode::Standby => {
                        println!("Device is currently under remote control (Standby).");
                    }
                    crate::device::models::DeviceMode::Armed => {
                        println!("Device is currently under remote control (Armed).");
                    }
                    crate::device::models::DeviceMode::Remote => {
                        println!("Device is currently under remote control (Remote/Firing).");
                    }
                }
            }
            Err(e) => println!("Error reading remote mode state: {}", e),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle current settings display action
    /// 
    /// Retrieves and displays ARM and FIRE current settings.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for current queries
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_current_settings(&mut device)?;
    /// ```
    pub fn handle_current_settings(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        println!("Reading current settings...");
        
        // Display ARM current
        match device.read_arm_current() {
            Ok(current) => println!("ARM Current: {}mA", current),
            Err(e) => println!("Error reading ARM current: {}", e),
        }
        
        // Display FIRE current
        match device.read_fire_current() {
            Ok(current) => println!("FIRE Current: {}mA", current),
            Err(e) => println!("Error reading FIRE current: {}", e),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle stage parameters display action
    /// 
    /// Prompts user for stage number and displays complete stage parameters.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for parameter queries
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_stage_parameters(&mut device)?;
    /// ```
    pub fn handle_stage_parameters(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        print!("Enter stage number (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(stage) if (1..=5).contains(&stage) => {
                println!("Reading complete parameters for stage {}...", stage);
                
                match device.get_stage_parameters(stage) {
                    Ok(params) => {
                        println!("Stage {} Parameters:", params.stage_number);
                        println!("  ARM Current: {}mA", params.arm_current_ma);
                        println!("  FIRE Current: {}mA", params.fire_current_ma);
                        println!("  Voltage Limit: {:.1}V", params.volt_limit_v);
                        println!("  Voltage Start: {:.1}V", params.volt_start_v);
                        println!("  Total Power: {:.1} {}", params.power_total, params.total_units);
                        println!("  Per LED Power: {:.1} {}", params.power_per_led, params.per_led_units);
                    }
                    Err(e) => println!("Error reading stage parameters: {}", e),
                }
            }
            _ => println!("Invalid stage number. Must be 1-5."),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle stage ARM current display action
    /// 
    /// Prompts user for stage number and displays stage ARM current.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for ARM current queries
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_stage_arm_current(&mut device)?;
    /// ```
    pub fn handle_stage_arm_current(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        print!("Enter stage number (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(stage) if (1..=5).contains(&stage) => {
                println!("Reading ARM current for stage {}...", stage);
                
                match device.get_stage_arm_current(stage) {
                    Ok(current) => println!("Stage {} ARM Current: {}mA", stage, current),
                    Err(e) => println!("Error reading stage ARM current: {}", e),
                }
            }
            _ => println!("Invalid stage number. Must be 1-5."),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle stage voltage parameters display action
    /// 
    /// Prompts user for stage number and displays voltage limit and start values.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for voltage queries
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_stage_voltage_parameters(&mut device)?;
    /// ```
    pub fn handle_stage_voltage_parameters(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        print!("Enter stage number (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(stage) if (1..=5).contains(&stage) => {
                println!("Reading voltage parameters for stage {}...", stage);
                
                // Display voltage limit
                match device.get_stage_volt_limit(stage) {
                    Ok(limit) => println!("Stage {} Voltage Limit: {:.1}V", stage, limit),
                    Err(e) => println!("Error reading voltage limit: {}", e),
                }
                
                // Display voltage start
                match device.get_stage_volt_start(stage) {
                    Ok(start) => println!("Stage {} Voltage Start: {:.1}V", stage, start),
                    Err(e) => println!("Error reading voltage start: {}", e),
                }
            }
            _ => println!("Invalid stage number. Must be 1-5."),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle ARM current setting action
    /// 
    /// Prompts user for ARM current value and sets it on the device.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for ARM current setting
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = InfoActionHandlers::handle_set_arm_current(&mut device)?;
    /// ```
    pub fn handle_set_arm_current(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        print!("Enter ARM current in mA: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u16>() {
            Ok(current) => {
                println!("Setting ARM current to {}mA...", current);
                
                match device.set_arm_current(current) {
                    Ok(()) => println!("ARM current set successfully."),
                    Err(e) => println!("Error setting ARM current: {}", e),
                }
            }
            Err(_) => println!("Invalid current value. Must be a whole number."),
        }
        
        println!();
        Ok(true)
    }
    
    /// Handle information action based on choice
    /// 
    /// Routes information and status menu choices to appropriate handlers.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<Option<bool>>` - Some(bool) if handled, None if not an info choice
    /// 
    /// # Example
    /// ```
    /// if let Some(continue_menu) = InfoActionHandlers::handle_info_choice(&mut device, "10")? {
    ///     // Information choice was handled
    /// }
    /// ```
    pub fn handle_info_choice(device: &mut LumidoxDevice, choice: &str) -> Result<Option<bool>> {
        match choice {
            "9" => Ok(Some(Self::handle_device_status(device)?)),
            "10" => Ok(Some(Self::handle_remote_mode_state(device)?)),
            "11" => Ok(Some(Self::handle_current_settings(device)?)),
            "12" => Ok(Some(Self::handle_stage_parameters(device)?)),
            "13" => Ok(Some(Self::handle_stage_arm_current(device)?)),
            "14" => Ok(Some(Self::handle_stage_voltage_parameters(device)?)),
            "15" => Ok(Some(Self::handle_set_arm_current(device)?)),
            _ => Ok(None)
        }
    }
    
    /// Get stage number input from user
    /// 
    /// Prompts user for stage number with validation.
    /// 
    /// # Returns
    /// * `Result<Option<u8>>` - Some(stage) if valid, None if invalid
    /// 
    /// # Example
    /// ```
    /// if let Some(stage) = InfoActionHandlers::get_stage_number_input()? {
    ///     println!("Selected stage: {}", stage);
    /// }
    /// ```
    pub fn get_stage_number_input() -> Result<Option<u8>> {
        print!("Enter stage number (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u8>() {
            Ok(stage) if (1..=5).contains(&stage) => Ok(Some(stage)),
            _ => {
                println!("Invalid stage number. Must be 1-5.");
                Ok(None)
            }
        }
    }
    
    /// Validate ARM current input
    /// 
    /// Validates user input for ARM current values.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `Result<Option<u16>>` - Some(current) if valid, None if invalid
    /// 
    /// # Example
    /// ```
    /// if let Some(current) = InfoActionHandlers::validate_arm_current_input("500")? {
    ///     println!("Valid ARM current: {}mA", current);
    /// }
    /// ```
    pub fn validate_arm_current_input(input: &str) -> Result<Option<u16>> {
        match input.trim().parse::<u16>() {
            Ok(current) => {
                if current == 0 {
                    println!("Warning: ARM current is 0mA. This may affect device operation.");
                }
                Ok(Some(current))
            }
            Err(_) => {
                println!("Invalid input: '{}'. ARM current must be a whole number.", input.trim());
                Ok(None)
            }
        }
    }
}
