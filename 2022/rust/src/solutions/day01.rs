pub fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut groups = vec![];
    let mut current = vec![];
    for line in input.lines() {
        if let Ok(n) = line.trim().parse::<usize>() {
            current.push(n);
        } else {
            groups.push(current);
            current = vec![];
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }
    groups
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|g| g.iter().sum())
        .max()
        .unwrap_or_default()
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    let mut sums: Vec<usize> = input.iter().map(|g| g.iter().sum()).collect();
    sums.sort();
    sums.iter().rev().take(3).sum()
}
