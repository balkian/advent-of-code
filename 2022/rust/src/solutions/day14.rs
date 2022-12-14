use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Tile {
    Rock,
    Sand,
}

type Coord = (i64, i64);
#[derive(Debug)]
pub struct Input {
    tiles: HashMap<Coord, Tile>,
    max_y: i64,
    max_x: i64,
}

pub fn parse(input: &str) -> Input {
    let mut tiles = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for line in input.lines() {
        let mut last: Option<Coord> = None;
        for token in line.trim().split(" -> ") {
            let (a, b) = token.split_once(',').unwrap();
            let x: i64 = a.parse().unwrap();
            let y: i64 = b.parse().unwrap();
            if let Some((x_p, y_p)) = last {
                let (x, x_p) = if x <= x_p { (x, x_p) } else { (x_p, x) };
                let (y, y_p) = if y <= y_p { (y, y_p) } else { (y_p, y) };
                for i in x..=x_p {
                    if i > max_x {
                        max_x = i;
                    }
                    for j in y..=y_p {
                        tiles.insert((i, j), Tile::Rock);
                        if j > max_y {
                            max_y = j;
                        }
                    }
                }
            }
            last = Some((x, y));
        }
    }

    Input {
        tiles,
        max_x,
        max_y,
    }
}

pub fn part1(input: &Input) -> usize {
    let mut counter = 0;
    let mut tiles = input.tiles.clone();
    'outer: loop {
        let mut x = 500;
        for y in 0.. {
            if y > input.max_y {
                break 'outer;
            }
            if !tiles.contains_key(&(x, y + 1)) {
                continue;
            } else if x == 0 || x > input.max_x {
                break 'outer;
            } else if !tiles.contains_key(&(x - 1, y + 1)) {
                x -= 1;
            } else if !tiles.contains_key(&(x + 1, y + 1)) {
                x += 1;
            } else {
                tiles.insert((x, y), Tile::Sand);
                counter += 1;
                break;
            }
        }
    }
    counter
}

pub fn part2(input: &Input) -> usize {
    let mut counter = 0;
    let mut tiles = input.tiles.clone();
    let max_y = input.max_y + 1;
    'outer: loop {
        let mut x = 500;
        for y in 0..max_y {
            if !tiles.contains_key(&(x, y + 1)) {
                continue;
            } else if !tiles.contains_key(&(x - 1, y + 1)) {
                x -= 1;
            } else if !tiles.contains_key(&(x + 1, y + 1)) {
                x += 1;
            } else {
                if y == 0 {
                    break 'outer;
                }
                tiles.insert((x, y), Tile::Sand);
                counter += 1;
                continue 'outer;
            }
        }
        tiles.insert((x, max_y), Tile::Sand);
        counter += 1;
    }
    counter + 1
}
