use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Coord = (isize, isize, isize);

pub fn parse(i: &str) -> Vec<Coord> {
    i.lines()
        .filter(|&line| !line.is_empty()).map(|line| {
                let coords: Vec<isize> = line
                    .trim()
                    .split(",")
                    .map(|num| num.parse().expect("could not parse"))
                    .collect();
                match coords[..] {
                    [x, y, z] => (x, y, z),
                    _ => panic!("failed to parse line {line}"),
                }
            })
        .collect()
}

pub fn part1(coords: &[Coord]) -> usize {
    solve(coords, false)
}

pub fn solve(coords: &[Coord], tillend: bool) -> usize {
    let mut circuits: HashMap<usize, Vec<usize>> = coords
        .iter()
        .enumerate()
        .map(|(ix, _coord)| (ix, vec![ix]))
        .collect();
    let mut belonging: Vec<usize> = (0..coords.len()).collect();
    let mut distances: BinaryHeap<Reverse<(isize, (usize, usize))>> = coords
        .iter()
        .enumerate()
        .flat_map(|(ix1, c1)| {
            coords.iter().enumerate().filter(move |(ix2, _)| ix1 < *ix2).map(move |(ix2, c2)| {
                    (
                        (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2) + (c1.2 - c2.2).pow(2),
                        (ix1, ix2),
                    )
            })
        })
        .map(Reverse)
        .collect();

    for connected in 0.. {
        let Some(Reverse((_dist, (ix1, ix2)))) = distances.pop() else {
            panic!("not enough pairs");
        };
        let circuit1 = belonging[ix1];
        let circuit2 = belonging[ix2];
        if circuit1 == circuit2 {
            continue;
        }
        let toremove = circuit1.min(circuit2);
        let removed = circuits.remove(&toremove).expect("circuit without boxes!");
        let target = circuit1.max(circuit2);
        let group = circuits.get_mut(&target).expect("target without boxes");
        for pos in removed {
            belonging[pos] = target;
            group.push(pos);
        }
        if tillend {
            if circuits.len() == 1 {
                return (coords[ix1].0 * coords[ix2].0) as usize;
            }
        } else if connected > 1_000 {
            break;
        }
    }
    let mut sizes: Vec<_> = circuits.values().map(|g| g.len()).collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

pub fn part2(coords: &[Coord]) -> usize {
    solve(coords, true)
}
