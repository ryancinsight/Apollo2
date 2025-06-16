//! Power debugging CLI commands
//!
//! This module provides CLI commands for debugging power value display
//! inconsistencies between CLI and GUI interfaces using the unified
//! power operations system.

use crate::core::{LumidoxError, Result};
use crate::core::operations::power::{UnifiedPowerOperations, PowerDebugOperations, PowerUnit};

use crate::ui::cli::device::create_device_controller_with_fallback;

/// Power debugging CLI commands
pub struct PowerDebugCommands;

impl PowerDebugCommands {
    /// Run comprehensive power debugging analysis
    /// 
    /// Executes a complete diagnostic analysis to identify the root cause
    /// of power value display inconsistencies between CLI and GUI.
    /// 
    /// # Arguments
    /// * `port_name` - Optional port name for device connection
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if analysis fails
    pub fn run_comprehensive_analysis(port_name: Option<String>) -> Result<()> {
        println!("=== LUMIDOX II POWER DEBUGGING TOOL ===\n");
        
        // Create device connection
        let mut device = match port_name {
            Some(port) => create_device_controller_with_fallback(Some(port), true, true, false)?,
            None => create_device_controller_with_fallback(None, true, true, false)?,
        };
        
        println!("Device connected successfully. Starting analysis...\n");
        
        // Run comprehensive debugging analysis
        PowerDebugOperations::display_cli_debugging_report(&mut device)?;
        
        Ok(())
    }
    
    /// Quick hardcoding check
    /// 
    /// Performs a quick check to determine if the device is returning
    /// hardcoded values for all stages.
    /// 
    /// # Arguments
    /// * `port_name` - Optional port name for device connection
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if check fails
    pub fn quick_hardcoding_check(port_name: Option<String>) -> Result<()> {
        println!("=== QUICK HARDCODING CHECK ===\n");
        
        // Create device connection
        let mut device = match port_name {
            Some(port) => create_device_controller_with_fallback(Some(port), true, true, false)?,
            None => create_device_controller_with_fallback(None, true, true, false)?,
        };
        
        println!("Checking for hardcoded power values...");
        
        match PowerDebugOperations::quick_hardcoding_check(&mut device)? {
            true => {
                println!("✗ HARDCODING DETECTED: All stages report identical power values");
                println!("This indicates a serious issue with device communication or smart card configuration.");
                println!("\nRecommendations:");
                println!("1. Check device smart card configuration");
                println!("2. Verify device calibration");
                println!("3. Test with different smart card if available");
            }
            false => {
                println!("✓ No hardcoding detected: Stages report different power values");
                println!("If GUI shows identical values, the issue is in GUI implementation.");
            }
        }
        
        Ok(())
    }
    
