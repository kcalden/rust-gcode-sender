use crate::hwinterface::{
    HWInterface,
};
use serialport::prelude::*;

pub struct PortSettings {
    pub port_name: String,
    pub serial_settings: serialport::SerialPortSettings,
}

pub struct SerialInterface {
    settings: PortSettings,
    port: Option<Box<dyn SerialPort>>,
}

impl HWInterface for SerialInterface {
    type Settings = PortSettings;
   
    fn apply(&mut self, settings: PortSettings) {
        self.settings = settings;
    }

    /// Return a list of the possible settings
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

    fn connect(&mut self) {
        let mut s = serialport::SerialPortSettings::default();
    }

    fn reset(&mut self) {
    }

    fn receive(&mut self) -> String {
        String::from("")
    }

    fn send(&mut self, msg: String) {
       
    }
}