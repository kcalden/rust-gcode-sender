mod hwinterface;
mod machine;
mod parser;
mod util;
use hwinterface::*;
use serial_interface::{SerialInterface};

fn main() {
    let new_serial_interface: SerialInterface = Default::default();
    println!("COM Port -> {}", new_serial_interface.settings.port_name);
    println!("Baudrate -> {}", new_serial_interface.settings.serial_settings.baud_rate);
    
    let port_list = new_serial_interface.list();
    for port in port_list.iter() {
        println!("{}", port);
    }
}
