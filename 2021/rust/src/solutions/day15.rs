use crate::aoc_sample;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            Some(
                line.chars()
                    .map(|num| num.to_string().parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0))));

    let target = (input.len() - 1, input[input.len() - 1].len() - 1);

    while let Some(Reverse((weight, (i, j)))) = heap.pop() {
        if *distances.entry((i, j)).or_insert(usize::MAX) <= weight {
            continue;
        }
        distances.insert((i, j), weight);
        if (i, j) == target {
            break;
        }
        if i > 0 {
            heap.push(Reverse((weight + input[i - 1][j], (i - 1, j))));
        }
        if j > 0 {
            heap.push(Reverse((weight + input[i][j - 1], (i, j - 1))));
        }
        if i < input.len() - 1 {
            heap.push(Reverse((weight + input[i + 1][j], (i + 1, j))));
        }
        if j < input[i].len() - 1 {
            heap.push(Reverse((weight + input[i][j + 1], (i, j + 1))));
        }
    }
    distances[&target]
}
pub fn part2(input: &[Vec<usize>]) -> usize {
    let input: Vec<Vec<usize>> = input
        .iter()
        .cycle()
        .take(5 * input.len())
        .enumerate()
        .map(|(ix, row)| {
            row.iter()
                .cycle()
                .take(5 * row.len())
                .enumerate()
                .map(|(jx, cell)| {
                    let mut newval = cell + (ix / input.len()) + (jx / input[0].len());
                    while newval > 9 {
                        newval -= 9;
                    }
                    newval
                })
                .collect()
        })
        .collect();
    part1(&input)
}

aoc_sample!(day15sample1part1, "../../day15.sample1", part1, 40);
aoc_sample!(day15sample1part2, "../../day15.sample1", part2, 315);
