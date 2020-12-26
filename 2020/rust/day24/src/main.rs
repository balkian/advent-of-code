use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
enum Pos {
    NW,
    NE,
    W,
    E,
    SE,
    SW,
}

type Coord = (isize, isize);

fn parse_line(line: Vec<Pos>) -> Coord {
    let mut coord = (0, 0);
    for pos in line {
        let delta = match pos {
            Pos::NW => (1, -1),
            Pos::NE => (1, 1),
            Pos::E => (0, 2),
            Pos::W => (0, -2),
            Pos::SW => (-1, -1),
            Pos::SE => (-1, 1),
        };
        coord.0 += delta.0;
        coord.1 += delta.1;
    }
    coord
}

fn game(black: &[Coord]) -> HashSet<Coord> {
    let mut alive: HashSet<Coord> = black.iter().copied().collect();
    for _ in 0..100 {
        let mut counter: HashMap<Coord, usize> = HashMap::new();
        for c in &alive {
            for (dy, dx) in &[(0, -2), (0, 2), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                *counter.entry((c.0 + dy, c.1 + dx)).or_default() += 1;
            }
        }
        let mut next = HashSet::new();
        for (c, count) in counter {
            match (count, alive.contains(&c)) {
                (1..=2, true) => {
                    next.insert(c);
                }
                (2, false) => {
                    next.insert(c);
                }
                _ => {}
            }
        }
        alive = next;
    }
    alive
}

fn main() {
    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = read_to_string(file).expect("could not read file");
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let mut res: Vec<Pos> = vec![];
            let mut last = ' ';
            for c in line.chars() {
                match (c, last) {
                    ('e', 'n') => res.push(Pos::NE),
                    ('e', 's') => res.push(Pos::SE),
                    ('e', _) => res.push(Pos::E),
                    ('w', 'n') => res.push(Pos::NW),
                    ('w', 's') => res.push(Pos::SW),
                    ('w', _) => res.push(Pos::W),
                    (c, _) if ['s', 'n'].contains(&c) => {}
                    _ => panic!("invalid: {} {}", c, last),
                }
                last = c;
            }
            res
        })
        .map(parse_line)
        .collect();

    let mut count: HashMap<Coord, usize> = HashMap::new();
    for coord in coords {
        *count.entry(coord).or_default() += 1;
    }
    let black: Vec<Coord> = count
        .into_iter()
        .filter(|(_, times)| (*times % 2) == 1)
        .map(|(coord, _)| coord)
        .collect();

    println!("Part 1: {}", black.len());
    let alive = game(&black);
    println!("Part 2: {}", alive.len());
}
