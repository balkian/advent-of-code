pub fn parse(input: &str) -> &str {
    input
}

fn first_number(
    input: &str,
    sequence: impl IntoIterator<Item = usize>,
    patterns: &[&str],
) -> Option<usize> {
    for i in sequence.into_iter() {
        let input = &input[i..];
        if let Some(c) = input.chars().next() {
            if let Some(i) = c.to_digit(10) {
                return Some(i as usize);
            }
            for (ix, pat) in patterns.iter().enumerate() {
                if input.starts_with(pat) {
                    return Some(ix + 1);
                }
            }
        } else {
            return None;
        }
    }
    return None;
}

pub fn solve(input: &str, patterns: &[&str]) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let d1 = first_number(line, 0..line.len(), patterns).unwrap();
            let d2 = first_number(line, (0..line.len()).rev(), patterns).unwrap();
            format!("{}{}", d1, d2).parse::<usize>().unwrap()
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, &[])
}

pub fn part2(input: &str) -> usize {
    let replaced: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    solve(input, &replaced)
}
