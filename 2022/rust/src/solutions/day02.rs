use itertools::Itertools;

pub fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|w| match w {
                    "X" | "A" => 1,
                    "Y" | "B" => 2,
                    "Z" | "C" => 3,
                    _ => panic!("unknown symbol"),
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn win(a: usize) -> usize {
    (a + 1) % 3 + 1
}

fn lose(a: usize) -> usize {
    (a + 2) % 3 + 1
}

pub fn part1(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .map(|(a, b)| match (a-1, b-1) {
            (a,b) if a == b => 3,
            (a, b) if b == win(a) => 6,
            _ => 0,
        } + b)
        .sum()
}

pub fn part2(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .map(|(a, b)| match b {
            1 => lose(*a),
            2 => 3 + a,
            3 => 6 + win(*a),
            _ => unreachable!("invalid outcome"),
        })
        .sum()
}
