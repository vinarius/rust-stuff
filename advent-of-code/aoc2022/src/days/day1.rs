use std::fs::read_to_string;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    println!("Running day 1 part 1");

    let input = read_to_string("input/day1.txt").unwrap();
    let mut totals: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(n) => {
                total += n;
            }
            Err(_) => {
                totals.push(total);
                total = 0;
            }
        }
    }

    let max_calorie_count = totals.iter().max().unwrap();

    println!("The elf with the most calories is carrying {max_calorie_count} calories");
}

fn part2() {
    println!("Running day 1 part 2");
}
