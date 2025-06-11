//! Interactive menu functionality for Lumidox II Controller CLI
//!
//! This module handles the interactive menu system, user input processing,
//! and device information display for interactive mode operations.

use std::io::{self, Write};
use std::time::Duration;
use std::thread;

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::{device::create_device_controller_with_optimization, ports::welcome_message};

/// Display interactive menu and handle user input
pub fn interactive_menu(device: &mut LumidoxDevice) -> Result<bool> {
    loop {
        println!("-- Select an action --");
        
        // Display stage options with power info
        for stage in 1..=5 {
            if let Ok(power_info) = device.get_power_info(stage) {
                println!("{}) Turn on stage {}: {} {}, {} {}", 
                    stage, stage, power_info.total_power, power_info.total_units, 
                    power_info.per_power, power_info.per_units);
            } else {
                println!("{}) Turn on stage {}", stage, stage);
            }
        }
        
        if let Ok(max_current) = device.get_max_current() {
            println!("6) Turn on stage with specific current (up to {}mA).", max_current);
        } else {
            println!("6) Turn on stage with specific current.");
        }

        println!("7) Arm device (prepare for firing).");
        println!("8) Turn off device.");
        println!("9) Quit program.");
        println!();
        println!("--- Device Status & Information ---");
        println!("10) Show device status.");
        println!("11) Read remote mode state.");
        println!("12) Read ARM/FIRE current settings.");
        println!("--- Stage Parameter Information ---");
        println!("13) Show complete stage parameters.");
        println!("14) Read stage ARM current.");
        println!("15) Read stage voltage parameters.");
        println!("--- Current Control ---");
        println!("16) Set ARM current.");
        println!();
        
        print!("Please enter choice number, then press ENTER: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        match choice {
            "1" | "2" | "3" | "4" | "5" => {
                let stage = choice.parse::<u8>().unwrap();
                println!();
                println!("Firing stage {}.", stage);
                println!();
                device.fire_stage(stage)?;
                return Ok(true);
            }
            "6" => {
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
                            Ok(_) => return Ok(true),
                            Err(e) => {
                                println!("Error: {}. Aborting action.", e);
                                println!();
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        println!();
                        println!("Invalid input. Aborting action");
                        println!();
                        return Ok(true);
                    }
                }
            }
            "7" => {
                println!();
                println!("Arming device.");
                device.arm()?;
                println!("Device is now armed and ready for firing.");
                println!();
                return Ok(true);
            }
            "8" => {
                println!();
                println!("Turning off device.");
                device.turn_off()?;
                println!();
                return Ok(true);
            }
            "9" => {
                println!();
                println!("Turning off device.");
                device.shutdown()?;
                println!("To resume using the controller in local mode, please cycle the power with on/off switch.");
                thread::sleep(Duration::from_millis(1000));
                println!("Quitting program...");
                thread::sleep(Duration::from_millis(1000));
                println!();
                return Ok(false);
            }
            "10" => {
                println!();
                println!("Reading device status...");
                match device.read_device_state() {
                    Ok(state_desc) => println!("Device State: {}", state_desc),
                    Err(e) => println!("Error reading device state: {}", e),
                }
                match device.read_current_settings() {
                    Ok(current_summary) => println!("Current Settings: {}", current_summary),
                    Err(e) => println!("Error reading current settings: {}", e),
                }
                println!();
                return Ok(true);
            }
            "11" => {
                println!();
                println!("Reading remote mode state...");
                match device.read_remote_mode() {
                    Ok(mode) => println!("Remote Mode State: {:?}", mode),
                    Err(e) => println!("Error reading remote mode state: {}", e),
                }
                println!();
                return Ok(true);
            }
            "12" => {
                println!();
                println!("Reading current settings...");
                match device.read_arm_current() {
                    Ok(current) => println!("ARM Current: {}mA", current),
                    Err(e) => println!("Error reading ARM current: {}", e),
                }
                match device.read_fire_current() {
                    Ok(current) => println!("FIRE Current: {}mA", current),
                    Err(e) => println!("Error reading FIRE current: {}", e),
                }
                println!();
                return Ok(true);
            }
            "13" => {
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
                return Ok(true);
            }
            "14" => {
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
                return Ok(true);
            }
            "15" => {
                println!();
                print!("Enter stage number (1-5): ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                match input.trim().parse::<u8>() {
                    Ok(stage) if (1..=5).contains(&stage) => {
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
                    _ => println!("Invalid stage number. Must be 1-5."),
                }
                println!();
                return Ok(true);
            }
            "16" => {
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
                    Err(_) => println!("Invalid current value."),
                }
                println!();
                return Ok(true);
            }
            _ => {
                println!();
                println!("Not a valid choice. Please try again.");
                println!();
            }
        }
    }
}

/// Run the application in interactive mode
pub fn run_interactive_mode(port_name: Option<String>) -> Result<()> {
    run_interactive_mode_with_optimization(port_name, true)
}

/// Run the application in interactive mode with specified optimization setting
pub fn run_interactive_mode_with_optimization(port_name: Option<String>, optimize_transitions: bool) -> Result<()> {
    let port_name = if let Some(port) = port_name {
        port
    } else {
        welcome_message()?
    };

    let mut device = create_device_controller_with_optimization(&port_name, optimize_transitions)?;
    println!("{} has been connected!", port_name);

    println!("--------------------------------------");
    if let Some(info) = device.info() {
        println!("Controller Firmware Version: {}", info.firmware_version);
        println!("Device Model Number: {}", info.model_number);
        println!("Device Serial Number: {}", info.serial_number);
        println!("Device Wavelength: {}", info.wavelength);
    }
    println!();

    let mut continue_loop = true;
    while continue_loop {
        continue_loop = interactive_menu(&mut device)?;
    }

    Ok(())
}
