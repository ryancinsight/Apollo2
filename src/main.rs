// Import our custom modules
mod core;
mod communication;
mod device;
mod ui;

use core::Result;

/// Main entry point with conditional compilation for interface selection
///
/// The interface is determined at compile time via Cargo features:
/// - Default build: Both CLI and GUI available, auto-detects environment
/// - CLI-only build: `cargo build --features cli --no-default-features`
/// - GUI-only build: `cargo build --features gui --no-default-features`
fn main() -> Result<()> {
    // Conditional compilation based on available features
    #[cfg(all(feature = "gui", feature = "cli"))]
    {
        // Both interfaces available - auto-detect or provide selection
        run_dual_mode()
    }

    #[cfg(all(feature = "gui", not(feature = "cli")))]
    {
        // GUI-only build
        run_gui_only()
    }

    #[cfg(all(feature = "cli", not(feature = "gui")))]
    {
        // CLI-only build
        run_cli_only()
    }

    #[cfg(not(any(feature = "gui", feature = "cli")))]
    {
        // No interface features enabled - this should not happen with proper feature configuration
        compile_error!("At least one interface feature (gui or cli) must be enabled");
    }
}

/// Run application with both interfaces available (default build)
///
/// Auto-detects the environment and chooses the appropriate interface:
/// - If CLI arguments are provided, uses CLI interface
/// - If no CLI arguments and in GUI environment, launches GUI interface
/// - If no CLI arguments and in terminal environment, uses CLI interface
#[cfg(all(feature = "gui", feature = "cli"))]
fn run_dual_mode() -> Result<()> {
    
    use std::env;

    // Check if CLI arguments are provided (beyond just the program name)
    let args: Vec<String> = env::args().collect();
    let has_cli_args = args.len() > 1;

    if has_cli_args {
        // CLI arguments provided, use CLI interface
        run_cli_interface()
    } else if is_gui_environment() {
        // No CLI arguments and GUI environment available, attempt GUI interface
        match ui::run_gui(None, true, false, true) {
            Ok(()) => Ok(()),
            Err(_) => {
                // GUI failed, fallback to CLI
                eprintln!("GUI interface failed to initialize, falling back to CLI mode...");
                run_cli_interface()
            }
        }
    } else {
        // No CLI arguments and terminal environment, use CLI interface
        run_cli_interface()
    }
}

/// Run GUI-only interface (GUI-only build)
#[cfg(all(feature = "gui", not(feature = "cli")))]
fn run_gui_only() -> Result<()> {
    // Launch GUI interface with auto-detection enabled
    ui::run_gui(None, true, false, true)
        .map_err(|e| core::LumidoxError::ConfigError(format!("GUI failed: {}", e)))
}

/// Run CLI-only interface (CLI-only build)
#[cfg(all(feature = "cli", not(feature = "gui")))]
fn run_cli_only() -> Result<()> {
    run_cli_interface()
}

/// Run the CLI interface with argument parsing
#[cfg(feature = "cli")]
fn run_cli_interface() -> Result<()> {
    use clap::Parser;
    use ui::Cli;

    let cli = Cli::parse();

    // Validate CLI arguments
    cli.validate();

    // Determine optimization setting
    let optimize_transitions = cli.optimize_transitions();

    if cli.is_command_mode() {
        run_command_mode(&cli, optimize_transitions)
    } else {
        run_interactive_mode(&cli, optimize_transitions)
    }
}

/// Run CLI in command mode (specific command execution)
#[cfg(feature = "cli")]
fn run_command_mode(cli: &ui::Cli, optimize_transitions: bool) -> Result<()> {
    use ui::{Commands, run_command_mode_with_optimization, list_serial_ports};

    match &cli.command {
        Some(Commands::ListPorts) => {
            list_serial_ports()?;
        }
        Some(Commands::DetectPorts) | Some(Commands::TestBaud { .. }) | Some(Commands::PortDiagnostics) => {
            // Port detection commands don't need device connection
            run_command_mode_with_optimization(cli.command.as_ref().unwrap().clone(), "".to_string(), optimize_transitions)?;
        }
        Some(command) => {
            // Commands that need device connection
            if cli.auto {
                // Use auto-detection
                run_auto_command(command, optimize_transitions, cli.verbose)?;
            } else {
                // Manual port specification required
                let port_name = cli.port.clone().ok_or_else(|| {
                    core::LumidoxError::InvalidInput("Port must be specified for non-interactive mode (use --auto for automatic detection)".to_string())
                })?;

                run_command_mode_with_optimization(command.clone(), port_name, optimize_transitions)?;
            }
        }
        None => {
            // This shouldn't happen in command mode, but handle gracefully
            return Err(core::LumidoxError::InvalidInput("No command specified".to_string()));
        }
    }

    Ok(())
}

/// Run CLI in interactive mode
#[cfg(feature = "cli")]
fn run_interactive_mode(cli: &ui::Cli, optimize_transitions: bool) -> Result<()> {
    use ui::run_interactive_mode_with_optimization;

    if cli.verbose {
        println!("Running in CLI Interactive mode");
    }

    run_interactive_mode_with_optimization(cli.port.clone(), cli.auto, cli.verbose, optimize_transitions)
}

/// Execute a command with auto-detected device
#[cfg(feature = "cli")]
fn run_auto_command(command: &ui::Commands, optimize_transitions: bool, verbose: bool) -> Result<()> {
    use ui::cli::device::create_device_controller_auto;
    use ui::Commands;

    let mut device = create_device_controller_auto(optimize_transitions, verbose)?;

    match command {
        Commands::Stage1 => { println!("Firing stage 1."); device.fire_stage(1)? }
        Commands::Stage2 => { println!("Firing stage 2."); device.fire_stage(2)? }
        Commands::Stage3 => { println!("Firing stage 3."); device.fire_stage(3)? }
        Commands::Stage4 => { println!("Firing stage 4."); device.fire_stage(4)? }
        Commands::Stage5 => { println!("Firing stage 5."); device.fire_stage(5)? }
        Commands::Current { value } => { println!("Firing with {}mA.", value); device.fire_with_current(*value)? }
        Commands::Arm => { println!("Arming device."); device.arm()? }
        Commands::Off => { println!("Turning off device."); device.turn_off()? }
        Commands::Info => {
            if let Some(info) = device.info() {
                println!("Controller Firmware Version: {}", info.firmware_version);
                println!("Device Model Number: {}", info.model_number);
                println!("Device Serial Number: {}", info.serial_number);
                println!("Device Wavelength: {}", info.wavelength);
            } else {
                println!("Device information not available");
            }
        }
        _ => {
            // For other commands, this shouldn't happen in auto mode, but handle gracefully
            return Err(core::LumidoxError::InvalidInput("Command not supported in auto mode".to_string()));
        }
    }

    Ok(())
}

/// Detect if we're running in a GUI environment
///
/// This is a simple heuristic that checks for common GUI environment indicators.
/// In a production application, this could be more sophisticated.
#[cfg(all(feature = "gui", feature = "cli"))]
fn is_gui_environment() -> bool {
    // Check for common GUI environment variables
    std::env::var("DISPLAY").is_ok() || // X11 on Linux/Unix
    std::env::var("WAYLAND_DISPLAY").is_ok() || // Wayland on Linux
    cfg!(target_os = "windows") || // Windows typically has GUI
    cfg!(target_os = "macos") // macOS typically has GUI
}