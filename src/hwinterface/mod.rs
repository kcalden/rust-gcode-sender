pub mod serial_interface;

use std::collections::HashMap;

// pub enum SettingType {
//     StringType(String),
//     FloatType(f32),
//     IntType(i32),
// }

// pub type Settings<SettingEnums> = HashMap<String, SettingEnums>;

/// Trait for a hardware interface
///
/// This is what will actually control the machine
pub trait HWInterface {
    type Settings;

    /// Apply settings to the object
    fn apply(&mut self, settings: Vec<Self::Settings>);

    /// Default settings for the interface
    fn default_settings() -> Vec<Self::Settings>;

    /// Close connection to the device
    fn disconnect(&mut self);

    /// Find and list connectable devices
    fn list() -> Vec<String>;

    /// Open connection to the device
    fn connect(&mut self);

    /// Reset device
    fn reset(&mut self);

    /// Receive data from the device
    fn receive(&mut self) -> String;

    /// Send data to the device
    fn send(&mut self, msg: String);
}
