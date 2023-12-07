use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 3");

    let mut sum1 = 0;

    let mut sum2 = 0;

    let mut group: Vec<String> = Vec::with_capacity(3);
    if let Ok(lines) = read_lines("assets/day_3.txt") {
        for line in lines {
            let str = line.expect("line error");
            part_1(&str, &mut sum1);

            if group.len() == 3 {
                part_2(&group, &mut sum2);
                group.clear();
                group.push(str);
            } else {
                group.push(str);
            }
        }
    }

    part_2(&group, &mut sum2);

    println!("PART_1 SUM {}", sum1);
    println!("PART_2 SUM {}", sum2);
}

fn part_1(str: &str, sum: &mut u32) {
    let half = str.len() / 2;

    let first = &str[0..half];
    let second = &str[half..str.len()];
    let mut matches: Vec<char> = Vec::new();

    for c_first in first.chars() {
        for c_second in second.chars() {
            if c_first == c_second {
                if !matches.contains(&c_first) {
                    matches.push(c_first);
                }
            }
        }
    }

    if !matches.is_empty() {
        if matches.len() > 1 {
            println!("matches {:?}", matches);
        }
        // println!("{:?}", matches);

        for c in &matches {
            *sum = *sum + priority(&c);
            // println!("{:?} {}", matches, priority);
        }
    }
}

fn part_2(group: &Vec<String>, sum: &mut u32) {
    let badge = get_badge(group);
    let i = priority(&badge);
    *sum = *sum + i;
}

fn get_badge(group: &Vec<String>) -> char {
    let first = group.get(0).unwrap();
    let second = group.get(1).unwrap();
    let third = group.get(2).unwrap();

    let mut matches: Vec<char> = Vec::new();
    for f_char in first.chars() {
        for s_char in second.chars() {
            if f_char == s_char {
                if !matches.contains(&f_char) {
                    matches.push(f_char);
                }
            }
        }
    }

    for t_char in third.chars() {
        if matches.contains(&t_char) {
            return t_char;
        }
    }

    panic!("no badge found in group {:?}", group)
}

fn priority(c: &char) -> u32 {
    let minus = if c.is_lowercase() { 96 } else { 38 };
    *c as u32 - minus
}