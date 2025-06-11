//! Device information command definitions
//!
//! This module contains protocol commands for reading device identification
//! and configuration information including firmware version, model number,
//! serial number, and wavelength specifications.

/// Firmware version command (0x02)
/// 
/// Returns the firmware version string from the device.
pub const FIRMWARE_VERSION: &[u8] = b"02";

/// Model number commands (8 bytes)
/// 
/// Command sequence to read the complete device model number.
/// Each command returns one byte of the model number string.
/// Commands: 0x6c through 0x73
pub const MODEL_COMMANDS: [&[u8]; 8] = [
    b"6c", b"6d", b"6e", b"6f", b"70", b"71", b"72", b"73"
];

/// Serial number commands (12 bytes)
/// 
/// Command sequence to read the complete device serial number.
/// Each command returns one byte of the serial number string.
/// Commands: 0x60 through 0x6b
pub const SERIAL_COMMANDS: [&[u8]; 12] = [
    b"60", b"61", b"62", b"63", b"64", b"65", 
    b"66", b"67", b"68", b"69", b"6a", b"6b"
];

/// Wavelength commands (5 bytes)
/// 
/// Command sequence to read the device wavelength specification.
/// Each command returns one byte of the wavelength string.
/// Commands: 0x76, 0x81, 0x82, 0x89, 0x8a
pub const WAVELENGTH_COMMANDS: [&[u8]; 5] = [
    b"76", b"81", b"82", b"89", b"8a"
];
