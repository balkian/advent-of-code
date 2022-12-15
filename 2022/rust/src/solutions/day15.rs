/// This code stinks. I barely had time today and I'm still not used to nalg.
use nalgebra::{Point2, Vector2};
use regex::Regex;
use std::collections::HashSet;

type Coord = Point2<isize>;
type Pair = [Coord; 2];
type Square = (Coord, Coord);

pub fn parse(input: &str) -> Vec<Pair> {
    let re = Regex::new(r"=([-]?\d+)").unwrap();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords = re
                .captures_iter(line)
                .map(|cap| cap[1].parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            [
                Point2::new(coords[0], coords[1]),
                Point2::new(coords[2], coords[3]),
            ]
        })
        .collect()
}

fn dist(c1: &Coord, c2: &Coord) -> usize {
    (c1 - c2).abs().sum() as usize
}

struct Beacon(Coord, usize);

impl Beacon {
    fn covers_point(&self, p: &Coord) -> bool {
        dist(&self.0, p) <= self.1
    }

    fn covers_square(&self, sq: &Square) -> bool {
        for x in &[sq.0[0], sq.1[0]] {
            for y in &[sq.0[1], sq.1[1]] {
                if !self.covers_point(&Point2::new(*x, *y)) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn part1(input: &[Pair]) -> usize {
    part1_gen(input, 2000000)
}

pub fn part1_gen(input: &[Pair], target: isize) -> usize {
    let mut distances: Vec<_> = input
        .iter()
        .map(|[sensor, beacon]| (sensor, dist(sensor, beacon)))
        .collect();
    let taken: HashSet<Coord> = input.iter().flatten().copied().collect();
    distances.sort_by(|a, b| b.1.cmp(&a.1));

    let min_x = distances
        .iter()
        .map(|(p, rad)| p[0] - *rad as isize)
        .min()
        .unwrap();
    let max_x = distances
        .iter()
        .map(|(p, rad)| p[0] + *rad as isize)
        .max()
        .unwrap();

    let mut unr = 0;
    for i in min_x..=max_x {
        let p = Point2::new(i, target);
        if !taken.contains(&p)
            && distances
                .iter()
                .any(|(beacon, radius)| dist(&p, beacon) <= *radius)
        {
            unr += 1;
        }
    }
    input
        .iter()
        .filter(|[_sensor, beacon]| beacon[1] == target)
        .count();
    input
        .iter()
        .filter(|[sensor, _beacon]| sensor[1] == target)
        .count();
    unr
}

pub fn part2(input: &[Pair]) -> usize {
    let mut distances: Vec<Beacon> = input
        .iter()
        .map(|[sensor, beacon]| Beacon(*sensor, dist(sensor, beacon)))
        .collect();
    let taken: HashSet<Coord> = input.iter().flatten().copied().collect();
    distances.sort_by(|a, b| b.1.cmp(&a.1));

    let min = Point2::new(0, 0);
    let max = Point2::new(4000000, 4000000);
    let mut chunks: Vec<Square> = vec![(min, max)];

    while let Some(chunk) = chunks.pop() {
        if dist(&chunk.0, &chunk.1) == 0 {
            if taken.contains(&chunk.0) {
                continue;
            }
            for b in distances.iter() {
                debug_assert!(!b.covers_square(&chunk));
            }

            return (chunk.0[0] * 4000000 + chunk.0[1]) as usize;
        }
        let (min, max) = chunk;
        let delta = Vector2::new(1, 1);
        let mid_l = min + (max - min) / 2;
        let mid_r = mid_l + delta;
        for i in [
            (min, mid_l),
            (Point2::new(mid_r[0], min[1]), Point2::new(max[0], mid_r[1])),
            (Point2::new(min[0], mid_l[1]), Point2::new(mid_l[0], max[1])),
            (mid_r, max),
        ] {
            if i.0 != chunk.0 || i.1 != chunk.1 {
                if distances.iter().any(|b| b.covers_square(&i)) {
                    continue;
                }
                if chunk.0[0] > chunk.1[0] || chunk.0[1] > chunk.1[1] {
                    continue;
                }
                chunks.push(i);
            }
        }
    }
    panic!("no solution found");
}
