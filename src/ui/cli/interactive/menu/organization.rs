//! Menu organization and structure for interactive CLI
//!
//! This module defines the organizational structure of menu options,
//! including grouping, categorization, and hierarchical relationships.
//! 
//! The organization system provides:
//! - Logical grouping of related menu options
//! - Hierarchical menu structure definition
//! - Menu section management
//! - Option metadata and descriptions

/// Menu section definitions for organized display
/// 
/// This enum defines the major sections of the interactive menu
/// to provide clear organization and user navigation.
#[derive(Debug, Clone, PartialEq)]
pub enum MenuSection {
    /// Primary stage firing operations (options 1-5)
    StageFiring,
    /// Device control operations (options 6-9)
    DeviceControl,
    /// Device status and information (options 10-12)
    DeviceStatus,
    /// Stage parameter information (options 13-15)
    StageParameters,
    /// Current control operations (option 16)
    CurrentControl,
}

/// Menu option metadata for enhanced organization
/// 
/// Contains detailed information about each menu option including
/// its purpose, requirements, and organizational context.
#[derive(Debug, Clone)]
pub struct MenuOption {
    /// The numeric option identifier
    pub number: u8,
    /// Short description of the option
    pub description: String,
    /// The section this option belongs to
    pub section: MenuSection,
    /// Whether this option requires additional user input
    pub requires_input: bool,
    /// Whether this option performs a device operation
    pub is_device_operation: bool,
}

/// Menu organization utilities and structure management
pub struct MenuOrganization;

impl MenuOrganization {
    /// Get all menu options with their metadata
    /// 
    /// Returns a complete list of all available menu options with
    /// their associated metadata for organization and display purposes.
    /// 
    /// # Returns
    /// * `Vec<MenuOption>` - Complete list of menu options with metadata
    /// 
    /// # Example
    /// ```
    /// let options = MenuOrganization::get_all_options();
    /// assert_eq!(options.len(), 16);
    /// ```
    pub fn get_all_options() -> Vec<MenuOption> {
        vec![
            // Stage firing options (1-5)
            MenuOption {
                number: 1,
                description: "Turn on stage 1".to_string(),
                section: MenuSection::StageFiring,
                requires_input: false,
                is_device_operation: true,
            },
            MenuOption {
                number: 2,
                description: "Turn on stage 2".to_string(),
                section: MenuSection::StageFiring,
                requires_input: false,
                is_device_operation: true,
            },
            MenuOption {
                number: 3,
                description: "Turn on stage 3".to_string(),
                section: MenuSection::StageFiring,
                requires_input: false,
                is_device_operation: true,
            },
            MenuOption {
                number: 4,
                description: "Turn on stage 4".to_string(),
                section: MenuSection::StageFiring,
                requires_input: false,
                is_device_operation: true,
            },
            MenuOption {
                number: 5,
                description: "Turn on stage 5".to_string(),
                section: MenuSection::StageFiring,
                requires_input: false,
                is_device_operation: true,
            },
            
            // Device control options (6-9)
            MenuOption {
                number: 6,
                description: "Turn on stage with specific current".to_string(),
                section: MenuSection::DeviceControl,
                requires_input: true,
                is_device_operation: true,
            },
            MenuOption {
                number: 7,
                description: "Arm device (prepare for firing)".to_string(),
                section: MenuSection::DeviceControl,
                requires_input: false,
                is_device_operation: true,
            },
            MenuOption {
                number: 8,
                description: "Turn off device".to_string(),
                section: MenuSection::DeviceControl,
                requires_input: false,
                is_device_operation: true,
            },
            MenuOption {
                number: 9,
                description: "Quit program".to_string(),
                section: MenuSection::DeviceControl,
                requires_input: false,
                is_device_operation: true,
            },
            
            // Device status options (10-12)
            MenuOption {
                number: 10,
                description: "Show device status".to_string(),
                section: MenuSection::DeviceStatus,
                requires_input: false,
                is_device_operation: false,
            },
            MenuOption {
                number: 11,
                description: "Read remote mode state".to_string(),
                section: MenuSection::DeviceStatus,
                requires_input: false,
                is_device_operation: false,
            },
            MenuOption {
                number: 12,
                description: "Read ARM/FIRE current settings".to_string(),
                section: MenuSection::DeviceStatus,
                requires_input: false,
                is_device_operation: false,
            },
            
            // Stage parameter options (13-15)
            MenuOption {
                number: 13,
                description: "Show complete stage parameters".to_string(),
                section: MenuSection::StageParameters,
                requires_input: true,
                is_device_operation: false,
            },
            MenuOption {
                number: 14,
                description: "Read stage ARM current".to_string(),
                section: MenuSection::StageParameters,
                requires_input: true,
                is_device_operation: false,
            },
            MenuOption {
                number: 15,
                description: "Read stage voltage parameters".to_string(),
                section: MenuSection::StageParameters,
                requires_input: true,
                is_device_operation: false,
            },
            
            // Current control options (16)
            MenuOption {
                number: 16,
                description: "Set ARM current".to_string(),
                section: MenuSection::CurrentControl,
                requires_input: true,
                is_device_operation: true,
            },
        ]
    }
    
