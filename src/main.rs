use clap::{Parser, Subcommand};
use serialport::{SerialPort, SerialPortType};
use std::io::{self, Write, Read};
use std::time::Duration;
use std::thread;
use anyhow::{Result, Context};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LumidoxError {
    #[error("Serial communication error: {0}")]
    SerialError(#[from] serialport::Error),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Device communication error: {0}")]
    DeviceError(String),
}

#[derive(Parser)]
#[command(name = "lumidox-ii-controller")]
#[command(about = "Lumidox II Controller PC Application")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// COM port to connect to (e.g., COM3)
    #[arg(short, long)]
    port: Option<String>,
    
    /// Run in interactive mode (default)
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
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
    /// Turn off device
    Off,
    /// Show device information
    Info,
    /// List available COM ports
    ListPorts,
}

struct LumidoxController {
    port: Box<dyn SerialPort>,
}

impl LumidoxController {
    fn new(port_name: &str) -> Result<Self> {
        let port = serialport::new(port_name, 19200)
            .timeout(Duration::from_millis(1000))
            .open()
            .context("Failed to open serial port")?;
        
        Ok(LumidoxController { port })
    }

    fn checksum(data: &[u8]) -> Vec<u8> {
        let mut value = 0u32;
        for &byte in &data[1..] {
            value += byte as u32;
        }
        value %= 256;
        format!("{:02x}", value).into_bytes()
    }

    fn hexc2dec(buffer: &[u8]) -> i32 {
        if buffer.len() < 5 {
            return 0;
        }
        
        let mut newval = 0i32;
        let mut divvy = 4096i32;
        
        for pn in 1..5 {
            let vally = buffer[pn];
            let subby = if vally < 97 { 48 } else { 87 };
            newval += ((vally - subby) as i32) * divvy;
            divvy /= 16;
        }
        
        if newval > 32767 {
            newval -= 65536;
        }
        
        newval
    }

    fn get_com_val(&mut self, command: &[u8], value: u16) -> Result<i32> {
        let mut cmd = vec![b'*'];
        cmd.extend_from_slice(command);
        
        if value == 0 {
            cmd.extend_from_slice(b"0000");
        } else {
            cmd.extend_from_slice(format!("{:04x}", value).as_bytes());
        }
        
        let checksum = Self::checksum(&cmd);
        cmd.extend_from_slice(&checksum);
        cmd.push(b'\r');
        
        self.port.write_all(&cmd)?;
        
        let mut response = Vec::new();
        let mut buffer = [0u8; 1];
        
        loop {
            match self.port.read(&mut buffer) {
                Ok(1) => {
                    response.push(buffer[0]);
                    if buffer[0] == b'^' {
                        break;
                    }
                }
                Ok(0) => break,
                Ok(_) => {
                    // Handle case where more than 1 byte is read (shouldn't happen with 1-byte buffer)
                    response.push(buffer[0]);
                    if buffer[0] == b'^' {
                        break;
                    }
                }
                Err(e) => return Err(LumidoxError::IoError(e).into()),
            }
        }
        
        Ok(Self::hexc2dec(&response))
    }

    fn get_firmware_version(&mut self) -> Result<String> {
        let version = self.get_com_val(b"02", 0)?;
        Ok(version.to_string())
    }

    fn get_model_number(&mut self) -> Result<String> {
        let mut model = String::new();
        let commands = [b"6c", b"6d", b"6e", b"6f", b"70", b"71", b"72", b"73"];
        
        for cmd in &commands {
            let val = self.get_com_val(*cmd, 0)?;
            if val > 0 && val < 256 {
                model.push(val as u8 as char);
            }
        }
        
        Ok(model)
    }

    fn get_serial_number(&mut self) -> Result<String> {
        let mut serial = String::new();
        let commands = [b"60", b"61", b"62", b"63", b"64", b"65", b"66", b"67", b"68", b"69", b"6a", b"6b"];
        
        for cmd in &commands {
            let val = self.get_com_val(*cmd, 0)?;
            if val > 0 && val < 256 {
                serial.push(val as u8 as char);
            }
        }
        
        Ok(serial)
    }

    fn get_wavelength(&mut self) -> Result<String> {
        let mut wavelength = String::new();
        let commands = [b"76", b"81", b"82", b"89", b"8a"];
        
        for cmd in &commands {
            let val = self.get_com_val(*cmd, 0)?;
            if val > 0 && val < 256 {
                wavelength.push(val as u8 as char);
            }
        }
        
        Ok(wavelength)
    }

    fn fire_stage(&mut self, stage: u8) -> Result<()> {
        // Set mode to 3
        self.get_com_val(b"15", 3)?;
        thread::sleep(Duration::from_millis(100));
        
        let current_cmd = match stage {
            1 => b"78",
            2 => b"80", 
            3 => b"88",
            4 => b"90",
            5 => b"98",
            _ => return Err(LumidoxError::InvalidInput("Invalid stage number".to_string()).into()),
        };
        
        let current = self.get_com_val(current_cmd, 0)?;
        self.get_com_val(b"41", current as u16)?;
        
        Ok(())
    }

