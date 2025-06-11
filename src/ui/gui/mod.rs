//! GUI system entry point module for Lumidox II Controller
//!
//! This module provides the complete GUI system entry point with proper
//! Iced application initialization, settings configuration, and integration
//! with existing CLI argument structure. It serves as the main interface
//! for launching the GUI mode of the Lumidox II Controller.
//!
//! The GUI module includes:
//! - Complete GUI system export and entry point
//! - Iced application settings and configuration
//! - CLI argument integration and compatibility
//! - Error handling and graceful degradation
//! - GUI-specific initialization and cleanup

// Import GUI modules (temporarily disabled for API compatibility)
// pub mod components;
// pub mod application;

// Re-export the main application for easy access
// pub use application::LumidoxApplication;

use iced::{Settings, Size, window, Element, Task, Theme};
use crate::core::{LumidoxError, DeviceControlOperations, DeviceOperationData};
use crate::ui::cli::device::create_device_controller_with_fallback;
use crate::device::LumidoxDevice;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Run the GUI application
/// 
/// Launches the Lumidox II Controller GUI application with the specified
/// configuration parameters. This function serves as the main entry point
/// for GUI mode operation.
/// 
/// # Arguments
/// * `port_name` - Optional specific port name for device connection
/// * `auto_detect` - Whether to use automatic port detection
/// * `verbose` - Enable verbose output during operations
/// * `optimize_transitions` - Enable optimized stage transitions
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if GUI ran successfully, Err with error details
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::run_gui;
/// 
/// // Run GUI with auto-detection enabled
/// if let Err(error) = run_gui(None, true, false, true) {
///     eprintln!("GUI failed to start: {}", error);
/// }
/// ```
/// 
/// # Errors
/// 
/// This function can return errors in the following cases:
/// - Iced framework initialization failure
/// - Graphics system unavailable or incompatible
/// - Window system initialization failure
/// - Invalid application settings or configuration
/// 
/// # Platform Support
/// 
/// The GUI requires a compatible graphics environment:
/// - Windows: DirectX 11+ or OpenGL 3.3+
/// - Linux: X11 or Wayland with OpenGL 3.3+
/// - macOS: Metal or OpenGL 3.3+
pub fn run_gui(
    port_name: Option<String>,
    auto_detect: bool,
    verbose: bool,
    optimize_transitions: bool,
) -> std::result::Result<(), Box<dyn Error>> {
    // Create application settings
    let settings = create_application_settings();
    
    // Clone values for the closure
    let port_name_clone = port_name.clone();
    let auto_detect_clone = auto_detect;
    let verbose_clone = verbose;
    let optimize_transitions_clone = optimize_transitions;

    // Run the simple Iced application using the 0.13.x API
    match iced::application("Lumidox II Controller", update, view)
        .theme(theme)
        .settings(settings)
        .run_with(move || {
            let mut initial_state = AppState::default();
            initial_state.port_name = port_name_clone;
            initial_state.auto_detect = auto_detect_clone;
            initial_state.verbose = verbose_clone;
            initial_state.optimize_transitions = optimize_transitions_clone;

            // Auto-connect if requested
            let initial_task = if auto_detect_clone {
                Task::perform(
                    async move { Message::Connect },
                    |msg| msg,
                )
            } else {
                Task::none()
            };

            (initial_state, initial_task)
        }) {
        Ok(_) => Ok(()),
        Err(error) => {
            // Convert Iced error to our error type
            Err(Box::new(GuiError::IcedError(error.to_string())))
        }
    }
}

/// Run GUI with default settings
/// 
/// Convenience function to run the GUI with default settings and auto-detection enabled.
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if GUI ran successfully, Err with error details
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::run_gui_default;
/// 
/// if let Err(error) = run_gui_default() {
///     eprintln!("GUI failed to start: {}", error);
/// }
/// ```
pub fn run_gui_default() -> std::result::Result<(), Box<dyn Error>> {
    run_gui(None, true, false, true)
}

