use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 4");

    if let Ok(lines) = read_lines("assets/day_4.txt") {


        let mut sum1 = 0;
        let mut sum2 = 0;

        for line in lines {
            let str = line.expect("line error");


            if let Some((lhs, rhs)) = str.split_once(",") {
                let first = Pair::of(lhs);
                let second = Pair::of(rhs);

                if first.is_inside(&second) || second.is_inside(&first) {
                    sum1 += 1;
                }

                if first.overlaps(&second) || second.overlaps(&first){
                    sum2 += 1;
                }
            }
        }

        println!("PART_1 SUM {}", sum1);
        println!("PART_2 SUM {}", sum2);
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Pair {
        start: u32,
        end: u32,
    }

    impl Pair {
        fn of(str: &str) -> Pair {
            let array: Vec<&str> = str.split("-").collect();
            let start = array[0].parse::<u32>().expect("parse error");
            let end = array[1].parse::<u32>().expect("parse error");
            Pair {
                start,
                end,
            }
        }

        fn is_inside(&self, pair: &Pair) -> bool {
            self.start <= pair.start && self.end >= pair.end
        }

        fn overlaps(&self, pair: &Pair) -> bool {
            (self.start <= pair.start && self.end >= pair.start)
                || (self.end <= pair.start && self.end >= pair.end)
        }
    }
}