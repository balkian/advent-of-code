extern crate nalgebra as na;
use std::collections::HashSet;

type Pos = na::Point2<isize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
enum Dir {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

const VECTORS: [na::Vector2<isize>; 4] = [
    na::vector![-1, 0],
    na::vector![1, 0],
    na::vector![0, -1],
    na::vector![0, 1],
];

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, other: Dir) -> Pos {
        let delta = match other {
            Dir::Left => VECTORS[0],
            Dir::Right => VECTORS[1],
            Dir::Up => VECTORS[2],
            Dir::Down => VECTORS[3],
        };
        self + delta
    }
}

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Clone, Default, Debug)]
pub struct Grid {
    obstacles: HashSet<Pos>,
    limits: Pos,
}

impl Guard {
    fn advance(&mut self, grid: &Grid) -> bool {
        self.pos = loop {
            let next = self.pos + self.dir;
            if !grid.obstacles.contains(&next) {
                break next;
            }
            self.dir = match self.dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        };
        (0..=grid.limits.x).contains(&self.pos.coords.x)
            && (0..=grid.limits.y).contains(&self.pos.y)
    }
}

pub fn parse(i: &str) -> (Guard, Grid) {
    let mut pos = na::point![0, 0];
    let mut dir = Dir::Up;
    let mut obstacles: HashSet<Pos> = Default::default();
    let mut limits = na::point![0, 0];
    for (y, line) in i.lines().filter(|line| !line.is_empty()).enumerate() {
        let y = y as isize;
        limits.y = limits.y.max(y);
        for (x, cell) in line.trim().chars().enumerate() {
            let x = x as isize;
            limits.x = limits.x.max(x);
            let thispos = na::point![x, y];
            match cell {
                '^' => {
                    dir = Dir::Up;
                    pos = thispos;
                }
                '<' => {
                    dir = Dir::Left;
                    pos = thispos;
                }
                'v' => {
                    dir = Dir::Down;
                    pos = thispos;
                }
                '>' => {
                    dir = Dir::Right;
                    pos = thispos;
                }
                '.' => {}
                '#' => {
                    obstacles.insert(thispos);
                }
                _ => panic!("invalid tile"),
            }
        }
    }
    (Guard { pos, dir }, Grid { obstacles, limits })
}

pub fn part1((guard, grid): &(Guard, Grid)) -> usize {
    let mut visited = HashSet::new();
    let mut g = guard.clone();
    visited.insert(g.pos);
    loop {
        visited.insert(g.pos);
        if !g.advance(grid) {
            break;
        }
    }
    visited.len()
}

pub fn part2((guard, grid): &(Guard, Grid)) -> usize {
    let mut path: HashSet<Pos> = Default::default();
    path.insert(guard.pos);
    let mut grid = grid.clone();
    let mut g = guard.clone();
    let mut opts = vec![];
    let mut visited = HashSet::new();
    loop {
        let mut prev = g.clone();
        if !g.advance(&grid) {
            break;
        }
        if path.contains(&g.pos) {
            continue;
        }
        path.insert(g.pos);
        let opt = g.pos;
        visited.clear();
        grid.obstacles.insert(opt); // Add an obstacle in front
        loop {
            visited.insert(prev.clone());
            if !prev.advance(&grid) {
                break;
            }
            if visited.contains(&prev) {
                opts.push(opt);
                break;
            }
        }
        grid.obstacles.remove(&opt);
    }
    opts.len()
}
