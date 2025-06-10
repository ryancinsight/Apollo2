//! CLI sub-module for Lumidox II Controller
//!
//! This module organizes CLI functionality into logical components:
//! - args: Command-line argument parsing and definitions
//! - ports: Serial port management and selection
//! - interactive: Interactive menu and user interaction
//! - commands: Command execution logic
//! - device: Device controller creation and management

pub mod args;
pub mod ports;
pub mod interactive;
pub mod commands;
pub mod device;

// Re-export commonly used items for convenience
pub use args::{Cli, Commands};
pub use ports::list_serial_ports;
pub use interactive::run_interactive_mode;
pub use commands::run_command_mode;
