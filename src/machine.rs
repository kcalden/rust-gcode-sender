use crate::hwinterface;

type HWInterface = Box<dyn hwinterface::HardwareInterface>;

struct Machine {
    filepath: String,
    program: Vec<String>,
    interface: Option<HWInterface>,
    comm_history: Vec<String>,
}
