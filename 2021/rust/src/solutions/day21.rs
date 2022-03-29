use std::collections::HashMap;
const MAX_POS: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    pos: [usize; 2],
    totals: [usize; 2],
    rolls: usize,
    turn: usize,
}

impl Board {
    fn new(pos: &[usize]) -> Board {
        Board {
            pos: [pos[0] - 1, pos[1] - 1],
            totals: [0; 2],
            rolls: 0,
            turn: 0,
        }
    }

    fn turn(&mut self, rolled: &[usize], max: usize) -> bool {
        self.rolls += rolled.len();
        let total = rolled.iter().sum::<usize>();

        self.pos[self.turn] = (self.pos[self.turn] + total) % MAX_POS;
        self.totals[self.turn] += self.pos[self.turn] + 1;

        let won = self.totals[self.turn] >= max;
        self.turn = (self.turn + 1) % self.pos.len();
        won
    }
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let num = line.rsplit(':').next().unwrap().trim();
            num.parse::<usize>().unwrap()
        })
        .collect()
}

pub fn part1(input: &[usize]) -> usize {
    let mut board = Board::new(input);
    let mut rolled = vec![];
    for i in 0.. {
        rolled.push(1 + (i % 100));
        if rolled.len() < 3 {
            continue;
        }
        if board.turn(&rolled, 1000) {
            return board.rolls * board.totals.iter().min().unwrap();
        }
        rolled.clear();
    }
    unimplemented!();
}

pub fn part2(input: &[usize]) -> usize {
    let mut universes: HashMap<Board, usize> = Default::default();
    universes.insert(Board::new(input), 1);

    let mut temp: HashMap<Board, usize> = Default::default();
    let mut done: HashMap<Board, usize> = Default::default();

    let mut rolled = vec![vec![]; 1];
    for _rolls in 0..3 {
        let mut new_rolled = vec![];
        for roll in rolled.iter_mut() {
            for i in 1..=3 {
                let mut new_roll = roll.clone();
                new_roll.push(i);
                new_rolled.push(new_roll);
            }
        }
        rolled = new_rolled;
    }

    while !universes.is_empty() {
        for (univ, times) in universes.iter() {
            for roll in rolled.iter() {
                let mut new_univ = univ.clone();
                if new_univ.turn(roll, 21) {
                    *done.entry(new_univ).or_default() += times;
                } else {
                    *temp.entry(new_univ).or_default() += times;
                }
            }
        }

        temp = std::mem::replace(&mut universes, temp);
        temp.clear();
    }

    let (won, _times) = done.iter().fold((0, 0), |mut counter, (u, times)| {
        counter.1 += times;
        if u.totals[0] > u.totals[1] {
            counter.0 += times;
        }
        counter
    });
    won
}
