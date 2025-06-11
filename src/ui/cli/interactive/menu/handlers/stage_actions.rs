//! Stage action handlers for interactive CLI menu
//!
//! This module handles the execution of stage-related actions including
//! stage firing operations and custom current firing. It provides specialized
//! handlers for stage menu selections with proper error handling and user feedback.
//!
//! The stage action handlers system provides:
//! - Stage firing with validation and feedback
//! - Custom current firing with input validation
//! - Error handling and user-friendly messages
//! - Integration with device control operations

use crate::core::{Result, operations::{StageOperations, DeviceOperationData}};
use crate::device::LumidoxDevice;
use std::io::{self, Write};

/// Stage action handlers utilities and functionality
pub struct StageActionHandlers;

impl StageActionHandlers {
    /// Handle stage firing action
    /// 
    /// Executes a stage firing operation for the specified stage number
    /// with appropriate user feedback and error handling.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for firing operations
    /// * `stage` - Stage number to fire (1-5)
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = StageActionHandlers::handle_stage_firing(&mut device, 3)?;
    /// ```
    pub fn handle_stage_firing(device: &mut LumidoxDevice, stage: u8) -> Result<bool> {
        if !(1..=5).contains(&stage) {
            println!();
            println!("Invalid stage number: {}. Must be 1-5.", stage);
            println!();
            return Ok(true);
        }
        
        println!();
        println!("Firing stage {}.", stage);
        println!();

        // Use unified operation layer
        match StageOperations::fire_stage_unified(device, stage) {
            Ok(response) => {
                // CLI-specific presentation of the unified result
                println!("{}", response.message);
                if let DeviceOperationData::StageFiring { current_ma, .. } = response.data {
                    if let Some(current) = current_ma {
                        println!("Current used: {}mA", current);
                    }
                }
                println!();
            }
            Err(e) => {
                println!("Error firing stage {}: {}", stage, e);
                println!();
            }
        }
        
        Ok(true)
    }
    