    fn fire_with_current(&mut self, current_ma: u16) -> Result<()> {
        let max_current = self.get_com_val(b"98", 0)? as u16;
        
        if current_ma > max_current {
            return Err(LumidoxError::InvalidInput(
                format!("Cannot fire above {}mA", max_current)
            ).into());
        }
        
        self.get_com_val(b"15", 3)?;
        thread::sleep(Duration::from_millis(100));
        self.get_com_val(b"41", current_ma)?;
        
        Ok(())
    }

    fn turn_off_device(&mut self) -> Result<()> {
        self.get_com_val(b"15", 1)?;
        thread::sleep(Duration::from_millis(1000));
        Ok(())
    }

    fn get_power_info(&mut self, stage: u8) -> Result<(f32, String, f32, String)> {
        let (total_cmd, per_cmd, total_units_cmd, per_units_cmd) = match stage {
            1 => (b"7b", b"7c", b"7d", b"7e"),
            2 => (b"83", b"84", b"85", b"86"),
            3 => (b"8b", b"8c", b"8d", b"8e"),
            4 => (b"93", b"94", b"95", b"96"),
            5 => (b"9b", b"9c", b"9d", b"9e"),
            _ => return Err(LumidoxError::InvalidInput("Invalid stage number".to_string()).into()),
        };
        
        let total_power = self.get_com_val(total_cmd, 0)? as f32 / 10.0;
        let per_power = self.get_com_val(per_cmd, 0)? as f32 / 10.0;
        let total_units_idx = self.get_com_val(total_units_cmd, 0)?;
        let per_units_idx = self.get_com_val(per_units_cmd, 0)?;
        
        let total_units = decode_total_units(total_units_idx);
        let per_units = decode_per_units(per_units_idx);
        
        Ok((total_power, total_units, per_power, per_units))
    }
}

fn decode_total_units(index: i32) -> String {
    match index {
        0 => "W TOTAL RADIANT POWER".to_string(),
        1 => "mW TOTAL RADIANT POWER".to_string(),
        2 => "W/cm² TOTAL IRRADIANCE".to_string(),
        3 => "mW/cm² TOTAL IRRADIANCE".to_string(),
        4 => "".to_string(),
        5 => "A TOTAL CURRENT".to_string(),
        6 => "mA TOTAL CURRENT".to_string(),
        _ => "UNKNOWN UNITS".to_string(),
    }
}

fn decode_per_units(index: i32) -> String {
    match index {
        0 => "W PER WELL".to_string(),
        1 => "mW PER WELL".to_string(),
        2 => "W TOTAL RADIANT POWER".to_string(),
        3 => "mW TOTAL RADIANT POWER".to_string(),
        4 => "mW/cm² PER WELL".to_string(),
        5 => "mW/cm²".to_string(),
        6 => "J/s".to_string(),
        7 => "".to_string(),
        8 => "A PER WELL".to_string(),
        9 => "mA PER WELL".to_string(),
        _ => "UNKNOWN UNITS".to_string(),
    }
}

fn list_serial_ports() -> Result<()> {
    let ports = serialport::available_ports()?;
    
    println!("Available COM ports:");
    for port in ports {
        match &port.port_type {
            SerialPortType::UsbPort(info) => {
                println!("  {}: USB Serial Port - {}", port.port_name, 
                    info.product.as_ref().unwrap_or(&"Unknown".to_string()));
            }
            _ => {
                println!("  {}: {}", port.port_name, 
                    match &port.port_type {
                        SerialPortType::PciPort => "PCI Port",
                        SerialPortType::BluetoothPort => "Bluetooth Port",
                        _ => "Unknown Port Type",
                    });
            }
        }
    }
    
    Ok(())
}

