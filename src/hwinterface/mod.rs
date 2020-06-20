pub mod serial_interface;

use std::collections::HashMap;

/// Trait for a hardware interface
///
/// This is what will actually control the machine
pub trait HardwareInterface {
    type Settings;

    /// Apply settings to the object
    fn apply(&mut self, settings: Self::Settings);

    /// Default settings for the interface
    // fn default_settings() -> Self::Settings;

    /// Close connection to the device
    fn disconnect(&mut self);

    /// Find and list connectable devices
    fn list() -> Vec<String>;

    /// Open connection to the device
    fn connect(&mut self);

    /// Reset device
    fn reset(&mut self);

    /// Receive data from the device
    fn receive(&mut self) -> Option<String>;

    /// Send data to the device
    fn send(&mut self, msg: String);
}
