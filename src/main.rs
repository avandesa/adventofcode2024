use clap::Parser;

use crate::solver::Solver;

mod solutions;
mod solver;

fn main() {
    let args = Args::parse();
    dbg!(&args);

    let input_filename = if args.sample {
        "sample.txt"
    } else {
        "input.txt"
    };
    let input_path = format!("inputs/{:02}/{input_filename}", args.day);
    let input = std::fs::read_to_string(&input_path).unwrap();

    let solver: Box<dyn Solver> = match args.day {
        01 => solutions::day_01::Solver01::new(&input),
        _ => todo!(),
    };

    let part1 = solver.part_01();
    println!("Day {:02}, Part 1:\n{}", args.day, part1);
    let part2 = solver.part_02();
    println!("day {:02}, Part 2:\n{}", args.day, part2);
}

#[derive(Clone, Debug, Parser)]
pub struct Args {
    pub day: u8,

    #[clap(long)]
    pub sample: bool,
}