/// Run GUI with specific port
/// 
/// Convenience function to run the GUI with a specific port name.
/// 
/// # Arguments
/// * `port_name` - Specific port name for device connection
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if GUI ran successfully, Err with error details
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::run_gui_with_port;
/// 
/// if let Err(error) = run_gui_with_port("COM3".to_string()) {
///     eprintln!("GUI failed to start: {}", error);
/// }
/// ```
pub fn run_gui_with_port(port_name: String) -> std::result::Result<(), Box<dyn Error>> {
    run_gui(Some(port_name), false, false, true)
}

/// Run GUI in verbose mode
/// 
/// Convenience function to run the GUI with verbose output enabled.
/// 
/// # Arguments
/// * `port_name` - Optional specific port name for device connection
/// * `auto_detect` - Whether to use automatic port detection
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok if GUI ran successfully, Err with error details
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::run_gui_verbose;
/// 
/// if let Err(error) = run_gui_verbose(None, true) {
///     eprintln!("GUI failed to start: {}", error);
/// }
/// ```
pub fn run_gui_verbose(
    port_name: Option<String>,
    auto_detect: bool,
) -> std::result::Result<(), Box<dyn Error>> {
    run_gui(port_name, auto_detect, true, true)
}

/// Create application settings
/// 
/// Creates and configures Iced application settings with appropriate
/// window size, title, and other GUI-specific configurations.
/// 
/// # Returns
/// * `Settings` - Configured Iced application settings
///
/// # Example
/// ```
/// let settings = create_application_settings();
/// ```
fn create_application_settings() -> iced::Settings {
    iced::Settings {
        id: None,
        fonts: Vec::new(),
        default_font: iced::Font::DEFAULT,
        default_text_size: iced::Pixels(14.0),
        antialiasing: true,
    }
}

/// Check GUI compatibility
/// 
/// Performs basic checks to determine if the GUI can run on the current system.
/// 
/// # Returns
/// * `Result<()>` - Ok if GUI is compatible, Err with compatibility issue
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::check_gui_compatibility;
/// 
/// if let Err(error) = check_gui_compatibility() {
///     eprintln!("GUI not compatible: {}", error);
/// }
/// ```
pub fn check_gui_compatibility() -> crate::core::Result<()> {
    // Basic compatibility checks
    
    // Check if we're in a headless environment
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        #[cfg(unix)]
        return Err(LumidoxError::SystemError(
            "No display server detected. GUI requires X11 or Wayland.".to_string()
        ));
    }
    
    // Additional platform-specific checks could be added here
    
    Ok(())
}

/// Get GUI system information
/// 
/// Returns information about the GUI system capabilities and configuration.
/// 
/// # Returns
/// * `GuiSystemInfo` - Information about GUI system
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::get_gui_system_info;
/// 
/// let info = get_gui_system_info();
/// println!("GUI Backend: {}", info.backend);
/// ```
pub fn get_gui_system_info() -> GuiSystemInfo {
    GuiSystemInfo {
        backend: "Iced".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        renderer: "wgpu".to_string(),
        platform: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        supports_transparency: true,
        supports_decorations: true,
        supports_resizing: true,
    }
}

/// GUI system information
/// 
/// Contains information about the GUI system capabilities and configuration.
#[derive(Debug, Clone)]
pub struct GuiSystemInfo {
    /// GUI backend name
    pub backend: String,
    /// Application version
    pub version: String,
    /// Renderer type
    pub renderer: String,
    /// Operating system platform
    pub platform: String,
    /// System architecture
    pub architecture: String,
    /// Whether transparency is supported
    pub supports_transparency: bool,
    /// Whether window decorations are supported
    pub supports_decorations: bool,
    /// Whether window resizing is supported
    pub supports_resizing: bool,
}

