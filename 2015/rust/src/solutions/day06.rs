use sscanf::{scanf};
use crate::aoc_test;

pub type Coord = (usize, usize);

type Input = Vec<(String, Coord, Coord)>;
type RefInput<'a> = &'a [(String, Coord, Coord)];

pub fn parse(input: &str) -> Input {

    input.lines().filter(|l| !l.is_empty()).map(|line| {
        let (stt, min_x, min_y, max_x, max_y) = scanf!( line,
                                                        "{} {},{} through {},{}",
                                                        String, usize, usize, usize, usize ).unwrap();
        (stt, (min_x, min_y), (max_x, max_y))
    }).collect()
}

pub fn part1(input: RefInput) -> usize {
    let mut matrix = [[false ;1000];1000];
    for cmd in input {
        let lights: Vec<Coord> = (cmd.1.0..=cmd.2.0).flat_map(|x| (cmd.1.1..=cmd.2.1).map(move |y| (x, y))).collect();
        match cmd.0.as_str() {
            "toggle" => {
                for (x,y) in lights {
                    matrix[x][y] ^= true;
                }
            },
            "turn on" => {
                for (x,y) in lights {
                    matrix[x][y] = true;
                }
            },
            "turn off" => {
                for (x,y) in lights {
                    matrix[x][y] = false;
                }
            },
            _ => {
                panic!("unknown command");
            }
        }
    }
    matrix.iter().map(|row| row.iter().filter(|x| **x).count()).sum::<usize>()
}

pub fn part2(input: RefInput) -> usize {
    let mut matrix = [[0usize ;1000];1000];
    for cmd in input {
        let lights: Vec<Coord> = (cmd.1.0..=cmd.2.0).flat_map(|x| (cmd.1.1..=cmd.2.1).map(move |y| (x, y))).collect();
        match cmd.0.as_str() {
            "toggle" => {
                for (x,y) in lights {
                    matrix[x][y] += 2;
                }
            },
            "turn on" => {
                for (x,y) in lights {
                    matrix[x][y] += 1;
                }
            },
            "turn off" => {
                for (x,y) in lights {
                    matrix[x][y] = matrix[x][y].saturating_sub(1);
                }
            },
            _ => {
                panic!("unknown command");
            }
        }
    }
    matrix.iter().flat_map(|row| row.iter()).sum::<usize>()
}

aoc_test![
    part1, simple, &parse("turn on 0,0 through 999,999"), 1000000;
    ];
