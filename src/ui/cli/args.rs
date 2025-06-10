//! CLI argument parsing for Lumidox II Controller
//!
//! This module defines the command-line interface structure including
//! the main CLI arguments and all available commands.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lumidox-ii-controller")]
#[command(about = "Lumidox II Controller PC Application")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// COM port to connect to (e.g., COM3)
    #[arg(short, long)]
    pub port: Option<String>,

    /// Run in interactive mode (default)
    #[arg(short, long)]
    pub interactive: bool,

    /// Disable optimized stage transitions (always use full safety sequence)
    #[arg(long)]
    pub no_optimize: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Fire stage 1
    Stage1,
    /// Fire stage 2
    Stage2,
    /// Fire stage 3
    Stage3,
    /// Fire stage 4
    Stage4,
    /// Fire stage 5
    Stage5,
    /// Fire with specific current in mA
    Current {
        /// Current value in mA
        #[arg(value_name = "MILLIAMPS")]
        value: u16
    },
    /// Arm the device (prepare for firing)
    Arm,
    /// Turn off device
    Off,
    /// Show device information
    Info,
    /// List available COM ports
    ListPorts,
}
