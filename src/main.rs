use clap::Parser;

// Import our custom modules
mod core;
mod communication;
mod device;
mod ui;

use core::{LumidoxError, Result};
use ui::{Cli, Commands, run_interactive_mode, run_command_mode, list_serial_ports};





fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ListPorts) => {
            list_serial_ports()?;
        }
        Some(command) => {
            // Non-interactive mode
            let port_name = cli.port.ok_or_else(|| {
                LumidoxError::InvalidInput("Port must be specified for non-interactive mode".to_string())
            })?;

            run_command_mode(command, port_name)?;
        }
        None => {
            // Interactive mode
            run_interactive_mode(cli.port)?;
        }
    }

    Ok(())
}
