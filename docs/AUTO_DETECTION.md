# Automated COM Port and Baud Rate Detection

The Lumidox II Controller application now includes comprehensive automated detection capabilities for COM ports and baud rates, eliminating the need for manual configuration in most cases.

## Features

### 🔍 **Automated Port Detection**
- Scans all available serial ports
- Filters for USB Serial Port devices (FTDI-based)
- Tests device compatibility using protocol commands
- Ranks ports by compatibility score
- Identifies Lumidox II devices automatically

### ⚡ **Automated Baud Rate Detection**
- Tests common baud rates (19200, 9600, 38400, 57600, 115200)
- Validates communication with device identification commands
- Ranks baud rates by response quality
- Supports both quick and thorough detection modes

### 🚀 **Unified Auto-Connection**
- Combines port and baud rate detection
- Provides fallback to manual selection
- Caches successful connections for faster reconnection
- Comprehensive error reporting and user guidance

## Command Line Usage

### Basic Auto-Detection
```bash
# Use auto-detection for any command
lumidox-ii-controller --auto info

# Use auto-detection with verbose output
lumidox-ii-controller --auto --verbose status

# Interactive mode with auto-detection
lumidox-ii-controller --auto
```

### Port Detection Commands
```bash
# List all available ports
lumidox-ii-controller list-ports

# Detect compatible Lumidox II ports
lumidox-ii-controller detect-ports

# Test baud rates on a specific port
lumidox-ii-controller test-baud COM3

# Show detailed port diagnostics
lumidox-ii-controller port-diagnostics
```

### Manual Override
```bash
# Specify port manually (traditional method)
lumidox-ii-controller --port COM3 info

# Mix auto-detection with manual port specification
lumidox-ii-controller --port COM3 --auto info
```

## Modular CLI Operations Architecture

The Lumidox II Controller now features a **7-level deep hierarchical CLI operations architecture** that provides clean separation of concerns and enhanced modularity.

### Architecture Overview
```
src/ui/cli/commands/operations/          (Level 5)
├── device_control/                      (Level 6)
│   ├── stage_firing.rs                  (Level 7) - Stage 1-5 firing operations
│   ├── current_control.rs               (Level 7) - Custom current control
│   └── power_control.rs                 (Level 7) - Arm/Off power control
├── information/                         (Level 6)
│   ├── device_info.rs                   (Level 7) - Device information retrieval
│   ├── status_reading.rs                (Level 7) - Status reading operations
│   └── state_reading.rs                 (Level 7) - State reading operations
├── parameters/                          (Level 6)
│   ├── current_settings.rs              (Level 7) - Current setting operations
│   └── stage_parameters.rs              (Level 7) - Stage parameter operations
└── port_management/                     (Level 6)
    ├── detection.rs                     (Level 7) - Port detection operations
    ├── testing.rs                       (Level 7) - Baud rate testing
    └── diagnostics.rs                   (Level 7) - Port diagnostics
```

### Operations Coordinator Integration
```rust
use lumidox_ii_controller::ui::cli::commands::operations::{
    OperationsCoordinator, StageFiringOperations, CurrentControlOperations,
    PowerControlOperations, DeviceInfoOperations, StatusReadingOperations,
    StateReadingOperations
};

// Create operations coordinator
let coordinator = OperationsCoordinator::new();

// Execute commands through specialized operations
let context = &mut CommandExecutionContext::new(device);
let config = &CommandExecutionConfig::default();

// Stage firing operation
let result = coordinator.execute_command(&Commands::Stage1, context, config)?;

// Device info operation
let result = coordinator.execute_command(&Commands::Info, context, config)?;
```

### Specialized Operation Modules

#### Device Control Operations
```rust
use lumidox_ii_controller::ui::cli::commands::operations::device_control::{
    StageFiringOperations, CurrentControlOperations, PowerControlOperations
};

// Stage firing with comprehensive validation and testing
let stage_ops = StageFiringOperations::new();
let result = stage_ops.execute(&Commands::Stage1, context, config)?;

// Custom current control with safety validation
let current_ops = CurrentControlOperations::new();
let result = current_ops.execute(&Commands::Current { value: 1500 }, context, config)?;

// Power control operations (Arm/Off)
let power_ops = PowerControlOperations::new();
let result = power_ops.execute(&Commands::Arm, context, config)?;
```

#### Information Operations
```rust
use lumidox_ii_controller::ui::cli::commands::operations::information::{
    DeviceInfoOperations, StatusReadingOperations, StateReadingOperations
};

// Device information retrieval
let info_ops = DeviceInfoOperations::new();
let result = info_ops.execute(&Commands::Info, context, config)?;

// Status reading with comprehensive health checks
let status_ops = StatusReadingOperations::new();
let result = status_ops.execute(&Commands::Status, context, config)?;

// State reading operations
let state_ops = StateReadingOperations::new();
let result = state_ops.execute(&Commands::ReadState, context, config)?;
```

### Benefits of Modular Architecture

- **Single Responsibility**: Each module handles exactly one type of operation (<150 lines)
- **Comprehensive Testing**: 67+ unit tests across all operation modules
- **Clean Separation**: Device control, information, parameters, and port management are isolated
- **Easy Extension**: New operations can be added without affecting existing code
- **Type Safety**: Strong typing ensures correct operation routing and validation
- **Backward Compatibility**: Existing CLI commands work unchanged

## Programming Interface

