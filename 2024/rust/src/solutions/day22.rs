use std::collections::{HashMap, HashSet};

pub fn parse(i: &str) -> Vec<usize> {
    i.lines().filter(|line| !line.is_empty()).map(|line| line.parse::<usize>().expect("could not parse number")).collect()
}

const MOD: usize = 16777216;

fn prune(secret: usize) -> usize {
    secret % MOD
}
fn mix(n: usize, secret: usize) -> usize {
    n ^ secret
}

fn evolve(mut secret: usize) -> usize {
    secret = prune(mix(secret * 64, secret));
    secret = prune(mix(secret / 32, secret));
    secret = prune(mix(secret * 2048, secret));
    secret
}

pub fn part1(i: &[usize]) -> usize {
    i.iter().map(|n| (0..2000).fold(*n, |acc, _n| evolve(acc)))
 //       .inspect(|secret| {dbg!(secret);})
        .sum::<usize>()
}

pub fn part2(i: &[usize]) -> usize {
    let mut values = vec![vec![]; 2001];
    values[0] = i.iter().copied().collect();
    for i in 1..=2000 {
        values[i] = values[i-1].iter().map(|v| evolve(*v)).collect();

    }
    values.iter_mut().for_each(|vt| vt.iter_mut().for_each(|v| *v = *v % 10));
    let mut diffs = vec![vec![]; 2000];
    for i in 0..2000 {
        diffs[i] = values[i + 1].iter().zip(values[i].iter()).map(|(a, b)| (*a as isize) - (*b as isize)).collect();

    }

    let mut seen: HashMap<Vec<isize>, HashMap<usize, usize>> = Default::default();
    for t in 4..2000 {
        for j in 0..i.len() {
            let k = vec![
                diffs[t-3][j], 
                diffs[t-2][j], 
                diffs[t-1][j], 
                diffs[t][j]
            ];
            seen.entry(k)
                .or_default()
                .entry(j)
                .or_insert(values[t+1][j]);
        }
    }
    seen.iter().map(|(k, v)| v.iter().map(|(_j, v)| v).sum::<usize>()).max().expect("there should be a maximum")
}
