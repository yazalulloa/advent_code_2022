use crate::utils::read_lines;

const debug: bool = false;

pub fn run() {
    println!("\nDay 9");

    let mut positions: Vec<Position> = Vec::new();

    positions.push(Position::new(0, 0));
    let mut head_x: i32 = 0;
    let mut head_y: i32 = 0;

    let mut tail_x: i32 = 0;
    let mut tail_y: i32 = 0;


    if let Ok(lines) = read_lines("assets/day_9.txt") {
        for line in lines {
            let str = line.expect("line error");
            let direction = Direction::from_char(str.chars().nth(0).unwrap());
            let number = str[1..].trim().parse::<i32>().expect("parse error");


            if debug {
                println!("{} {:?} {}", str, direction, number);
            }

            let mut i: i32 = 0;


            while i < number {
                i += 1;

                match direction {
                    Direction::Left => head_x -= 1,
                    Direction::Right => head_x += 1,
                    Direction::Up => head_y += 1,
                    Direction::Down => head_y -= 1,
                }

                if let None = positions.iter().find(|position| position.x == head_x && position.y == head_y) {
                    positions.push(Position::new(head_x, head_y));
                }

                if let Some(position) = positions.iter_mut().find(|position| position.x == tail_x && position.y == tail_y) {
                    if !position.visited {
                        position.visit();
                    }
                }


                if debug {
                    println!("positions {}", positions.len());
                }

                if head_x == tail_x {
                    let diff = (head_y.abs() - tail_y.abs()).abs();
                    if diff > 1 {
                        if head_y > tail_y {
                            tail_y += 1;
                        } else {
                            tail_y -= 1;
                        }
                    }
                } else if head_y == tail_y {
                    let diff = (head_x.abs() - tail_x.abs()).abs();
                    if diff > 1 {
                        if head_x > tail_x {
                            tail_x += 1;
                        } else {
                            tail_x -= 1;
                        }
                    }
                } else {
                    let it_touches =
                        (head_x == tail_x + 1 && (head_y == tail_y + 1 || head_y == tail_y - 1)) ||
                            (head_x == tail_x - 1 && (head_y == tail_y + 1 || head_y == tail_y - 1)) ||
                            (head_y == tail_y + 1 && (head_x == tail_x + 1 || head_x == tail_x - 1)) ||
                            (head_y == tail_y - 1 && (head_x == tail_x + 1 || head_x == tail_x - 1));

                    if !it_touches {
                        if head_x > tail_x {
                            tail_x += 1;
                        } else {
                            tail_x -= 1;
                        }

                        if head_y > tail_y {
                            tail_y += 1;
                        } else {
                            tail_y -= 1;
                        }
                    }
                }

                if debug {
                    print_positions(head_y, head_x, tail_y, tail_x, &positions);

                    println!("{}", "=".repeat(20))
                }
            }
        }
    }

    println!("HEAD {} {}", head_x, head_y);
    println!("TAIL {} {}", tail_x, tail_y);

    let sum = positions.iter().filter(|position| position.visited).map(|_p| 1i32)
        .sum::<i32>();

    println!("PART_1 {}", sum);

    if debug {
        print_positions(head_y, head_x, tail_y, tail_x, &positions);
    }
}

fn print_positions(head_y: i32, head_x: i32, tail_y: i32, tail_x: i32, vec: &Vec<Position>) {
    if false {
        return;
    }

    let max_y = vec.iter().map(|p| p.y).max().unwrap();
    let min_y = vec.iter().map(|p| p.y).min().unwrap();

    let max_x = vec.iter().map(|p| p.x).max().unwrap();
    let min_x = vec.iter().map(|p| p.x).min().unwrap();

    let mut current_y = max_y;

    while current_y >= min_y {
        let mut current_x = min_x;

        while current_x <= max_x {
            if current_x == head_x && current_y == head_y {
                print!("H");
            } else if current_x == tail_x && current_y == tail_y {
                print!("T");
            } else if let Some(position) = vec.iter().find(|p| p.x == current_x && p.y == current_y) {
                if position.visited {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!("_");
            }

            current_x += 1;
        }

        println!();
        current_y -= 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(character: char) -> Direction {
        match character {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("unknown direction"),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
    visited: bool,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x,
            y,
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }

    fn move_to(&mut self, direction: Direction, number: i32) {
        match direction {
            Direction::Left => self.x -= number,
            Direction::Right => self.x += number,
            Direction::Up => self.y += number,
            Direction::Down => self.y -= number,
        }
    }
}