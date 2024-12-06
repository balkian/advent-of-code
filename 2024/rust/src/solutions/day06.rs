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

const GOLEFT: na::Vector2<isize> = na::vector![-1, 0];
const GORIGHT: na::Vector2<isize> = na::vector![1, 0];
const GOUP: na::Vector2<isize> = na::vector![0, -1];
const GODOWN: na::Vector2<isize> = na::vector![0, 1];

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, other: Dir) -> Pos {
        let delta = match other {
            Dir::Left => GOLEFT,
            Dir::Right => GORIGHT,
            Dir::Up => GOUP,
            Dir::Down => GODOWN,
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
    /// Updates the position of the Guard until it reaches the end of the grid (true)
    /// or an obstacle (false)
    fn straight<F: FnMut(&Self)>(&mut self, grid: &Grid, mut updater: F) -> bool {
        loop {
            let next = self.pos + self.dir;
            if !((0..=grid.limits.x).contains(&self.pos.coords.x)
                && (0..=grid.limits.y).contains(&self.pos.y))
            {
                return false;
            }
            if grid.obstacles.contains(&next) {
                self.dir = match self.dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                };
                return true;
            }
            updater(self);
            self.pos = next;
        }
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
    let mut visit = |g: &Guard| {
        visited.insert(g.pos);
    };
    loop {
        if !g.straight(grid, &mut visit) {
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
    let mut wave = vec![];
    loop {
        let start = g.clone();
        let done = !g.straight(&grid, |g| {
            wave.push(g.pos);
        });
        wave.push(g.pos);
        for opt in wave.drain(..).skip(1) {
            if path.contains(&opt) {
                continue;
            }
            let mut prev = start.clone();
            visited.clear();
            grid.obstacles.insert(opt);
            loop {
                visited.insert(prev.clone());
                if !prev.straight(&grid, |_| {}) {
                    break;
                }
                if visited.contains(&prev) {
                    opts.push(opt);
                    break;
                }
            }
            grid.obstacles.remove(&opt);
            path.insert(opt);
        }
        if done {
            break;
        }
    }
    opts.len()
}
