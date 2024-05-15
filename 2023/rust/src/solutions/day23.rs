use std::collections::{HashMap,HashSet};

type Point = (usize, usize);

#[derive(Debug,Clone)]
struct Path {
    last: (usize, usize),
    previous: HashSet<(usize, usize)>,
    distance: usize,
}

#[derive(Debug,Clone)]
pub struct Grid {
    tiles: Vec<Vec<char>>,
    graph: HashMap<Point, HashMap<Point, usize>>,
}

impl Grid {
    fn new(tiles: Vec<Vec<char>>) -> Self {
        let graph = Default::default();
        let mut grid = Self{tiles, graph};
        let mut new_intersections = vec![];
        new_intersections.push(grid.start());
        let end = grid.end();
        while let Some(origin) = new_intersections.pop() {
            if grid.graph.contains_key(&origin) {
                continue;
            }
            let mut visited: HashSet<Point> = Default::default();
            let mut pending: Vec<(Point, usize)> = vec![];
            visited.insert(origin);
            for opt in grid.options_from_tiles(origin, false) {
                pending.push((opt, 1));
            }
            while let Some((n, dist)) = pending.pop() {
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(n);
                let opts = grid.options_from_tiles(n, false);
                if opts.len() > 2 || n == end {
                    // We found an intersection
                    let d = grid.graph.entry(origin).or_default().entry(n).or_default();
                    if *d < dist {
                        *d = dist;
                    }
                    new_intersections.push(n);
                } else {
                    for opt in opts {
                        pending.push((opt, dist+1));
                    }
                }
            }

        }
        grid
    } 

    fn options_from_graph(&self, pos: (usize, usize)) -> impl Iterator<Item = (&Point, &usize)> {
        self.graph.get(&pos)
            .expect("origin not in graph")
            .into_iter()
    }

    fn options_from_tiles(&self, pos: (usize, usize), slippery: bool) -> Vec<(usize, usize)>{
        let mut next = match (slippery, self.tiles[pos.0][pos.1]) {
            (_, '#') => vec![],
            (true, '>') => vec![(pos.0, pos.1+1)],
            (true, '<') => vec![(pos.0, pos.1-1)],
            (true, 'v') => vec![(pos.0+1, pos.1)],
            (true, '^') => vec![(pos.0-1, pos.1)],
            (_, '.') | (false, 'v' | '^' | '>' | '<') => vec![(pos.0, pos.1+1),
                        (pos.0, pos.1-1),
                        (pos.0+1, pos.1),
                        (pos.0-1, pos.1)],
            _ => panic!("unknown char"),
        };
        next.retain(|pos| {
            pos.0 > 0 && pos.1 > 0 && pos.0 < self.tiles.len() && self.tiles[pos.0][pos.1] != '#'

        });
        next
    }

    fn start(&self) -> (usize, usize) {
        (1, self.tiles[0].iter().position(|&t| t == '.').expect("start not found"))

    }
    fn end(&self) -> (usize, usize) {
        (self.tiles.len()-2, self.tiles[self.tiles.len()-1].iter().position(|&t| t == '.').expect("end not found"))

    }
}

pub fn parse(input: &str) -> Grid {
    let tiles = input.lines().filter(|line| !line.is_empty()).map(|line| {
        line.chars().collect()
    }).collect();
    Grid::new(tiles)
}

pub fn part1(input: &Grid) -> usize {
    let s = input.start();
    let e = input.end();
    let mut paths: Vec<Path> = vec![];
    paths.push(Path{last: s, previous: Default::default(), distance: 1});
    let mut done : Vec<Path> = Default::default();
    while let Some(last) = paths.pop() {
        for opt in input.options_from_tiles(last.last, true) {
            if last.previous.contains(&opt) {
                continue;
            }
            let mut dup = last.clone();
            dup.previous.insert(dup.last);
            dup.last = opt;
            dup.distance += 1;
            if opt == e {
                done.push(dup)
            } else {
                paths.push(dup);
            }
        }
    }
    done.into_iter().map(|p| p.distance).max().unwrap() + 1
}

pub fn part2(input: &Grid) -> usize {
    let s = input.start();
    let e = input.end();
    let mut paths: Vec<Path> = vec![];
    paths.push(Path{last: s, previous: Default::default(), distance: 1});
    let mut max = usize::MIN;
    while let Some(last) = paths.pop() {
        for (opt, dist) in input.options_from_graph(last.last) {
            if last.previous.contains(opt) {
                continue;
            }
            let mut dup = last.clone();
            dup.previous.insert(dup.last);
            dup.last = *opt;
            dup.distance += dist;
            if opt == &e {
                max = max.max(dup.distance);
            } else {
                paths.push(dup);
            }
        }
    }
    max + 1
}
