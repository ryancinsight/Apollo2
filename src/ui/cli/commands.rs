//! Command execution logic for Lumidox II Controller CLI
//!
//! This module handles the execution of specific commands in non-interactive mode,
//! providing direct command-line access to device operations.

use crate::core::Result;
use super::{args::Commands, device::create_device_controller};

/// Run a specific command in non-interactive mode
pub fn run_command_mode(command: Commands, port_name: String) -> Result<()> {
    let mut device = create_device_controller(&port_name)?;

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
        Commands::ListPorts => unreachable!(),
    }

    Ok(())
}
