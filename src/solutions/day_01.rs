use itertools::Itertools;

use crate::{solver::Solver, utils::sorted};

#[derive(Debug)]
pub struct Solver01 {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

impl Solver for Solver01 {
    fn new(input: &str) -> Self
    where
        Self: Sized,
    {
        let (left_list, right_list) = input
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .collect();
        Self {
            left_list,
            right_list,
        }
    }

    fn part_01(&self) -> String {
        sorted(&self.left_list)
            .into_iter()
            .zip(sorted(&self.right_list))
            .map(|(a, b)| a.abs_diff(b))
            .sum::<u32>()
            .to_string()
    }

    fn part_02(&self) -> String {
        let right_occurrences = self.right_list.iter().copied().counts();

        self.left_list
            .iter()
            .copied()
            .map(|id| id * (*right_occurrences.get(&id).unwrap_or(&0) as u32))
            .sum::<u32>()
            .to_string()
    }
}