    /// Compare CLI vs GUI power values
    /// 
    /// Displays power values as they would appear in CLI for comparison
    /// with GUI display to identify discrepancies.
    /// 
    /// # Arguments
    /// * `port_name` - Optional port name for device connection
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if comparison fails
    pub fn compare_cli_gui_values(port_name: Option<String>) -> Result<()> {
        println!("=== CLI vs GUI POWER VALUE COMPARISON ===\n");
        
        // Create device connection
        let mut device = match port_name {
            Some(port) => create_device_controller_with_fallback(Some(port), true, true, false)?,
            None => create_device_controller_with_fallback(None, true, true, false)?,
        };
        
        println!("Retrieving power values using unified operations...\n");
        
        // Get power values for all stages using unified operations
        match UnifiedPowerOperations::get_all_stages_power_unified(&mut device, None) {
            Ok(response) => {
                if let crate::core::operations::DeviceOperationData::AllStagesPower { stages_data, .. } = response.data {
                    println!("CLI Power Values (using unified operations):");
                    println!("┌───────┬─────────────────────┬─────────────────────┬─────────────────────┐");
                    println!("│ Stage │    Total Power      │   Per-Well Power    │      Current        │");
                    println!("├───────┼─────────────────────┼─────────────────────┼─────────────────────┤");
                    
                    for stage_data in &stages_data {
                        let (total_power, total_units, per_power, per_units) = stage_data.get_display_values();
                        let current_display = stage_data.get_current_display();
                        
                        println!("│   {}   │ {:>8.1} {:>10} │ {:>8.1} {:>10} │ {:>19} │",
                            stage_data.stage_number,
                            total_power,
                            Self::truncate_units(&total_units),
                            per_power,
                            Self::truncate_units(&per_units),
                            current_display
                        );
                    }
                    
                    println!("└───────┴─────────────────────┴─────────────────────┴─────────────────────┘");
                    
                    // Check for identical values
                    let total_powers: Vec<f32> = stages_data.iter()
                        .map(|s| s.raw_power_info.total_power)
                        .collect();
                    
                    let unique_powers: std::collections::HashSet<_> = total_powers.iter()
                        .map(|&p| (p * 1000.0) as i32)
                        .collect();
                    
                    if unique_powers.len() == 1 && stages_data.len() > 1 {
                        println!("\n⚠ WARNING: All stages show identical power values!");
                        println!("This indicates hardcoded values or device communication issues.");
                    } else {
                        println!("\n✓ Power values show appropriate variation between stages.");
                        println!("If your GUI shows identical values, the issue is in GUI implementation.");
                    }
                    
                    println!("\nInstructions for GUI comparison:");
                    println!("1. Open the GUI interface");
                    println!("2. Navigate to power display section");
                    println!("3. Compare the values shown above with GUI display");
                    println!("4. If GUI shows identical values for stages 2-5, the GUI has hardcoded values");
                }
            }
            Err(e) => {
                println!("✗ Failed to retrieve power values: {}", e);
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    /// Test unit conversion system
    /// 
    /// Tests the unit conversion system to ensure mathematical conversions
    /// are working correctly vs just label changes.
    /// 
    /// # Arguments
    /// * `port_name` - Optional port name for device connection
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if test fails
    pub fn test_unit_conversion(port_name: Option<String>) -> Result<()> {
        println!("=== UNIT CONVERSION SYSTEM TEST ===\n");
        
        // Create device connection
        let mut device = match port_name {
            Some(port) => create_device_controller_with_fallback(Some(port), true, true, false)?,
            None => create_device_controller_with_fallback(None, true, true, false)?,
        };
        
        println!("Testing unit conversion for Stage 1...\n");
        
        // Test conversion from mW to W
        match UnifiedPowerOperations::get_stage_power_unified(&mut device, 1, Some(PowerUnit::Watts)) {
            Ok(response) => {
                if let crate::core::operations::DeviceOperationData::PowerMeasurement { power_data, .. } = response.data {
                    let raw_total = power_data.raw_power_info.total_power;
                    let converted_total = power_data.converted_data.total_power;
                    
                    println!("Raw value (mW):       {:.3}", raw_total);
                    println!("Converted value (W):  {:.6}", converted_total);
                    println!("Expected conversion:  {:.6}", raw_total * 0.001);
                    
                    let expected = raw_total * 0.001;
                    if (converted_total - expected).abs() < 0.000001 {
                        println!("✓ Unit conversion is working correctly (mathematical conversion)");
                    } else {
                        println!("✗ Unit conversion failed - values don't match expected calculation");
                        println!("This indicates the conversion is not mathematical or has errors");
                    }
                } else {
                    println!("✗ Unexpected response format");
                }
            }
            Err(e) => {
                println!("✗ Failed to test unit conversion: {}", e);
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    /// Truncate unit strings for table display
    fn truncate_units(units: &str) -> String {
        if units.len() > 10 {
            format!("{}...", &units[..7])
        } else {
            units.to_string()
        }
    }
    
    /// Display help for power debugging commands
    pub fn display_help() {
        println!("=== POWER DEBUGGING COMMANDS ===\n");
        println!("Available commands:");
        println!("  power-debug analysis [PORT]    - Run comprehensive power debugging analysis");
        println!("  power-debug quick [PORT]       - Quick check for hardcoded values");
        println!("  power-debug compare [PORT]     - Compare CLI vs GUI power values");
        println!("  power-debug units [PORT]       - Test unit conversion system");
        println!("  power-debug help               - Show this help message");
        println!("\nArguments:");
        println!("  PORT                           - Optional serial port name (auto-detected if not specified)");
        println!("\nExamples:");
        println!("  power-debug analysis");
        println!("  power-debug quick COM3");
        println!("  power-debug compare /dev/ttyUSB0");
        println!("\nDescription:");
        println!("These commands help diagnose power value display inconsistencies between");
        println!("CLI and GUI interfaces. Use 'analysis' for comprehensive debugging or");
        println!("'quick' for fast hardcoding detection.");
    }
}

/// Parse and execute power debugging commands
/// 
/// Parses command line arguments for power debugging and executes the
/// appropriate debugging function.
/// 
/// # Arguments
/// * `args` - Command line arguments
/// 
/// # Returns
/// * `Result<()>` - Success or error if command fails
pub fn execute_power_debug_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        PowerDebugCommands::display_help();
        return Ok(());
    }
    
    let command = &args[0];
    let port_name = args.get(1).cloned();
    
    match command.as_str() {
        "analysis" => PowerDebugCommands::run_comprehensive_analysis(port_name),
        "quick" => PowerDebugCommands::quick_hardcoding_check(port_name),
        "compare" => PowerDebugCommands::compare_cli_gui_values(port_name),
        "units" => PowerDebugCommands::test_unit_conversion(port_name),
        "help" => {
            PowerDebugCommands::display_help();
            Ok(())
        }
        _ => {
            println!("Unknown power debug command: {}", command);
            println!("Use 'power-debug help' for available commands.");
            Err(LumidoxError::InvalidInput(format!("Unknown command: {}", command)))
        }
    }
}
