pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.trim()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect(),
                )
            }
        })
        .collect()
}

pub fn part1(banks: &[Vec<usize>]) -> usize {
    banks
        .iter()
        .map(|bank| {
            let mut a = bank[0];
            if bank.len() == 1 {
                return a;
            }
            let mut max = a;
            for b in &bank[1..bank.len()] {
                max = max.max(a * 10 + b);
                if *b > a {
                    a = *b;
                }
            }
            max
        })
        .sum()
}

fn maxdigit(bank: &[usize], start: usize, end: usize) -> (usize, usize) {
    bank[start..end]
        .iter()
        .enumerate()
        .rev()
        .fold((usize::MAX, 0), |(maxidx, max), (idx, val)| {
            if *val >= max {
                (idx, *val)
            } else {
                (maxidx, max)
            }
        })
}

pub fn part2(banks: &[Vec<usize>]) -> usize {
    banks
        .iter()
        .map(|bank| {
            let mut joltage = 0;
            let mut start = 0;
            for i in (0..12).rev() {
                let end = bank.len() - i;
                let (delta, max) = maxdigit(bank, start, end);
                joltage = joltage * 10 + max;
                start += delta + 1;
            }

            joltage
        })
        .sum()
}
