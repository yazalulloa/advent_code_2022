use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 8");

    let mut trees: Vec<Vec<usize>> = Vec::new();

    if let Ok(lines) = read_lines("assets/day_8.txt") {
        for line in lines {
            let str = line.expect("line error");
            let mut vec: Vec<usize> = Vec::with_capacity(str.len());

            for character in str.chars() {
                let number = character.to_digit(10).expect("parse error") as usize;
                vec.push(number);
            }
            trees.push(vec);
        }
    }

    let mut visible_count = trees.len() * 4 - 4;
    let mut scenic_score = 0;
    for (i, vec) in trees.iter().enumerate() {
        if i == 0 || i == trees.len() - 1 {
            continue;
        }

        for (j, &value) in vec.iter().enumerate() {
            if j == 0 || j == vec.len() - 1 {
                continue;
            }

            let visible_left = is_visible_left(value, j, vec);
            let visible_right = is_visible_right(value, j, vec);
            let visible_top = is_visible_top(value, i, j, &trees);
            let visible_bottom = is_visible_bottom(value, i, j, &trees);

            if visible_left.0 || visible_right.0 || visible_top.0 || visible_bottom.0 {
                visible_count += 1;
            }

            let score = visible_left.1
                * visible_right.1
                * visible_top.1 * visible_bottom.1;

            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    println!("PART_1 {}", visible_count);
    println!("PART_2 {}", scenic_score);
}


fn is_visible_left(value: usize, j: usize, row: &Vec<usize>) -> (bool, usize) {
    let mut index = isize::try_from(j).expect("i32 parse error") - 1;

    let mut is_visible = false;
    let mut visible_count = 0;

    while index > -1 {
        let i1 = usize::try_from(index).unwrap();
        let left_value = row[i1];
        is_visible = value > left_value;

        if is_visible || index == 0 {
            visible_count += 1;
        }

        if !is_visible {
            if index != 0 {
                visible_count += 1;
            }

            break;
        }

        index -= 1;
    }

    (is_visible, visible_count)
}

fn is_visible_right(value: usize, j: usize, row: &Vec<usize>) -> (bool, usize) {
    let mut index = j + 1;

    let mut is_visible = false;
    let mut visible_count = 0;

    while index < row.len() {
        let right_value = row[index];
        is_visible = value > right_value;

        if is_visible || index == 0 {
            visible_count += 1;
        }

        if !is_visible {
            if index != 0 {
                visible_count += 1;
            }
            break;
        }

        index += 1;
    }

    (is_visible, visible_count)
}

fn is_visible_top(value: usize, i: usize, j: usize, trees: &Vec<Vec<usize>>) -> (bool, usize) {
    let mut index = isize::try_from(i).expect("i32 parse error") - 1;
    let mut is_visible = false;
    let mut visible_count = 0;

    while index > -1 {
        let vec = trees[usize::try_from(index).unwrap()].clone();
        let top_value = vec[j];
        is_visible = value > top_value;

        if is_visible || index == 0 {
            visible_count += 1;
        }

        if !is_visible {
            if index != 0 {
                visible_count += 1;
            }
            break;
        }

        index -= 1;
    }

    (is_visible, visible_count)
}


fn is_visible_bottom(value: usize, i: usize, j: usize, trees: &Vec<Vec<usize>>) -> (bool, usize) {
    let mut index = i + 1;
    let mut is_visible = false;
    let mut visible_count = 0;

    while index < trees.len() {
        let vec = trees[index].clone();
        let bottom_value = vec[j];
        is_visible = value > bottom_value;

        if is_visible || index == 0 {
            visible_count += 1;
        }

        if !is_visible {
            if index != 0 {
                visible_count += 1;
            }
            break;
        }

        index += 1;
    }

    (is_visible, visible_count)
}