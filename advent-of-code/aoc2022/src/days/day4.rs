pub fn run () {
    const INPUT: &str = include_str!("../../input/day4.txt");

    part1(INPUT);

    println!();
}

fn part1(input: &str) -> i32 {
    println!("Running day 4 part 1");

    let mut assignment_pairs_fully_contained = 0;



    5
}

fn part2(input: &str) {
    println!("Running day 4 part 2");

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(TEST_INPUT));
    }
}
