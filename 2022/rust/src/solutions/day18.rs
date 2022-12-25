use std::collections::{HashMap, HashSet};
type Pos = Vec<usize>;

pub fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.trim()
                .split(',')
                .map(|n| 1 + n.parse::<usize>().unwrap()) // To allow for wiggle room in part2
                .collect()
        })
        .collect()
}

fn are_attached(pos1: &Pos, pos2: &Pos) -> bool {
    pos1.iter().zip(pos2.iter()).fold((0, 0), |acc, (c1, c2)| {
        if c1 == c2 {
            (acc.0 + 1, acc.1)
        } else if c1.abs_diff(*c2) == 1 {
            (acc.0, acc.1 + 1)
        } else {
            acc
        }
    }) == (2, 1)
}

pub fn part1(input: &[Pos]) -> usize {
    // dbg!(&input.len());
    let mut sums: HashMap<usize, Vec<&Pos>> = Default::default();
    for p in input {
        sums.entry(p.iter().sum::<usize>()).or_default().push(p);
    }
    let mut sides = 6 * input.len();
    for p in input {
        let sum = p.iter().sum::<usize>();
        for op in sums
            .get(&(sum - 1))
            .into_iter()
            .chain(sums.get(&(sum + 1)).into_iter())
            .flatten()
        {
            if are_attached(p, op) {
                sides -= 1;
            }
        }
    }
    sides
}
pub fn part2(input: &[Pos]) -> usize {
    let maxs = input.iter().fold(vec![0, 0, 0], |mut acc, p| {
        p.iter()
            .enumerate()
            .for_each(|(ix, c)| acc[ix] = std::cmp::max(acc[ix], *c));
        acc
    });
    // dbg!(&maxs);
    let lava: HashSet<&Pos> = input.iter().collect();
    let mut stack: Vec<Pos> = vec![];
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut touched = 0;

    let start = maxs.iter().copied().map(|i| i + 1).collect();
    let limits: Vec<_> = maxs.iter().copied().map(|i| i + 2).collect();
    stack.push(start);
    while let Some(next) = stack.pop() {
        // dbg!(&next);
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next.clone());
        for (idx, val) in next.iter().enumerate() {
            if *val < limits[idx] {
                let mut previous = next.clone();
                previous[idx] = val + 1;
                if lava.contains(&previous) {
                    touched += 1;
                } else {
                    stack.push(previous);
                }
            }
            if *val > 0 {
                let mut copied = next.clone();
                copied[idx] = val - 1;
                if lava.contains(&copied) {
                    touched += 1;
                } else {
                    stack.push(copied);
                }
            }
        }
    }
    touched
}
