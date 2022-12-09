use nalgebra::{clamp, Point2, Vector2};
use std::collections::HashSet;

type Coord = Point2<isize>;

#[derive(Clone)]
pub struct Step(usize, Vector2<isize>);

#[derive(Clone, Debug)]
pub struct Rope<const N: usize> {
    knots: [Coord; N],
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        Rope {
            knots: [Point2::new(0, 0); N],
        }
    }

    pub fn write_positions(&self, positions: &mut HashSet<Coord>) {
        positions.insert(*self.knots.last().unwrap());
    }
    pub fn apply(&self, step: &Step) -> Self {
        let mut out = self.clone();
        let mut head_idx = 0;
        let mut tail_idx = 1;
        out.knots[head_idx] += step.1;
        while tail_idx < N {
            let win = &mut out.knots[head_idx..=tail_idx];

            let dist = win[0] - win[1];
            if dist.abs().max() > 1 {
                let dir = dist.map(|c| clamp(c, -1, 1));
                win[1] += dir;
            }
            head_idx += 1;
            tail_idx += 1;
        }
        out
    }
}

pub fn parse(input: &str) -> Vec<Step> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (dir, steps) = line.trim().rsplit_once(' ').unwrap();
            let steps = steps.parse::<usize>().unwrap();
            let dir = match dir {
                "U" => Vector2::new(1, 0),
                "D" => Vector2::new(-1, 0),
                "L" => Vector2::new(0, -1),
                "R" => Vector2::new(0, 1),
                _ => panic!("invalid character"),
            };
            Step(steps, dir)
        })
        .collect()
}

pub fn part1(input: &[Step]) -> usize {
    solve::<2>(input)
}

pub fn part2(input: &[Step]) -> usize {
    solve::<10>(input)
}

pub fn solve<const N: usize>(input: &[Step]) -> usize {
    let mut rope: Rope<N> = Rope::new();
    let mut positions: HashSet<Coord> = Default::default();
    rope.write_positions(&mut positions);

    for step in input {
        let mut step = step.clone();
        loop {
            rope = rope.apply(&step);
            rope.write_positions(&mut positions);
            if step.0 > 1 {
                step = Step(step.0 - 1, step.1);
            } else {
                break;
            }
        }
    }
    positions.len()
}
