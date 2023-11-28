pub fn run() {
    const INPUT: &str = include_str!("../../input/day5.txt");

    part1(INPUT);

    // println!();
    //
    // part2(INPUT);
}

fn part1(input: &str) {
    println!("Running day 5 part 1");

    println!("input:");

    const ERROR: &str = "problem statement guarantees input structure";
    let mut supply_crates: Vec<&str> = Vec::new();
    let mut instructions: Vec<[i32; 3]> = Vec::new();

    for line in input.lines() {
        if !line.starts_with("move") && !line.trim().is_empty() {
            supply_crates.push(line);
        } else {
            let sanitized_instructions: Vec<i32> = Vec::new();
            let split_by_whitespace: Vec<&str> = line.split_whitespace().collect();
            let quantity: i32 = split_by_whitespace
                .get(1)
                .expect(ERROR)
                .parse()
                .expect(ERROR);
            let start: i32 = split_by_whitespace
                .get(3)
                .expect(ERROR)
                .parse()
                .expect(ERROR);
            let end: i32 = split_by_whitespace
                .get(5)
                .expect(ERROR)
                .parse()
                .expect(ERROR);

            instructions.push([quantity, start, end]);
        }
    }

    dbg!("supply_crates: {supply_crates}");
}

fn part2(input: &str) {
    println!("Running day 5 part 2");

    todo!();
}
