pub enum Rotation {
    Left(usize),
    Right(usize),
}

pub fn parse(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| match line.as_bytes()[0] {
            b'R' => Rotation::Right(line[1..].trim().parse().unwrap()),
            b'L' => Rotation::Left(line[1..].trim().parse().unwrap()),
            line => {
                panic!("wrong input {line}");
            }
        })
        .collect()
}

const MAX_DIAL: usize = 100;

pub fn part1(rotations: &[Rotation]) -> usize {
    let mut dial = 50;
    let mut zeros = 0;

    for rot in rotations {
        match rot {
            Rotation::Left(turns) => {
                let turns = turns % MAX_DIAL;
                dial = (dial + MAX_DIAL - turns) % MAX_DIAL;
            }
            Rotation::Right(turns) => {
                let turns = turns % MAX_DIAL;
                dial = (dial + turns) % MAX_DIAL;
            }
        }
        if dial == 0 {
            zeros += 1;
        }
    }

    zeros
}

pub fn part2(rotations: &[Rotation]) -> usize {
    let mut dial = 50;
    let mut zeros = 0;

    for rot in rotations {
        match rot {
            Rotation::Left(turns) => {
                zeros += turns.div_euclid(MAX_DIAL);
                let turns = turns % MAX_DIAL;
                if turns > dial {
                    if dial > 0 {
                        zeros += 1;
                    }
                    dial += MAX_DIAL - turns;
                } else {
                    dial -= turns;
                    if dial == 0 {
                        zeros += 1;
                    }
                }
            }
            Rotation::Right(turns) => {
                zeros += turns.div_euclid(MAX_DIAL);
                let turns = turns % MAX_DIAL;
                dial += turns;
            }
        }
        if dial >= MAX_DIAL {
            zeros += 1;
            dial -= MAX_DIAL;
        }
    }
    zeros
}
