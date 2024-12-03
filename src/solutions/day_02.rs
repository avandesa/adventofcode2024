use itertools::Itertools;

use crate::solver::Solver;

const MIN_CHANGE: i32 = 1;
const MAX_CHANGE: i32 = 3;

#[derive(Debug)]
struct Report(Vec<i32>);

impl Report {
    fn is_safe(&self) -> bool {
        let differences = self
            .0
            .iter()
            .tuple_windows()
            .map(|(a, b)| *a - *b)
            .collect::<Vec<_>>();

        differences
            .iter()
            .copied()
            .all(|diff| (MIN_CHANGE..=MAX_CHANGE).contains(&diff))
            || differences
                .iter()
                .copied()
                .all(|diff| (-MAX_CHANGE..=-MIN_CHANGE).contains(&diff))
    }

    fn remove_level(&self, idx: usize) -> Self {
        let mut new = self.0.clone();
        new.remove(idx);
        Self(new)
    }

    fn removed_levels(&self) -> impl Iterator<Item = Self> + use<'_> {
        (0..self.0.len()).map(|idx| self.remove_level(idx))
    }

    fn is_safe_with_removal(&self) -> bool {
        self.removed_levels().any(|r| r.is_safe())
    }
}

#[derive(Debug)]
pub struct Solver02 {
    reports: Vec<Report>,
}

impl Solver for Solver02 {
    fn new(input: &str) -> Self
    where
        Self: Sized,
    {
        let reports = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|val| val.parse().unwrap())
                    .collect()
            })
            .map(Report)
            .collect();

        Self { reports }
    }

    fn part_01(&self) -> String {
        self.reports
            .iter()
            .filter(|r| r.is_safe())
            .count()
            .to_string()
    }

    fn part_02(&self) -> String {
        self.reports
            .iter()
            .filter(|r| r.is_safe() || r.is_safe_with_removal())
            .count()
            .to_string()
    }
}
