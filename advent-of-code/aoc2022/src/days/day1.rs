use std::fs::read_to_string;

pub fn run() {
    let totals = part1();
    part2(totals);
}

fn part1() -> Vec<i32> {
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

    totals
}

fn part2(totals: Vec<i32>) {
    println!("Running day 1 part 2");

    let mut top_three_totals: Vec<i32> = Vec::new();

    for i in 0..totals.len() {
        let total = totals[i];

        // println!("total: {total}");

        if top_three_totals.len() < 3 {
            top_three_totals.push(total);
            // println!("top_three_totals: {top_three_totals:?}");
            top_three_totals.sort();
            // println!("top_three_totals: {top_three_totals:?}");
            continue;
        }

        for j in 0..top_three_totals.len() {
            if total > top_three_totals[j] {
                println!("1 top_three_totals: {top_three_totals:?}");
                println!("total: {total}");
                top_three_totals[j] = total;
                println!("2 top_three_totals: {top_three_totals:?}");
                top_three_totals.sort();
                println!("3 top_three_totals: {top_three_totals:?}");
                println!();
                break;
            }
        }
    }

    // println!("top_three_totals: {top_three_totals:?}");

    // 204869 - too high
    let calorie_count_of_top_3: i32 = top_three_totals.iter().sum();

    println!("The top three elves are together carrying {calorie_count_of_top_3} calories.");
}
