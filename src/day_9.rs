use std::collections::HashSet;

use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 9");


    let mut snake = Snake::new();

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

    //snake.print();
    println!("HEAD {:?}\nTAIL {:?}", snake.head, snake.tail);
    println!("PART_1 {}", snake.visited.len());
    println!("PART_2 {}", snake.last_knot_visits.len());
   // println!("{:?}", snake.rope);
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    id: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Snake {
    head: Position,
    tail: Position,
    rope: Vec<Position>,
    visited: HashSet<Position>,
    last_knot_visits: HashSet<Position>,
    positions: HashSet<Position>,
}

impl Snake {
    pub const DIR: [(i32, i32); 4] = [
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
    ];

    fn new() -> Snake {
        let head = Position { id: 0, x: 0, y: 0 };
        let tail = Position { id: 1, x: 0, y: 0 };
        let mut rope = Vec::new();
        let visited = HashSet::new();
        let last_knot_visits = HashSet::new();
        let positions = HashSet::new();

        for i in 0..10 {
            rope.push(Position { id: i, x: 0, y: 0 });
        }

        Snake {
            head,
            tail,
            rope,
            visited,
            last_knot_visits,
            positions,
        }
    }


    fn make_move(&mut self, dir: &Direction) {
        move_head(dir, &mut self.head);
        move_tail(&self.head, &mut self.tail);

        move_head(dir, &mut self.rope[0]);

        let mut last_knot = self.rope[0];
        for i in 1..self.rope.len() {
            let mut tail = self.rope.get_mut(i).unwrap();
            move_tail(&last_knot, &mut tail);
            last_knot = tail.clone();
        }

        self.last_knot_visits.insert(self.rope[self.rope.len() - 1]);

        // let delta = Self::DIR[*dir as usize];
        // println!("DELTA {:?}", delta);
        // self.head.x += delta.0;
        // self.head.y += delta.1;
        //
        // let row_diff = self.head.x - self.tail.x;
        // let col_diff = self.head.y - self.tail.y;
        //
        // if row_diff == 0 && col_diff.abs() > 1 {
        //     self.tail.y += col_diff.signum();
        // } else if col_diff == 0 && row_diff.abs() > 1 {
        //     self.tail.x += row_diff.signum();
        // } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
        //     self.tail.x += row_diff.signum();
        //     self.tail.y += col_diff.signum();
        // }

        self.positions.insert(self.head);
        self.visited.insert(self.tail);
    }

    fn print(&self) {
        let max_x = self.positions.iter().map(|p| p.x).max().unwrap() + 3;
        let min_x = self.positions.iter().map(|p| p.x).min().unwrap() - 3;


        let max_y = self.positions.iter().map(|p| p.y).max().unwrap() + 3;
        let min_y = self.positions.iter().map(|p| p.y).min().unwrap() - 3;


        for mut y in min_y..max_y {
            y *= -1;
            for x in min_x..max_x {
                let position = Position { id: 0, x, y };

                if self.head == position {
                    print!("H");
                } else if self.tail == position {
                    print!("T");
                } else if self.visited.contains(&position) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn move_head(dir: &Direction, head: &mut Position) {
    let delta = Snake::DIR[*dir as usize];

    head.x += delta.0;
    head.y += delta.1;
}

fn move_tail(head: &Position, tail: &mut Position) {
    let row_diff = head.x - tail.x;
    let col_diff = head.y - tail.y;

    if row_diff == 0 && col_diff.abs() > 1 {
        tail.y += col_diff.signum();
    } else if col_diff == 0 && row_diff.abs() > 1 {
        tail.x += row_diff.signum();
    } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
        tail.x += row_diff.signum();
        tail.y += col_diff.signum();
    }
}