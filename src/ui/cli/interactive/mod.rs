//! Interactive CLI sub-module for Lumidox II Controller
//!
//! This module provides a comprehensive interactive menu system organized into
//! a deep hierarchical structure for better maintainability and single responsibility:
//!
//! ## Module Structure (6+ levels deep)
//! ```
//! src/ui/cli/interactive/
//! ├── menu/                           (Level 4)
//! │   ├── display/                    (Level 5)
//! │   │   ├── stage_options.rs        (Level 6) - Stage menu display
//! │   │   ├── status_options.rs       (Level 6) - Status menu display
//! │   │   └── mod.rs                  (Level 6) - Display coordination
//! │   ├── handlers/                   (Level 5)
//! │   │   ├── stage_actions.rs        (Level 6) - Stage action handlers
//! │   │   ├── device_actions.rs       (Level 6) - Device control handlers
//! │   │   ├── info_actions.rs         (Level 6) - Information handlers
//! │   │   └── mod.rs                  (Level 6) - Handler coordination
//! │   └── mod.rs                      (Level 5) - Menu system coordination
//! ├── input/                          (Level 4)
//! │   ├── validation.rs               (Level 5) - Input validation utilities
//! │   ├── parsing.rs                  (Level 5) - Input parsing utilities
//! │   └── mod.rs                      (Level 5) - Input processing coordination
//! └── mod.rs                          (Level 4) - Interactive system coordination
//! ```
//!
//! ## System Architecture
//! The interactive system provides:
//! - **Menu System**: Organized display and action handling
//! - **Input Processing**: Validation, parsing, and error handling
//! - **Action Coordination**: Routing and execution of user commands
//! - **Error Handling**: Comprehensive error management and user feedback
//! - **Device Integration**: Seamless integration with device operations

pub mod menu;
pub mod input;

// Re-export commonly used items for convenience
pub use menu::{MenuSystem, MenuDisplay, MenuActionHandlers};
pub use input::{InputProcessor, InputValidator, InputParser, MenuChoice};

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::device::create_device_controller_with_fallback;

/// Interactive CLI system coordination utilities and functionality
pub struct InteractiveSystem;

impl InteractiveSystem {
    /// Run interactive mode with device connection
    /// 
    /// Establishes device connection and runs the interactive menu system
    /// with comprehensive error handling and user feedback.
    /// 
    /// # Arguments
    /// * `port_name` - Optional specific port name to connect to
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `optimize_transitions` - Whether to optimize device state transitions
    /// * `verbose` - Whether to enable verbose output
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error during interactive operation
    /// 
    /// # Example
    /// ```
    /// InteractiveSystem::run_interactive_mode(None, true, true, false)?;
    /// ```
    pub fn run_interactive_mode(
        port_name: Option<String>,
        auto_detect: bool,
        optimize_transitions: bool,
        verbose: bool
    ) -> Result<()> {
        // Establish device connection
        let mut device = create_device_controller_with_fallback(
            port_name,
            auto_detect,
            optimize_transitions,
            verbose
        )?;

        println!("Device connected successfully!");
        
        // Display device information
        Self::display_device_info(&device)?;
        
        // Run the interactive menu system
        MenuSystem::run_menu_loop(&mut device)?;
        
        Ok(())
    }
    
    /// Display device information header
    /// 
    /// Shows device information including firmware version, model, serial number,
    /// and wavelength if available.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device for information queries
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error during information display
    /// 
    /// # Example
    /// ```
    /// InteractiveSystem::display_device_info(&device)?;
    /// ```
    pub fn display_device_info(device: &LumidoxDevice) -> Result<()> {
        println!("--------------------------------------");
        
        if let Some(info) = device.info() {
            println!("Controller Firmware Version: {}", info.firmware_version);
            println!("Device Model Number: {}", info.model_number);
            println!("Device Serial Number: {}", info.serial_number);
            println!("Device Wavelength: {}", info.wavelength);
        } else {
            println!("Device information not available");
        }
        
        println!();
        Ok(())
    }
    
    /// Run enhanced interactive mode with retry logic
    /// 
    /// Enhanced version with retry logic for connection failures and
    /// improved error handling throughout the interactive session.
    /// 
    /// # Arguments
    /// * `port_name` - Optional specific port name to connect to
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `optimize_transitions` - Whether to optimize device state transitions
    /// * `verbose` - Whether to enable verbose output
    /// * `max_input_attempts` - Maximum attempts for invalid input
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error during interactive operation
    /// 
    /// # Example
    /// ```
    /// InteractiveSystem::run_enhanced_interactive_mode(None, true, true, false, 3)?;
    /// ```
    pub fn run_enhanced_interactive_mode(
        port_name: Option<String>,
        auto_detect: bool,
        optimize_transitions: bool,
        verbose: bool,
        max_input_attempts: u8
    ) -> Result<()> {
        // Establish device connection
        let mut device = create_device_controller_with_fallback(
            port_name,
            auto_detect,
            optimize_transitions,
            verbose
        )?;

        println!("Device connected successfully!");
        
        // Display device information
        Self::display_device_info(&device)?;
        
        // Run the enhanced interactive menu system
        MenuSystem::run_enhanced_menu_loop(&mut device, max_input_attempts)?;
        
        Ok(())
    }
    
    /// Validate interactive system integrity
    /// 
    /// Validates that all interactive system components are properly
    /// configured and accessible.
    /// 
    /// # Returns
    /// * `Result<()>` - Success if interactive system is valid
    /// 
    /// # Example
    /// ```
    /// InteractiveSystem::validate_system()?;
    /// ```
    pub fn validate_system() -> Result<()> {
        // Validate menu system
        MenuSystem::validate_system()?;
        
        // Additional system validation could be added here
        
        Ok(())
    }
    
