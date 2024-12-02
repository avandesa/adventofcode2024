use clap::Parser;
use std::time::{Duration, Instant};

use crate::solver::Solver;

mod solutions;
mod solver;
mod utils;

fn main() {
    let args = Args::parse();

    let input_filename = if args.sample {
        "sample.txt"
    } else {
        "input.txt"
    };
    let input_path = format!("inputs/{:02}/{input_filename}", args.day);
    let input = std::fs::read_to_string(&input_path).unwrap();

    let solver: Box<dyn Solver> = match args.day {
        01 => solutions::Solver01::new(&input),
        02 => solutions::Solver02::new(&input),
        _ => todo!(),
    };

    let (part1, elapsed) = time(|| solver.part_01());
    println!("Day {:02}, Part 1 ({elapsed:?}): {}", args.day, part1);
    let (part2, elapsed) = time(|| solver.part_02());
    println!("Day {:02}, Part 2 ({elapsed:?}): {}", args.day, part2);
}

#[derive(Clone, Debug, Parser)]
pub struct Args {
    pub day: u8,

    #[clap(long)]
    pub sample: bool,
}

fn time<F: FnOnce() -> String>(f: F) -> (String, Duration) {
    let now = Instant::now();
    let result = f();
    let elapsed = now.elapsed();
    (result, elapsed)
}
