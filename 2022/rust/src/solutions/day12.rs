use std::cmp::Ordering;
use std::collections::{binary_heap::BinaryHeap, HashSet};

type Coord = (usize, usize);

type Grid = Vec<Vec<usize>>;

#[derive(Debug)]
pub struct Input {
    grid: Grid,
    start: Coord,
    target: Coord,
}

#[derive(Eq, Clone)]
struct Path {
    total: usize,
    coords: Vec<Coord>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.coords.len().cmp(&self.coords.len())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.coords.len() == other.coords.len()
    }
}

impl Input {
    fn dijkstra(&self, source: Coord) -> Option<Path> {
        let mut opts = BinaryHeap::new();
        let mut visited = HashSet::new();
        opts.push(Path {
            total: 0,
            coords: vec![source],
        });
        while let Some(next) = opts.pop() {
            let last = next.coords.last().unwrap();
            if visited.contains(last) {
                continue;
            }
            visited.insert(*last);
            if last.0 == self.target.0 && last.1 == self.target.1 {
                return Some(next);
            }
            // dbg!(next.coords.len(), &last);
            let mut dirs = vec![];
            if last.0 > 0 {
                dirs.push((last.0 - 1, last.1));
            }
            if last.1 > 0 {
                dirs.push((last.0, last.1 - 1));
            }
            if last.0 < self.grid.len() - 1 {
                dirs.push((last.0 + 1, last.1));
            }
            if last.1 < self.grid[last.0].len() - 1 {
                dirs.push((last.0, last.1 + 1));
            }
            let this_height = self.grid[last.0][last.1];
            for d in dirs {
                let height = self.grid[d.0][d.1];
                let diff = height.abs_diff(this_height);
                if height < this_height + 2 {
                    let mut next_coords = next.coords.clone();
                    next_coords.push(d);

                    opts.push(Path {
                        total: next.total + diff,
                        coords: next_coords,
                    })
                }
            }
        }
        None
    }
}

pub fn parse(input: &str) -> Input {
    let mut start: Option<Coord> = None;
    let mut target: Option<Coord> = None;
    let grid = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(jx, line)| {
            line.chars()
                .enumerate()
                .map(|(ix, c)| {
                    let c = match c {
                        'S' => {
                            start = Some((jx, ix));
                            'a'
                        }
                        'E' => {
                            target = Some((jx, ix));
                            'z'
                        }
                        _ => c,
                    };
                    (c as usize) - ('a' as usize)
                })
                .collect()
        })
        .collect();
    Input {
        grid,
        start: start.unwrap(),
        target: target.unwrap(),
    }
}

pub fn part1(input: &Input) -> usize {
    let best = input.dijkstra(input.start).unwrap();
    best.coords.len() - 1
}

pub fn part2(input: &Input) -> usize {
    input
        .grid
        .iter()
        .enumerate()
        .flat_map(|(ix, row)| {
            row.iter().enumerate().filter_map(move |(jx, cell)| {
                if *cell != 0 {
                    return None;
                }
                input.dijkstra((ix, jx)).map(|res| res.coords.len())
            })
        })
        .min()
        .unwrap()
        - 1
}
