use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Elf {
    id: u32,
    items: Vec<u32>,
    items_total: u32,
}


impl Display for Elf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {:?}", self.id, self.items_total, self.items)
    }
}

impl Elf {
    fn new(id: u32, items: &Vec<u32>) -> Elf {
        Elf {
            id,
            items: items.to_vec(),
            items_total: items.iter().sum(),
        }
    }
}

pub fn run() {
    println!("\nDay 1");
    let elves = parse();

    highest(&elves);
    sum_top(3, &elves);
}

fn highest(elves: &Vec<Elf>) {
    let highest = elves.first()
        .unwrap();
    println!("HIGHEST {} ", highest);
}

fn sum_top(capacity: usize, elves: &Vec<Elf>) {
    println!("TOP {}", capacity);
    let tops = &elves[0..capacity];
    for elf in tops {
        println!("{}", elf)
    }

    let sum = tops.iter().map(|s| s.items_total).sum::<u32>();
    println!("SUM {}", sum);
}


fn parse() -> Vec<Elf> {
    let file_str: String = fs::read_to_string("assets/day_1.txt")
        .expect("file error")
        .parse()
        .expect("parse error");

    let split = file_str.split("\n");
    let mut elves: Vec<Elf> = Vec::new();
    let mut counter: u32 = 0;

    let mut items: Vec<u32> = Vec::new();

    for line in split {
        if line.is_empty() && !items.is_empty() {
            counter += 1;
            elves.push(Elf::new(counter, &items.clone()));
            items.clear();
            continue;
        }

        let item = line.trim().parse::<u32>().unwrap();
        items.push(item);
    }

    if !items.is_empty() {
        counter += 1;
        elves.push(Elf::new(counter, &items.clone()));
        items.clear();
    }

    elves.sort_by(|lhs, rhs| rhs.items_total.cmp(&lhs.items_total));
    elves
}
