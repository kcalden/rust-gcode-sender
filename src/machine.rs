use crate::hwinterface;

type HWInterface = Box<dyn hwinterface::HardwareInterface>;

/// Contains information aobut the machine
struct Machine {
    /// Unique name of the machine
    name: String,

    /// Path to the loaded GCode file
    filepath: String,

    /// Lines of the program
    /// 
    /// TODO: Create enums to describe each line
    program: Vec<String>,

    /// Current line to be sent to the machine
    program_index: u32,

    /// Communication interface
    interface: Option<HWInterface>,

    /// Communication history
    comm_history: Vec<String>,
}
