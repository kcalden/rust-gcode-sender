use crate::hwinterface::{HWInterface, Settings, SettingType};
use serialport::prelude::*;

pub struct SerialInterface {
    port_name: String,
    baud_rate: u32,
    port: Option<Box<dyn SerialPort>>,
}

impl HWInterface for SerialInterface {
    fn apply(&mut self, settings: Settings) {
                                            

    }

    fn list_settings() -> Settings {
        let mut settings_list: Settings = Default::default();

        settings_list.insert(String::from("com_port"), SettingType::StringType(String::from("<port path>")));
        settings_list.insert(String::from("baudrate"), SettingType::IntType(250000));

        settings_list
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
