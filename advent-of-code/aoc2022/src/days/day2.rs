use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("input/day2.txt").unwrap();

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

enum Outcome {
    Win,
    Loss,
    Tie,
}

fn part1(input: &str) {
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

        let opponent_move = split.get(0).unwrap_or(&Input::A);
        let intended_outcome = split.get(1).unwrap_or(&Input::X);
        let mut score = 0;

        let outcome = match intended_outcome {
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
