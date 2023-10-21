mod day1;

pub fn run_day(day: u8) {
    match day {
        1 => day1::run(),
        _ => println!("Day {day} not implemented")
    }
}
