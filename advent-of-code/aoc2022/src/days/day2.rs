use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("input/day2.txt").expect("Failed to read input");

    part1(&input);

    println!();

    part2(&input);
}

// opponent is A-C, player is X-Z
#[derive(Debug)]
enum Input {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn map_input_to_move(input: &Input) -> Move {
    match input {
        Input::A => Move::Rock,
        Input::B => Move::Paper,
        Input::C => Move::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn part1(input: &str) {
    println!("Running day 2 part 1");
    let mut total_score = 0;

    for game in input.lines() {
        let split: Vec<Input> = game
            .split_whitespace()
            .map(|x| match x {
                "A" => Input::A,
                "B" => Input::B,
                "C" => Input::C,
                "X" => Input::X,
                "Y" => Input::Y,
                "Z" => Input::Z,
                _ => panic!("Invalid input"),
            })
            .collect();

        let opponent_move = split.get(0).unwrap_or(&Input::A);
        let player_move = split.get(1).unwrap_or(&Input::X);
        let mut score = 0;

        let outcome = match player_move {
            Input::X => match opponent_move {
                Input::A => Outcome::Tie,
                Input::B => Outcome::Loss,
                Input::C => Outcome::Win,
                _ => panic!("Invalid input"),
            },
            Input::Y => match opponent_move {
                Input::A => Outcome::Win,
                Input::B => Outcome::Tie,
                Input::C => Outcome::Loss,
                _ => panic!("Invalid input"),
            },
            Input::Z => match opponent_move {
                Input::A => Outcome::Loss,
                Input::B => Outcome::Win,
                Input::C => Outcome::Tie,
                _ => panic!("Invalid input"),
            },
            _ => panic!("Invalid input"),
        };

        match outcome {
            Outcome::Win => score += 6,
            Outcome::Tie => score += 3,
            Outcome::Loss => (),
        }

        match player_move {
            Input::X => score += 1,
            Input::Y => score += 2,
            Input::Z => score += 3,
            _ => (),
        }

        total_score += score;
    }

    println!("Total score: {}", total_score);
    // 11150
}

fn part2(input: &str) {
    println!("Running day 2 part 2");

    let mut total_score = 0;

    for game in input.lines() {
        let split: Vec<Input> = game
            .split_whitespace()
            .map(|x| match x {
                "A" => Input::A,
                "B" => Input::B,
                "C" => Input::C,
                "X" => Input::X,
                "Y" => Input::Y,
                "Z" => Input::Z,
                _ => panic!("Invalid input"),
            })
            .collect();

        let opponent_move = split.get(0).expect("Invalid input");
        let mapped_opponent_move = map_input_to_move(opponent_move);
        let intended_outcome = split.get(1).expect("Invalid input");
        let mut score = 0;

        let mapped_outcome = match intended_outcome {
            Input::X => Outcome::Loss,
            Input::Y => Outcome::Tie,
            Input::Z => Outcome::Win,
            _ => panic!("Invalid input"),
        };

        // println!("mapped_opponent_move: {mapped_opponent_move:?}");
        // println!("mapped_outcome: {mapped_outcome:?}");
        // println!("score: {score}");

        match mapped_outcome {
            Outcome::Win => score += 6,
            Outcome::Tie => score += 3,
            Outcome::Loss => (),
        }

        // println!("score: {score}");

        let player_move = match mapped_opponent_move {
            Move::Rock => match mapped_outcome {
                Outcome::Win => Move::Paper,
                Outcome::Tie => Move::Rock,
                Outcome::Loss => Move::Scissors,
            },
            Move::Paper => match mapped_outcome {
                Outcome::Win => Move::Scissors,
                Outcome::Tie => Move::Paper,
                Outcome::Loss => Move::Rock,
            },
            Move::Scissors => match mapped_outcome {
                Outcome::Win => Move::Rock,
                Outcome::Tie => Move::Scissors,
                Outcome::Loss => Move::Paper,
            },
        };

        // println!("player_move: {player_move:?}");

        match player_move {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scissors => score += 3,
        }

        total_score += score;
    }

    println!("Total score: {}", total_score);

    // 8295
}