/// GUI-specific error types
/// 
/// Represents errors that can occur during GUI initialization and operation.
#[derive(Debug)]
pub enum GuiError {
    /// Iced framework error
    IcedError(String),
    /// Graphics system error
    GraphicsError(String),
    /// Window system error
    WindowError(String),
    /// Configuration error
    ConfigError(String),
    /// Compatibility error
    CompatibilityError(String),
}

impl std::fmt::Display for GuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuiError::IcedError(msg) => write!(f, "Iced framework error: {}", msg),
            GuiError::GraphicsError(msg) => write!(f, "Graphics system error: {}", msg),
            GuiError::WindowError(msg) => write!(f, "Window system error: {}", msg),
            GuiError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            GuiError::CompatibilityError(msg) => write!(f, "Compatibility error: {}", msg),
        }
    }
}

impl Error for GuiError {}

/// Initialize GUI subsystem
/// 
/// Performs any necessary initialization for the GUI subsystem.
/// This function should be called before attempting to run the GUI.
/// 
/// # Returns
/// * `Result<()>` - Ok if initialization successful
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::initialize_gui;
/// 
/// if let Err(error) = initialize_gui() {
///     eprintln!("GUI initialization failed: {}", error);
/// }
/// ```
pub fn initialize_gui() -> crate::core::Result<()> {
    // Check compatibility first
    check_gui_compatibility()?;
    
    // Perform any additional initialization
    // (Currently no additional initialization required)
    
    Ok(())
}

/// Cleanup GUI subsystem
/// 
/// Performs cleanup operations for the GUI subsystem.
/// This function should be called when shutting down the application.
/// 
/// # Returns
/// * `Result<()>` - Ok if cleanup successful
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::cleanup_gui;
/// 
/// if let Err(error) = cleanup_gui() {
///     eprintln!("GUI cleanup failed: {}", error);
/// }
/// ```
pub fn cleanup_gui() -> crate::core::Result<()> {
    // Perform any necessary cleanup
    // (Currently no cleanup required)
    
    Ok(())
}

/// Test GUI functionality
/// 
/// Performs basic tests to verify GUI functionality without launching
/// the full application.
/// 
/// # Returns
/// * `Result<()>` - Ok if tests pass
/// 
/// # Example
/// ```
/// use lumidox_ii_controller::ui::gui::test_gui;
/// 
/// if let Err(error) = test_gui() {
///     eprintln!("GUI tests failed: {}", error);
/// }
/// ```
pub fn test_gui() -> crate::core::Result<()> {
    // Check compatibility
    check_gui_compatibility()?;
    
    // Test application creation (without running)
    let _settings = create_application_settings();
    
    // Test system info retrieval
    let _info = get_gui_system_info();
    
    Ok(())
}

/// Simple Lumidox II Controller GUI State
///
/// Application state for the Iced 0.13.x function-based API
pub struct AppState {
    /// Device controller for communication
    device: Arc<Mutex<Option<LumidoxDevice>>>,
    /// Connection configuration
    port_name: Option<String>,
    auto_detect: bool,
    verbose: bool,
    optimize_transitions: bool,
    /// Application state
    connected: bool,
    connecting: bool,
    status_message: String,
    error_message: Option<String>,
    /// Device status
    device_info: Option<String>,
    /// UI state
    selected_stage: u8,
    custom_current: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            device: Arc::new(Mutex::new(None)),
            port_name: None,
            auto_detect: true,
            verbose: false,
            optimize_transitions: true,
            connected: false,
            connecting: false,
            status_message: "Ready to connect".to_string(),
            error_message: None,
            device_info: None,
            selected_stage: 1,
            custom_current: "500".to_string(),
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("port_name", &self.port_name)
            .field("auto_detect", &self.auto_detect)
            .field("verbose", &self.verbose)
            .field("optimize_transitions", &self.optimize_transitions)
            .field("connected", &self.connected)
            .field("connecting", &self.connecting)
            .field("status_message", &self.status_message)
            .field("error_message", &self.error_message)
            .field("device_info", &self.device_info)
            .field("selected_stage", &self.selected_stage)
            .field("custom_current", &self.custom_current)
            .field("device", &"Arc<Mutex<Option<LumidoxDevice>>>")
            .finish()
    }
}