### Quick Auto-Connection
```rust
use lumidox_ii_controller::communication::{AutoConnector, AutoConnectConfig};

// Quick auto-connection (recommended for interactive use)
let config = AutoConnector::quick_config();
let (device, result) = AutoConnector::auto_connect(&config)?;

println!("Connected to {} at {} baud", 
    result.port_name.unwrap(), 
    result.baud_rate.unwrap());
```

### Thorough Auto-Connection
```rust
// Thorough auto-connection (recommended for automated systems)
let config = AutoConnector::thorough_config();
let (device, result) = AutoConnector::auto_connect(&config)?;
```

### Custom Configuration
```rust
use lumidox_ii_controller::communication::{
    AutoConnectConfig, PortDetectionConfig, BaudDetectionConfig
};

let config = AutoConnectConfig {
    port_config: PortDetectionConfig {
        usb_ports_only: true,
        test_device_identification: true,
        preferred_vendor_ids: vec![0x0403], // FTDI
        ..Default::default()
    },
    baud_config: BaudDetectionConfig {
        test_baud_rates: vec![19200, 9600, 38400],
        attempts_per_rate: 2,
        comprehensive_testing: false,
        ..Default::default()
    },
    verbose: true,
    enable_caching: true,
    max_detection_time: Duration::from_secs(15),
};

let (device, result) = AutoConnector::auto_connect(&config)?;
```

### Port Detection Only
```rust
use lumidox_ii_controller::communication::{PortDetector, PortDetectionConfig};

let config = PortDetectionConfig::default();
let candidates = PortDetector::detect_ports(&config)?;

for candidate in candidates {
    println!("Port: {} - Score: {} - {}", 
        candidate.port_info.port_name,
        candidate.compatibility_score,
        candidate.score_reason);
}
```

### Baud Rate Detection Only
```rust
use lumidox_ii_controller::communication::{BaudDetector, BaudDetectionConfig};

let config = BaudDetectionConfig::default();
let results = BaudDetector::test_all_baud_rates("COM3", &config)?;

for result in results {
    if result.success {
        println!("Working baud rate: {} (score: {})", 
            result.baud_rate, result.quality_score);
    }
}
```

## Detection Process

### 1. Port Scanning
The system scans all available serial ports and applies filtering criteria:

- **USB Port Priority**: USB Serial Ports receive higher compatibility scores
- **Vendor ID Matching**: FTDI devices (VID: 0x0403) are prioritized
- **Device Identification**: Attempts to communicate using Lumidox II protocol
- **Compatibility Scoring**: Ranks ports from 0-100 based on multiple factors

### 2. Baud Rate Testing
For each compatible port, the system tests common baud rates:

- **Default First**: Tests 19200 baud (Lumidox II default) first
- **Common Rates**: Tests 9600, 38400, 57600, 115200 if needed
- **Protocol Validation**: Sends device identification commands
- **Quality Scoring**: Ranks baud rates by response consistency

### 3. Connection Establishment
The system establishes connection using the best match:

- **Highest Score**: Selects port and baud rate with highest combined score
- **Device Validation**: Confirms device responds to Lumidox II protocol
- **Information Retrieval**: Reads device firmware, model, and serial number
- **Connection Caching**: Stores successful parameters for future use

## Troubleshooting

### No Ports Detected
```bash
# Check if device is connected and drivers are installed
lumidox-ii-controller port-diagnostics

# List all available ports (including non-USB)
lumidox-ii-controller list-ports
```

**Common Solutions:**
- Ensure Lumidox II Controller is connected via USB
- Install FTDI drivers from https://www.ftdichip.com/FTDrivers.htm
- Check USB cable and connections
- Try a different USB port

### Auto-Detection Fails
```bash
# Use verbose mode to see detailed detection process
lumidox-ii-controller --auto --verbose info

# Test specific port manually
lumidox-ii-controller test-baud COM3
```

**Common Solutions:**
- Device may be in local mode (switch to remote mode)
- Another application may be using the port
- Baud rate may be non-standard (try manual specification)
- Device may need power cycle

### Slow Detection
```bash
# Use quick detection mode
lumidox-ii-controller --auto info  # Uses quick config by default
```

**Optimization Options:**
- Quick config tests fewer baud rates (faster)
- Thorough config tests all rates (more reliable)
- Manual port specification skips port detection
- Caching speeds up subsequent connections

## Configuration Options

### Port Detection Settings
- `usb_ports_only`: Only scan USB serial ports (default: true)
- `test_device_identification`: Test protocol communication (default: true)
- `preferred_vendor_ids`: Prioritize specific USB vendor IDs
- `identification_timeout`: Timeout for device tests (default: 2s)

### Baud Rate Detection Settings
- `test_baud_rates`: List of baud rates to test
- `attempts_per_rate`: Number of attempts per baud rate (default: 2)
- `comprehensive_testing`: Test all rates vs. stop at first good match
- `test_timeout`: Timeout per baud rate test (default: 1.5s)

### Auto-Connection Settings
- `verbose`: Enable detailed output during detection
- `enable_caching`: Cache successful connections (default: true)
- `max_detection_time`: Maximum time for entire process (default: 30s)

## Examples

See `examples/auto_detection_demo.rs` for a complete demonstration of all auto-detection features.

## Migration from Manual Configuration

### Before (Manual)
```bash
lumidox-ii-controller --port COM3 info
```

### After (Automatic)
```bash
lumidox-ii-controller --auto info
```

The auto-detection system is designed to be a drop-in replacement for manual port specification while providing better reliability and user experience.
