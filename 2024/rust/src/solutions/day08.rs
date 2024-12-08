use gcd::Gcd;
use nalgebra::Point2;
use std::collections::{HashMap, HashSet};

type Pos = Point2<isize>;

#[derive(Debug, Clone)]
pub struct Input {
    antennas: HashMap<char, Vec<Pos>>,
    limits: Pos,
}

pub fn parse(i: &str) -> Input {
    let mut antennas: HashMap<char, Vec<Pos>> = Default::default();
    let mut limits = Point2::new(0, 0);
    for (i, line) in i.trim().lines().enumerate() {
        limits.x = limits.x.max(i as isize);
        for (j, c) in line.trim().chars().enumerate() {
            limits.y = limits.y.max(j as isize);
            match c {
                '.' => continue,
                c if c.is_alphanumeric() => {
                    antennas
                        .entry(c)
                        .or_default()
                        .push(Point2::new(i as isize, j as isize));
                }
                _ => panic!("unknown character {c}"),
            }
        }
    }
    Input { antennas, limits }
}
pub fn part1(i: &Input) -> usize {
    let mut nodes: HashSet<Pos> = Default::default();
    for ants in i.antennas.values() {
        for (i, a) in ants.iter().enumerate() {
            for b in &ants[0..i] {
                let d = a - b;
                nodes.insert(a + d);
                nodes.insert(b - d);
                if d.x % 3 == 0 && d.y % 3 == 0 {
                    let d2 = d / 3;
                    nodes.insert(b + d2);
                    nodes.insert(a - d2);
                }
            }
        }
    }
    nodes
        .into_iter()
        .filter(|p| p.x >= 0 && p.y >= 0 && p.x <= i.limits.x && p.y <= i.limits.y)
        .count()
}
pub fn part2(i: &Input) -> usize {
    let mut nodes: HashSet<Pos> = Default::default();
    for ants in i.antennas.values() {
        for (ix, a) in ants.iter().enumerate() {
            for b in &ants[0..ix] {
                let d = a - b;
                let div = d.x.unsigned_abs().gcd(d.y.unsigned_abs()) as isize;
                let d = d / div;
                let mut p: Pos = *a;
                while p.x >= 0 && p.y >= 0 && p.x <= i.limits.x && p.y <= i.limits.y {
                    nodes.insert(p);
                    p += d;
                }
                p = *a;
                while p.x >= 0 && p.y >= 0 && p.x <= i.limits.x && p.y <= i.limits.y {
                    nodes.insert(p);
                    p -= d;
                }
            }
        }
    }
    nodes.len()
}
