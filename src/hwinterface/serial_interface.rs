use crate::hwinterface::{
    HWInterface,
};
use serialport::prelude::*;

pub enum PortSettings {
    PortName(String),
    BaudRate(u32),
    DataBits(serialport::DataBits),
    FlowControl(serialport::FlowControl),
    Parity(serialport::Parity),
    StopBits(serialport::StopBits),
    Timeout(std::time::Duration),
}

pub struct SerialInterface {
    interface_settings: Vec<PortSettings>,
    port: Option<Box<dyn SerialPort>>,
}

impl HWInterface for SerialInterface {
    type Settings = PortSettings;
   
    fn apply(&mut self, settings: Vec<PortSettings>) {
        self.interface_settings = settings
    }

    /// Return a list of the possible settings
    fn default_settings() -> Vec<PortSettings> {
        vec![
            PortSettings::PortName(String::from("")),
            PortSettings::BaudRate(250000),
            PortSettings::DataBits(serialport::DataBits::Eight),
            PortSettings::FlowControl(serialport::FlowControl::None),
            PortSettings::Parity(serialport::Parity::None),
            PortSettings::StopBits(serialport::StopBits::One),
            PortSettings::Timeout(std::time::Duration::from_millis(250)),
        ]
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

    }

    fn reset(&mut self) {
    }

    fn receive(&mut self) -> String {
        String::from("")
    }

    fn send(&mut self, msg: String) {
       
    }
}
