use std::cmp::{max, min};
use std::fs;

fn left(point: (usize, usize)) -> (usize, usize) {
    (point.0, point.1 - 1)
}
fn right(point: (usize, usize)) -> (usize, usize) {
    (point.0, point.1 + 1)
}
fn down(point: (usize, usize)) -> (usize, usize) {
    (point.0 + 1, point.1)
}
fn up(point: (usize, usize)) -> (usize, usize) {
    (point.0 - 1, point.1)
}

const CLAY: char = '#';
const WET: char = '|';
const STILL_WATER: char = '~';
const EMPTY: char = '.';

struct Map {
    grid: Vec<Vec<char>>,
    min_x: usize,
    min_y: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut points = vec![];
        for line in input.lines() {
            let sides: Vec<_> = line.split(", ").collect();
            let mut values: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
            for side in sides {
                let subtokens: Vec<_> = side.split('=').collect();
                let var = subtokens[0];
                let mut it = subtokens[1]
                    .split("..")
                    .map(|x| x.parse::<usize>().unwrap())
                    .cycle();
                let limits = it.next().unwrap()..=it.next().unwrap();
                match var {
                    "x" => values.1.extend(limits),
                    "y" => values.0.extend(limits),
                    _ => panic!("unknown character"),
                };
            }
            for &y in &values.0 {
                for &x in &values.1 {
                    points.push((y, x));
                }
            }
        }
        let (min_y, min_x, max_y, max_x) = points.iter().fold(
            (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
            |acc, point| {
                (
                    min(acc.0, point.0),
                    min(acc.1, point.1),
                    max(acc.2, point.0),
                    max(acc.3, point.1),
                )
            },
        );
        let min_x = min(500, min_x - 1);
        let min_y = min(0, min_y - 1);
        let max_x = max_x + 1;
        let max_y = max_y + 1;

        let mut grid = vec![vec!(EMPTY; (max_x - min_x) + 1); (max_y - min_y) + 1];
        for point in points {
            grid[point.0 - min_y][point.1 - min_x] = CLAY;
        }
        Map { grid, min_x, min_y }
    }
    fn flow(&mut self) {
        let points = &mut self.grid;
        let mut sources = vec![(1 - self.min_y, 500 - self.min_x)];
        while let Some(pos) = sources.pop() {
            if pos.0 == points.len() - 2 {
                continue;
            }

            if pos.0 >= points.len() || (points[pos.0][pos.1] != EMPTY) {
                continue;
            }

            let d = down(pos);

            match points[d.0][d.1] {
                EMPTY => {
                    points[pos.0][pos.1] = WET;
                    sources.push(d);
                    continue;
                }
                WET => {
                    points[pos.0][pos.1] = WET;
                    continue;
                }
                _ => {}
            }

            let mut has_drain = false;

            let mut line = vec![pos];
            for func in [left, right] {
                let mut newpos = pos;
                for _ in 0.. {
                    newpos = func(newpos);
                    if points[newpos.0][newpos.1] == CLAY {
                        break;
                    }
                    line.push(newpos);
                    let d = down(newpos);
                    if points[d.0][d.1] == EMPTY {
                        sources.push(d);
                        has_drain = true;
                        break;
                    }
                }
            }
            let mat = if has_drain {
                WET
            } else {
                let u = up(pos);
                points[u.0][u.1] = EMPTY;
                sources.push(u);
                STILL_WATER
            };
            for point in line {
                points[point.0][point.1] = mat;
            }
        }
    }
    fn print(&self) {
        let points = &self.grid;
        for row in points.iter() {
            for x in row.iter() {
                print!("{}", x);
            }
            println!();
        }
        println!();
    }
}

fn solve1(input: &str) -> (usize, usize) {
    let mut map = Map::parse(input);
    map.flow();
    map.print();
    (
        map.grid
            .iter()
            .flatten()
            .filter(|&v| matches!(*v, STILL_WATER | WET))
            .count(),
        map.grid
            .iter()
            .flatten()
            .filter(|&v| matches!(*v, STILL_WATER))
            .count(),
    )
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (sol1, sol2) = solve1(&input);
    println!("Solution 1: {}", sol1);
    println!("Solution 2: {}", sol2);
}

#[test]
fn test_example() {
    let input = fs::read_to_string("example").unwrap();
    assert_eq!(solve1(&input).0, 57);
}
