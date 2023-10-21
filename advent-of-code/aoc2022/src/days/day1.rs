use std::fs::read_to_string;

pub fn run() {
    println!("Day 1 executed");

    let input = read_to_string("input/day1.txt").unwrap();

    println!("Part 1: {}", &input.chars().count());
}
