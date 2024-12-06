extern crate nalgebra as na;
use std::collections::HashSet;

type Pos = na::Point2<isize>;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Default,Hash)]
enum Dir {
    Left,
    #[default]
    Right,
    Up,
    Down
}

//impl Default for Dir {
//    fn default() -> Self {
//        Dir::Right
//    }
//}

impl std::ops::Add<Pos> for Dir {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        let delta = match self {
            Dir::Left => {
                na::vector![-1, 0]
            },
            Dir::Right => {
                na::vector![1, 0]
            },
            Dir::Up => {
                na::vector![0, -1]
            },
            Dir::Down => {
                na::vector![0, 1]
            }
        };
        other + delta
    }
}


#[derive(Clone, Default, Debug)]
pub struct Guard {
    pos: Pos,
    dir: Dir,
    obstacles: HashSet<Pos>,
    limits: Pos,
}

impl Guard {
    fn advance(&mut self) -> bool {
        self.pos = loop {
            let next = self.dir + self.pos;
            if !self.obstacles.contains(&next) {
                break next;
            }
            self.dir = match self.dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        };
        (0..=self.limits.x).contains(&self.pos.coords.x) && (0..=self.limits.y).contains(&self.pos.y)
    }
}

pub fn parse(i: &str) -> Guard {
    let mut pos = na::point![0, 0];
    let mut dir = Dir::Up;
    let mut obstacles: HashSet<Pos> = Default::default();
    let mut limits = na::point![0, 0];
    for (y, line) in i.lines().filter(|line| !line.is_empty()) .enumerate() {
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
                },
                '<' => {
                    dir = Dir::Left;
                    pos = thispos;
                },
                'v' => {
                    dir = Dir::Down;
                    pos = thispos;
                },
                '>' => {
                    dir = Dir::Right;
                    pos = thispos;
                },
                '.' => {
                },
                '#' => {
                    obstacles.insert(thispos);
                },
                _ => panic!("invalid tile"),
            }
        }
    }
    Guard{pos, dir, obstacles, limits}
}

pub fn part1(g: &Guard) -> usize {
    let mut visited = HashSet::new();
    let mut g = g.clone();
    visited.insert(g.pos);
    loop {
        visited.insert(g.pos);
        if !g.advance() {
            break;
        }
    }
    visited.len()
}

pub fn part2(g: &Guard) -> usize {
    todo!();
}
