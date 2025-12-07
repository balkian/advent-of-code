use std::collections::{HashSet, HashMap};

type Pos = (isize, isize);
type Input = (Pos, HashSet<Pos>);

pub fn parse(i: &str) -> Input {
    let (start, splitters) = i.lines().enumerate().flat_map(|(ix, line)| {
        line.chars().enumerate().map(move |(jx, cell)| ((ix as isize, jx as isize), cell))
        }).fold((None, HashSet::new()), |(ref mut start, mut splitters), (pos, c)| {
            match c {
                '^'  => {
                    splitters.insert(pos);
                }
                'S' => {
                    *start = Some(pos)
                }
                _ => {
                }
            }
            (*start, splitters)

    });
    (start.expect("start not found"), splitters)
}

pub fn part1((start, splitters): &Input) -> usize {
    let mut beams = HashSet::new();
    beams.insert(start.1);
    let maxlevel: isize = splitters.iter().map(|(level, _)| level).max().copied().unwrap_or_default();

    let mut collisions = 0;
    for level in start.0..=maxlevel {
        let split: Vec<_> = beams.extract_if(|x| splitters.contains(&(level, *x))).collect();
        for x in split {
            collisions += 1;
            beams.insert(x-1);
            beams.insert(x+1);
        }
    }
    collisions
}

pub fn part2((start, splitters): &Input) -> usize {
    let mut beams = HashMap::new();
    beams.insert(start.1, 1);
    let maxlevel: isize = splitters.iter().map(|(level, _)| level).max().copied().unwrap_or_default();

    for level in start.0..=maxlevel {
        let split: Vec<_> = beams.extract_if(|x, _| splitters.contains(&(level, *x))).collect();
        for (x, val) in split {
            *beams.entry(x-1).or_default() += val;
            *beams.entry(x+1).or_default() += val;
        }
    }
    beams.values().sum()
}
