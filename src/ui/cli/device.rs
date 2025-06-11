//! Device controller creation for Lumidox II Controller CLI
//!
//! This module handles device controller creation and initialization
//! for CLI operations with support for automated port detection,
//! baud rate detection, and manual configuration.

use crate::core::{LumidoxError, Result};
use crate::communication::{ProtocolHandler, protocol::constants, AutoConnector, AutoConnectConfig};
use crate::device::LumidoxDevice;

/// Create a new device controller from a port name
pub fn create_device_controller(port_name: &str) -> Result<LumidoxDevice> {
    create_device_controller_with_optimization(port_name, true)
}

/// Create a new device controller from a port name with specified optimization setting
pub fn create_device_controller_with_optimization(port_name: &str, optimize_transitions: bool) -> Result<LumidoxDevice> {
    let port = serialport::new(port_name, constants::DEFAULT_BAUD_RATE)
        .timeout(constants::DEFAULT_TIMEOUT)
        .open()
        .map_err(LumidoxError::SerialError)?;

    let protocol = ProtocolHandler::new(port)?;
    let mut device = LumidoxDevice::new_with_optimization(protocol, optimize_transitions);
    device.initialize()?;

    Ok(device)
}

/// Create a device controller using automated detection
pub fn create_device_controller_auto(optimize_transitions: bool, verbose: bool) -> Result<LumidoxDevice> {
    let mut config = if verbose {
        AutoConnector::thorough_config()
    } else {
        AutoConnector::quick_config()
    };

    config.verbose = verbose;

    if verbose {
        println!("Starting automated Lumidox II Controller detection...");
    }

    let (mut device, result) = AutoConnector::auto_connect(&config)?;

    // Set optimization setting
    device.set_optimize_transitions(optimize_transitions);

    if verbose {
        println!("Successfully connected to {} at {} baud using {} method",
            result.port_name.unwrap_or_else(|| "unknown".to_string()),
            result.baud_rate.unwrap_or(0),
            match result.connection_method {
                crate::communication::ConnectionMethod::AutoDetected => "auto-detection",
                crate::communication::ConnectionMethod::Cached => "cached settings",
                crate::communication::ConnectionMethod::Manual => "manual configuration",
                crate::communication::ConnectionMethod::Fallback => "fallback",
            });

        if let Some(info) = &result.device_info {
            println!("Device: {} v{} (S/N: {})",
                info.model_number, info.firmware_version, info.serial_number);
        }

        println!("Connection time: {:.2}s", result.connection_time.as_secs_f32());
    }

    Ok(device)
}

/// Create a device controller with fallback from auto to manual
pub fn create_device_controller_with_fallback(
    port_name: Option<String>,
    auto_detect: bool,
    optimize_transitions: bool,
    verbose: bool
) -> Result<LumidoxDevice> {
    // Try auto-detection first if requested
    if auto_detect {
        match create_device_controller_auto(optimize_transitions, verbose) {
            Ok(device) => return Ok(device),
            Err(e) => {
                if verbose {
                    println!("Auto-detection failed: {}", e);
                    println!("Falling back to manual port selection...");
                }
            }
        }
    }

    // Use manual port selection
    let port = if let Some(port_name) = port_name {
        port_name
    } else {
        // If no port specified and auto-detection failed, get user input
        crate::ui::cli::ports::get_user_port_selection()?
    };

    create_device_controller_with_optimization(&port, optimize_transitions)
}
