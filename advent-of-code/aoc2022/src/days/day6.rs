pub fn run() {
    const INPUT: &str = include_str!("../../input/day6.txt");

    part1(INPUT);

    // println!();
    //
    // part2(INPUT);
}

struct Character {
    value: char,
    index: i32,
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn part1(input: &str) -> i32 {
    println!("Running day 6 part 1");

    let mut i = 0;
    let mut unique_chars: Vec<Character> = vec![];

    // mjqjpqmgbljsphdztnvjfqwrcgsmlb

    for char in input.chars() {
        i += 1;

        if !unique_chars.contains(&char) {
            unique_chars.push(char);
            j += 1;
        }


    }

    println!("unique_chars: {unique_chars:?}");

    i + 1
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
