use clap::Parser;

/// Run advent-of-code 2024 for a given day.
#[derive(Parser)]
struct Cli {
    /// The day to run advent-of-code, between 1 and 25
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let args = Cli::parse();
    println!("Running advent-of-code 2024, day {:?}", args.day);
}
