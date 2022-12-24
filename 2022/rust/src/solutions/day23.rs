use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

type Coord = [isize; 2];

static POSITIONS: [Coord; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

#[derive(Clone, Debug)]
pub struct Map {
    elves: HashSet<Coord>,
    rounds: usize,
}

impl Map {
    fn bounds(&self) -> ([isize; 2], [isize; 2]) {
        self.elves.iter().fold(
            ([isize::MAX, isize::MIN], [isize::MAX, isize::MIN]),
            |acc, coord| {
                (
                    [min(acc.0[0], coord[0]), max(acc.0[1], coord[0])],
                    [min(acc.1[0], coord[1]), max(acc.1[1], coord[1])],
                )
            },
        )
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!();
        let ([min_y, max_y], [min_x, max_x]) = self.bounds();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = if self.elves.contains(&[y, x]) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
    fn empty_size(&self) -> usize {
        let ([min_y, max_y], [min_x, max_x]) = self.bounds();
        ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - self.elves.len()
    }

    fn evolve(&mut self) -> bool {
        let mut moved = false;
        let mut proposals: HashMap<Coord, Vec<Coord>> = Default::default();

        for elf in self.elves.iter() {
            if let Some(next) = self.propose(elf) {
                proposals.entry(next).or_default().push(*elf);
            }
        }
        for (proposed, mut elves) in proposals.into_iter() {
            if elves.len() > 1 {
                continue;
            }
            let elf = elves.pop().unwrap();
            moved = true;
            self.elves.remove(&elf);
            self.elves.insert(proposed);
        }
        self.rounds += 1;
        moved
    }

    fn propose(&self, elf: &Coord) -> Option<Coord> {
        if (elf[0] - 1..=elf[0] + 1)
            .flat_map(|i| {
                (elf[1] - 1..=elf[1] + 1).filter_map(move |j| {
                    if &[i, j] == elf {
                        None
                    } else {
                        self.elves.get(&[i, j])
                    }
                })
            })
            .count()
            == 0
        {
            return None;
        }
        'positions: for ix in (0..POSITIONS.len()).map(|ix| (ix + self.rounds) % POSITIONS.len()) {
            let delta = POSITIONS[ix];
            let fixed = delta.iter().position(|c| c == &0).unwrap();
            let nonfixed = (fixed + 1) % 2;
            let mut pos = *elf;
            pos[nonfixed] = elf[nonfixed] + delta[nonfixed];
            for di in -1..=1 {
                let mut pos = pos;
                pos[fixed] += di;
                if self.elves.get(&pos).is_some() {
                    continue 'positions;
                }
            }
            return Some(pos);
        }
        None
    }
}
pub fn parse(input: &str) -> Map {
    Map {
        rounds: 0,
        elves: input
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(i, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(j, c)| match c {
                        '#' => Some([i as isize, j as isize]),
                        '.' => None,
                        _ => panic!("wrong character {c}"),
                    })
            })
            .collect(),
    }
}

pub fn part1(input: &Map) -> usize {
    let mut map = input.clone();
    for _i in 0..10 {
        map.evolve();
    }
    map.empty_size()
}

pub fn part2(input: &Map) -> usize {
    let mut map = input.clone();
    // map.print();
    for i in 1.. {
        if !map.evolve() {
            return i;
        }
        // println!("Round {i}");
        // map.print();
    }
    unreachable!();
}
