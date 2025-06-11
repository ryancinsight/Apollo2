use clap::Parser;

// Import our custom modules
mod core;
mod communication;
mod device;
mod ui;

use core::{LumidoxError, Result};
use ui::{Cli, Commands, run_interactive_mode_with_optimization, run_command_mode_with_optimization, list_serial_ports};





fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine optimization setting (enabled by default, disabled with --no-optimize)
    let optimize_transitions = !cli.no_optimize;

    match cli.command {
        Some(Commands::ListPorts) => {
            list_serial_ports()?;
        }
        Some(command) => {
            // Non-interactive mode
            let port_name = cli.port.ok_or_else(|| {
                LumidoxError::InvalidInput("Port must be specified for non-interactive mode".to_string())
            })?;

            run_command_mode_with_optimization(command, port_name, optimize_transitions)?;
        }
        None => {
            // Interactive mode
            run_interactive_mode_with_optimization(cli.port, optimize_transitions)?;
        }
    }

    Ok(())
}
