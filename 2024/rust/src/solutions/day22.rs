use std::collections::{VecDeque, HashMap};

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

    let mut seen: HashMap<VecDeque<isize>, HashMap<usize, usize>> = Default::default();
    let mut windows: Vec<VecDeque<isize>> = Default::default();
    for _ in 0..i.len() {
        windows.push(VecDeque::from(vec![0]));

    }
    for t in 0..3 {
        for (j, w) in windows.iter_mut().enumerate() {
            w.push_back((values[t+1][j] as isize) - (values[t][j] as isize));
        }
    }

    for t in 3..2000 {
        for (j, w) in windows.iter_mut().enumerate() {
            w.pop_front();
            w.push_back((values[t+1][j] as isize) - (values[t][j] as isize));

            seen.entry(w.clone())
                .or_default()
                .entry(j)
                .or_insert(values[t+1][j]);
        }
    }
    seen.values().map(|v| v.iter().map(|(_j, v)| v).sum::<usize>()).max().expect("there should be a maximum")
}
