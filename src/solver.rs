pub trait Solver {
    fn new(input: &str) -> Box<Self>
    where
        Self: Sized;
    fn part_01(&self) -> String;
    fn part_02(&self) -> String;
}
