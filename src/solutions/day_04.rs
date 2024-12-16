use crate::solver::Solver;
use itertools::{iproduct, Itertools};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Char {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy, Debug)]
struct Coords(usize, usize);

impl Coords {
    fn north(self, n: usize) -> Self {
        Self(self.0, self.1 - n)
    }

    fn east(self, n: usize) -> Self {
        Self(self.0 + n, self.1)
    }

    fn south(self, n: usize) -> Self {
        Self(self.0, self.1 + n)
    }

    fn west(self, n: usize) -> Self {
        Self(self.0 - n, self.1)
    }
}

#[derive(Clone, Debug)]
struct Matrix {
    width: usize,
    height: usize,
    data: Vec<Vec<Char>>,
}

impl Matrix {
    fn new(data: Vec<Vec<Char>>) -> Self {
        Self {
            width: data.get(0).map(|row| row.len()).unwrap_or_default(),
            height: data.len(),
            data,
        }
    }

    fn get(&self, coords: Coords) -> Char {
        self.data[coords.1][coords.0]
    }

    fn chars_iter(&self, target: Char) -> impl Iterator<Item = Coords> + use<'_> {
        iproduct!(0..self.width, 0..self.height)
            .map(|(x, y)| Coords(x, y))
            .filter(move |coords| self.get(*coords) == target)
    }

    fn fanout(&self, from: Coords) -> impl Iterator<Item = [Char; 4]> + use<'_> {
        [
            self.north(from),
            self.northeast(from),
            self.east(from),
            self.southeast(from),
            self.south(from),
            self.southwest(from),
            self.west(from),
            self.northwest(from),
        ]
        .into_iter()
        .filter_map(|x| x)
    }

    fn line<F>(&self, from: Coords, next_coord: F) -> [Char; 4]
    where
        F: Fn(Coords, usize) -> Coords,
    {
        let (a, b, c, d) = (0..4)
            .map(|n| self.get(next_coord(from, n)))
            .collect_tuple()
            .unwrap();
        [a, b, c, d]
    }

    fn north(&self, from: Coords) -> Option<[Char; 4]> {
        (from.1 >= 3).then(|| self.line(from, |from, n| from.north(n)))
    }

    fn northeast(&self, from: Coords) -> Option<[Char; 4]> {
        (from.0 <= self.width - 4 && from.1 >= 3)
            .then(|| self.line(from, |f, n| f.north(n).east(n)))
    }

    fn east(&self, from: Coords) -> Option<[Char; 4]> {
        (from.0 <= self.width - 4).then(|| self.line(from, |f, n| f.east(n)))
    }

    fn southeast(&self, from: Coords) -> Option<[Char; 4]> {
        (from.0 <= self.width - 4 && from.1 <= self.height - 4)
            .then(|| self.line(from, |f, n| f.south(n).east(n)))
    }

    fn south(&self, from: Coords) -> Option<[Char; 4]> {
        (from.1 <= self.height - 4).then(|| self.line(from, |f, n| f.south(n)))
    }

    fn southwest(&self, from: Coords) -> Option<[Char; 4]> {
        (from.0 >= 3 && from.1 <= self.height - 4)
            .then(|| self.line(from, |f, n| f.south(n).west(n)))
    }

    fn west(&self, from: Coords) -> Option<[Char; 4]> {
        (from.0 >= 3).then(|| self.line(from, |f, n| f.west(n)))
    }

    fn northwest(&self, from: Coords) -> Option<[Char; 4]> {
        (from.0 >= 3 && from.1 >= 3).then(|| self.line(from, |f, n| f.north(n).west(n)))
    }

    fn diag_corners(&self, center: Coords) -> ((Char, Char), (Char, Char)) {
        (
            (
                self.get(center.north(1).west(1)),
                self.get(center.south(1).east(1)),
            ),
            (
                self.get(center.north(1).east(1)),
                self.get(center.south(1).west(1)),
            ),
        )
    }
}

pub struct Solver04(Matrix);

impl Solver for Solver04 {
    fn new(input: &str) -> Self
    where
        Self: Sized,
    {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'X' => Char::X,
                        'M' => Char::M,
                        'A' => Char::A,
                        'S' => Char::S,
                        _ => panic!("unexpected character: `{c}`"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self(Matrix::new(data))
    }

    fn part_01(&self) -> String {
        self.0
            .chars_iter(Char::X)
            .map(|c| self.0.fanout(c))
            .flatten()
            .filter(|seq| seq == &[Char::X, Char::M, Char::A, Char::S])
            .count()
            .to_string()
    }

    fn part_02(&self) -> String {
        self.0
            .chars_iter(Char::A)
            .filter(|Coords(x, y)| {
                (1..self.0.width - 1).contains(x) && (1..self.0.height - 1).contains(y)
            })
            .map(|a| self.0.diag_corners(a))
            .filter(|(down, up)| {
                (*down == (Char::S, Char::M) || *down == (Char::M, Char::S))
                    && (*up == (Char::S, Char::M) || *up == (Char::M, Char::S))
            })
            .count()
            .to_string()
    }
}
