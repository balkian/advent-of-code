use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone)]
pub struct Map<T> {
    pub grid: Vec<Vec<T>>,
}

type HeatMap = Map<usize>;

pub fn parse(input: &str) -> HeatMap {
    Map {
        grid: input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("could not convert char to number") as usize)
                    .collect()
            })
            .collect(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
    fn try_add(self, Coord { x, y }: &Coord, ysize: usize, xsize: usize) -> Option<Coord> {
        match self {
            Direction::North if *y > 0 => Some(Coord { x: *x, y: *y - 1 }),
            Direction::South if *y < ysize - 1 => Some(Coord { x: *x, y: *y + 1 }),
            Direction::West if *x > 0 => Some(Coord { x: *x - 1, y: *y }),
            Direction::East if *x < xsize - 1 => Some(Coord { x: *x + 1, y: *y }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Path {
    path: Vec<Coord>,
    count: usize,
    direction: Direction,
    value: isize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Path {
    fn new(position: Coord, direction: Direction) -> Path {
        Path {
            path: vec![position],
            direction,
            value: 0,
            count: 0,
        }
    }

    fn explore(&self, min_same: usize, max_same: usize, ysize: usize, xsize: usize) -> Vec<Path> {
        let mut opts: Vec<Path> = vec![];
        for direction in [North, South, West, East] {
            if direction == self.direction.opposite() {
                continue;
            }

            let mut new_path = self.clone();
            new_path.direction = direction;
            if new_path.direction == self.direction {
                if new_path.count < max_same {
                    new_path.count += 1;
                } else {
                    continue;
                }
            } else if new_path.count < min_same {
                continue;
            } else {
                new_path.count = 1;
            };
            let last = self.path.last().expect("empty path");
            if let Some(new_dir) = direction.try_add(last, ysize, xsize) {
                new_path.path.push(new_dir);
                opts.push(new_path);
            }
        }
        opts
    }
}

pub fn solve(map: &HeatMap, min_same: usize, max_same: usize) -> usize {
    let c1 = Coord { x: 0, y: 0 };
    let mut paths = BinaryHeap::from([Path::new(c1, East), Path::new(c1, South)]);
    let target = Coord {
        y: map.grid.len() - 1,
        x: map.grid[0].len() - 1,
    };
    let ysize = map.grid.len();
    let xsize = map.grid[0].len();
    let mut visited: HashSet<(Coord, Direction, usize)> = Default::default();
    while let Some(path) = paths.pop() {
        // dbg!(&paths.len());
        // dbg!(path.value);
        let coord = path.path.last().expect("empty path");
        // dbg!(&coord);
        if coord == &target {
            if path.count < min_same {
                continue;
            }
            // dbg!(&path.path);
            // dbg!(&path.path.iter().fold(0, |acc, coord| acc + map.grid[coord.y][coord.x]));
            return path.value.try_into().expect("sum is not positive");
        }
        if visited.contains(&(*coord, path.direction, path.count)) {
            continue;
        } else {
            visited.insert((*coord, path.direction, path.count));
        }
        for mut path in path.explore(min_same, max_same, ysize, xsize) {
            let coord = path.path.last().expect("empty path");
            path.value += map.grid[coord.y][coord.x] as isize;
            paths.push(path);
        }
    }
    panic!("no solution found")
}
pub fn part1(map: &HeatMap) -> usize {
    solve(map, 1, 3)
}

pub fn part2(map: &HeatMap) -> usize {
    solve(map, 4, 10)
}
