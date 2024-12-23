use std::collections::HashMap;

pub fn parse(i: &str) -> Vec<usize> {
    i.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().expect("could not parse number"))
        .collect()
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
    i.iter()
        .map(|n| (0..2000).fold(*n, |acc, _n| evolve(acc)))
        //       .inspect(|secret| {dbg!(secret);})
        .sum::<usize>()
}

pub fn part2(i: &[usize]) -> usize {
    let mut seen: HashMap<[isize; 4], usize> = HashMap::with_capacity(20 * i.len());
    let mut seen_now: HashMap<[isize; 4], usize> = HashMap::with_capacity(2000);
    for secret in i {
        let mut secret = *secret;
        let mut window = [0isize; 4];
        let mut last_val = 0isize;
        for t in 0..3 {
            let new_val = (secret % 10) as isize;
            window[t] = new_val - last_val;
            last_val = new_val;
            secret = evolve(secret);
        }
        for _t in 3..=2000 {
            let new_val = (secret % 10) as isize;
            window[3] = new_val - last_val;
            last_val = new_val;
            seen_now.entry(window).or_insert(new_val as usize);
            secret = evolve(secret);
            window.rotate_left(1);
        }
        for (w, t) in seen_now.drain() {
            *seen.entry(w).or_default() += t;
        }
    }
    *seen.values().max().expect("there should be a maximum")
}
