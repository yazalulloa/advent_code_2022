use std::collections::VecDeque;

use crate::utils::read_lines;

const RADIX: u32 = 10;

pub fn run() {
    println!("\nDay 5");

    let mut stack_strs: Vec<String> = Vec::new();

    let mut stack_found = false;

    let mut stacks: Vec<Stack> = Vec::new();

    let mut stacks2: Vec<Stack> = Vec::new();

    if let Ok(lines) = read_lines("assets/day_5.txt") {
        for line in lines {
            let str = line.expect("line error");

            if !stack_found && !str.is_empty() {
                stack_strs.push(str.clone());
            }

            if str.is_empty() {
                if !stack_found {
                    if let Some(last) = stack_strs.last() {
                        for (i, el) in last.chars().enumerate() {
                            if el.is_digit(RADIX) {
                                let id = el.to_digit(RADIX).expect("parse error");
                                stacks.push(Stack::of(id, i));
                            }
                        }
                    }

                    for row in &stack_strs {
                        for stack in stacks.iter_mut() {
                            if stack.index_key >= row.len() {
                                continue;
                            }

                            if let Some(character) = row.chars().nth(stack.index_key) {
                                if character != ' ' && !character.is_digit(RADIX) {
                                    stack.push_back(character);
                                }
                            }
                        }
                    }

                    for stack in &stacks {
                        stacks2.push(stack.clone());
                    }
                }
                stack_found = true;
            } else {
                if stack_found {
                    let mut moves: Vec<usize> = Vec::new();

                    let mut number = String::from("");
                    for character in str.chars() {
                        if character.is_digit(RADIX) {
                            number.push(character);
                        } else {
                            if !number.is_empty() {
                                moves.push(number.parse::<usize>().expect("parse error"));
                                number.clear();
                            }
                        }
                    }

                    if !number.is_empty() {
                        moves.push(number.parse::<usize>().expect("parse error"));
                        number.clear();
                    }

                    let amount = moves[0];
                    let origin = moves[1] - 1;
                    let dest = moves[2] - 1;

                    {
                        let mut counter = 0;
                        let mut new_values: VecDeque<char> = VecDeque::new();

                        while counter < amount {

                            if let Some(character) = &stacks[origin].pop() {
                                stacks[dest].push_front(*character);
                            }

                            if let Some(character) = &stacks2[origin].pop() {
                                new_values.push_back(*character);
                            }

                            counter += 1;
                        }

                        while !new_values.is_empty() {
                            if let Some(character) = new_values.pop_back() {
                                stacks2[dest].push_front(character);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut code = String::from("");

    for stack in &stacks {
        code.push(stack.first().expect("parse error"));
    }

    println!("PART_1 {}", code);

    code.clear();

    for stack in &stacks2 {
        code.push(stack.first().expect("parse error"));
    }

    println!("PART_2 {}", code);
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Stack {
    id: u32,
    index_key: usize,
    stack: VecDeque<char>,
}

impl Stack {
    fn of(id: u32, index_key: usize) -> Stack {
        Stack::new(id, index_key)
    }
    fn new(id: u32, index_key: usize) -> Stack {
        Stack {
            id,
            index_key,
            stack: VecDeque::new(),
        }
    }

    fn push_back(&mut self, value: char) {
        self.stack.push_back(value);
    }

    fn push_front(&mut self, value: char) {
        self.stack.push_front(value);
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop_front()
    }

    fn first(&self) -> Option<char> {
        self.stack.front().map(|s| *s)
    }
}