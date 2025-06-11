//! CLI argument parsing for Lumidox II Controller
//!
//! This module defines the command-line interface structure including
//! the main CLI arguments and all available commands.

use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "lumidox-ii-controller")]
#[command(about = "Lumidox II Controller PC Application")]
#[command(version = "1.0.0")]
#[command(long_about = "Lumidox II Controller PC Application\n\nCommand-line interface for direct command execution and interactive terminal interface.\nInterface selection is determined at compile time via Cargo features.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// COM port to connect to (e.g., COM3)
    #[arg(short, long)]
    pub port: Option<String>,

    /// Automatically detect COM port and baud rate
    #[arg(short, long)]
    pub auto: bool,

    /// Use verbose output during auto-detection
    #[arg(short, long)]
    pub verbose: bool,

    /// Run in interactive mode (default for CLI)
    #[arg(short, long)]
    pub interactive: bool,

    /// Disable optimized stage transitions (always use full safety sequence)
    #[arg(long)]
    pub no_optimize: bool,
}

#[derive(Subcommand, Clone)]
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
    /// Display current device status (state, currents, operational status)
    Status,
    /// Read and display current remote mode state
    ReadState,
    /// Read current ARM current setting
    ReadArmCurrent,
    /// Read current FIRE current setting
    ReadFireCurrent,
    /// Set ARM current value
    SetArmCurrent {
        /// ARM current value in mA
        #[arg(value_name = "MILLIAMPS")]
        value: u16
    },
    /// Display complete stage parameters (ARM current, FIRE current, voltages, power)
    StageInfo {
        /// Stage number (1-5)
        #[arg(value_name = "STAGE")]
        stage: u8
    },
    /// Read ARM current for specific stage
    StageArm {
        /// Stage number (1-5)
        #[arg(value_name = "STAGE")]
        stage: u8
    },
    /// Read voltage parameters for specific stage
    StageVoltages {
        /// Stage number (1-5)
        #[arg(value_name = "STAGE")]
        stage: u8
    },
    /// List available COM ports
    ListPorts,
    /// Detect compatible Lumidox II ports automatically
    DetectPorts,
    /// Test baud rates on a specific port
    TestBaud {
        /// Port name to test (e.g., COM3)
        #[arg(value_name = "PORT")]
        port: String
    },
    /// Show detailed port diagnostics and compatibility information
    PortDiagnostics,
}

impl Cli {
    /// Validate CLI arguments for logical consistency
    ///
    /// Checks for conflicting argument combinations and provides clear error messages
    /// for invalid usage patterns.
    ///
    /// # Panics
    ///
    /// This function will call `std::process::exit(1)` if invalid argument combinations
    /// are detected, providing clear error messages to guide the user.
    ///
    /// # Examples
    ///
    /// ```
    /// let cli = Cli::parse();
    /// cli.validate(); // Will exit with error if invalid combinations detected
    /// ```
    pub fn validate(&self) {
        // Check for logical conflicts
        if self.interactive && self.command.is_some() {
            eprintln!("Error: --interactive flag cannot be used with specific commands.");
            eprintln!("Interactive mode provides its own command interface.");
            eprintln!("Use either:");
            eprintln!("  --interactive            (for CLI interactive mode)");
            eprintln!("  <command> [options]      (for direct CLI command execution)");
            process::exit(1);
        }
    }

    /// Get the optimize transitions setting
    ///
    /// Returns true if optimized transitions should be used, false if the full
    /// safety sequence should always be used. This inverts the `no_optimize` flag
    /// to provide a positive boolean for easier use in the application logic.
    ///
    /// # Returns
    ///
    /// * `bool` - True if optimized transitions should be used, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// let cli = Cli::parse();
    /// if cli.optimize_transitions() {
    ///     println!("Using optimized stage transitions");
    /// } else {
    ///     println!("Using full safety sequence for all transitions");
    /// }
    /// ```
    pub fn optimize_transitions(&self) -> bool {
        !self.no_optimize
    }

    /// Check if the application should run in CLI interactive mode
    ///
    /// Returns true if interactive mode is explicitly requested or if no specific
    /// command is provided (default behavior for CLI mode).
    ///
    /// # Returns
    ///
    /// * `bool` - True if CLI interactive mode should be used
    ///
    /// # Examples
    ///
    /// ```
    /// let cli = Cli::parse();
    /// if cli.is_interactive_mode() {
    ///     run_interactive_cli(&cli);
    /// }
    /// ```
    pub fn is_interactive_mode(&self) -> bool {
        self.interactive || self.command.is_none()
    }

    /// Check if the application should run a specific CLI command
    ///
    /// Returns true if a specific command is provided.
    ///
    /// # Returns
    ///
    /// * `bool` - True if a specific CLI command should be executed
    ///
    /// # Examples
    ///
    /// ```
    /// let cli = Cli::parse();
    /// if cli.is_command_mode() {
    ///     run_cli_command(&cli);
    /// }
    /// ```
    pub fn is_command_mode(&self) -> bool {
        self.command.is_some()
    }

    /// Get usage mode description for logging and debugging
    ///
    /// Returns a human-readable string describing the current usage mode
    /// based on the provided arguments.
    ///
    /// # Returns
    ///
    /// * `&'static str` - Description of the current usage mode
    ///
    /// # Examples
    ///
    /// ```
    /// let cli = Cli::parse();
    /// println!("Running in {} mode", cli.get_mode_description());
    /// ```
    pub fn get_mode_description(&self) -> &'static str {
        if self.command.is_some() {
            "CLI Command"
        } else {
            "CLI Interactive"
        }
    }
}
