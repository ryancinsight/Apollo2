//! Serial port management for Lumidox II Controller CLI
//!
//! This module handles serial port discovery, listing, and user selection
//! with validation and user-friendly error messages.

use serialport::SerialPortType;
use std::io::{self, Write};
use crate::core::Result;

/// List available serial ports
pub fn list_serial_ports() -> Result<()> {
    let ports = serialport::available_ports()?;
    
    println!("Available COM ports:");
    for port in ports {
        match &port.port_type {
            SerialPortType::UsbPort(info) => {
                println!("  {}: USB Serial Port - {}", port.port_name, 
                    info.product.as_ref().unwrap_or(&"Unknown".to_string()));
            }
            _ => {
                println!("  {}: {}", port.port_name, 
                    match &port.port_type {
                        SerialPortType::PciPort => "PCI Port",
                        SerialPortType::BluetoothPort => "Bluetooth Port",
                        _ => "Unknown Port Type",
                    });
            }
        }
    }
    
    Ok(())
}

/// Get user port selection with validation
pub fn get_user_port_selection() -> Result<String> {
    list_serial_ports()?;
    println!();
    
    loop {
        print!("Please enter the COM port name (e.g., COM3): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let port_name = input.trim();
        
        if port_name.is_empty() {
            continue;
        }
        
        // Validate port exists
        let ports = serialport::available_ports()?;
        if ports.iter().any(|p| p.port_name == port_name) {
            return Ok(port_name.to_string());
        } else {
            println!("Port {} not found. Please try again.", port_name);
        }
    }
}

/// Display welcome message and get port selection
pub fn welcome_message() -> Result<String> {
    println!("Welcome to the Analytical Sales & Services, Inc. Lumidox II Controller PC App!");
    println!();
    println!("Before we get started please make sure to do the following:");
    println!("  * Have proper PPE for skin & eyes to protect from high powered LEDs.");
    println!("  * Ensure those around also have the same level of PPE.");
    println!("  * Connect a light device to the Lumidox II controller.");
    println!("  * Connect a USB cable from the PC to the Lumidox II controller.");
    println!("  * Connect the Lumidox II controller to AC mains with the power adapter.");
    println!("  * Power on the Lumidox II controller to show the main menu on it's display.");
    println!();
    
    print!("Press ENTER after the above is complete: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    get_user_port_selection()
}
