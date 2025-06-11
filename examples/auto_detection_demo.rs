//! Demonstration of automated COM port and baud rate detection
//!
//! This example shows how to use the new automated detection features
//! to automatically find and connect to Lumidox II Controller devices.

use lumidox_ii_controller::core::Result;
use lumidox_ii_controller::communication::{AutoConnector, AutoConnectConfig, PortDetector, PortDetectionConfig,
                   BaudDetector, BaudDetectionConfig};

fn main() -> Result<()> {
    println!("=== Lumidox II Controller Auto-Detection Demo ===\n");

    // Demo 1: Port Detection
    println!("1. Detecting compatible ports...");
    demo_port_detection()?;
    println!();

    // Demo 2: Baud Rate Detection (if ports are available)
    println!("2. Testing baud rate detection...");
    demo_baud_detection()?;
    println!();

    // Demo 3: Full Auto-Connection
    println!("3. Attempting full auto-connection...");
    demo_auto_connection()?;
    println!();

    // Demo 4: Port Diagnostics
    println!("4. Detailed port diagnostics...");
    demo_port_diagnostics()?;

    Ok(())
}

/// Demonstrate port detection functionality
fn demo_port_detection() -> Result<()> {
    let config = PortDetectionConfig::default();
    let candidates = PortDetector::detect_ports(&config)?;
    
    if candidates.is_empty() {
        println!("   No compatible ports found.");
        println!("   Make sure a Lumidox II Controller is connected via USB.");
    } else {
        println!("   Found {} compatible port(s):", candidates.len());
        for (index, candidate) in candidates.iter().enumerate() {
            println!("   {}. {} - {} (Score: {})", 
                index + 1,
                candidate.port_info.port_name,
                candidate.score_reason,
                candidate.compatibility_score);
            
            if let Some(details) = &candidate.device_details {
                if let Some(fw) = &details.firmware_version {
                    println!("      Firmware: {}", fw);
                }
                if let Some(model) = &details.model_number {
                    println!("      Model: {}", model);
                }
                if let Some(serial) = &details.serial_number {
                    println!("      Serial: {}", serial);
                }
            }
        }
    }
    
    Ok(())
}

/// Demonstrate baud rate detection functionality
fn demo_baud_detection() -> Result<()> {
    let port_config = PortDetectionConfig::default();
    let candidates = PortDetector::detect_ports(&port_config)?;
    
    if let Some(best_candidate) = candidates.first() {
        let port_name = &best_candidate.port_info.port_name;
        println!("   Testing baud rates on {}...", port_name);
        
        let baud_config = BaudDetectionConfig {
            test_baud_rates: vec![19200, 9600, 38400], // Quick test
            attempts_per_rate: 1,
            comprehensive_testing: false,
            ..Default::default()
        };
        
        match BaudDetector::test_all_baud_rates(port_name, &baud_config) {
            Ok(results) => {
                for result in results {
                    let status = if result.success { "✓" } else { "✗" };
                    println!("   {} {} baud - Score: {} ({}/{})", 
                        status,
                        result.baud_rate,
                        result.quality_score,
                        result.successful_responses,
                        result.total_attempts);
                }
            }
            Err(e) => {
                println!("   Error testing baud rates: {}", e);
            }
        }
    } else {
        println!("   No ports available for baud rate testing.");
    }
    
    Ok(())
}

/// Demonstrate full auto-connection functionality
fn demo_auto_connection() -> Result<()> {
    let config = AutoConnectConfig {
        verbose: true,
        ..AutoConnector::quick_config()
    };
    
    match AutoConnector::auto_connect(&config) {
        Ok((device, result)) => {
            println!("   ✓ Successfully connected!");
            println!("   Port: {}", result.port_name.unwrap_or_else(|| "unknown".to_string()));
            println!("   Baud Rate: {} baud", result.baud_rate.unwrap_or(0));
            println!("   Method: {:?}", result.connection_method);
            println!("   Connection Time: {:.2}s", result.connection_time.as_secs_f32());
            
            if let Some(info) = device.info() {
                println!("   Device Info:");
                println!("     Firmware: {}", info.firmware_version);
                println!("     Model: {}", info.model_number);
                println!("     Serial: {}", info.serial_number);
                println!("     Wavelength: {}", info.wavelength);
            }
        }
        Err(e) => {
            println!("   ✗ Auto-connection failed: {}", e);
            println!("   This is normal if no Lumidox II Controller is connected.");
        }
    }
    
    Ok(())
}

/// Demonstrate port diagnostics functionality
fn demo_port_diagnostics() -> Result<()> {
    match AutoConnector::get_port_diagnostics() {
        Ok(diagnostics) => {
            for line in diagnostics {
                println!("   {}", line);
            }
        }
        Err(e) => {
            println!("   Error getting diagnostics: {}", e);
        }
    }
    
    Ok(())
}
