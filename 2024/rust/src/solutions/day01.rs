use std::str::FromStr;

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<usize> = line.trim().split_whitespace().map(|num| usize::from_str(num).expect("invalid number")).collect();
            (nums[0], nums[1])
        }).collect()
}

pub fn part1(pairs: &[(usize, usize)]) -> usize {
    let (mut num1, mut num2) = pairs.iter().fold((vec![], vec![]), |(mut l1, mut l2), (n1, n2)| { l1.push(n1); l2.push(n2); (l1, l2)});
    num1.sort();
    num2.sort();
    num1.iter().zip(num2.iter()).fold(0, |acc, (&n1, &n2)| { acc + (n1.abs_diff(*n2))})
}

pub fn part2(pairs: &[(usize, usize)]) -> usize {
    todo!()
}
