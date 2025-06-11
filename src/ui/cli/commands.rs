//! Command execution logic for Lumidox II Controller CLI
//!
//! This module handles the execution of specific commands in non-interactive mode,
//! providing direct command-line access to device operations.

use crate::core::Result;
use crate::communication::{PortDetector, PortDetectionConfig, BaudDetector, BaudDetectionConfig, AutoConnector};
use super::{args::Commands, device::create_device_controller_with_optimization};

/// Run a specific command in non-interactive mode
pub fn run_command_mode(command: Commands, port_name: String) -> Result<()> {
    run_command_mode_with_optimization(command, port_name, true)
}

/// Run a specific command in non-interactive mode with specified optimization setting
pub fn run_command_mode_with_optimization(command: Commands, port_name: String, optimize_transitions: bool) -> Result<()> {
    let mut device = create_device_controller_with_optimization(&port_name, optimize_transitions)?;

    match command {
        Commands::Stage1 => {
            println!("Firing stage 1.");
            device.fire_stage(1)?
        }
        Commands::Stage2 => {
            println!("Firing stage 2.");
            device.fire_stage(2)?
        }
        Commands::Stage3 => {
            println!("Firing stage 3.");
            device.fire_stage(3)?
        }
        Commands::Stage4 => {
            println!("Firing stage 4.");
            device.fire_stage(4)?
        }
        Commands::Stage5 => {
            println!("Firing stage 5.");
            device.fire_stage(5)?
        }
        Commands::Current { value } => {
            println!("Firing with {}mA.", value);
            device.fire_with_current(value)?
        }
        Commands::Arm => {
            println!("Arming device.");
            device.arm()?
        }
        Commands::Off => {
            println!("Turning off device.");
            device.turn_off()?
        }
        Commands::Info => {
            if let Some(info) = device.info() {
                println!("Controller Firmware Version: {}", info.firmware_version);
                println!("Device Model Number: {}", info.model_number);
                println!("Device Serial Number: {}", info.serial_number);
                println!("Device Wavelength: {}", info.wavelength);
            } else {
                println!("Device information not available");
            }
        }
        Commands::Status => {
            println!("Reading device status...");
            // Read device state
            match device.read_device_state() {
                Ok(state_desc) => println!("Device State: {}", state_desc),
                Err(e) => println!("Error reading device state: {}", e),
            }
            // Read current settings
            match device.read_current_settings() {
                Ok(current_summary) => println!("Current Settings: {}", current_summary),
                Err(e) => println!("Error reading current settings: {}", e),
            }
        }
        Commands::ReadState => {
            println!("Reading remote mode state...");
            match device.read_remote_mode() {
                Ok(mode) => println!("Remote Mode State: {:?}", mode),
                Err(e) => println!("Error reading remote mode state: {}", e),
            }
        }
        Commands::ReadArmCurrent => {
            println!("Reading ARM current setting...");
            match device.read_arm_current() {
                Ok(current) => println!("ARM Current: {}mA", current),
                Err(e) => println!("Error reading ARM current: {}", e),
            }
        }
        Commands::ReadFireCurrent => {
            println!("Reading FIRE current setting...");
            match device.read_fire_current() {
                Ok(current) => println!("FIRE Current: {}mA", current),
                Err(e) => println!("Error reading FIRE current: {}", e),
            }
        }
        Commands::SetArmCurrent { value } => {
            println!("Setting ARM current to {}mA...", value);
            match device.set_arm_current(value) {
                Ok(()) => println!("ARM current set successfully."),
                Err(e) => println!("Error setting ARM current: {}", e),
            }
        }
        Commands::StageInfo { stage } => {
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
        Commands::StageArm { stage } => {
            println!("Reading ARM current for stage {}...", stage);
            match device.get_stage_arm_current(stage) {
                Ok(current) => println!("Stage {} ARM Current: {}mA", stage, current),
                Err(e) => println!("Error reading stage ARM current: {}", e),
            }
        }
        Commands::StageVoltages { stage } => {
            println!("Reading voltage parameters for stage {}...", stage);
            match device.get_stage_volt_limit(stage) {
                Ok(limit) => println!("Stage {} Voltage Limit: {:.1}V", stage, limit),
                Err(e) => println!("Error reading voltage limit: {}", e),
            }
            match device.get_stage_volt_start(stage) {
                Ok(start) => println!("Stage {} Voltage Start: {:.1}V", stage, start),
                Err(e) => println!("Error reading voltage start: {}", e),
            }
        }
        Commands::ListPorts => unreachable!(),
        Commands::DetectPorts => {
            println!("Detecting compatible Lumidox II Controller ports...");
            let config = PortDetectionConfig::default();
            match PortDetector::detect_ports(&config) {
                Ok(candidates) => {
                    if candidates.is_empty() {
                        println!("No compatible ports found.");
                    } else {
                        println!("Found {} compatible port(s):", candidates.len());
                        for (index, candidate) in candidates.iter().enumerate() {
                            println!("{}. {} - {} (Score: {})",
                                index + 1,
                                candidate.port_info.port_name,
                                candidate.score_reason,
                                candidate.compatibility_score);

                            if let Some(details) = &candidate.device_details {
                                if let Some(fw) = &details.firmware_version {
                                    println!("   Firmware: {}", fw);
                                }
                                if let Some(model) = &details.model_number {
                                    println!("   Model: {}", model);
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("Error detecting ports: {}", e),
            }
        }
        Commands::TestBaud { port } => {
            println!("Testing baud rates on port {}...", port);
            let config = BaudDetectionConfig::default();
            match BaudDetector::test_all_baud_rates(&port, &config) {
                Ok(results) => {
                    println!("Baud rate test results:");
                    for result in results {
                        let status = if result.success { "✓" } else { "✗" };
                        println!("{} {} baud - Score: {} ({}/{})",
                            status,
                            result.baud_rate,
                            result.quality_score,
                            result.successful_responses,
                            result.total_attempts);

                        if let Some(info) = &result.device_info {
                            if let Some(fw) = &info.firmware_version {
                                println!("    Firmware: {}", fw);
                            }
                        }
                    }
                }
                Err(e) => println!("Error testing baud rates: {}", e),
            }
        }
        Commands::PortDiagnostics => {
            println!("Running port diagnostics...");
            match AutoConnector::get_port_diagnostics() {
                Ok(diagnostics) => {
                    for line in diagnostics {
                        println!("{}", line);
                    }
                }
                Err(e) => println!("Error running diagnostics: {}", e),
            }
        }
    }

    Ok(())
}
