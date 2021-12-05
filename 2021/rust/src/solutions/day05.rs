use sscanf::scanf;
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Line::from_str)
        .collect()
}

pub fn part1(input: &[Line]) -> usize {
    part(input, true)
}

pub fn part2(input: &[Line]) -> usize {
    part(input, false)
}

pub fn part(input: &[Line], filter: bool) -> usize {
    let mut counts: HashMap<Coord, usize> = HashMap::new();
    for line in input.iter().filter(|line| !filter || line.is_straight()) {
        for point in line.points() {
            *counts.entry(point).or_default() += 1;
        }
    }
    counts.iter().filter(|(_, &v)| v > 1).count()
}

type Coord = (usize, usize);

#[derive(Debug)]
pub struct Line {
    min: Coord,
    max: Coord,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.min.0 == self.max.0 || self.min.1 == self.max.1
    }

    fn points(&self) -> Vec<Coord> {
        let mut ys: Vec<usize> = if self.min.0 <= self.max.0 {
            (self.min.0..=self.max.0).collect()
        } else {
            (self.max.0..=self.min.0).rev().collect()
        };
        let mut xs: Vec<usize> = if self.min.1 <= self.max.1 {
            (self.min.1..=self.max.1).collect()
        } else {
            (self.max.1..=self.min.1).rev().collect()
        };
        if ys.len() == 1 {
            ys = ys.into_iter().cycle().take(xs.len()).collect();
        } else if xs.len() == 1 {
            xs = xs.into_iter().cycle().take(ys.len()).collect();
        }
        debug_assert_eq!(xs.len(), ys.len());
        ys.into_iter().zip(xs.into_iter()).collect()
    }

    fn from_str(s: &str) -> Self {
        let (y0, x0, y1, x1) = scanf!(s, "{},{} -> {},{}", usize, usize, usize, usize).unwrap();
        Line {
            min: (y0, x0),
            max: (y1, x1),
        }
    }
}

#[test]
fn test_example() {
    let input = &parse(
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 12);
}
