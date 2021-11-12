#![allow(clippy::ptr_arg)]

use std::cmp::min;

const OPEN: char = '.';
const TREE: char = '|';
const LUMBER: char = '#';

fn main() {
    let input = std::fs::read_to_string("input").expect("could not read file");
    println!("Solution 1: {}", solve1(&input, 10));
    println!("Solution 2: {}", solve1(&input, 1000000000));
}

type Grid = Vec<Vec<char>>;

fn count(grid: &Grid, (i, j): (usize, usize), t: char) -> usize {
    count_w(grid, (i, j), 3, t)
}

fn count_all(grid: &Grid, t: char) -> usize {
    count_w(grid, (0, 0), usize::MAX, t)
}
fn count_w(grid: &Grid, (i, j): (usize, usize), size: usize, t: char) -> usize {
    let w1 = size / 2;
    let w2 = size - w1;
    let range_i = i.saturating_sub(w1)..min(grid.len(), i + w2);
    grid.get(range_i)
        .into_iter()
        .flatten()
        .map(|row| row.get(j.saturating_sub(w1)..min(row.len(), j + w2)))
        .flatten()
        .flatten()
        .filter(|tile| **tile == t)
        .count()
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn solve1(input: &str, minutes: usize) -> usize {
    let mut grid: Grid = parse(input);

    let mut scores = vec![];
    let mut freqs = vec![];

    let mut missing = minutes;
    while missing > 0 {
        missing -= 1;
        let mut ng = grid.clone();
        for i in 0..ng.len() {
            for j in 0..grid[i].len() {
                match grid[i][j] {
                    OPEN => {
                        if count(&grid, (i, j), TREE) >= 3 {
                            ng[i][j] = TREE;
                        }
                    }
                    TREE => {
                        if count(&grid, (i, j), LUMBER) >= 3 {
                            ng[i][j] = LUMBER;
                        }
                    }
                    LUMBER => {
                        if count(&grid, (i, j), LUMBER) < 2 || count(&grid, (i, j), TREE) < 1 {
                            ng[i][j] = OPEN;
                        }
                    }
                    _ => {
                        panic!("unknown character");
                    }
                }
            }
        }
        grid = ng;
        let score = count_all(&grid, TREE) * count_all(&grid, LUMBER);
        let last = scores.iter().rev().position(|&x| x == score);
        scores.push(score);
        freqs.push(last);
        if let Some(freq) = last {
            if freqs.iter().rev().take(freq).all(|&x| x == Some(freq)) {
                missing %= freq + 1;
            }
        }
    }
    *scores.last().unwrap()
}

#[allow(dead_code)]
fn print(grid: &Grid) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

#[test]
fn test_example() {
    let input = "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
    assert_eq!(solve1(input, 10), 1147);
}

#[test]
fn test_count() {
    let example = "###";
    let grid = &parse(example);
    assert_eq!(count(grid, (0, 1), LUMBER), 3);
}

#[test]
fn test_count1() {
    let example = "111
232
343
";
    let grid = &parse(example);
    assert_eq!(count(grid, (0, 0), '1'), 2);
    assert_eq!(count(grid, (0, 0), '2'), 1);
    assert_eq!(count(grid, (0, 0), '3'), 1);
    assert_eq!(count(grid, (0, 1), '1'), 3);
    assert_eq!(count(grid, (0, 1), '2'), 2);
    assert_eq!(count(grid, (0, 1), '3'), 1);
    assert_eq!(count(grid, (1, 1), '3'), 3);
}
