//! Protocol command definitions for Lumidox II Controller
//!
//! This module defines all device command codes and command arrays used
//! for communicating with the Lumidox II device over serial protocol.

/// Firmware version command
pub const FIRMWARE_VERSION: &[u8] = b"02";

/// Device mode setting command
pub const SET_MODE: &[u8] = b"15";

/// Current setting command
pub const SET_CURRENT: &[u8] = b"41";

/// Model number commands (8 bytes)
pub const MODEL_COMMANDS: [&[u8]; 8] = [
    b"6c", b"6d", b"6e", b"6f", b"70", b"71", b"72", b"73"
];

/// Serial number commands (12 bytes)
pub const SERIAL_COMMANDS: [&[u8]; 12] = [
    b"60", b"61", b"62", b"63", b"64", b"65", 
    b"66", b"67", b"68", b"69", b"6a", b"6b"
];

/// Wavelength commands (5 bytes)
pub const WAVELENGTH_COMMANDS: [&[u8]; 5] = [
    b"76", b"81", b"82", b"89", b"8a"
];

/// Stage current commands
pub const STAGE_CURRENTS: [&[u8]; 5] = [
    b"78", // Stage 1
    b"80", // Stage 2
    b"88", // Stage 3
    b"90", // Stage 4
    b"98", // Stage 5
];