    /// Handle custom current firing action
    /// 
    /// Prompts user for current input and executes custom current firing
    /// with input validation and error handling.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for firing operations
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = StageActionHandlers::handle_custom_current_firing(&mut device)?;
    /// ```
    pub fn handle_custom_current_firing(device: &mut LumidoxDevice) -> Result<bool> {
        println!();
        print!("Please enter current in mA (no decimals), then press ENTER: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let current_str = input.trim();
        
        match current_str.parse::<u16>() {
            Ok(current) => {
                println!();
                println!("Firing with {}mA.", current);
                println!();
                
                match device.fire_with_current(current) {
                    Ok(_) => {
                        println!("Fired with {}mA successfully.", current);
                        println!();
                    }
                    Err(e) => {
                        println!("Error firing with {}mA: {}", current, e);
                        println!("Aborting action.");
                        println!();
                    }
                }
            }
            Err(_) => {
                println!();
                println!("Invalid input. Current must be a number (no decimals).");
                println!("Aborting action.");
                println!();
            }
        }
        
        Ok(true)
    }
    
    /// Handle stage action based on choice
    /// 
    /// Routes stage-related menu choices to appropriate handlers.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for operations
    /// * `choice` - User menu choice string
    /// 
    /// # Returns
    /// * `Result<Option<bool>>` - Some(bool) if handled, None if not a stage choice
    /// 
    /// # Example
    /// ```
    /// if let Some(continue_menu) = StageActionHandlers::handle_stage_choice(&mut device, "3")? {
    ///     // Stage choice was handled
    /// }
    /// ```
    pub fn handle_stage_choice(device: &mut LumidoxDevice, choice: &str) -> Result<Option<bool>> {
        match choice {
            "1" | "2" | "3" | "4" | "5" => {
                let stage = choice.parse::<u8>().unwrap();
                Ok(Some(Self::handle_stage_firing(device, stage)?))
            }
            "6" => {
                Ok(Some(Self::handle_custom_current_firing(device)?))
            }
            _ => Ok(None)
        }
    }
    
    /// Validate current input
    /// 
    /// Validates user input for current values with appropriate error messages.
    /// 
    /// # Arguments
    /// * `input` - User input string
    /// 
    /// # Returns
    /// * `Result<Option<u16>>` - Some(current) if valid, None if invalid
    /// 
    /// # Example
    /// ```
    /// if let Some(current) = StageActionHandlers::validate_current_input("500")? {
    ///     println!("Valid current: {}mA", current);
    /// }
    /// ```
    pub fn validate_current_input(input: &str) -> Result<Option<u16>> {
        match input.trim().parse::<u16>() {
            Ok(current) => {
                if current == 0 {
                    println!("Warning: Current value is 0mA. This may not produce any output.");
                }
                Ok(Some(current))
            }
            Err(_) => {
                println!("Invalid input: '{}'. Current must be a whole number (no decimals).", input.trim());
                Ok(None)
            }
        }
    }
    
    /// Get current input from user
    /// 
    /// Prompts user for current input with validation and retry logic.
    /// 
    /// # Arguments
    /// * `max_attempts` - Maximum number of input attempts
    /// 
    /// # Returns
    /// * `Result<Option<u16>>` - Some(current) if valid input received, None if max attempts reached
    /// 
    /// # Example
    /// ```
    /// if let Some(current) = StageActionHandlers::get_current_input(3)? {
    ///     println!("User entered: {}mA", current);
    /// }
    /// ```
    pub fn get_current_input(max_attempts: u8) -> Result<Option<u16>> {
        for attempt in 1..=max_attempts {
            print!("Please enter current in mA (no decimals), then press ENTER: ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if let Some(current) = Self::validate_current_input(&input)? {
                return Ok(Some(current));
            }
            
            if attempt < max_attempts {
                println!("Please try again. ({}/{} attempts)", attempt, max_attempts);
            }
        }
        
        println!("Maximum attempts reached. Aborting action.");
        Ok(None)
    }
    
    /// Handle custom current firing with retry logic
    /// 
    /// Enhanced version of custom current firing with multiple input attempts.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device for firing operations
    /// * `max_attempts` - Maximum number of input attempts
    /// 
    /// # Returns
    /// * `Result<bool>` - True to continue menu loop, false to exit
    /// 
    /// # Example
    /// ```
    /// let continue_menu = StageActionHandlers::handle_custom_current_firing_with_retry(&mut device, 3)?;
    /// ```
    pub fn handle_custom_current_firing_with_retry(device: &mut LumidoxDevice, max_attempts: u8) -> Result<bool> {
        println!();
        
        if let Some(current) = Self::get_current_input(max_attempts)? {
            println!();
            println!("Firing with {}mA.", current);
            println!();
            
            match device.fire_with_current(current) {
                Ok(_) => {
                    println!("Fired with {}mA successfully.", current);
                    println!();
                }
                Err(e) => {
                    println!("Error firing with {}mA: {}", current, e);
                    println!("Aborting action.");
                    println!();
                }
            }
        } else {
            println!();
        }
        
        Ok(true)
    }
    
    /// Display stage firing confirmation
    /// 
    /// Shows confirmation message before executing stage firing.
    /// 
    /// # Arguments
    /// * `stage` - Stage number to fire
    /// * `device` - Reference to device for power information
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if power information cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// StageActionHandlers::display_stage_firing_confirmation(3, &device)?;
    /// ```
    pub fn display_stage_firing_confirmation(stage: u8, device: &mut LumidoxDevice) -> Result<()> {
        println!();
        println!("Preparing to fire stage {}.", stage);
        
        if let Ok(power_info) = device.get_power_info(stage) {
            println!("Stage {} power: {} {}, {} {}", 
                stage, power_info.total_power, power_info.total_units, 
                power_info.per_power, power_info.per_units);
        }
        
        Ok(())
    }
    
    /// Display custom current firing confirmation
    /// 
    /// Shows confirmation message before executing custom current firing.
    /// 
    /// # Arguments
    /// * `current` - Current value in mA
    /// * `device` - Reference to device for maximum current check
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if device information cannot be retrieved
    /// 
    /// # Example
    /// ```
    /// StageActionHandlers::display_custom_current_confirmation(500, &device)?;
    /// ```
    pub fn display_custom_current_confirmation(current: u16, device: &mut LumidoxDevice) -> Result<()> {
        println!();
        println!("Preparing to fire with {}mA.", current);
        
        if let Ok(max_current) = device.get_max_current() {
            if current > max_current {
                println!("Warning: Requested current ({}mA) exceeds maximum ({}mA).", current, max_current);
            } else {
                println!("Current is within device limits (max: {}mA).", max_current);
            }
        }
        
        Ok(())
    }
}
