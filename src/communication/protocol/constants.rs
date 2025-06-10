//! Protocol constants for Lumidox II Controller communication
//!
//! This module defines all protocol-level constants including command markers,
//! timeouts, baud rates, and other configuration values used in serial communication.

use std::time::Duration;

/// Command start marker
pub const CMD_START: u8 = b'*';

/// Response end marker
pub const RESPONSE_END: u8 = b'^';

/// Command terminator
pub const CMD_TERMINATOR: u8 = b'\r';

/// Default timeout for serial operations
pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(1000);

/// Default baud rate
pub const DEFAULT_BAUD_RATE: u32 = 19200;
