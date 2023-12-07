use crate::utils::read_lines;

pub fn run() {
    println!("\nDay 2");


    let mut part_1_score = 0;
    let mut part_2_score = 0;
    if let Ok(lines) = read_lines("assets/day_2.txt") {
        for line in lines {
            let str = &*line.expect("line error");
            part_1(str, &mut part_1_score);
            part_2(str, &mut part_2_score);
        }
    }

    println!("PART_1 SCORE {}", part_1_score);
    println!("PART_2 SCORE {}", part_2_score);
}

fn part_1(line: &str, total_score: &mut usize) {
    let mut chars = line.chars();
    let opponent = chars.nth(0).unwrap();
    let me = chars.nth(1).unwrap();

    let opponent_move = Move::of(opponent);
    let my_move = Move::of(me);

    let end_result = my_move.end_result(opponent_move);

    let points = end_result.inherent_points() + my_move.inherent_points();

    *total_score = *total_score + points;

    //println!("{:?} {:?} {:?} POINTS {points}", opponent_move, end_result, my_move);
}

fn part_2(line: &str, total_score: &mut usize) {
    let mut chars = line.chars();
    let opponent = chars.nth(0).unwrap();
    let me = chars.nth(1).unwrap();

    let opponent_move = Move::of(opponent);
    let end_result = EndResult::of(me);
    let my_move = end_result.matching_move(opponent_move);

    //let win_score = my_move.win_score(opponent_move);
    //let round_score = win_score + my_move.move_score();

    let points = end_result.inherent_points() + my_move.inherent_points();


    //println!("{:?} {:?} {:?} POINTS {points}", opponent_move, end_result, my_move);

    *total_score = *total_score + points;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move")
    }

    fn drawing_move(self) -> Self {
        self
    }
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn of(c: char) -> Move {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("invalid move")
        }
    }

    fn end_result(self, theirs: Move) -> EndResult {
        match (self, theirs) {
            (Move::Rock, Move::Rock) => EndResult::Draw,
            (Move::Rock, Move::Paper) => EndResult::Loss,
            (Move::Rock, Move::Scissors) => EndResult::Win,
            (Move::Paper, Move::Rock) => EndResult::Win,
            (Move::Paper, Move::Paper) => EndResult::Draw,
            (Move::Paper, Move::Scissors) => EndResult::Loss,
            (Move::Scissors, Move::Rock) => EndResult::Loss,
            (Move::Scissors, Move::Paper) => EndResult::Win,
            (Move::Scissors, Move::Scissors) => EndResult::Draw,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum EndResult {
    Win,
    Draw,
    Loss,
}

impl EndResult {
    fn of(c: char) -> EndResult {
        match c {
            'X' => EndResult::Loss,
            'Y' => EndResult::Draw,
            'Z' => EndResult::Win,
            _ => panic!("invalid EndResult")
        }
    }

    fn inherent_points(self) -> usize {
        match self {
            EndResult::Win => 6,
            EndResult::Draw => 3,
            EndResult::Loss => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            EndResult::Win => theirs.winning_move(),
            EndResult::Draw => theirs.drawing_move(),
            EndResult::Loss => theirs.losing_move(),
        }
    }
}

