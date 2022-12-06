use itertools::Itertools;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn solve(input: &str, num: usize) -> usize {
    input
        .as_bytes()
        .windows(num)
        .enumerate()
        .find(|(_idx, win)| win.iter().duplicates().next().is_none())
        .unwrap()
        .0
        + num
}

pub fn part1(input: &str) -> usize {
    solve(input, 4)
}

pub fn part2(input: &str) -> usize {
    solve(input, 14)
}
