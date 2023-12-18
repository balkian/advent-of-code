use std::collections::HashMap;
use std::iter::once;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn is_damaged(&self) -> bool {
        self != &Spring::Operational
    }
    fn is_operational(&self) -> bool {
        self != &Spring::Damaged
    }
}

#[derive(Debug)]
pub struct Row {
    springs: Vec<Spring>,
    damaged: Vec<usize>,
}

fn split_by_windows(
    springs: &[Spring],
    size: usize,
) -> impl Iterator<Item = (&[Spring], &[Spring])> {
    index_by_windows(springs, size).map(move |ix| (&springs[..ix], &springs[ix + size..]))
}
fn index_by_windows(springs: &[Spring], size: usize) -> impl Iterator<Item = usize> + '_ {
    springs
        .windows(size)
        .enumerate()
        .skip(1)
        .rev()
        .skip(1)
        .rev()
        .filter(move |(idx, w)| {
            springs[idx - 1].is_operational()
                && w.iter().all(|s| s.is_damaged())
                && springs[idx + size].is_operational()
        })
        .map(move |(idx, _)| idx)
}
fn resolve(springs: &[Spring], damaged: &[usize]) -> usize {
    let mut memo = HashMap::<(&[Spring], &[usize]), usize>::new();
    let mut reused = 0;
    let res = resolve_memoized(springs, damaged, &mut memo, &mut reused);
    // dbg!(&memo.len(), reused);
    res
}
fn resolve_memoized<'b, 'c>(springs: &'b[Spring], damaged: &'b[usize],
     memo: &mut HashMap<(&'b [Spring], &'b [usize]), usize>,
    counter: &mut usize
) -> usize {
    if let Some(sol) = memo.get(&(springs, damaged)) {
        *counter += 1;
        return *sol;
    }
    let sol = if damaged.is_empty() {
        if springs.iter().all(|s| s.is_operational()) {
            1
        } else {
            0
        }
    } else {
        let idx_max = damaged.len() / 2;
        let max = damaged[idx_max];
        let (g1, g2) = damaged.split_at(idx_max);
        let g2 = &g2[1..]; // remove idx_max group
        let mut total = 0;
        for (head, tail) in split_by_windows(springs, max) {
            let r = resolve_memoized(head, g1, memo, counter);
            if r > 0 {
                total += r * resolve_memoized(tail, g2, memo, counter);
            }
        }
        total
    };
    memo.insert((springs, damaged), sol);
    sol
}

fn surround(springs: &mut Vec<Spring>) {
    springs.insert(0, Spring::Operational);
    springs.push(Spring::Operational);
}

pub fn parse(input: &str) -> Vec<Row> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (springs, groups) = line.split_once(' ').expect("could not split in two {line}");
            let mut springs: Vec<_> = springs
                .chars()
                .map(|c| match c {
                    '#' => Spring::Damaged,
                    '.' => Spring::Operational,
                    '?' => Spring::Unknown,
                    _ => panic!("Unknown character {c}"),
                })
                .collect();
            surround(&mut springs);

            let damaged = groups
                .split(',')
                .map(|n| n.parse::<usize>().expect("could not parse number {n}"))
                .collect();
            Row { springs, damaged }
        })
        .collect()
}

pub fn part1(rows: &[Row]) -> usize {
    rows.iter()
        .map(|row| resolve(&row.springs, &row.damaged))
        .sum()
}

pub fn part2(rows: &[Row]) -> usize {
    rows.iter()
        .enumerate()
        .map(|(_ix, row)| {
            // dbg!(ix);
            let mut springs: Vec<Spring> = row.springs[1..][..row.springs.len() - 2]
                .iter()
                .cloned()
                .chain(once(Spring::Unknown))
                .cycle()
                .take((row.springs.len() - 2) * 5 + 4)
                .collect();
            surround(&mut springs);
            let damaged: Vec<usize> = row
                .damaged
                .iter()
                .cloned()
                .cycle()
                .take(5 * row.damaged.len())
                .collect();
            resolve(&springs, &damaged)
        })
        .sum()
}
