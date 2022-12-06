use regex::Regex;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum Ins {
    Rect(usize, usize),
    Row(usize, usize),
    Col(usize, usize),
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let grid = self.0;
        writeln!(f)?;
        for y in 0..grid[0].len() {
            for col in grid.iter() {
                write!(f, "{}", if col[y] { "x" } else { "." })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Clone)]
struct Grid([[bool; 6]; 50]);

impl Grid {
    fn new() -> Self {
        Grid([[false; 6]; 50])
    }

    fn apply(self, ins: &Ins) -> Self {
        let mut out = self.clone();
        let grid = &mut out.0;
        match *ins {
            Ins::Rect(x, y) => {
                for row in grid[0..x].iter_mut() {
                    for cell in row[0..y].iter_mut() {
                        *cell = true;
                    }
                }
            }
            Ins::Col(x, steps) => {
                grid[x].rotate_right(steps);
            }
            Ins::Row(y, steps) => {
                let n = self.0.len();
                for (x, col) in grid.iter_mut().enumerate() {
                    col[y] = self.0[(x + n - steps) % n][y];
                }
            }
        }
        out
    }
}

pub fn parse(input: &str) -> Vec<Ins> {
    let re = Regex::new(r"rect (?P<x>\d+)x(?P<y>\d+)|rotate (column x=(?P<column>\d+)|row y=(?P<row>\d+)) by (?P<steps>\d+)").unwrap();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cap = re.captures(line.trim()).unwrap();
            if let (Some(x), Some(y)) = (cap.name("x"), cap.name("y")) {
                Ins::Rect(x.as_str().parse().unwrap(), y.as_str().parse().unwrap())
            } else if let (Some(row), Some(steps)) = (cap.name("row"), cap.name("steps")) {
                Ins::Row(
                    row.as_str().parse().unwrap(),
                    steps.as_str().parse().unwrap(),
                )
            } else if let (Some(col), Some(steps)) = (cap.name("column"), cap.name("steps")) {
                Ins::Col(
                    col.as_str().parse().unwrap(),
                    steps.as_str().parse().unwrap(),
                )
            } else {
                panic!("unknown line");
            }
        })
        .collect()
}

pub fn part1(input: &[Ins]) -> usize {
    let mut grid = Grid::new();
    for ins in input {
        grid = grid.apply(ins);
    }
    eprintln!("{}", &grid);
    grid.0
        .iter()
        .flat_map(|col| col.iter())
        .filter(|&c| *c)
        .count()
}

pub fn part2(_input: &[Ins]) -> String {
    // Visual inspection of part1
    String::from("see output")
}
