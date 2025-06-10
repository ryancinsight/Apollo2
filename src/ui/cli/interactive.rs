//! Interactive menu functionality for Lumidox II Controller CLI
//!
//! This module handles the interactive menu system, user input processing,
//! and device information display for interactive mode operations.

use std::io::{self, Write};
use std::time::Duration;
use std::thread;

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::{device::create_device_controller, ports::welcome_message};

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
    let port_name = if let Some(port) = port_name {
        port
    } else {
        welcome_message()?
    };

    let mut device = create_device_controller(&port_name)?;
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
