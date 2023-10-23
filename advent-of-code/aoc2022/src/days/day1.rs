use std::fs::read_to_string;

pub fn run() {
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

    println!("totals: {:?}", totals);

    let mut max_pos = 0;

    for i in 0..totals.len() {
        println!("i: {}, totals[i]: {}", i, totals[i]);

        if totals[i] > totals[max_pos] {
            max_pos = i;
        }
    }

    println!("Max: {}", totals[max_pos]);

    max_pos += 1;

    println!("Max pos: {}", max_pos);
}
