use nalgebra::Point2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;

use std::fmt;

type Pos = Point2<usize>;

#[derive(Clone, PartialEq, Eq, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match &self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Right => '>',
            Dir::Left => '<',
        };
        write!(f, "{c}")
    }
}

impl Dir {
    fn clockwise(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn reverse(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, other: Dir) -> Pos {
        match other {
            Dir::Up => Point2::new(self.coords.x, self.coords.y - 1),
            Dir::Down => Point2::new(self.coords.x, self.coords.y + 1),
            Dir::Right => Point2::new(self.coords.x + 1, self.coords.y),
            Dir::Left => Point2::new(self.coords.x - 1, self.coords.y),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
enum Tile {
    Empty,
    Wall,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match &self {
            Tile::Empty => '.',
            Tile::Wall => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Elf {
    pos: Pos,
    dir: Dir,
}

#[derive(Clone)]
pub struct Grid {
    elf: Elf,
    end: Pos,
    tiles: Vec<Vec<Tile>>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if y == self.end.coords.y && x == self.end.coords.x {
                    write!(f, "E")?;
                } else if y == self.elf.pos.coords.y && x == self.elf.pos.coords.x {
                    write!(f, "{:?}", self.elf.dir)?;
                } else {
                    write!(f, "{c:?}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn isempty(&self, pos: Pos) -> bool {
        match self.tiles[pos.coords.y][pos.coords.x] {
            Tile::Wall => false,
            Tile::Empty => true,
        }
    }
}

pub fn parse(i: &str) -> Grid {
    let mut elf = None;
    let mut end = None;
    let mut tiles = vec![];
    let mut lines = i.trim().lines().enumerate();
    for (y, line) in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    elf = Some(Elf {
                        pos: Point2::new(x, y),
                        dir: Dir::Right,
                    });
                    row.push(Tile::Empty);
                }
                'E' => {
                    end = Some(Point2::new(x, y));
                    row.push(Tile::Empty);
                }
                '#' => {
                    row.push(Tile::Wall);
                }
                '.' => {
                    row.push(Tile::Empty);
                }
                '\n' | '\r' => {
                    break;
                }
                _ => {
                    panic!("unknown char {c}")
                }
            }
        }
        tiles.push(row);
    }
    let elf = elf.expect("robot not found");

    Grid {
        tiles,
        end: end.expect("end not found"),
        elf,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    elf: Elf,
    cost: usize,
    from: Elf,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(g: &Grid) -> Option<(usize, HashMap<Elf, Vec<Elf>>)> {
    let start = g.elf;
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<Elf, usize> = Default::default();
    let mut edges_to: HashMap<Elf, Vec<Elf>> = Default::default();
    heap.push(State {
        elf: start,
        cost: 0,
        from: start,
    });
    let checkandadd = |from: Elf,
                       pos: Pos,
                       dir: Dir,
                       cost: usize,
                       heap: &mut BinaryHeap<_>,
                       dist: &mut HashMap<_, _>|
     -> bool {
        let newelf = Elf { pos, dir };
        if g.isempty(pos) {
            if let Some(nc) = dist.get(&newelf) {
                if *nc <= cost {
                    return false;
                }
            }
            heap.push(State {
                elf: newelf,
                cost,
                from,
            });
            true
        } else {
            false
        }
    };
    let mut min = None;
    while let Some(State { cost, elf, from }) = heap.pop() {
        //dbg!(&elf);
        if elf.pos == g.end {
            if let Some(min) = min {
                if min < cost {
                    break;
                }
            } else {
                min = Some(cost);
            }
        }
        if let Some(oc) = dist.get_mut(&elf) {
            if *oc <= cost {
                if *oc == cost {
                    edges_to.entry(elf).or_default().push(from);
                }
                continue;
            }
            *oc = cost;
        } else {
            dist.insert(elf, cost);
            edges_to.entry(elf).or_default().push(from);
        }

        checkandadd(
            elf,
            elf.pos + elf.dir,
            elf.dir,
            cost + 1,
            &mut heap,
            &mut dist,
        );
        let left = elf.dir.clockwise();
        checkandadd(elf, elf.pos, left, cost + 1000, &mut heap, &mut dist);
        let right = left.reverse();
        checkandadd(elf, elf.pos, right, cost + 1000, &mut heap, &mut dist);
        let back = elf.dir.reverse();
        checkandadd(elf, elf.pos, back, cost + 2000, &mut heap, &mut dist);
    }
    min.map(|min| (min, edges_to))
}

pub fn part1(g: &Grid) -> usize {
    let min = dijkstra(g);
    min.expect("solution not found").0
}

pub fn part2(g: &Grid) -> usize {
    let min = dijkstra(g);
    let edges_to = min.expect("solution not found").1;
    let mut paths = vec![
        vec![Elf {
            pos: g.end,
            dir: Dir::Up,
        }],
        vec![Elf {
            pos: g.end,
            dir: Dir::Left,
        }],
        vec![Elf {
            pos: g.end,
            dir: Dir::Down,
        }],
        vec![Elf {
            pos: g.end,
            dir: Dir::Right,
        }],
    ];

    let mut seen: HashSet<Pos> = Default::default();
    while let Some(mut path) = paths.pop() {
        let last = path[path.len() - 1];
        seen.insert(last.pos);
        if last.pos == g.elf.pos {
            continue;
        }
        let Some(edges) = edges_to.get(&last) else {
            continue;
        };
        let mut opts = edges.iter();
        let first = opts.by_ref().next().unwrap();
        for other in opts.by_ref() {
            let mut c = path.clone();
            c.push(*other);
            paths.push(c);
        }
        path.push(*first);
        paths.push(path);
    }
    seen.len()
}
