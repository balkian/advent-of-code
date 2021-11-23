use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

use scan_fmt::scan_fmt;
use std::cmp::{max, Ordering};

#[macro_use]
extern crate more_asserts;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bot {
    pos: Coord,
    r: usize,
}

const MAX_DIM: usize = 3;
type Coord = [isize; MAX_DIM];

#[derive(Debug, PartialEq, Eq, Clone)]
struct BoundingBox {
    min_coords: Coord,
    max_coords: Coord,
    in_range: usize,
    dims: [usize; MAX_DIM],
    dist_origin: usize,
}

impl Ord for BoundingBox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.in_range
            .cmp(&(other.in_range))
            .then_with(|| other.dist_origin.cmp(&self.dist_origin))
    }
}

impl PartialOrd for BoundingBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BoundingBox {
    fn new(min_coords: Coord, max_coords: Coord, bots: &[Bot]) -> Self {
        let mut s = Self {
            min_coords,
            max_coords,
            in_range: 0,
            dist_origin: 0,
            dims: [0, 0, 0],
        };
        s.dims = s
            .max_coords
            .iter()
            .zip(s.min_coords.iter())
            .map(|(a, b)| (a - b) as usize)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        s.dist_origin = s.distance([0, 0, 0]);
        s.in_range = bots.iter().filter(|b| s.in_range(b)).count();
        s
    }
    fn split(&self, bots: &[Bot]) -> Option<Vec<BoundingBox>> {
        if self.dims.iter().sum::<usize>() == 0 {
            return None;
        }

        let mut opts: HashSet<(Coord, Coord)> = HashSet::new();

        let midp: Coord = self
            .min_coords
            .iter()
            .zip(self.max_coords.iter())
            .map(|(a, b)| (a + b) / 2)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for i in 0..8 {
            let mut minc = self.min_coords;
            let mut maxc = midp;
            for dim in 0..MAX_DIM {
                if self.dims[dim] > 0 && i & (1 << dim) == (1 << dim) {
                    minc[dim] = midp[dim] + 1;
                    maxc[dim] = self.max_coords[dim];
                }
            }
            opts.insert((minc, maxc));
        }

        if opts.len() < 2 {
            return None;
        }
        let num_results = self
            .dims
            .iter()
            .fold(1, |acc, d| if *d > 0 { acc * 2 } else { acc });
        debug_assert_eq!(opts.len(), num_results);
        Some(
            opts.into_iter()
                .map(|(new_min, new_max)| BoundingBox::new(new_min, new_max, bots))
                .collect(),
        )
    }

    fn in_range(&self, bot: &Bot) -> bool {
        dist(bot.pos, self.min_coords) + dist(bot.pos, self.max_coords)
            <= (2 * bot.r + self.dims.iter().sum::<usize>())
    }

    fn distance(&self, pos: Coord) -> usize {
        ((dist(self.min_coords, pos) + dist(self.max_coords, pos)) + 1) / 2
    }
}

impl Bot {
    fn distance(&self, other: &Bot) -> usize {
        dist(self.pos, other.pos)
    }
    fn in_range(&self, other: &Bot) -> bool {
        let d = self.distance(other);
        d <= max(self.r, other.r)
    }
}

fn dist(p1: Coord, p2: Coord) -> usize {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<isize>() as usize
}

fn parse(input: &str) -> Vec<Bot> {
    let mut bots = vec![];
    for line in input.lines() {
        let (x, y, z, r) =
            scan_fmt!(line, "pos=<{d},{d},{d}>, r={d}", isize, isize, isize, usize).unwrap();
        bots.push(Bot { pos: [x, y, z], r });
    }
    bots
}

fn solve1(input: &str) -> usize {
    let bots = parse(input);
    let strongest = bots.iter().max_by_key(|bot| bot.r).unwrap();
    let count = bots.iter().filter(|bot| bot.in_range(strongest)).count();
    println!("Solution 1: {}", count);
    count
}

fn solve2(input: &str) -> usize {
    let bots = parse(input);

    let (min, max): (Vec<_>, Vec<_>) = (0..MAX_DIM)
        .into_iter()
        .map(|i| {
            bots.iter()
                .map(|b| b.pos[i])
                .minmax()
                .into_option()
                .unwrap()
        })
        .unzip();

    let b = BoundingBox::new(min.try_into().unwrap(), max.try_into().unwrap(), &bots);
    debug_assert_eq!(b.in_range, bots.len());

    let mut boxes = BinaryHeap::from([b]);

    while let Some(b) = boxes.pop() {
        if let Some(new_boxes) = b.split(&bots) {
            for new_b in new_boxes {
                if new_b.in_range > 0 {
                    boxes.push(new_b);
                }
            }
            debug_assert_ge!(boxes.iter().map(|b| b.in_range).sum::<usize>(), bots.len());
            continue;
        }
        println!("Solution 2: {:?}", b.dist_origin);
        return b.dist_origin;
    }
    panic!("Solution not found");
}

fn main() {
    let input = &std::fs::read_to_string("input").expect("could not read file");
    solve1(input);
    solve2(input);
}

#[test]
fn test_split() {
    let bots = &vec![];
    let min_coords = [10, 12, 10];
    let max_coords = [12, 14, 12];
    let mut a = BoundingBox::new(min_coords, max_coords, bots);
    let opts = a.split(bots);
    assert!(opts.is_some());
    let opts = opts.unwrap();
    assert_eq!(opts.len(), 8);
    let mut boxes = vec![a];
    let mut visited = HashSet::new();
    loop {
        if let Some(b) = boxes.pop() {
            if let Some(vs) = b.split(bots) {
                for v in vs {
                    visited.insert((v.min_coords, v.max_coords));
                    if v.min_coords == v.max_coords && v.max_coords == [12, 12, 12] {
                        break;
                    }
                    boxes.push(v);
                }
                dbg!(&visited);
            }
        } else {
            break;
        }
    }

    let points_visited: HashSet<Vec<isize>> = visited
        .iter()
        .flat_map(|(min, max)| {
            (0..MAX_DIM)
                .into_iter()
                .map(|i| min[i]..=max[i])
                .multi_cartesian_product()
        })
        .collect();

    let allpoints: HashSet<Vec<isize>> = (0..MAX_DIM)
        .into_iter()
        .map(|i| min_coords[i]..=max_coords[i])
        .multi_cartesian_product()
        .collect();
    let not_visited: Vec<_> = allpoints.difference(&points_visited).collect();
    dbg!(&not_visited);
    assert!(not_visited.is_empty());
}

#[test]
fn test_range() {
    let bots = &parse(include_str!("../example2"));
    let a = BoundingBox::new([12, 12, 12], [12, 12, 12], &bots);
    assert_eq!(a.in_range, 5);
}

#[test]
fn test_example() {
    assert_eq!(solve2(include_str!("../example2")), 36);
}
