use std::collections::HashSet;

use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 9");


    let mut snake = Snake::default();

    if let Ok(lines) = read_lines("assets/day_9.txt") {
        for line in lines {
            let str = line.expect("line error");

            let (dir, dist) = str.split_once(" ").expect("split error");
            let dir = Direction::from_char(dir);
            let dist = dist.parse::<i32>().expect("parse error");

            //  println!("{} {:?} {}", str, dir, dist);
            for _ in 0..dist {
                snake.make_move(&dir);

                // snake.print();
                // println!("{}", "=".repeat(20));
            }


            // println!("END {} {:?} {}", dist, dir, "=".repeat(20));

            /* snake.print();
             println!("{}", "=".repeat(20));*/
        }
    }

    // snake.print();
    println!("HEAD {:?}\nTAIL {:?}", snake.head, snake.tail);
    println!("PART_1 {}", snake.visited.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(character: &str) -> Direction {
        match character {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Snake {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
    positions: HashSet<(i32, i32)>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Snake {
    const DIR: [(i32, i32); 4] = [
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
    ];

    fn make_move(&mut self, dir: &Direction) {
        let delta = Self::DIR[*dir as usize];
        // println!("DELTA {:?}", delta);
        self.head.0 += delta.0;
        self.head.1 += delta.1;

        let row_diff = self.head.0 - self.tail.0;
        let col_diff = self.head.1 - self.tail.1;

        if row_diff == 0 && col_diff.abs() > 1 {
            self.tail.1 += col_diff.signum();
        } else if col_diff == 0 && row_diff.abs() > 1 {
            self.tail.0 += row_diff.signum();
        } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
            self.tail.0 += row_diff.signum();
            self.tail.1 += col_diff.signum();
        }

        self.positions.insert(self.head);
        self.visited.insert(self.tail);
    }

    fn print(&self) {
        let max_x = *self.positions.iter().map(|(x, y)| (x, y).0).max().unwrap() + 3;
        let min_x = *self.positions.iter().map(|(x, y)| (x, y).0).min().unwrap() - 3;


        let max_y = *self.positions.iter().map(|(x, y)| (x, y).1).max().unwrap() + 3;
        let min_y = *self.positions.iter().map(|(x, y)| (x, y).1).min().unwrap() - 3;


        for mut y in min_y..max_y {
            y *= -1;
            for x in min_x..max_x {
                if self.head == (x, y) {
                    print!("H");
                } else if self.tail == (x, y) {
                    print!("T");
                } else if self.visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}