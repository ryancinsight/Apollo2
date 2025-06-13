//! Lumidox II Controller Library
//!
//! This library provides comprehensive control and communication capabilities
//! for Lumidox II Controller devices, including automated port detection,
//! baud rate detection, device control, and protocol handling.
//!
//! # Features
//!
//! - **Automated Connection**: Automatic COM port and baud rate detection
//! - **Device Control**: Complete device control including firing, arming, and configuration
//! - **Protocol Handling**: Low-level protocol communication and command processing
//! - **Error Handling**: Comprehensive error types and handling utilities
//! - **CLI Interface**: Command-line interface for interactive and batch operations
//!
//! # Quick Start
//!
//! ## Automatic Connection
//! ```no_run
//! use lumidox_ii_controller::communication::{AutoConnector, AutoConnectConfig};
//!
//! let config = AutoConnector::quick_config();
//! let (device, result) = AutoConnector::auto_connect(&config)?;
//! println!("Connected to {} at {} baud", 
//!     result.port_name.unwrap(), 
//!     result.baud_rate.unwrap());
//! # Ok::<(), lumidox_ii_controller::core::LumidoxError>(())
//! ```
//!
//! ## Manual Connection
//! ```no_run
//! use lumidox_ii_controller::{communication::ProtocolHandler, device::LumidoxDevice};
//! use serialport;
//!
//! let port = serialport::new("COM3", 19200)
//!     .timeout(std::time::Duration::from_millis(1000))
//!     .open()?;
//! let protocol = ProtocolHandler::new(port)?;
//! let mut device = LumidoxDevice::new(protocol);
//! device.initialize()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Device Control
//! ```no_run
//! # use lumidox_ii_controller::{communication::ProtocolHandler, device::LumidoxDevice};
//! # use serialport;
//! # let port = serialport::new("COM3", 19200).timeout(std::time::Duration::from_millis(1000)).open()?;
//! # let protocol = ProtocolHandler::new(port)?;
//! # let mut device = LumidoxDevice::new(protocol);
//! # device.initialize()?;
//! // Arm the device
//! device.arm()?;
//!
//! // Fire stage 1
//! device.fire_stage(1)?;
//!
//! // Fire with custom current
//! device.fire_with_current(500)?; // 500mA
//!
//! // Turn off device
//! device.turn_off()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Core functionality
pub mod core;

// Communication and protocol handling
pub mod communication;

// Device control and management
pub mod device;

// User interface components
pub mod ui;

// Re-export commonly used items for convenience
pub use core::{LumidoxError, Result};
pub use communication::{ProtocolHandler, AutoConnector};
pub use device::LumidoxDevice;
