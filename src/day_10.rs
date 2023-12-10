use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 10");


    let mut tube = CathodeRayTube::new();
    if let Ok(lines) = read_lines("assets/day_10.txt") {
        for line_res in lines {
            let line = line_res.expect("line error");

            tube.process(&line);
        }
    }

    println!();
    println!("CYCLE {}", tube.cycles);
    println!("PART_1 {}", tube.signal_strength_sum);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CathodeRayTube {
    register: i32,
    cycles: i32,
    current_pixel: i32,
    pub signal_strength_sum: i32,
}

const MILESTONES: [i32; 6] = [20, 60, 100, 140, 180, 220];

impl CathodeRayTube {
    fn new() -> CathodeRayTube {
        CathodeRayTube {
            register: 1,
            cycles: 0,
            signal_strength_sum: 0,
            current_pixel: 0,
        }
    }

    fn signal_strength(&mut self) -> i32 {
        self.register * self.cycles
    }
    fn increment_cycle(&mut self) {
        self.cycles += 1;

        if MILESTONES.contains(&self.cycles) {
            self.signal_strength_sum += self.signal_strength();
        }

        if self.current_pixel == self.register
            || self.current_pixel == self.register + 1
            || self.current_pixel == self.register - 1 {

            print!("{}", "█");
            //print!("{}", "#");
        } else {
            print!("{}", " ");
        }
        self.current_pixel += 1;
        if self.current_pixel > 39 {
            self.current_pixel = 0;
            println!()
        }
    }
    fn process(&mut self, line: &str) {
        if line == "noop" {
            self.increment_cycle();
        } else {
            let value = line[5..].trim().parse::<i32>().expect("parse error");

            for x in 0..2 {
                self.increment_cycle();
            }

            self.register += value;
        }
    }
}