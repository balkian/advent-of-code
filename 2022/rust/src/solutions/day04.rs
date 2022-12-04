use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Elf {
    start: usize,
    end: usize,
}

pub fn parse(input: &str) -> Vec<(Elf, Elf)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line
                .split(',')
                .map(|e| {
                    let (start, end) = e
                        .split('-')
                        .map(|i| i.parse().expect("unexpected number"))
                        .collect_tuple()
                        .unwrap();
                    Elf { start, end }
                })
                .collect_tuple()
                .unwrap();
            if a.end - a.start > b.end - b.start {
                (a, b)
            } else {
                (b, a)
            }
        })
        .collect()
}

pub fn part1(input: &[(Elf, Elf)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.start <= b.start && a.end >= b.end)
        .count()
}

pub fn part2(input: &[(Elf, Elf)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            let long = RangeInclusive::new(a.start, a.end);
            let mut short = RangeInclusive::new(b.start, b.end);
            short.any(|i| long.contains(&i))
        })
        .count()
}
