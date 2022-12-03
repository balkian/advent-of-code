use itertools::Itertools;
use std::collections::HashSet;

type Sack = [HashSet<usize>; 2];

pub fn parse(input: &str) -> Vec<Sack> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.trim();
            line.as_bytes()
                .chunks(line.len() / 2)
                .map(|w| {
                    w.iter()
                        .map(|l| match *l as char {
                            'a'..='z' => l - ('a' as u8) + 1,
                            'A'..='Z' => l - ('A' as u8) + 27,
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
    let joint: Vec<HashSet<usize>> = input.iter().map(|sack| &sack[0] | &sack[1]).collect();
    joint
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.into_iter().reduce(|a, b| &a & &b).unwrap())
        .map(|union| dbg!(union).iter().sum::<usize>())
        .sum::<usize>()
}
