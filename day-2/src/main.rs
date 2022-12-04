use std::fs;

const INPUT_FILE_PATH: &str =
    "/home/austin/Documents/coding-competitions/advent-of-code/2022/day-2/input.txt";

enum MoveTypes {
    Rock,
    Paper,
    Scissors
}

enum OutcomeTypes {
    Lose,
    Draw,
    Win
}

fn compute_score_part_1 (my_move: &MoveTypes, opponent_move: &MoveTypes) -> i32 {
    let shape_score = match my_move {
        MoveTypes::Rock => 1,
        MoveTypes::Paper => 2,
        MoveTypes::Scissors => 3
    };

    let win_score = match my_move {
        MoveTypes::Paper => match opponent_move {
            MoveTypes::Rock => 6,
            MoveTypes::Paper => 3,
            MoveTypes::Scissors => 0,
        }
        MoveTypes::Rock => match opponent_move {
            MoveTypes::Rock => 3,
            MoveTypes::Paper => 0,
            MoveTypes::Scissors => 6,
        },
        MoveTypes::Scissors => match opponent_move {
            MoveTypes::Rock => 0,
            MoveTypes::Paper => 6,
            MoveTypes::Scissors => 3,
        },
    };

    shape_score + win_score
}

fn compute_total_score_part_1 (input: &String) -> i32 {
    let mut my_last_move = MoveTypes::Paper;
    let mut opponent_last_move = MoveTypes::Paper;
    let mut total_score: i32 = 0;
    for c in input.chars() {
        match c {
            'A' => opponent_last_move = MoveTypes::Rock,
            'B' => opponent_last_move = MoveTypes::Paper,
            'C' => opponent_last_move = MoveTypes::Scissors,
            'X' => my_last_move = MoveTypes::Rock,
            'Y' => my_last_move = MoveTypes::Paper,
            'Z' => my_last_move = MoveTypes::Scissors,
            '\n' => {
                total_score += compute_score_part_1(&my_last_move, &opponent_last_move);
            },
            _ => ()
        };
    }

    total_score
}

fn compute_score_part_2 (outcome: &OutcomeTypes, opponent_move: &MoveTypes) -> i32 {
    let win_score = match outcome {
        OutcomeTypes::Lose => 0,
        OutcomeTypes::Draw => 3,
        OutcomeTypes::Win => 6
    };

    let shape_score = match opponent_move {
        MoveTypes::Rock => match outcome {
            OutcomeTypes::Lose => 3,
            OutcomeTypes::Draw => 1,
            OutcomeTypes::Win => 2
        },
        MoveTypes::Paper => match outcome {
            OutcomeTypes::Lose => 1,
            OutcomeTypes::Draw => 2,
            OutcomeTypes::Win => 3
        },
        MoveTypes::Scissors => match outcome {
            OutcomeTypes::Lose => 2,
            OutcomeTypes::Draw => 3,
            OutcomeTypes::Win => 1
        }
    };

    win_score + shape_score
}

fn compute_total_score_part_2 (input: &String) -> i32 {
    let mut outcome = OutcomeTypes::Draw;
    let mut opponent_last_move = MoveTypes::Paper;
    let mut total_score: i32 = 0;
    for c in input.chars() {
        match c {
            'A' => opponent_last_move = MoveTypes::Rock,
            'B' => opponent_last_move = MoveTypes::Paper,
            'C' => opponent_last_move = MoveTypes::Scissors,
            'X' => outcome = OutcomeTypes::Lose,
            'Y' => outcome = OutcomeTypes::Draw,
            'Z' => outcome = OutcomeTypes::Win,
            '\n' => {
                total_score += compute_score_part_2(&outcome, &opponent_last_move);
            },
            _ => ()
        };
    }

    total_score
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_PATH).expect("Should be able to read the file.");

    let score_part_1 = compute_total_score_part_1(&contents);
    println!("Result part 1: {}", score_part_1);

    let score_part_2 = compute_total_score_part_2(&contents);
    println!("Result part 2: {}", score_part_2);

}
