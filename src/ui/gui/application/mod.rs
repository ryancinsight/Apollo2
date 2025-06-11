//! Application logic coordination module for Lumidox II Controller GUI
//!
//! This module provides the main application logic coordination with complete
//! Iced Application trait implementation. It defines the LumidoxApplication
//! struct that serves as the primary GUI application entry point with proper
//! state management, message handling, and UI construction.
//!
//! The application module includes:
//! - Complete Iced Application trait implementation
//! - Unified state management with device controller integration
//! - Message routing through HandlerCoordinator
//! - UI construction using LayoutComponents and ComponentFactory
//! - Proper async Command handling and subscription management
//! - Integration with CLI arguments and existing device operations

// Import application modules
pub mod state;
pub mod messages;
pub mod handlers;

// Re-export application components for easy access
pub use state::{UnifiedState, AppView, NotificationType};
pub use messages::{Message, DeviceMessage, UiMessage, MessageFactory};
pub use handlers::HandlerCoordinator;

use iced::{Application, Task, Element, Subscription, Theme, executor};
use crate::ui::gui::components::{ComponentFactory, LayoutComponents};
use crate::device::LumidoxDevice;
use crate::core::{LumidoxError, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Main Lumidox II Controller GUI Application
/// 
/// Implements the Iced Application trait to provide a complete GUI interface
/// for the Lumidox II Controller with proper state management, message handling,
/// and device integration.
#[derive(Debug)]
pub struct LumidoxApplication {
    /// Unified application and device state
    state: UnifiedState,
    /// Shared device controller for async operations
    device: Arc<Mutex<Option<LumidoxDevice>>>,
}

impl Application for LumidoxApplication {
    /// Message type for the application
    type Message = Message;
    
    /// Theme type for styling
    type Theme = Theme;
    
    /// Executor type for async operations
    type Executor = executor::Default;
    
    /// Flags type for initialization parameters
    /// (port_name, auto_detect, verbose, optimize_transitions)
    type Flags = (Option<String>, bool, bool, bool);
    
    /// Create new application instance
    /// 
    /// Initializes the application with CLI arguments and sets up initial state.
    /// 
    /// # Arguments
    /// * `flags` - Initialization flags from CLI arguments
    /// 
    /// # Returns
    /// * `(Self, Task<Self::Message>)` - Application instance and initial task
    ///
    /// # Example
    /// ```
    /// let (app, task) = LumidoxApplication::new((None, true, false, true));
    /// ```
    fn new(flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let (port_name, auto_detect, verbose, optimize_transitions) = flags;
        
        // Initialize unified state with device configuration
        let state = UnifiedState::with_device_config(
            port_name.clone(),
            auto_detect,
            verbose,
            optimize_transitions,
        );
        
        // Initialize shared device controller
        let device = Arc::new(Mutex::new(None));
        
        let app = Self {
            state,
            device,
        };
        
        // Initial task - attempt auto-connection if enabled
        let initial_task = if auto_detect {
            Task::perform(
                async move {
                    Message::Device(DeviceMessage::Connect {
                        port_name,
                        auto_detect: true,
                        verbose,
                        optimize_transitions,
                    })
                },
                |msg| msg,
            )
        } else {
            Task::none()
        };

        (app, initial_task)
    }
    
    /// Get application title
    /// 
    /// Returns the window title for the application.
    /// 
    /// # Returns
    /// * `String` - Application window title
    /// 
    /// # Example
    /// ```
    /// let title = app.title();
    /// assert_eq!(title, "Lumidox II Controller");
    /// ```
    fn title(&self) -> String {
        "Lumidox II Controller".to_string()
    }
    
    /// Update application state
    /// 
    /// Processes messages and updates application state using the HandlerCoordinator.
    /// 
    /// # Arguments
    /// * `message` - Message to process
    /// 
    /// # Returns
    /// * `Task<Self::Message>` - Task for further processing
    ///
    /// # Example
    /// ```
    /// let task = app.update(message);
    /// ```
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        // Delegate message handling to HandlerCoordinator
        HandlerCoordinator::update(&mut self.state, message, self.device.clone())
    }
    
    /// Build application view
    /// 
    /// Constructs the complete UI using LayoutComponents and ComponentFactory
    /// based on current application state.
    /// 
    /// # Returns
    /// * `Element<Self::Message>` - Complete application UI
    /// 
    /// # Example
    /// ```
    /// let ui = app.view();
    /// ```
    fn view(&self) -> Element<Self::Message> {
        // Create UI components using ComponentFactory
        let stage_controls = ComponentFactory::create_stage_controls(&self.state);
        let device_controls = ComponentFactory::create_device_controls(&self.state);
        let current_controls = ComponentFactory::create_current_controls(&self.state);
        let status_display = ComponentFactory::create_status_display(&self.state);
        let info_display = ComponentFactory::create_info_display(&self.state);
        
        // Get device info for header
        let device_info = self.state.get_device_info_display();
        
        // Create complete layout using LayoutComponents
        LayoutComponents::create_application_layout(
            stage_controls,
            device_controls,
            current_controls,
            status_display,
            info_display,
            self.state.is_device_connected(),
            device_info.as_deref(),
        )
    }
    
    /// Create application subscription
    /// 
    /// Sets up periodic updates and other subscriptions for the application.
    /// 
    /// # Returns
    /// * `Subscription<Self::Message>` - Application subscriptions
    /// 
    /// # Example
    /// ```
    /// let subscription = app.subscription();
    /// ```
    fn subscription(&self) -> Subscription<Self::Message> {
        // Create periodic update subscription
        HandlerCoordinator::create_subscription()
    }
    
    /// Get application theme
    /// 
    /// Returns the current theme for the application.
    /// 
    /// # Returns
    /// * `Self::Theme` - Current application theme
    /// 
    /// # Example
    /// ```
    /// let theme = app.theme();
    /// ```
    fn theme(&self) -> Self::Theme {
        // Use default theme for now - could be made configurable
        Theme::default()
    }
}

