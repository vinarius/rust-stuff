pub fn run() {
    const INPUT: &str = include_str!("../../input/day4.txt");

    part1(INPUT);

    println!();

    part2(INPUT);
}

fn part1(input: &str) -> i32 {
    println!("Running day 4 part 1");

    let mut assignment_pairs_fully_contained = 0;
    const ERROR: &str = "problem statement guarantees a min and max range separated by a -";

    for line in input.lines() {
        let (range_one, range_two) = line
            .split_once(',')
            .expect("problem statement guarantees two ranges separated by a comma");

        // println!("range_one: {}", range_one);
        // println!("range_two: {}", range_two);

        let (min_one, max_one) = range_one.split_once('-').expect(ERROR);

        // println!("min_one: {min_one}");
        // println!("max_one: {max_one}");

        let (min_two, max_two) = range_two.split_once('-').expect(ERROR);

        // println!("min_two: {min_two}");
        // println!("max_two: {max_two}");

        let min_one: i32 = min_one.parse().expect("min_one should be a number");
        let max_one: i32 = max_one.parse().expect("max_one should be a number");
        let min_two: i32 = min_two.parse().expect("min_two should be a number");
        let max_two: i32 = max_two.parse().expect("max_two should be a number");
        let is_fully_contained = (min_one >= min_two && max_one <= max_two)
            || (min_two >= min_one && max_two <= max_one);

        // println!("is_fully_contained: {is_fully_contained}");

        if is_fully_contained {
            assignment_pairs_fully_contained += 1;
        }
    }

    println!("assignment_pairs_fully_contained: {assignment_pairs_fully_contained}",);

    assignment_pairs_fully_contained
}

fn part2(input: &str) -> i32 {
    println!("Running day 4 part 2");

    let mut assignment_pairs_partially_overlapping = 0;
    const ERROR: &str = "problem statement guarantees a min and max range separated by a -";

    for line in input.lines() {
                let (range_one, range_two) = line
            .split_once(',')
            .expect("problem statement guarantees two ranges separated by a comma");

        // println!("range_one: {}", range_one);
        // println!("range_two: {}", range_two);

        let (min_one, max_one) = range_one.split_once('-').expect(ERROR);

        // println!("min_one: {min_one}");
        // println!("max_one: {max_one}");

        let (min_two, max_two) = range_two.split_once('-').expect(ERROR);

        // println!("min_two: {min_two}");
        // println!("max_two: {max_two}");

        let min_one: i32 = min_one.parse().expect("min_one should be a number");
        let max_one: i32 = max_one.parse().expect("max_one should be a number");
        let min_two: i32 = min_two.parse().expect("min_two should be a number");
        let max_two: i32 = max_two.parse().expect("max_two should be a number");
        let is_partially_contained = (max_one >= min_two && min_one <= max_two) || (max_two >= min_one && max_two <= min_one);

        // println!("is_fully_contained: {is_fully_contained}");

        if is_partially_contained {
            assignment_pairs_partially_overlapping += 1;
        }
    }

    println!("assignment_pairs_partially_overlapping: {assignment_pairs_partially_overlapping}");

    assignment_pairs_partially_overlapping
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

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(TEST_INPUT));
    }
}
