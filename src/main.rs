mod hwinterface;
use hwinterface::*;
use serial_interface::SerialInterface;

fn main() {
    let port_list = SerialInterface::list();
    for port in port_list.iter() {
        println!("{}", port);
    }
}