impl LumidoxApplication {
    /// Create application with specific configuration
    /// 
    /// Alternative constructor for creating application with specific
    /// configuration parameters.
    /// 
    /// # Arguments
    /// * `port_name` - Optional specific port name
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `verbose` - Enable verbose output
    /// * `optimize_transitions` - Enable optimized stage transitions
    /// 
    /// # Returns
    /// * `(Self, Task<Message>)` - Application instance and initial task
    ///
    /// # Example
    /// ```
    /// let (app, task) = LumidoxApplication::with_config(None, true, false, true);
    /// ```
    pub fn with_config(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> (Self, Task<Message>) {
        Self::new((port_name, auto_detect, verbose, optimize_transitions))
    }
    
    /// Get current application state
    /// 
    /// Returns a reference to the current unified application state.
    /// 
    /// # Returns
    /// * `&UnifiedState` - Current application state
    /// 
    /// # Example
    /// ```
    /// let state = app.get_state();
    /// ```
    pub fn get_state(&self) -> &UnifiedState {
        &self.state
    }
    
    /// Get mutable application state
    /// 
    /// Returns a mutable reference to the current unified application state.
    /// 
    /// # Returns
    /// * `&mut UnifiedState` - Mutable current application state
    /// 
    /// # Example
    /// ```
    /// let state = app.get_state_mut();
    /// ```
    pub fn get_state_mut(&mut self) -> &mut UnifiedState {
        &mut self.state
    }
    
    /// Get device controller
    /// 
    /// Returns a reference to the shared device controller.
    /// 
    /// # Returns
    /// * `&Arc<Mutex<Option<LumidoxDevice>>>` - Shared device controller
    /// 
    /// # Example
    /// ```
    /// let device = app.get_device();
    /// ```
    pub fn get_device(&self) -> &Arc<Mutex<Option<LumidoxDevice>>> {
        &self.device
    }
    
    /// Check if device is connected
    /// 
    /// Convenience method to check device connection status.
    /// 
    /// # Returns
    /// * `bool` - True if device is connected
    /// 
    /// # Example
    /// ```
    /// if app.is_device_connected() {
    ///     println!("Device is connected");
    /// }
    /// ```
    pub fn is_device_connected(&self) -> bool {
        self.state.is_device_connected()
    }
    
    /// Check if application is busy
    /// 
    /// Convenience method to check if application is currently busy
    /// with an operation.
    /// 
    /// # Returns
    /// * `bool` - True if application is busy
    /// 
    /// # Example
    /// ```
    /// if app.is_busy() {
    ///     println!("Application is busy");
    /// }
    /// ```
    pub fn is_busy(&self) -> bool {
        self.state.is_busy()
    }
    
    /// Get current view
    /// 
    /// Returns the current application view.
    /// 
    /// # Returns
    /// * `AppView` - Current application view
    /// 
    /// # Example
    /// ```
    /// let view = app.get_current_view();
    /// ```
    pub fn get_current_view(&self) -> AppView {
        self.state.app_state.current_view.clone()
    }
    
    /// Set device controller
    /// 
    /// Updates the shared device controller with a new device instance.
    /// 
    /// # Arguments
    /// * `device` - New device instance
    /// 
    /// # Example
    /// ```
    /// app.set_device_controller(Some(device));
    /// ```
    pub async fn set_device_controller(&self, device: Option<LumidoxDevice>) {
        let mut device_guard = self.device.lock().await;
        *device_guard = device;
    }
    
    /// Validate application state
    /// 
    /// Performs validation checks on the current application state
    /// to ensure consistency and correctness.
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if state is valid, Err with validation error
    /// 
    /// # Example
    /// ```
    /// if let Err(error) = app.validate_state() {
    ///     eprintln!("State validation failed: {}", error);
    /// }
    /// ```
    pub fn validate_state(&self) -> Result<()> {
        // Validate state consistency
        crate::ui::gui::application::state::StateManager::validate_state_consistency(&self.state)
    }
    
    /// Synchronize with device
    /// 
    /// Synchronizes application state with the current device state.
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if synchronization successful
    /// 
    /// # Example
    /// ```
    /// if let Err(error) = app.synchronize_with_device().await {
    ///     eprintln!("Synchronization failed: {}", error);
    /// }
    /// ```
    pub async fn synchronize_with_device(&mut self) -> Result<()> {
        let device_guard = self.device.lock().await;
        let device_ref = device_guard.as_ref();
        
        crate::ui::gui::application::state::StateManager::synchronize_with_device(
            &mut self.state,
            device_ref,
        )
    }
    
    /// Handle application shutdown
    /// 
    /// Performs cleanup operations when the application is shutting down.
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if shutdown successful
    /// 
    /// # Example
    /// ```
    /// if let Err(error) = app.shutdown().await {
    ///     eprintln!("Shutdown failed: {}", error);
    /// }
    /// ```
    pub async fn shutdown(&mut self) -> Result<()> {
        // Disconnect device if connected
        if self.state.is_device_connected() {
            let mut device_guard = self.device.lock().await;
            if let Some(device) = device_guard.take() {
                // Perform any necessary device cleanup
                drop(device);
            }
        }
        
        // Clear application state
        self.state.clear();
        
        Ok(())
    }
}
