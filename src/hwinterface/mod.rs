pub mod serial_interface;

/// Trait for a hardware interface
///
/// This is what will actually control the machine
pub trait HardwareInterface {
    /// Close connection to the device
    fn disconnect(&mut self);

    /// Find and list connectable devices
    fn list(&self) -> Vec<String>;

    /// Open connection to the device
    fn connect(&mut self);

    /// Reset device
    fn reset(&mut self);

    /// Receive data from the device
    fn receive(&mut self) -> Option<String>;

    /// Send data to the device
    fn send(&mut self, msg: String);
}
