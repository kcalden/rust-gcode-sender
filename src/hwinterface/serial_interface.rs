use crate::hwinterface::{
    HardwareInterface,
};
use serialport::prelude::*;

/// Serial Port settings
pub struct PortSettings {
    /// Path to the serial port
    pub port_name: String,

    /// Settings for the serial port
    pub serial_settings: serialport::SerialPortSettings,
}

/// Serial Interface
pub struct SerialInterface {
    /// Serial Interface Settings
    settings: PortSettings,

    /// Serial port to communicate through
    port: Option<Box<dyn SerialPort>>,

    /// Serial buffer
    ser_buf: String,

    /// Current line buffer
    line_buf: String,

    /// Stored lines
    lines: Vec<String>,
}

impl HardwareInterface for SerialInterface {
    type Settings = PortSettings;

    /// Store settings to the serial port
    fn apply(&mut self, settings: PortSettings) {
        self.settings = settings;
    }

    /// Return a list of the possible settings
    /// 
    /// # Example
    /// 
    /// ```
    /// let serial_port_settings = SerialInterface::default_settings()
    /// 
    /// // Print the COM port name and baud rate
    /// 
    /// println!("COM Port -> {}", serial_port_settings.port_name);
    /// println!("Baudrate -> {}", serial_port_settings.serial_settings.baud_rate);
    /// ```
    fn default_settings() -> PortSettings {
        PortSettings{ 
            port_name: String::from(""),
            serial_settings: serialport::SerialPortSettings::default(),
        }
    }


    /// List serial ports
    ///
    /// # Example
    /// ```
    /// let port_list = SerialInterface::list();
    /// for port in port_list.iter() {
    ///    println!("{}", port);
    /// }
    /// ```
    fn list() -> Vec<String> {
        let mut portnames = Vec::new();
        if let Ok(port_infos) = serialport::available_ports() {
            for port_info in port_infos.iter() {
                if let serialport::SerialPortType::UsbPort(_) = port_info.port_type {
                    let portname = port_info.port_name.clone();
                    portnames.push(portname);
                }
            }
        }
        portnames
    }

    fn disconnect(&mut self) {
    }

    /// Connect to a serial port
    fn connect(&mut self) {
        self.port = match serialport::open_with_settings(
            &self.settings.port_name, 
            &self.settings.serial_settings
        ) {
            Ok(port) => {
                Some(port)
            },
            Err(_) => {
                eprintln!("Could not open serial port!");
                None
            }
        };

        if let Some(port) = &mut self.port {
            port.write_data_terminal_ready(true);
        }

    }

    /// Trigger DTR reset. This 
    fn reset(&mut self) {
        if let Some(port) = &mut self.port {
            port.write_data_terminal_ready(false);
            port.write_data_terminal_ready(true);
        }
    }

    /// Read a line from the serial buffer if there is a line there
    fn receive(&mut self) -> Option<String> {
        // Read data into the serial buffer
        if let Some(port) = &mut self.port {
            port.read_to_string(&mut self.ser_buf);
        }

        for next_char in self.ser_buf.chars() {
            match next_char {
                // Push the line to the line buffer at a new line
                '\n' | '\r' => {
                    if self.line_buf.len() > 0 {
                        self.lines.push(self.line_buf.clone());
                    }
                    self.line_buf.clear();
                },
                // Or just keep filling the line buffer
                _ => {
                    self.line_buf.push(next_char)
                }
            }
        }

        // Clear the serial buffer
        self.ser_buf.clear();

        // Return a line if one is available
        if self.lines.len() > 0 {
            Some(self.lines.remove(0));
        }
        None
    }

    fn send(&mut self, msg: String) {
       
    }
}