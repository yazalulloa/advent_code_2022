// use std::fs::File;
// use std::io;
// use std::io::BufRead;
// use std::path::Path;
//
// pub fn run() {
//     let mut part_2_score: usize = 0;
//     if let Ok(lines) = read_lines("assets/day_2.txt") {
//         for line in lines {
//             let str = &*line.expect("line error");
//             //part_1(str, &mut part_1_score);
//             part_2(str, &mut part_2_score);
//         }
//     }
//     println!("PART_2 SCORE {}", part_2_score);
// }
//
//
// fn part_2(line: &str, total_score: &mut usize) {
//     let mut chars = line.chars();
//     let first = chars.nth(0).unwrap();
//     let second = chars.nth(1).unwrap();
//
//     let outcome = Outcome::of(second);
//     let theirs = Move::of(first);
//
//     let round = Round {
//         theirs,
//         ours: outcome.matching_move(theirs),
//     };
//
//     *total_score = *total_score + round.our_score();
// }
//
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//     where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
//
// #[derive(Debug, Clone, Copy)]
// struct Round {
//     theirs: Move,
//     ours: Move,
// }
//
//
// #[derive(Debug, Clone, Copy)]
// enum Outcome {
//     Loss,
//     Draw,
//     Win,
// }
//
// #[derive(Debug, Clone, Copy)]
// enum Move {
//     Rock,
//     Paper,
//     Scissors,
// }
//
// impl Round {
//     fn outcome(self) -> Outcome {
//         self.ours.outcome(self.theirs)
//     }
//     fn our_score(self) -> usize {
//         self.ours.inherent_points() + self.outcome().inherent_points()
//     }
// }
//
// impl Outcome {
//     fn inherent_points(self) -> usize {
//         match self {
//             Outcome::Win => 6,
//             Outcome::Draw => 3,
//             Outcome::Loss => 0,
//         }
//     }
//
//     fn of(c: char) -> Outcome {
//         match c {
//             'X' => Outcome::Loss,
//             'Y' => Outcome::Draw,
//             'Z' => Outcome::Win,
//             _ => panic!("not a valid outcome: {c:?}")
//         }
//     }
//
//     fn matching_move(self, theirs: Move) -> Move {
//         match self {
//             Outcome::Win => theirs.winning_move(),
//             Outcome::Draw => theirs.drawing_move(),
//             Outcome::Loss => theirs.losing_move(),
//         }
//     }
// }
//
// impl Move {
//     const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];
//
//     fn winning_move(self) -> Self {
//         Self::ALL_MOVES
//             .iter()
//             .copied()
//             .find(|m| m.beats(self))
//             .expect("at least one move beats us")
//     }
//
//     fn losing_move(self) -> Self {
//         Self::ALL_MOVES
//             .iter()
//             .copied()
//             .find(|&m| self.beats(m))
//             .expect("we beat at least one move")
//     }
//
//     fn drawing_move(self) -> Self {
//         self
//     }
//
//     fn of(c: char) -> Move {
//         match c {
//             'A' | 'X' => Move::Rock,
//             'B' | 'Y' => Move::Paper,
//             'C' | 'Z' => Move::Scissors,
//             _ => panic!("invalid move: {c:?}")
//         }
//     }
//
//     fn beats(self, other: Move) -> bool {
//         matches!(
//             (self, other),
//             (Self::Rock, Self::Scissors)
//                 | (Self::Paper, Self::Rock)
//                 | (Self::Scissors, Self::Paper)
//         )
//     }
//
//     fn outcome(self, theirs: Move) -> Outcome {
//         if self.beats(theirs) {
//             Outcome::Win
//         } else if theirs.beats(self) {
//             Outcome::Loss
//         } else {
//             Outcome::Draw
//         }
//     }
//
//     fn inherent_points(self) -> usize {
//         match self {
//             Move::Rock => 1,
//             Move::Paper => 2,
//             Move::Scissors => 3,
//         }
//     }
// }