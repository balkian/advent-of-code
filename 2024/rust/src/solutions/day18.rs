use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Pos = (usize, usize);

pub fn parse(i: &str) -> Vec<Pos> {
    i.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut toks = line.trim().split(",");
            (
                toks.next()
                    .expect("failed to get first number")
                    .parse()
                    .expect("first number not a valid number"),
                toks.next()
                    .expect("failed to get second number")
                    .parse()
                    .expect("first number not a valid number"),
            )
        })
        .collect()
}

const SIZE: usize = 71;
const EXIT: (usize, usize) = (SIZE - 1, SIZE - 1);
const START: (usize, usize) = (0, 0);

/// We use Dijkstra to find an optimal path between
/// START and EXIT.
pub fn part1(i: &[Pos]) -> usize {
    let mut dists = [[None; SIZE]; SIZE];
    let mut blocked = [[false; SIZE]; SIZE];

    for pos in i.iter().take(1024) {
        blocked[pos.0][pos.1] = true;
    }

    let mut heap = BinaryHeap::new();
    let target = EXIT;
    heap.push(Reverse((0, START)));
    while let Some(Reverse((cost, pos))) = heap.pop() {
        if blocked[pos.0][pos.1] {
            continue;
        }
        if dists[pos.0][pos.1].is_some() {
            continue;
        }
        if target == pos {
            return cost;
        }
        dists[pos.0][pos.1] = Some(cost);
        if (1..SIZE).contains(&pos.0) {
            heap.push(Reverse((cost + 1, (pos.0 - 1, pos.1))));
        }
        if (1..SIZE).contains(&pos.1) {
            heap.push(Reverse((cost + 1, (pos.0, pos.1 - 1))));
        }
        if (0..SIZE - 1).contains(&pos.0) {
            heap.push(Reverse((cost + 1, (pos.0 + 1, pos.1))));
        }
        if (0..SIZE - 1).contains(&pos.1) {
            heap.push(Reverse((cost + 1, (pos.0, pos.1 + 1))));
        }
    }
    panic!("no solution found")
}

/// This problem can be stated as finding the path from START to EXIT that would
/// last the longest.
///
/// We use a modified version of Dijkstra, where the cost of a path is the minimum
/// number of bytes that need to fall before the path is no longer possible. We also
/// use a MaxHeap to prioritize paths that would last the longest.
pub fn part2(i: &[Pos]) -> String {
    let mut dists = [[None; SIZE]; SIZE];
    let mut weights = [[None; SIZE]; SIZE];

    for (ix, pos) in i.iter().enumerate() {
        weights[pos.0][pos.1] = Some(ix);
    }

    let mut heap = BinaryHeap::new();
    let target = EXIT;
    heap.push((dists[START.0][START.1].unwrap_or(i.len()), START));
    while let Some((cost, pos)) = heap.pop() {
        if target == pos {
            let (x, y) = i[cost];
            return format!("{x},{y}");
        }
        if dists[pos.0][pos.1].is_some() {
            continue;
        }
        dists[pos.0][pos.1] = Some(cost);

        let mut opts = vec![];
        if (1..SIZE).contains(&pos.0) {
            opts.push((pos.0 - 1, pos.1));
        }
        if (1..SIZE).contains(&pos.1) {
            opts.push((pos.0, pos.1 - 1));
        }
        if (0..SIZE - 1).contains(&pos.0) {
            opts.push((pos.0 + 1, pos.1));
        }
        if (0..SIZE - 1).contains(&pos.1) {
            opts.push((pos.0, pos.1 + 1));
        }
        for opt in opts {
            let ncost = cost.min(weights[opt.0][opt.1].unwrap_or(cost));
            heap.push((ncost, opt));
        }
    }
    panic!("solution not found");
}
