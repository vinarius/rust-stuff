use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../../input/day3.txt");

    part1(&input);
}

fn part1(input: &str) -> i32 {
    println!("Running day 3 part 1");

    let mut sum: i32 = 0;

    for line in input.lines() {
        // println!();
        // println!("line: {line}");

        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

        // println!("first_compartment: {first_compartment}");
        // println!("second_compartment: {second_compartment}");

        let unique_chars: HashSet<char> = first_compartment.chars().collect();
        let item_type = second_compartment
            .chars()
            .find(|x| unique_chars.contains(&x))
            .expect("problem states at least one duplicate is guaranteed");

        // println!("item_type: {item_type}");

        let priority = get_priority(item_type);

        // println!("priority: {priority}");

        sum += priority as i32;
    }

    println!("sum: {sum}");

    sum
}

fn get_priority(item_type: char) -> u8 {
    match item_type {
        'a'..='z' => item_type as u8 - b'a' + 1,
        'A'..='Z' => item_type as u8 - b'A' + 27,
        _ => unreachable!("something here"),
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn part1_test() {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, part1(test_input));
    }
}
