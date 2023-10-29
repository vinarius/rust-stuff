use std::fs::read_to_string;

pub fn run() {
    part1();
}

// opponent is A-C, player is X-Z
#[derive(Debug)]
enum PlayerInput {
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

fn get_score_for_round(opponent_move: &PlayerInput, player_move: &PlayerInput) -> i32 {
    let mut score = 0;

    let outcome = match player_move {
        PlayerInput::X => match opponent_move {
            PlayerInput::A => Outcome::Tie,
            PlayerInput::B => Outcome::Loss,
            PlayerInput::C => Outcome::Win,
            _ => panic!("Invalid input"),
        },
        PlayerInput::Y => match opponent_move {
            PlayerInput::A => Outcome::Win,
            PlayerInput::B => Outcome::Tie,
            PlayerInput::C => Outcome::Loss,
            _ => panic!("Invalid input"),
        },
        PlayerInput::Z => match opponent_move {
            PlayerInput::A => Outcome::Loss,
            PlayerInput::B => Outcome::Win,
            PlayerInput::C => Outcome::Tie,
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
        PlayerInput::X => score += 1,
        PlayerInput::Y => score += 2,
        PlayerInput::Z => score += 3,
        _ => (),
    }

    score
}

fn part1() {
    println!("Running day 2 part 2");
    let input = read_to_string("input/day2.txt").unwrap();
    let mut total_score = 0;

    for game in input.lines() {
        let split: Vec<PlayerInput> = game
            .split_whitespace()
            .map(|x| match x {
                "A" => PlayerInput::A,
                "B" => PlayerInput::B,
                "C" => PlayerInput::C,
                "X" => PlayerInput::X,
                "Y" => PlayerInput::Y,
                "Z" => PlayerInput::Z,
                _ => panic!("Invalid input"),
            })
            .collect();

        let opponent_move = split.get(0).unwrap_or(&PlayerInput::A);
        let player_move = split.get(1).unwrap_or(&PlayerInput::X);

        total_score += get_score_for_round(opponent_move, player_move);
    }

    println!("Total score: {}", total_score);
    // 10814 too low
}

