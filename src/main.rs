mod hwinterface;
mod serial_interface;

use hwinterface::HWInterface;
use serial_interface::SerialInterface;

fn main() {
    let port_list = SerialInterface::list();

    for port in port_list.iter() {
        println!("{}", port);
    }
}
