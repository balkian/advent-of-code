use regex::Regex;
use std::cmp::{max, min};
use std::fs;

/// This is the max number of lines / columns to print
const PRINTABLE_DIM: isize = 100;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Position(isize, isize);

#[derive(Debug, Clone)]
struct Point {
    pos: Position,
    vel: Position,
}

fn parse(input: &str) -> Vec<Point> {
    let re =
        Regex::new(r"position=<\s*([-]?\d+),\s*([-]?\d+)> velocity=<\s*([-]?\d+),\s*([-]?\d+)>")
            .expect("illegal regex");
    let mut points = vec![];
    for line in input.lines() {
        let cap = re.captures_iter(line).next().unwrap();
        points.push(Point {
            pos: Position(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            vel: Position(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
        });
    }
    points
}

fn evolve(old: &[Point]) -> Vec<Point> {
    let mut points = old.to_vec();
    for point in points.iter_mut() {
        point.pos.0 += point.vel.0;
        point.pos.1 += point.vel.1;
    }
    points
}

fn margins(points: &[Point]) -> (Position, Position) {
    let minp = points.iter().fold((isize::MAX, isize::MAX), |acc, point| {
        (min(acc.0, point.pos.0), min(acc.1, point.pos.1))
    });
    let maxp = points.iter().fold((isize::MIN, isize::MIN), |acc, point| {
        (max(acc.0, point.pos.0), max(acc.1, point.pos.1))
    });
    (Position(minp.0, minp.1), Position(maxp.0, maxp.1))
}

fn dimensions(points: &[Point]) -> Position {
    let (mx, my) = margins(points);
    Position(mx.1 - mx.0, my.1 - my.0)
}

fn print_matrix(points: &mut Vec<Point>) -> bool {
    let (minp, maxp) = margins(points);

    let dims = Position(maxp.0 - minp.0, maxp.1 - minp.1);
    if dims.0 > PRINTABLE_DIM || dims.1 > PRINTABLE_DIM {
        println!("Too big to print");
        return false;
    }

    points.sort_by_key(|p| (p.pos.1, p.pos.0));

    let mut next = 0;
    for j in minp.1..=maxp.1 {
        for i in minp.0..=maxp.0 {
            if next < points.len() && points[next].pos.0 == i && points[next].pos.1 == j {
                print!("#");
                while next < points.len() && points[next].pos.0 == i && points[next].pos.1 == j {
                    next += 1;
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    assert_eq!(next, points.len());
    println!();
    true
}

fn solve1(input: &str) {
    let mut points = parse(input);
    // let mut has_printed = false;
    let mut last_dims = dimensions(&points);
    for i in 0.. {
        let new_points = evolve(&points);
        let new_dims = dimensions(&new_points);
        if new_dims > last_dims {
            println!("Time: {}", i);
            print_matrix(&mut points);
            break;
        }
        points = new_points;
        last_dims = new_dims;
    }
}

fn main() {
    let example = &fs::read_to_string("example").unwrap();
    solve1(example);
    let input = &fs::read_to_string("input").unwrap();
    solve1(input);
}
