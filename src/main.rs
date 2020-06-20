mod hwinterface;
use hwinterface::*;
use serial_interface::{SerialInterface, PortSettings};

fn main() {
    let serial_port_settings = SerialInterface::default_settings();
    println!("COM Port -> {}", serial_port_settings.port_name);
    println!("Baudrate -> {}", serial_port_settings.serial_settings.baud_rate);

    let port_list = SerialInterface::list();
    for port in port_list.iter() {
        println!("{}", port);
    }
}
