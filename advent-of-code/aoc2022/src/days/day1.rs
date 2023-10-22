use std::fs::read_to_string;

pub fn run() {
    println!("Day 1 executed");

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

    totals.push(total);

    // TODO: get position, not value
    let max = totals.iter().max().unwrap();

    println!("max: {max}");
}
