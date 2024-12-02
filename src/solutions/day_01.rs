use crate::solver::Solver;

pub struct Solver01;

impl Solver for Solver01 {
    fn new(input: &str) -> Box<Self>
    where
        Self: Sized,
    {
        dbg!(&input);
        Box::new(Self)
    }

    fn part_01(&self) -> String {
        todo!()
    }

    fn part_02(&self) -> String {
        todo!()
    }
}
