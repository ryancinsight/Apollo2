//! User Interface module for Lumidox II Controller
//!
//! This module contains all user interface components organized into sub-modules:
//! - `cli`: Command-line interface with organized sub-components

pub mod cli;

// Re-export commonly used items for convenience
pub use cli::{Cli, Commands, run_interactive_mode, run_command_mode, list_serial_ports};
