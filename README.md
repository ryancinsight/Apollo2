# Lumidox II Controller

A Rust implementation of the Lumidox II Controller PC Application, reimplemented from the original Python script using the `clap` crate for command-line argument parsing.

## Features

- **Interactive Mode**: Full menu-driven interface matching the original Python application
- **Command-line Mode**: Direct command execution for automation and scripting
- **Serial Communication**: Communicates with Lumidox II controller via serial port
- **Device Control**: Fire stages 1-5, custom current control, device information retrieval
- **Safety Features**: Input validation and error handling

## Installation

1. Ensure you have Rust installed (https://rustup.rs/)
2. Clone or download this project
3. Build the application:
   ```powershell
   cargo build --release
   ```

## Usage

### Interactive Mode (Default)

Run without any arguments to start the interactive menu:
```powershell
cargo run
```

Or specify a COM port directly:
```powershell
cargo run -- --port COM3
```

### Command-line Mode

Execute specific commands directly:

#### Fire specific stages:
```powershell
cargo run -- --port COM3 stage1
cargo run -- --port COM3 stage2
cargo run -- --port COM3 stage3
cargo run -- --port COM3 stage4
cargo run -- --port COM3 stage5
```

#### Fire with custom current:
```powershell
cargo run -- --port COM3 current 500
```

#### Turn off device:
```powershell
cargo run -- --port COM3 off
```

#### Show device information:
```powershell
cargo run -- --port COM3 info
```

#### List available COM ports:
```powershell
cargo run -- list-ports
```

### Help

View all available commands and options:
```powershell
cargo run -- --help
```

## Safety Notice

Before using this application, ensure you have:
- Proper PPE for skin & eyes to protect from high powered LEDs
- Ensured those around also have the same level of PPE
- Connected a light device to the Lumidox II controller
- Connected a USB cable from the PC to the Lumidox II controller
- Connected the Lumidox II controller to AC mains with the power adapter
- Powered on the Lumidox II controller to show the main menu on its display

## Dependencies

- `clap`: Command-line argument parsing
- `serialport`: Serial communication
- `anyhow`: Error handling
- `thiserror`: Custom error types

## Architecture

The application is structured with:
- `LumidoxController`: Main controller struct handling serial communication
- `Commands`: Enum defining all available CLI commands
- Error handling with custom `LumidoxError` types
- Modular functions for device communication, menu handling, and port management

## Original Python Script

This Rust implementation is based on `lumidox_II_console_interface_rev1b.py` and maintains full compatibility with the original functionality while adding modern CLI capabilities through the `clap` crate.