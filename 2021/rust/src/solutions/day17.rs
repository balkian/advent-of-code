use sscanf::scanf;
use std::cmp::max;
use std::ops::Range;

pub type Target = (Range<isize>, Range<isize>);

pub fn parse(input: &str) -> Target {
    let (xmin, xmax, ymin, ymax) = scanf!(
        input.trim(),
        "target area: x={}..{}, y={}..{}",
        isize,
        isize,
        isize,
        isize,
    )
    .unwrap();
    (xmin..xmax + 1, ymin..ymax + 1)
}

pub fn part1(input: &Target) -> usize {
    records(input).into_iter().max().unwrap() as usize
}
fn records(input: &Target) -> Vec<isize> {
    // let mut minx = ((input.0.start as f64) * 2.0).sqrt() as isize - 2;
    let maxx = input.0.end;
    let maxy = max(input.1.start.abs(), input.1.end.abs()) + 1;

    let valid_x: Vec<isize> = (0..=maxx).collect();
    let valid_y: Vec<isize> = (-maxy..=maxy).collect();
    let mut records = vec![];
    for vx in &valid_x {
        for vy in &valid_y {
            let mut dx = *vx;
            let mut dy = *vy;
            let mut record = 0;
            let mut x = 0;
            let mut y = 0;
            while y > -maxy && x <= maxx {
                x += dx;
                y += dy;
                if y > record {
                    record = y
                }
                dy -= 1;
                if dx > 0 {
                    dx -= 1;
                }
                if x >= input.0.start && x < input.0.end && y >= input.1.start && y < input.1.end {
                    records.push(record);
                    break;
                }
            }
        }
    }
    records
}
pub fn part2(input: &Target) -> usize {
    records(input).len()
}

use crate::aoc_sample;

aoc_sample!(day17sample1part1, "../../day17.sample1", part1, 45);
aoc_sample!(day17sample1part2, "../../day17.sample1", part2, 112);
