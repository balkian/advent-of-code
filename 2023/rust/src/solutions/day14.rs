use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rock {
    Empty,
    Round,
    Stop,
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rock::Empty => '.',
            Rock::Round => 'O',
            Rock::Stop => '#',
        };
        write!(f, "{}", s)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Map(Vec<Vec<Rock>>);

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in self.0.iter() {
            for cell in row.iter() {
                cell.hash(state)
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        self.0.iter().try_for_each(|row| {
            row.iter().try_for_each(|c| write!(f, "{}", c))?;
            writeln!(f)
        })
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Map {
    fn tilt(&self, dir: Direction) -> Map {
        let mut grid = self.0.clone();
        if grid.is_empty() {
            return Map(grid);
        }
        grid.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|t| {
                if t == &Rock::Round {
                    *t = Rock::Empty;
                }
            });

        let y_size = self.0.len();
        let x_size = self.0[y_size - 1].len();

        // Calculate the order of travel based on direction
        let outer: Vec<Vec<(usize, usize)>> = match dir {
            Direction::Up => (0..x_size)
                .map(|x| (0..y_size).rev().map(|y| (y, x)).collect())
                .collect(),
            Direction::Down => (0..x_size)
                .map(|x| (0..y_size).map(|y| (y, x)).collect())
                .collect(),
            Direction::Left => (0..y_size)
                .map(|y| (0..x_size).rev().map(|x| (y, x)).collect())
                .collect(),
            Direction::Right => (0..y_size)
                .map(|y| (0..x_size).map(|x| (y, x)).collect())
                .collect(),
        };

        for it in outer {
            let mut count = 0;
            for (ix, (y, x)) in it.iter().enumerate() {
                match self.0[*y][*x] {
                    Rock::Round => {
                        count += 1;
                    }
                    Rock::Stop if count > 0 => {
                        for (y1, x1) in it[0..ix].iter().rev().take(count) {
                            grid[*y1][*x1] = Rock::Round;
                        }
                        count = 0;
                    }
                    _ => {}
                }
            }
            for (y1, x1) in it.iter().rev().take(count) {
                grid[*y1][*x1] = Rock::Round;
            }
        }
        let out = Map(grid);
        debug_assert_eq!(out.count_rocks(), self.count_rocks());
        out
    }

    fn value(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(ix, row)| row.iter().filter(|r| **r == Rock::Round).count() * (ix + 1))
            .sum::<usize>()
    }

    fn count_rocks(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter().filter(|c| **c == Rock::Round))
            .count()
    }
}

pub fn parse(input: &str) -> Map {
    Map(input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Rock::Stop,
                    '.' => Rock::Empty,
                    'O' => Rock::Round,
                    _ => panic!("unknown rock type {c}"),
                })
                .collect()
        })
        .collect())
}

pub fn part1(rocks: &Map) -> usize {
    let tilted = &rocks.tilt(Direction::Up);
    tilted.value()
}

pub fn part2(rocks: &Map) -> usize {
    let mut indices: HashMap<Map, usize> = Default::default();
    let mut rocks = rocks.clone();

    let mut counts = vec![];
    let mut i = 0;
    let max = 1000000000;
    while i < max {
        let mut new_rocks = rocks.tilt(Direction::Up);

        let count = rocks.value();
        counts.push(count);
        indices.insert(rocks, i);
        i += 1;
        new_rocks = new_rocks.tilt(Direction::Left);
        new_rocks = new_rocks.tilt(Direction::Down);
        new_rocks = new_rocks.tilt(Direction::Right);
        if let Some(previous) = indices.get(&new_rocks) {
            let cycle = i - previous;
            let remainder = (max - i) % cycle;
            let target = previous + remainder;
            // dbg!(previous, i, cycle, remainder, target, &counts[(target-2)..(target+3)]);
            return counts[target];
        } else {
            rocks = new_rocks;
        }
    }
    panic!("not found")
}
