use std::iter::zip;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize, 
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.trim()
        .lines()
        .map(|line| line.split_once(":").expect("coult not find colon")
            .1
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("could not parse int"))
            );
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    let races: Vec<Race> = zip(times, distances).map(|(time, distance)| {
        Race{time, distance}
            }).collect();
    races.iter().map(|race| {
        (1..race.time).map(|t| t*(race.time-t)).filter(|d| d > &race.distance).count()
      
    }).product()
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.trim()
        .lines()
        .map(|line| line.split_once(":").expect("coult not find colon")
            .1
            .replace(" ", "").parse::<usize>().expect("could not parse digit")
            );
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let mut x0 = 0;
    let mut x1 = time/2;
    let mut m = 0;
    let mut r;
    while x0 <= x1 {
        m = (x1-x0)/2+x0;
        r = m*(time-m);
        match r.cmp(&distance) {
            Ordering::Equal => {
                m += 1;
                break;
            },
            Ordering::Less => {
                m += 1;
                x0 = m;
            },
            Ordering::Greater => {
                x1 = m-1;
            }
        }
    }
    time - 2 * (m-1) - 1
}
