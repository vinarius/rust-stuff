use clap::Parser;

mod days;

/// My custom help message goes here!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let Args { day } = Args::parse();

    days::run_day(day);
}
