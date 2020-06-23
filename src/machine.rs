use crate::hwinterface;
use crate::parser;
use crate::util;

type HWInterface = Box<dyn hwinterface::HardwareInterface>;

enum MsgType {
    Normal(String),
    Error(String),
}

/// Contains information aobut the machine
pub struct Machine {
    /// Unique name of the machine
    name: String,

    /// Path to the loaded GCode file
    filepath: String,

    /// Lines of the program
    /// 
    /// TODO: Create enums to describe each line
    program: Vec<parser::Command>,

    /// Current line to be sent to the machine
    program_index: u32,

    /// Communication interface
    interface: Option<HWInterface>,

    /// Communication history
    comm_history: Vec<MsgType>,
}

impl Machine {
    fn load_from_file(&mut self, file_path: &str) {
        if let Some(contents) = util::load_file(file_path) {
            self.program = parser::parse_file(&contents);
            self.comm_history.push(MsgType::Normal(
                String::from(format!("Loaded file {}", file_path))));
        }else{
            self.comm_history.push(MsgType::Error(
                String::from(format!("Failed to load file {}", file_path))));
        }
    }
}