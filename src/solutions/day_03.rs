use std::sync::LazyLock;

use regex::Regex;

use crate::solver::Solver;

static RE_STRING: &str = r"mul\((\d+),(\d+)\)";
static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(RE_STRING).unwrap());

#[derive(Clone, Copy, Debug)]
struct Mul(u32, u32);

pub struct Solver03(Vec<Mul>);

impl Solver for Solver03 {
    fn new(input: &str) -> Self
    where
        Self: Sized,
    {
        let commands = RE
            .captures_iter(input)
            .map(|c| {
                let a = c.get(1).unwrap().as_str().parse().unwrap();
                let b = c.get(2).unwrap().as_str().parse().unwrap();
                Mul(a, b)
            })
            .collect();

        Self(commands)
    }

    fn part_01(&self) -> String {
        self.0
            .iter()
            .map(|Mul(a, b)| *a * *b)
            .sum::<u32>()
            .to_string()
    }

    fn part_02(&self) -> String {
        "todo!".to_string()
    }
}
