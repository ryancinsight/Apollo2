//! CLI sub-module for Lumidox II Controller
//!
//! This module organizes CLI functionality into logical components:
//! - args: Command-line argument parsing and definitions
//! - ports: Serial port management and selection
//! - interactive: Interactive menu system with hierarchical sub-modules
//!   - menu: Menu display and organization logic
//!   - input: User input processing and validation
//!   - handlers: Action handlers for menu options
//!   - runners: Application execution and lifecycle management
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
pub use interactive::{run_interactive_mode, run_interactive_mode_with_optimization};
pub use commands::{run_command_mode, run_command_mode_with_optimization};
