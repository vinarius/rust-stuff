pub fn run() {
    let input = include_str!("../../input/day3.txt");

    part1(&input);
}

fn part1(input: &str) -> i32 {
    println!("Running day 3 part 1");
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn part1_test() {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp\n
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n
PmmdzqPrVvPwwTWBwg\n
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n
ttgJtRGJQctTZtZT\n
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, part1(test_input));
    }
}
