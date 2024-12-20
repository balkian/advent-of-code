use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

type Grid = Vec<Vec<Tile>>;
type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub struct Problem {
    grid: Grid,
    start: Pos,
    end: Pos,
}

pub fn parse(i: &str) -> Problem {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    for (y, line) in i.lines().enumerate() {
        let mut row = vec![];
        for (x, cell) in line.trim().chars().enumerate() {
            match cell {
                'S' => {
                    start = Some((x, y));
                    row.push(Tile::Empty);
                }
                'E' => {
                    end = Some((x, y));
                    row.push(Tile::Empty);
                }
                '.' => {
                    row.push(Tile::Empty);
                }
                '#' => {
                    row.push(Tile::Wall);
                }
                _ => panic!("unknown char {cell}"),
            }
        }
        grid.push(row);
    }
    Problem {
        grid,
        start: start.expect("start pos not found"),
        end: end.expect("end pos not found"),
    }
}

fn neighbors(pos: Pos, grid: &Grid) -> Vec<Pos> {
    let mut opts = vec![];
    if pos.0 > 0 {
        opts.push((pos.0 - 1, pos.1));
    }
    if pos.0 < grid[pos.1].len() - 1 {
        opts.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        opts.push((pos.0, pos.1 - 1));
    }
    if pos.1 < grid[pos.1].len() - 1 {
        opts.push((pos.0, pos.1 + 1));
    }
    opts
}

fn find_cheats(
    start: Pos,
    current_pos: Pos,
    current_dist: usize,
    dists: &Vec<Vec<Option<usize>>>,
    grid: &Grid,
    remaining: usize,
    store: &mut HashMap<(Pos, Pos), usize>,
) {
    match (start == current_pos, dists[current_pos.1][current_pos.0]) {
        (false, Some(old)) if old > current_dist => {
            let diff = old - current_dist;
            store
                .entry((start, current_pos))
                .and_modify(|e| *e = diff.min(*e))
                .or_insert(diff);
        }
        (true, Some(_)) | (false, None) if remaining > 0 => {
            for n in neighbors(current_pos, grid) {
                find_cheats(
                    start,
                    n,
                    current_dist + 1,
                    dists,
                    grid,
                    remaining - 1,
                    store,
                );
            }
        }
        _ => {}
    }
}

pub fn part1(p: &Problem) -> usize {
    let mut dists: Vec<Vec<Option<usize>>> =
        p.grid.iter().map(|row| vec![None; row.len()]).collect();
    let mut path = vec![];
    let mut pending = vec![(0, p.start)];
    while let Some((d, pos)) = pending.pop() {
        if dists[pos.1][pos.0].is_none() {
            path.push(pos);
            dists[pos.1][pos.0] = Some(d);
            for n in neighbors(pos, &p.grid) {
                if matches!(p.grid[n.1][n.0], Tile::Empty) {
                    pending.push((d + 1, n));
                }
            }
        }
    }
    let mut cheats: HashMap<(Pos, Pos), usize> = Default::default();
    for startpos in path.iter() {
        find_cheats(
            *startpos,
            *startpos,
            dists[startpos.1][startpos.0].expect("no distance for point in path"),
            &dists,
            &p.grid,
            2,
            &mut cheats,
        );
    }
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for (_, d) in cheats.iter() {
        *counts.entry(*d).or_default() += 1;
    }
    //    dbg!(cheats.len(), &counts);
    cheats.retain(|_, d| *d >= 100);
    cheats.len()
}

pub fn part2(p: &Problem) -> usize {
    let mut dists: Vec<Vec<Option<usize>>> =
        p.grid.iter().map(|row| vec![None; row.len()]).collect();
    let mut path = vec![];
    let mut pending = vec![(0, p.start)];
    while let Some((d, pos)) = pending.pop() {
        if dists[pos.1][pos.0].is_none() {
            path.push(pos);
            dists[pos.1][pos.0] = Some(d);
            for n in neighbors(pos, &p.grid) {
                if matches!(p.grid[n.1][n.0], Tile::Empty) && dists[n.1][n.0].is_none() {
                    pending.push((d + 1, n));
                }
            }
        }
        if pos != p.end {
            assert_eq!(pending.len(), 1);
        }
    }
    let mut cheats: HashMap<(Pos, Pos), usize> = Default::default();
    let threshold = 100;
    let dists = dists;
    for start in path {
        let mut done: HashSet<Pos> = Default::default();
        let mut pending = vec![];
        done.insert(start);
        for n in neighbors(start, &p.grid) {
            pending.push(n);
        }
        let start_dist =
            dists[start.1][start.0].expect("all positions in path should have a distance") + 1;
        for d in (start_dist..).take(20) {
            let mut new_pending = vec![];
            for o in pending.drain(..) {
                if done.contains(&o) {
                    continue;
                }
                done.insert(o);
                match dists[o.1][o.0] {
                    Some(old) if old >= d + threshold => {
                        let diff = old - d;
                        if let Some(oldcheat) = cheats.get(&(start, o)) {
                            if *oldcheat > diff {
                                panic!("this should never happen");
                            }
                        }
                        cheats.insert((start, o), diff);
                    }
                    _ => {}
                }
                for n in neighbors(o, &p.grid) {
                    new_pending.push(n);
                }
            }
            pending = new_pending;
        }
    }
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for (_, d) in cheats.iter() {
        *counts.entry(*d).or_default() += 1;
    }
    cheats.len()
}
