use itertools::Itertools;

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|line| (!line.is_empty()).then(|| line.parse().unwrap()))
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .powerset()
        .filter(|ps| ps.iter().copied().sum::<usize>() == 150)
        .count()
}

pub fn part2(input: &[usize]) -> usize {
    let mut valid: Vec<usize> = input
        .iter()
        .powerset()
        .filter(|ps| ps.iter().copied().sum::<usize>() == 150)
        .map(|ps| ps.len())
        .collect();
    valid.sort_unstable();
    let min = valid[0];
    valid.into_iter().take_while(|s| *s == min).count()
}
