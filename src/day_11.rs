use std::collections::HashMap;

use crate::utils::read_lines;

const DEBUG: bool = false;

pub fn run() {
    println!("\nDay 11");
    let mut monkeys: Vec<Monkey> = Vec::new();
    if let Ok(lines) = read_lines("assets/day_11.txt") {
        let mut monkey_input: Vec<String> = Vec::new();
        for line_res in lines {
            let line = line_res.expect("line error");
            if !line.is_empty() {
                monkey_input.push(line);
                if monkey_input.len() == 6 {
                    let monkey = parse_monkey(&monkey_input);
                    monkeys.push(monkey);
                    monkey_input.clear();
                }
            }
        }
    }

    let mut part1Round = MonkeyRound::new(monkeys.clone(), 20, true);
    let mut part2Round = MonkeyRound::new(monkeys, 10000, false);

    println!("PART 1 {}", part1Round.process());
    println!("PART 2 {}", part2Round.process());
}


fn parse_monkey(input: &Vec<String>) -> Monkey {
    let id_line = input.get(0).unwrap().trim();

    let id = id_line.chars().nth(7).unwrap().to_digit(10).unwrap();

    let items_line = input.get(1).unwrap().trim();
    let items = items_line[16..].split(",").map(|item| item.trim().parse::<u64>().expect("parse error"))
        .collect::<Vec<u64>>();

    let operation_line = input.get(2).unwrap().trim();
    let op_params = operation_line[21..].split(" ").collect::<Vec<&str>>();

    let operation = match op_params[0] {
        "+" => Operation::ADD,
        "*" => Operation::MUL,
        _ => panic!("parse error")
    };
    let op_value = op_params[1];
    let operation_value = match op_value.parse::<u64>() {
        Ok(value) => Some(value),
        Err(_) => None
    };

    let divisor_line = input.get(3).unwrap().trim();
    let test_divisor = divisor_line[19..].parse::<u64>().expect("parse error");

    let true_line = input.get(4).unwrap().trim();
    let if_test_true_monkey = true_line[25..].trim().parse::<u32>().expect("parse error");

    let false_line = input.get(5).unwrap().trim();
    let if_test_false_monkey = false_line[25..].trim().parse::<u32>().expect("parse error");

    Monkey {
        id,
        items,
        operation,
        operation_value,
        test_divisor,
        if_test_true_monkey,
        if_test_false_monkey,
        items_inspected: 0,

    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MonkeyRound {
    monkeys: Vec<Monkey>,
    limit: u64,
    worry_relief: bool,
}

impl MonkeyRound {
    fn new(monkeys: Vec<Monkey>, limit: u64, worry_relief: bool) -> MonkeyRound {
        MonkeyRound {
            monkeys,
            limit,
            worry_relief,
        }
    }

    fn process(&mut self) -> u64 {
        let mut map: HashMap<u32, Vec<u64>> = HashMap::new();
        let divisor_product = self.monkeys.iter().map(|m| m.test_divisor).product::<u64>();
        for i in 0..self.limit {
            for monkey in self.monkeys.iter_mut() {
                //println!("{:?}", monkey);

                if let Some(vec) = map.get_mut(&monkey.id) {
                    monkey.items.append(vec);
                    vec.clear();
                }


                while monkey.items.len() > 0 {
                    if let Some(mut item) = monkey.items.pop() {
                        monkey.items_inspected += 1;

                        let operation_value = match monkey.operation_value {
                            Some(value) => value,
                            None => item
                        };

                        if DEBUG {
                            print!("ROUND {} {} {:?} {} ", i, item, monkey.operation, operation_value);
                        }
                        item %= divisor_product;

                        if DEBUG {
                            print!(" {:?} ", item);
                        }
                        match monkey.operation {
                            Operation::ADD => item += operation_value,
                            Operation::MUL => item *= operation_value,
                        }
                        if DEBUG {
                            print!("=> {} ", item);
                        }
                        if self.worry_relief {
                            item /= 3;
                        }

                        if DEBUG {
                            println!("=> {}", item);
                        }

                        let monkey_id = if item % monkey.test_divisor == 0 {
                            monkey.if_test_true_monkey
                        } else {
                            monkey.if_test_false_monkey
                        };


                        if let Some(vec) = map.get_mut(&monkey_id) {
                            vec.push(item);
                        } else {
                            map.insert(monkey_id, vec![item]);
                        }
                    }
                }
            }

            for monkey in self.monkeys.iter_mut() {
                if let Some(vec) = map.get_mut(&monkey.id) {
                    monkey.items.append(vec);
                    vec.clear();
                }
            }
        }

        self.monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));

        let monkey1 = self.monkeys.get(0).unwrap();
        let monkey2 = self.monkeys.get(1).unwrap();
        monkey1.items_inspected * monkey2.items_inspected
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Monkey {
    id: u32,
    items: Vec<u64>,
    operation: Operation,
    operation_value: Option<u64>,
    test_divisor: u64,
    if_test_true_monkey: u32,
    if_test_false_monkey: u32,
    items_inspected: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    ADD,
    MUL,
}

