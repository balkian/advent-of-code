use std::collections::HashMap;
type Pos = (isize, isize);
type Input = HashMap<Pos, u32>;

pub fn parse(i: &str) -> Input {
    i.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .take_while(|c| !c.is_whitespace())
                .enumerate()
                .map(move |(x, c)| {
                    (
                        (y as isize, x as isize),
                        c.to_digit(10).expect("could not convert number"),
                    )
                })
        })
        .collect()
}

fn get_paths(i: &Input) -> HashMap<Pos, Vec<Pos>> {
    let mut edges: HashMap<Pos, Vec<Pos>> = Default::default();
    let mut start = vec![];
    for (pos, value) in i {
        if value == &0 {
            start.push(*pos);
        }
        for neighbor in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            match i.get(&neighbor) {
                Some(neighbor_value) if *neighbor_value == value + 1 => {
                    edges.entry(*pos).or_default().push(neighbor);
                }
                _ => {}
            }
        }
    }
    let mut paths: HashMap<Pos, Vec<Pos>> = Default::default();
    for pos in start {
        let mut tails = vec![pos];
        let mut reachable: Vec<Pos> = Default::default();
        while let Some(nxt) = tails.pop() {
            if let Some(others) = edges.get(&nxt) {
                for other in others {
                    if i.get(other).is_some_and(|o| o == &9) {
                        reachable.push(*other);
                    } else {
                        tails.push(*other);
                    }
                }
            }
        }
        paths.insert(pos, reachable);
    }
    paths
}

pub fn part1(i: &Input) -> usize {
    let mut paths = get_paths(i);
    paths
        .values_mut()
        .map(|paths| {
            paths.sort();
            paths.dedup();
            paths.len()
        })
        .sum::<usize>()
}

pub fn part2(i: &Input) -> usize {
    let paths = get_paths(i);
    paths.values().map(|paths| paths.len()).sum::<usize>()
}
