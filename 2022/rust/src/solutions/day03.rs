use itertools::Itertools;
use std::collections::HashSet;

type Sack = [HashSet<usize>; 2];

pub fn parse(input: &str) -> Vec<Sack> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .as_bytes()
                .iter()
                .chunks(line.len() / 2)
                .into_iter()
                .map(|w| {
                    w.map(|l| match *l as char {
                        'a'..='z' => l - b'a' + 1,
                        'A'..='Z' => l - b'A' + 27,
                        _ => panic!("unknown character"),
                    } as usize)
                        .collect()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

pub fn part1(input: &[Sack]) -> usize {
    input
        .iter()
        .map(|sack| sack[0].intersection(&sack[1]).sum::<usize>())
        .sum()
}
pub fn part2(input: &[Sack]) -> usize {
    input
        .iter()
        .map(|sack| &sack[0] | &sack[1])
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.into_iter().reduce(|a, b| &a & &b).unwrap())
        .map(|union| union.iter().sum::<usize>())
        .sum::<usize>()
}
