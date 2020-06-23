use gcode;
use regex::Regex;

pub enum Command {
    MoveGCode(gcode::GCode),
    ParsedGCode(gcode::GCode),
    GenericGCode(String),
    HostCommand(String)

}

/// Parse file line by line
pub fn parse_file(file_path: &str) -> Vec<Command> {
    let regex_home_flags = Regex::new(r"^\s*(G28(\s+(X|Y|Z))*)").unwrap();
    let file_contents = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        _ => return Vec::new()
    };
    
    let mut commands = Vec::new();
    
    // Here I'm just going to do my own parsing before passing it to the gcode
    for src_line in file_contents.lines() {
        let src_line = src_line.trim();
        
        // Skip pure space
        if src_line.len() == 0 { continue }
        
        // Matches host commands
        if src_line.starts_with(";@"){
            let hcmd = src_line.replace(";@", "");
            commands.push(Command::HostCommand(hcmd));
        }
        
        // Skip comment if we haven't matched a host command
        if src_line.starts_with(";") {continue}

        // Handle special case of G28 with flags (3D printer firmwares)
        if src_line.starts_with("G28") {
            if let Some(caps) = regex_home_flags.captures(src_line){
                commands.push(Command::GenericGCode(String::from(&caps[1])))
            }
        }

        // Now, we can start parsing
        if let Some(gc) = gcode::parse(src_line).next() {
            if let gcode::Mnemonic::General = gc.mnemonic() {
                match gc.major_number() {
                    0 | 1 => {
                        commands.push(Command::MoveGCode(gc));
                        continue
                    },
                    _ => ()
                }
            }
            commands.push(Command::ParsedGCode(gc));
            continue
        }
    }

    commands
}
