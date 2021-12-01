pub fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

pub fn part1(input: &[isize]) -> usize {
    input.windows(2).filter(|win| win[1] > win[0]).count()
}

pub fn part2(input: &[isize]) -> isize {
    input
        .windows(3)
        .map(|win| win.iter().sum())
        .scan(isize::MAX, |acc, a| {
            let gt = a > *acc;
            *acc = a;
            if gt {
                Some(1)
            } else {
                Some(0)
            }
        })
        .sum()
}
