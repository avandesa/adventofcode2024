use std::collections::{HashMap, HashSet};

use crate::solver::Solver;

type RuleSet = HashMap<u32, HashSet<u32>>;

#[derive(Clone, Debug)]
struct Sequence(Vec<u32>);

impl Sequence {
    fn is_ordered_over(&self, rules: &RuleSet) -> Option<(usize, usize)> {
        for (i, after) in self.0.iter().enumerate().skip(1) {
            let Some(must_come_after) = rules.get(after) else {
                // no rules for this value
                continue;
            };

            for (j, before) in self.0.iter().enumerate().take(i) {
                if must_come_after.contains(before) {
                    return Some((j, i));
                }
            }
        }

        None
    }

    fn middle(&self) -> u32 {
        self.0[self.0.len() / 2]
    }

    fn reorder(&self, rules: &RuleSet) -> Option<Self> {
        self.is_ordered_over(rules).map(|(j, i)| {
            let mut new = self.clone();
            new.0.swap(j, i);
            new.reorder_in_place(rules);
            new
        })
    }

    fn reorder_in_place(&mut self, rules: &RuleSet) {
        while let Some((j, i)) = self.is_ordered_over(rules) {
            self.0.swap(j, i);
        }
    }
}

#[derive(Debug)]
pub struct Solver05 {
    rules: RuleSet,
    sequences: Vec<Sequence>,
}

impl Solver for Solver05 {
    fn new(input: &str) -> Self
    where
        Self: Sized,
    {
        let (rules, sequences) = input.split_once("\n\n").unwrap();
        let rules = rules
            .lines()
            .map(|l| {
                let (before, after) = l.split_once('|').unwrap();
                (before.parse().unwrap(), after.parse().unwrap())
            })
            .fold(RuleSet::new(), |mut rules, (before, after)| {
                rules.entry(before).or_default().insert(after);
                rules
            });

        let sequences = sequences
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|l| Sequence(l.split(',').map(|n| n.parse().unwrap()).collect()))
            .collect();

        Self { rules, sequences }
    }

    fn part_01(&self) -> String {
        self.sequences
            .iter()
            .filter(|s| s.is_ordered_over(&self.rules).is_none())
            .map(|s| s.middle())
            .sum::<u32>()
            .to_string()
    }

    fn part_02(&self) -> String {
        self.sequences
            .iter()
            .filter_map(|s| s.reorder(&self.rules))
            .map(|s| s.middle())
            .sum::<u32>()
            .to_string()
    }
}