fn get_user_port_selection() -> Result<String> {
    list_serial_ports()?;
    println!();
    
    loop {
        print!("Please enter the COM port name (e.g., COM3): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let port_name = input.trim();
        
        if port_name.is_empty() {
            continue;
        }
        
        // Validate port exists
        let ports = serialport::available_ports()?;
        if ports.iter().any(|p| p.port_name == port_name) {
            return Ok(port_name.to_string());
        } else {
            println!("Port {} not found. Please try again.", port_name);
        }
    }
}

fn welcome_message() -> Result<String> {
    println!("Welcome to the Analytical Sales & Services, Inc. Lumidox II Controller PC App!");
    println!();
    println!("Before we get started please make sure to do the following:");
    println!("  * Have proper PPE for skin & eyes to protect from high powered LEDs.");
    println!("  * Ensure those around also have the same level of PPE.");
    println!("  * Connect a light device to the Lumidox II controller.");
    println!("  * Connect a USB cable from the PC to the Lumidox II controller.");
    println!("  * Connect the Lumidox II controller to AC mains with the power adapter.");
    println!("  * Power on the Lumidox II controller to show the main menu on it's display.");
    println!();
    
    print!("Press ENTER after the above is complete: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    get_user_port_selection()
}

fn interactive_menu(controller: &mut LumidoxController) -> Result<bool> {
    loop {
        println!("-- Select an action --");
        
        // Display stage options with power info
        for stage in 1..=5 {
            if let Ok((total_power, total_units, per_power, per_units)) = controller.get_power_info(stage) {
                println!("{}) Turn on stage {}: {} {}, {} {}", 
                    stage, stage, total_power, total_units, per_power, per_units);
            } else {
                println!("{}) Turn on stage {}", stage, stage);
            }
        }
        
        if let Ok(max_current) = controller.get_com_val(b"98", 0) {
            println!("6) Turn on stage with specific current (up to {}mA).", max_current);
        } else {
            println!("6) Turn on stage with specific current.");
        }
        
        println!("7) Turn off device.");
        println!("8) Quit program.");
        println!();
        
        print!("Please enter choice number, then press ENTER: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        match choice {
            "1" | "2" | "3" | "4" | "5" => {
                let stage = choice.parse::<u8>().unwrap();
                println!();
                println!("Firing stage {}.", stage);
                println!();
                controller.fire_stage(stage)?;
                return Ok(true);
            }
            "6" => {
                println!();
                print!("Please enter current in mA (no decimals), then press ENTER: ");
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let current_str = input.trim();
                
                match current_str.parse::<u16>() {
                    Ok(current) => {
                        println!();
                        println!("Firing with {}mA.", current);
                        println!();
                        match controller.fire_with_current(current) {
                            Ok(_) => return Ok(true),
                            Err(e) => {
                                println!("Error: {}. Aborting action.", e);
                                println!();
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        println!();
                        println!("Invalid input. Aborting action");
                        println!();
                        return Ok(true);
                    }
                }
            }
            "7" => {
                println!();
                println!("Turning off device.");
                controller.turn_off_device()?;
                println!();
                return Ok(true);
            }
            "8" => {
                println!();
                println!("Turning off device.");
                controller.turn_off_device()?;
                controller.get_com_val(b"15", 0)?;
                thread::sleep(Duration::from_millis(1000));
                println!("To resume using the controller in local mode, please cycle the power with on/off switch.");
                thread::sleep(Duration::from_millis(1000));
                println!("Quitting program...");
                thread::sleep(Duration::from_millis(1000));
                println!();
                return Ok(false);
            }
            _ => {
                println!();
                println!("Not a valid choice. Please try again.");
                println!();
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::ListPorts) => {
            list_serial_ports()?;
            return Ok(());
        }
        Some(command) => {
            // Non-interactive mode
            let port_name = cli.port.ok_or_else(|| {
                LumidoxError::InvalidInput("Port must be specified for non-interactive mode".to_string())
            })?;
            
            let mut controller = LumidoxController::new(&port_name)?;
            
            match command {
                Commands::Stage1 => {
                    println!("Firing stage 1.");
                    controller.fire_stage(1)?
                }
                Commands::Stage2 => {
                    println!("Firing stage 2.");
                    controller.fire_stage(2)?
                }
                Commands::Stage3 => {
                    println!("Firing stage 3.");
                    controller.fire_stage(3)?
                }
                Commands::Stage4 => {
                    println!("Firing stage 4.");
                    controller.fire_stage(4)?
                }
                Commands::Stage5 => {
                    println!("Firing stage 5.");
                    controller.fire_stage(5)?
                }
                Commands::Current { value } => {
                    println!("Firing with {}mA.", value);
                    controller.fire_with_current(value)?
                }
                Commands::Off => {
                    println!("Turning off device.");
                    controller.turn_off_device()?
                }
                Commands::Info => {
                    println!("Controller Firmware Version: 1.{}", controller.get_firmware_version()?);
                    println!("Device Model Number: {}", controller.get_model_number()?);
                    println!("Device Serial Number: {}", controller.get_serial_number()?);
                    println!("Device Wavelength: {}", controller.get_wavelength()?);
                }
                Commands::ListPorts => unreachable!(),
            }
        }
        None => {
            // Interactive mode
            let port_name = if let Some(port) = cli.port {
                port
            } else {
                welcome_message()?
            };
            
            let mut controller = LumidoxController::new(&port_name)?;
            println!("{} has been connected!", port_name);
            
            // Initialize device
            controller.get_com_val(b"15", 1)?;
            thread::sleep(Duration::from_millis(100));
            
            println!("--------------------------------------");
            println!("Controller Firmware Version: 1.{}", controller.get_firmware_version()?);
            println!("Device Model Number: {}", controller.get_model_number()?);
            println!("Device Serial Number: {}", controller.get_serial_number()?);
            println!("Device Wavelength: {}", controller.get_wavelength()?);
            println!();
            
            let mut continue_loop = true;
            while continue_loop {
                continue_loop = interactive_menu(&mut controller)?;
            }
        }
    }
    
    Ok(())
}