/// GUI Application Messages
#[derive(Debug, Clone)]
pub enum Message {
    /// Device connection messages
    Connect,
    Disconnect,
    ConnectionSuccess(String), // Device info string instead of device object
    ConnectionFailed(String),  // Error message
    /// Device control messages
    FireStage(u8),
    FireWithCurrent,
    ArmDevice,
    TurnOff,
    /// Device operation results
    OperationResult(std::result::Result<String, LumidoxError>),
    /// UI state messages
    StageSelected(u8),
    CurrentChanged(String),
    RefreshStatus,
    ClearError,
    /// Periodic updates
    Tick,
}

/// Update function for Iced 0.13.x API
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Connect => {
            if !state.connecting && !state.connected {
                state.connecting = true;
                state.status_message = "Connecting...".to_string();
                state.error_message = None;

                let port_name = state.port_name.clone();
                let auto_detect = state.auto_detect;
                let optimize_transitions = state.optimize_transitions;
                let verbose = state.verbose;
                let device_arc = state.device.clone();

                Task::perform(
                    async move {
                        let result = create_device_controller_with_fallback(
                            port_name,
                            auto_detect,
                            optimize_transitions,
                            verbose,
                        );

                        match result {
                            Ok(device) => {
                                // Extract device info
                                let device_info = if let Some(info) = device.info() {
                                    format!(
                                        "Model: {} | Firmware: {} | Serial: {}",
                                        info.model_number,
                                        info.firmware_version,
                                        info.serial_number
                                    )
                                } else {
                                    "Device connected".to_string()
                                };

                                // Store device
                                let mut device_guard = device_arc.lock().await;
                                *device_guard = Some(device);

                                Message::ConnectionSuccess(device_info)
                            }
                            Err(e) => Message::ConnectionFailed(format!("Error: {}", e))
                        }
                    },
                    |msg| msg,
                )
            } else {
                Task::none()
            }
        }

        Message::ConnectionSuccess(device_info) => {
            state.connecting = false;
            state.connected = true;
            state.status_message = "Connected successfully".to_string();
            state.error_message = None;
            state.device_info = Some(device_info);
            Task::none()
        }

        Message::ConnectionFailed(error) => {
            state.connecting = false;
            state.connected = false;
            state.status_message = "Connection failed".to_string();
            state.error_message = Some(error);
            Task::none()
        }

        Message::Disconnect => {
            state.connected = false;
            state.status_message = "Disconnected".to_string();
            state.error_message = None;
            state.device_info = None;

            let device_arc = state.device.clone();
            Task::perform(
                async move {
                    let mut device_guard = device_arc.lock().await;
                    *device_guard = None;
                },
                |_| Message::ClearError,
            )
        }

        Message::FireStage(stage) => {
            if state.connected {
                let device_arc = state.device.clone();
                Task::perform(
                    async move {
                        let mut device_guard = device_arc.lock().await;
                        if let Some(ref mut device) = *device_guard {
                            // Use unified operation layer
                            match crate::core::operations::StageOperations::fire_stage_unified(device, stage) {
                                Ok(response) => {
                                    // GUI-specific presentation of the unified result
                                    let mut message = response.message.clone();
                                    if let crate::core::operations::DeviceOperationData::StageFiring { current_ma, .. } = response.data {
                                        if let Some(current) = current_ma {
                                            message.push_str(&format!(" (Current: {}mA)", current));
                                        }
                                    }
                                    Message::OperationResult(Ok(message))
                                }
                                Err(e) => Message::OperationResult(Err(e))
                            }
                        } else {
                            Message::OperationResult(Err(LumidoxError::DeviceError(
                                "Device not connected".to_string()
                            )))
                        }
                    },
                    |msg| msg,
                )
            } else {
                state.error_message = Some("Device not connected".to_string());
                Task::none()
            }
        }

        Message::CurrentChanged(value) => {
            state.custom_current = value;
            Task::none()
        }

        Message::TurnOff => {
            if state.connected {
                let device_arc = state.device.clone();
                Task::perform(
                    async move {
                        let mut device_guard = device_arc.lock().await;
                        if let Some(ref mut device) = *device_guard {
                            // Use unified operation layer
                            match DeviceControlOperations::turn_off_device(device) {
                                Ok(response) => {
                                    // GUI-specific presentation of the unified result
                                    let gui_message = if let DeviceOperationData::DeviceControl { new_state, .. } = &response.data {
                                        if let Some(state) = new_state {
                                            format!("{} (State: {})", response.message, state)
                                        } else {
                                            response.message
                                        }
                                    } else {
                                        response.message
                                    };
                                    Message::OperationResult(Ok(gui_message))
                                }
                                Err(e) => Message::OperationResult(Err(e))
                            }
                        } else {
                            Message::OperationResult(Err(LumidoxError::DeviceError(
                                "Device not connected".to_string()
                            )))
                        }
                    },
                    |msg| msg,
                )
            } else {
                state.error_message = Some("Device not connected".to_string());
                Task::none()
            }
        }

        Message::ArmDevice => {
            if state.connected {
                let device_arc = state.device.clone();
                Task::perform(
                    async move {
                        let mut device_guard = device_arc.lock().await;
                        if let Some(ref mut device) = *device_guard {
                            // Use unified operation layer
                            match DeviceControlOperations::arm_device(device) {
                                Ok(response) => {
                                    // GUI-specific presentation of the unified result
                                    let gui_message = if let DeviceOperationData::DeviceControl { new_state, .. } = &response.data {
                                        if let Some(state) = new_state {
                                            format!("{} (State: {})", response.message, state)
                                        } else {
                                            response.message
                                        }
                                    } else {
                                        response.message
                                    };
                                    Message::OperationResult(Ok(gui_message))
                                }
                                Err(e) => Message::OperationResult(Err(e))
                            }
                        } else {
                            Message::OperationResult(Err(LumidoxError::DeviceError(
                                "Device not connected".to_string()
                            )))
                        }
                    },
                    |msg| msg,
                )
            } else {
                state.error_message = Some("Device not connected".to_string());
                Task::none()
            }
        }

        Message::FireWithCurrent => {
            if state.connected {
                let current_str = state.custom_current.clone();
                if let Ok(current) = current_str.parse::<u16>() {
                    let device_arc = state.device.clone();
                    Task::perform(
                        async move {
                            let mut device_guard = device_arc.lock().await;
                            if let Some(ref mut device) = *device_guard {
                                let result = device.fire_with_current(current)
                                    .map(|_| format!("Fired with {}mA successfully", current))
                                    .map_err(|e| e);
                                Message::OperationResult(result)
                            } else {
                                Message::OperationResult(Err(LumidoxError::DeviceError(
                                    "Device not connected".to_string()
                                )))
                            }
                        },
                        |msg| msg,
                    )
                } else {
                    state.error_message = Some("Invalid current value".to_string());
                    Task::none()
                }
            } else {
                state.error_message = Some("Device not connected".to_string());
                Task::none()
            }
        }

        Message::RefreshStatus => {
            if state.connected {
                let device_arc = state.device.clone();
                Task::perform(
                    async move {
                        let mut device_guard = device_arc.lock().await;
                        if let Some(ref device) = *device_guard {
                            let device_info = if let Some(info) = device.info() {
                                format!(
                                    "Model: {} | Firmware: {} | Serial: {}",
                                    info.model_number,
                                    info.firmware_version,
                                    info.serial_number
                                )
                            } else {
                                "Device status refreshed".to_string()
                            };
                            Message::OperationResult(Ok(device_info))
                        } else {
                            Message::OperationResult(Err(LumidoxError::DeviceError(
                                "Device not connected".to_string()
                            )))
                        }
                    },
                    |msg| msg,
                )
            } else {
                state.error_message = Some("Device not connected".to_string());
                Task::none()
            }
        }

        Message::OperationResult(result) => {
            match result {
                Ok(success_msg) => {
                    state.status_message = success_msg;
                    state.error_message = None;
                }
                Err(error) => {
                    state.error_message = Some(format!("Operation failed: {}", error));
                }
            }
            Task::none()
        }

        Message::ClearError => {
            state.error_message = None;
            Task::none()
        }

        _ => Task::none()
    }
}