    /// Get interactive system statistics
    /// 
    /// Returns comprehensive statistics about the interactive system configuration.
    /// 
    /// # Returns
    /// * `InteractiveStatistics` - Interactive system statistics
    /// 
    /// # Example
    /// ```
    /// let stats = InteractiveSystem::get_statistics();
    /// println!("Menu options: {}", stats.menu_stats.total_options);
    /// ```
    pub fn get_statistics() -> InteractiveStatistics {
        let menu_stats = MenuSystem::get_statistics();
        
        InteractiveStatistics {
            menu_stats,
            module_depth: 6, // Current maximum module depth
            total_modules: 12, // Approximate count of modules in hierarchy
        }
    }
    
    /// Run interactive mode with connection retry
    /// 
    /// Attempts to establish device connection with retry logic before
    /// starting the interactive session.
    /// 
    /// # Arguments
    /// * `port_name` - Optional specific port name to connect to
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `optimize_transitions` - Whether to optimize device state transitions
    /// * `verbose` - Whether to enable verbose output
    /// * `max_connection_attempts` - Maximum connection retry attempts
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error during interactive operation
    /// 
    /// # Example
    /// ```
    /// InteractiveSystem::run_with_connection_retry(None, true, true, false, 3)?;
    /// ```
    pub fn run_with_connection_retry(
        port_name: Option<String>,
        auto_detect: bool,
        optimize_transitions: bool,
        verbose: bool,
        max_connection_attempts: u8
    ) -> Result<()> {
        let mut last_error = None;
        
        for attempt in 1..=max_connection_attempts {
            match create_device_controller_with_fallback(
                port_name.clone(),
                auto_detect,
                optimize_transitions,
                verbose
            ) {
                Ok(mut device) => {
                    println!("Device connected successfully on attempt {}!", attempt);
                    Self::display_device_info(&device)?;
                    return MenuSystem::run_menu_loop(&mut device);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_connection_attempts {
                        println!("Connection attempt {} failed. Retrying...", attempt);
                    }
                }
            }
        }
        
        if let Some(error) = last_error {
            Err(error)
        } else {
            Err(crate::core::LumidoxError::DeviceError(
                "Failed to establish device connection after all attempts".to_string()
            ))
        }
    }
}

/// Interactive system statistics
#[derive(Debug, Clone)]
pub struct InteractiveStatistics {
    /// Menu system statistics
    pub menu_stats: menu::MenuStatistics,
    /// Maximum module hierarchy depth
    pub module_depth: usize,
    /// Total number of modules in hierarchy
    pub total_modules: usize,
}

/// Legacy function for backward compatibility
/// 
/// Provides backward compatibility with the original interactive_menu function.
/// This function is deprecated and will be removed in future versions.
/// Use `MenuSystem::run_menu_loop` instead.
/// 
/// # Arguments
/// * `device` - Mutable reference to the device for operations
/// 
/// # Returns
/// * `Result<bool>` - True to continue, false to exit (for compatibility)
/// 
/// # Example
/// ```
/// let continue_menu = interactive_menu(&mut device)?;
/// ```
#[deprecated(note = "Use MenuSystem::run_menu_loop instead")]
pub fn interactive_menu(device: &mut LumidoxDevice) -> Result<bool> {
    match MenuSystem::display_and_get_choice(device) {
        Ok(choice) => MenuSystem::execute_choice(device, choice),
        Err(e) => {
            InputProcessor::display_input_error(&e);
            Ok(true)
        }
    }
}

/// Legacy function for backward compatibility
/// 
/// Provides backward compatibility with the original run_interactive_mode function.
/// This function is deprecated and will be removed in future versions.
/// Use `InteractiveSystem::run_interactive_mode` instead.
/// 
/// # Arguments
/// * `port_name` - Optional specific port name to connect to
/// 
/// # Returns
/// * `Result<()>` - Success or error during interactive operation
/// 
/// # Example
/// ```
/// run_interactive_mode(Some("COM3".to_string()))?;
/// ```
#[deprecated(note = "Use InteractiveSystem::run_interactive_mode instead")]
pub fn run_interactive_mode(port_name: Option<String>) -> Result<()> {
    InteractiveSystem::run_interactive_mode(port_name, false, false, true)
}

/// Legacy function for backward compatibility
/// 
/// Provides backward compatibility with the original run_interactive_mode_with_optimization function.
/// This function is deprecated and will be removed in future versions.
/// Use `InteractiveSystem::run_interactive_mode` instead.
/// 
/// # Arguments
/// * `port_name` - Optional specific port name to connect to
/// * `auto_detect` - Whether to use automatic port detection
/// * `verbose` - Whether to enable verbose output
/// * `optimize_transitions` - Whether to optimize device state transitions
/// 
/// # Returns
/// * `Result<()>` - Success or error during interactive operation
/// 
/// # Example
/// ```
/// run_interactive_mode_with_optimization(None, true, false, true)?;
/// ```
#[deprecated(note = "Use InteractiveSystem::run_interactive_mode instead")]
pub fn run_interactive_mode_with_optimization(
    port_name: Option<String>,
    auto_detect: bool,
    verbose: bool,
    optimize_transitions: bool
) -> Result<()> {
    InteractiveSystem::run_interactive_mode(port_name, auto_detect, optimize_transitions, verbose)
}