    /// Get options for a specific menu section
    /// 
    /// Returns all menu options that belong to the specified section
    /// for section-specific processing or display.
    /// 
    /// # Arguments
    /// * `section` - The menu section to get options for
    /// 
    /// # Returns
    /// * `Vec<MenuOption>` - Options belonging to the specified section
    /// 
    /// # Example
    /// ```
    /// let stage_options = MenuOrganization::get_options_for_section(MenuSection::StageFiring);
    /// assert_eq!(stage_options.len(), 5);
    /// ```
    pub fn get_options_for_section(section: MenuSection) -> Vec<MenuOption> {
        Self::get_all_options()
            .into_iter()
            .filter(|option| option.section == section)
            .collect()
    }
    
    /// Get a menu option by its number
    /// 
    /// Retrieves the metadata for a specific menu option by its number.
    /// 
    /// # Arguments
    /// * `number` - The menu option number to retrieve
    /// 
    /// # Returns
    /// * `Option<MenuOption>` - The menu option if found, None otherwise
    /// 
    /// # Example
    /// ```
    /// let option = MenuOrganization::get_option(5);
    /// assert!(option.is_some());
    /// assert_eq!(option.unwrap().description, "Turn on stage 5");
    /// ```
    pub fn get_option(number: u8) -> Option<MenuOption> {
        Self::get_all_options()
            .into_iter()
            .find(|option| option.number == number)
    }
    
    /// Get the section title for display
    /// 
    /// Returns a formatted title string for a menu section suitable
    /// for display in the user interface.
    /// 
    /// # Arguments
    /// * `section` - The menu section to get the title for
    /// 
    /// # Returns
    /// * `&'static str` - The formatted section title
    /// 
    /// # Example
    /// ```
    /// let title = MenuOrganization::get_section_title(MenuSection::DeviceStatus);
    /// assert_eq!(title, "--- Device Status & Information ---");
    /// ```
    pub fn get_section_title(section: MenuSection) -> &'static str {
        match section {
            MenuSection::StageFiring => "--- Stage Firing Operations ---",
            MenuSection::DeviceControl => "--- Device Control ---",
            MenuSection::DeviceStatus => "--- Device Status & Information ---",
            MenuSection::StageParameters => "--- Stage Parameter Information ---",
            MenuSection::CurrentControl => "--- Current Control ---",
        }
    }
    
    /// Get the total number of available menu options
    /// 
    /// Returns the total count of available menu options for
    /// validation and bounds checking purposes.
    /// 
    /// # Returns
    /// * `usize` - Total number of menu options
    /// 
    /// # Example
    /// ```
    /// let count = MenuOrganization::get_option_count();
    /// assert_eq!(count, 16);
    /// ```
    pub fn get_option_count() -> usize {
        Self::get_all_options().len()
    }
    
    /// Check if an option is a device operation
    /// 
    /// Determines whether a menu option performs an actual device
    /// operation (as opposed to just reading information).
    /// 
    /// # Arguments
    /// * `number` - The menu option number to check
    /// 
    /// # Returns
    /// * `bool` - True if the option performs a device operation
    /// 
    /// # Example
    /// ```
    /// assert!(MenuOrganization::is_device_operation(7)); // Arm device
    /// assert!(!MenuOrganization::is_device_operation(10)); // Show status
    /// ```
    pub fn is_device_operation(number: u8) -> bool {
        Self::get_option(number)
            .map(|option| option.is_device_operation)
            .unwrap_or(false)
    }
}
