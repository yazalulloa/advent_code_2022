use std::fs;

pub fn run() {
    println!("\nDay 6");

    let file_str: String = fs::read_to_string("assets/day_6.txt")
        .expect("file error")
        .parse()
        .expect("parse error");

    let mut marker: Vec<char> = Vec::with_capacity(4);
    let mut marker_index = 0;
    let mut marker_found = false;

    let mut msg_marker: Vec<char> = Vec::with_capacity(14);
    let mut msg_index = 0;
    let mut msg_marker_found = false;


    for (i, character) in file_str.chars().enumerate() {

        if !marker_found {
            if marker.len() == marker.capacity() {
                if has_duplicates(&marker) {
                    marker.remove(0);
                } else {
                    marker_index = i;
                    marker_found = true;
                }
            }

            marker.push(character);
        }

        if !msg_marker_found {
            if msg_marker.len() == msg_marker.capacity() {
                if has_duplicates(&msg_marker) {
                    msg_marker.remove(0);
                } else {
                    msg_index = i;
                    msg_marker_found = true;
                }
            }

            msg_marker.push(character);
        }

       if marker_found && msg_marker_found {
              break;
       }
    }

    println!("PART_1 MARKER {} {}", marker_index, String::from_iter(marker));
    println!("PART_2 MSG_MARKER {} {}", msg_index, String::from_iter(msg_marker));
}

fn has_duplicates(marker: &Vec<char>) -> bool {
    for i in 0..marker.len() {
        for j in 1..marker.len() {

            if i == j {
                continue;
            }

            if marker[i] == marker[j] {
                return true;
            }


           /* let lhs = marker[i];
            let rhs = marker[j];
            let x = lhs == rhs;
            if x {
                println!("{} {} {} {} {x}", i, j, lhs, rhs);
                return true;
            }*/
        }
    }

     false
}