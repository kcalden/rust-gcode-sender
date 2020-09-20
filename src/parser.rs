use gcode;
use regex::Regex;
use rayon::prelude::*;

pub enum Command {
    Move(gcode::GCode),
    Generic(gcode::GCode),
    Home(String),
    HostCommand(String),
    ToolChange(gcode::GCode),
    Nothing,
}

/// Parse file into a list of commands
pub fn parse(file_contents: &str) -> Vec<Command> {
    let lines:Vec<&str> = file_contents.lines()
    .filter(|src_line| {
        // Return host commands
        if src_line.starts_with(";@") {
            return true;
        }
        // Skip pure space or comments
        if src_line.len() == 0 || src_line.starts_with(";") {
            return false;
        }else{
            return true;
        }
    }).collect();

    // Finds home flags
    let regex_home_flags = Regex::new(r"^\s*(G28(\s+(X|Y|Z))*)").unwrap();

    let parse_iter = lines.par_iter()
    .map(|src_line| {
        let src_line = src_line.trim();
        // Matches host commands
        if src_line.starts_with(";@"){
            let hcmd = src_line.replace(";@", "");
            return Command::HostCommand(hcmd);
        }
        
        // Handle special case of G28 with flags (3D printer firmwares)
        if src_line.starts_with("G28") {
            if let Some(caps) = regex_home_flags.captures(src_line){
                return Command::Home(String::from(&caps[1]));
            }
        }
        
        // Now, we can start parsing
        if let Some(gc) = gcode::parse(src_line).next() {
            match gc.mnemonic() {
                gcode::Mnemonic::General => {
                    match gc.major_number() {
                        0 | 1 => {
                            return Command::Move(gc);
                        },
                        _ => return Command::Generic(gc)
                    }
                },
                gcode::Mnemonic::ToolChange => return Command::ToolChange(gc),
                _ => return Command::Generic(gc)
            }
        }else{
            return Command::Nothing;
        }
    });

    parse_iter.collect()
}