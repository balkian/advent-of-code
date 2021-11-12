use std::cmp::{max, min};
use std::collections::HashMap;
use std::{fmt, fs};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn down(&self) -> Point {
        Point(self.0 + 1, self.1)
    }
    fn left(&self) -> Point {
        Point(self.0, self.1 - 1)
    }
    fn right(&self) -> Point {
        Point(self.0, self.1 + 1)
    }
    fn up(&self) -> Point {
        Point(self.0 - 1, self.1)
    }
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
enum Material {
    Clay,
    Wet,
    StillWater,
}

use Material::*;

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Clay => '#',
            // Self::StillWater => '~',
            Wet => '|',
            StillWater => '~',
        };
        write!(f, "{}", ch)
    }
}

fn print(points: &HashMap<Point, Material>) {
    let (min_x, (max_y, max_x)) =
        points
            .keys()
            .fold((usize::MAX, (usize::MIN, usize::MIN)), |acc, point| {
                (
                    min(acc.0, point.1),
                    (max(acc.1 .0, point.0), max(acc.1 .1, point.1)),
                )
            });
    for y in 0..=max_y {
        for x in min_x..=max_x {
            if let Some(material) = points.get(&Point(y, x)) {
                print!("{}", material);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn flow(points: &mut HashMap<Point, Material>) {
    let ((min_y, _min_x), (max_y, _max_x)) = points.keys().fold(
        ((usize::MAX, usize::MAX), (usize::MIN, usize::MIN)),
        |acc, point| {
            (
                (min(acc.0 .1, point.0), min(acc.0 .0, point.1)),
                (max(acc.1 .0, point.0), max(acc.1 .1, point.1)),
            )
        },
    );
    let mut sources = vec![Point(min_y - 1, 500)];
    while let Some(pos) = sources.pop() {
        if pos.0 == max_y {
            println!("Reached end");
            continue;
        }
        if sources.contains(&pos) {
            continue;
        }

        let (left, right, down) = (pos.left(), pos.right(), pos.down());

        match points.get(&down) {
            None => {
                points.insert(down.clone(), Wet);
                sources.push(down);
                continue;
            }
            Some(Wet) => {
                points.insert(pos.clone(), Wet);
                continue;
            }
            _ => {}
        }

        let mut has_drain = false;

        let mut line = vec![pos.clone()];
        for func in [Point::left, Point::right] {
            let mut newpos = pos.clone();
            for delta in 0.. {
                newpos = func(&newpos);
                if points.get(&newpos) == Some(&Clay) {
                    break;
                }
                line.push(newpos.clone());
                if points.get(&newpos.down()).is_none() {
                    sources.push(newpos.clone());
                    has_drain = true;
                    break;
                }
            }
        }
        let mat = if has_drain {
            Wet
        } else {
            sources.push(pos.up());
            StillWater
        };
        for point in line {
            points.insert(point, mat);
        }
    }
}

fn parse(input: &str) -> HashMap<Point, Material> {
    let mut points = HashMap::new();
    for line in input.lines() {
        let sides: Vec<_> = line.split(", ").collect();
        let mut values: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
        for side in sides {
            let subtokens: Vec<_> = side.split("=").collect();
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
        for y in &values.0 {
            for x in &values.1 {
                points.insert(Point(*y, *x), Material::Clay);
            }
        }
    }
    points
}

fn solve1(input: &str) -> (usize, usize) {
    let mut points = parse(input);
    flow(&mut points);
    print(&points);
    (
        points
            .values()
            .filter(|v| matches!(v, StillWater | Wet))
            .count(),
        points.values().filter(|v| matches!(v, StillWater)).count(),
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
