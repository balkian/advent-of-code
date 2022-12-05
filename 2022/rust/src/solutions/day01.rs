pub fn parse(input: &str) -> Vec<usize> {
    let mut groups =
        input
            .lines()
            .map(|line| line.trim().parse::<usize>())
            .fold(vec![0], |mut acc, b| {
                if let Ok(b) = b {
                    let last = acc.len() - 1;
                    acc[last] += b;
                } else {
                    acc.push(0);
                }
                acc
            });
    groups.sort();
    groups
}

pub fn part1(input: &[usize]) -> usize {
    input.last().copied().unwrap_or_default()
}

pub fn part2(input: &[usize]) -> usize {
    input.iter().rev().take(3).sum()
}
