use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../../input/day3.txt");

    part1(&input);

    println!();

    part2(&input);
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

fn part2(input: &str) -> i32 {
    println!("Running day 3 part 2");

    let mut sum: i32 = 0;
    let mut lines = Vec::new();

    for line in input.lines() {
        lines.push(line);
    }

    let elf_groups = lines.chunks(3);
    const ERROR: &str = "problem statement guarantees groups of 3";

    for group in elf_groups {
        // println!("group: {group:?}");

        let first_elf = group.get(0).expect(ERROR);
        let second_elf = group.get(1).expect(ERROR);
        let third_elf = group.get(2).expect(ERROR);
        let mut candidate = first_elf.chars().next().expect("expected at least 1 character in the elf group");

        for char in first_elf.chars() {
            if second_elf.contains(&char.to_string()) && third_elf.contains(&char.to_string()) {
                candidate = char;
                break;
            }
        }

        // println!("candidate: {candidate}");

        let priority = get_priority(candidate as char);

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
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_test() {
        assert_eq!(157, part1(TEST_INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(70, part2(TEST_INPUT));
    }
}
