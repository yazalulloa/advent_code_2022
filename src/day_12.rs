use crate::utils::read_lines;

const DEBUG: bool = false;

pub fn run() {
    println!("\nDay 12");

    if let Ok(lines) = read_lines("assets/day_12.txt") {
        for line_res in lines {
            let line = line_res.expect("line error");
            if !line.is_empty() {}
        }
    }
}