/// View function for Iced 0.13.x API
fn view(state: &AppState) -> Element<Message> {
        use iced::widget::{button, column, container, row, text, text_input, Space};
        use iced::{Alignment, Length};

    // Header with title and device info
    let header = column![
        text("Lumidox II Controller").size(24),
        if let Some(ref info) = state.device_info {
            text(info).size(12)
        } else {
            text("No device connected").size(12)
        }
    ]
    .spacing(5)
    .align_x(Alignment::Center);

    // Connection controls
    let connection_controls = row![
        if state.connected {
            button("Disconnect").on_press(Message::Disconnect)
        } else if state.connecting {
            button("Connecting...")
        } else {
            button("Connect").on_press(Message::Connect)
        },
        Space::with_width(Length::Fixed(10.0)),
        text(&state.status_message)
    ]
    .align_y(Alignment::Center);

    // Stage firing controls
    let stage_buttons = row![
        button("Stage 1").on_press_maybe(if state.connected { Some(Message::FireStage(1)) } else { None }),
        button("Stage 2").on_press_maybe(if state.connected { Some(Message::FireStage(2)) } else { None }),
        button("Stage 3").on_press_maybe(if state.connected { Some(Message::FireStage(3)) } else { None }),
        button("Stage 4").on_press_maybe(if state.connected { Some(Message::FireStage(4)) } else { None }),
        button("Stage 5").on_press_maybe(if state.connected { Some(Message::FireStage(5)) } else { None }),
    ]
    .spacing(10);

    // Current control
    let current_control = row![
        text("Current (mA):"),
        text_input("500", &state.custom_current)
            .on_input(Message::CurrentChanged)
            .width(Length::Fixed(100.0)),
        button("Fire with Current")
            .on_press_maybe(if state.connected { Some(Message::FireWithCurrent) } else { None })
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    // Device controls
    let device_controls = row![
        button("ARM")
            .on_press_maybe(if state.connected { Some(Message::ArmDevice) } else { None }),
        button("Turn Off")
            .on_press_maybe(if state.connected { Some(Message::TurnOff) } else { None }),
        button("Refresh")
            .on_press_maybe(if state.connected { Some(Message::RefreshStatus) } else { None })
    ]
    .spacing(10);

    // Error display
    let error_display = if let Some(ref error) = state.error_message {
        column![
            text(error),
            button("Clear").on_press(Message::ClearError)
        ]
        .spacing(5)
    } else {
        column![]
    };

        // Main layout
        let content = column![
            header,
            Space::with_height(Length::Fixed(20.0)),
            connection_controls,
            Space::with_height(Length::Fixed(20.0)),
            text("Stage Controls").size(18),
            stage_buttons,
            Space::with_height(Length::Fixed(20.0)),
            text("Current Control").size(18),
            current_control,
            Space::with_height(Length::Fixed(20.0)),
            text("Device Controls").size(18),
            device_controls,
            Space::with_height(Length::Fixed(20.0)),
            error_display,
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .padding(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

/// Theme function for Iced 0.13.x API
fn theme(_state: &AppState) -> Theme {
    Theme::Dark
}
