pub fn run() {
    const INPUT: &str = include_str!("../../input/day6.txt");

    part1(INPUT);

    // println!();
    //
    // part2(INPUT);
}

fn part1(input: &str) -> i32 {
    println!("Running day 6 part 1");

    println!("input: {input}");

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
