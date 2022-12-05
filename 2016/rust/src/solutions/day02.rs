use std::cmp::{max, min};
pub type Dir = (isize, isize);

pub fn parse(input: &str) -> Vec<Vec<Dir>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => (-1, 0),
                    'D' => (1, 0),
                    'L' => (0, -1),
                    'R' => (0, 1),
                    _ => panic!("unknown symbol"),
                })
                .collect()
        })
        .collect()
}

pub fn solve<const N: usize>(input: &[Vec<Dir>], keypad: &[[char; N]]) -> String {
    let mut pos = (1isize, 1isize);
    let mut code = String::new();
    let max_index = keypad.len() as isize - 1;
    for line in input {
        for dir in line {
            let pos_0 = min(max(pos.0 + dir.0, 0), max_index);
            let pos_1 = min(max(pos.1 + dir.1, 0), max_index);
            if keypad[pos_0 as usize][pos_1 as usize] != '-' {
                (pos.0, pos.1) = (pos_0, pos_1);
            }
        }
        code.push(keypad[pos.0 as usize][pos.1 as usize]);
    }
    code
}
pub fn part1(input: &[Vec<Dir>]) -> String {
    static NUM_KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    solve(input, &NUM_KEYPAD)
}

pub fn part2(input: &[Vec<Dir>]) -> String {
    static ALPHA_KEYPAD: [[char; 5]; 5] = [
        ['-', '-', '1', '-', '-'],
        ['-', '2', '3', '4', '-'],
        ['5', '6', '7', '8', '9'],
        ['-', 'A', 'B', 'C', '-'],
        ['-', '-', 'D', '-', '-'],
    ];

    solve(input, &ALPHA_KEYPAD)
}
