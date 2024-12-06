use std::collections::HashMap;
use std::str::FromStr;

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| usize::from_str(num).expect("invalid number"));
            (
                nums.next().expect("no numbers available"),
                nums.next().expect("only one number available"),
            )
        })
        .collect()
}

pub fn part1(pairs: &[(usize, usize)]) -> usize {
    let (mut num1, mut num2) = pairs
        .iter()
        .fold((vec![], vec![]), |(mut l1, mut l2), (n1, n2)| {
            l1.push(n1);
            l2.push(n2);
            (l1, l2)
        });
    num1.sort();
    num2.sort();
    num1.iter()
        .zip(num2.iter())
        .fold(0, |acc, (&n1, &n2)| acc + n1.abs_diff(*n2))
}

pub fn part2(pairs: &[(usize, usize)]) -> usize {
    let (count1, count2) = pairs.iter().fold(
        (
            HashMap::<usize, usize>::new(),
            HashMap::<usize, usize>::new(),
        ),
        |(mut c1, mut c2), (n1, n2)| {
            *c1.entry(*n1).or_default() += 1;
            *c2.entry(*n2).or_default() += 1;
            (c1, c2)
        },
    );
    count1.into_iter().fold(0, |acc, (n, c)| {
        acc + n * c * count2.get(&n).copied().unwrap_or_default()
    })
}
