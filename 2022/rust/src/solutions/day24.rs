use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Pos = (usize, usize);

type Dir = (isize, isize);

#[derive(Debug, Clone)]
pub struct Blizzard(Pos, Dir);

impl Blizzard {
    fn update(&mut self, bounds: Pos) {
        self.0 .0 =
            1 + ((self.0 .0 - 1) as isize + self.1 .0).rem_euclid((bounds.0 - 1) as isize) as usize;
        self.0 .1 = (self.0 .1 as isize + self.1 .1).rem_euclid(bounds.1 as isize) as usize;
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    blizzards: Vec<Blizzard>,
    bounds: Pos,
    occupation: HashSet<(usize, Pos)>,
}

impl Game {
    fn fill_occupation(&mut self, time: usize) {
        for bl in self.blizzards.iter_mut() {
            self.occupation.insert((0, bl.0));
        }
        for i in 1..time {
            for bl in self.blizzards.iter_mut() {
                bl.update(self.bounds);
                self.occupation.insert((i, bl.0));
            }
        }
    }

    #[allow(dead_code)]
    fn draw(&self, time: usize) {
        for i in 0..=self.bounds.0 {
            for j in 0..=self.bounds.1 {
                let c = if self.occupation.contains(&(time, (i, j))) {
                    'X'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn inbounds(&self, pos: Pos) -> bool {
        pos.1 < self.bounds.1
            && ((pos.0 > 0) || pos.1 == 0)
            && ((pos.0 < self.bounds.0) || pos.1 == self.bounds.1 - 1)
    }
}

pub fn parse(input: &str) -> Game {
    let mut it = input.lines().filter(|l| !l.is_empty());
    let width = it.next().unwrap().trim().len() - 2;
    let mut blizzards: Vec<Blizzard> = vec![];
    let mut height = 0;
    for (i, line) in it.by_ref().enumerate() {
        height = i + 1;
        blizzards.extend(line.chars().filter(|c| *c != '#').enumerate().filter_map(
            move |(j, c)| {
                let dir = match c {
                    '>' => (0, 1),
                    '<' => (0, -1),
                    'v' => (1, 0),
                    '^' => (-1, 0),
                    '.' => return None,
                    _ => panic!("unexpected char: {c}"),
                };
                Some(Blizzard((height, j), dir))
            },
        ));
    }
    let mut game = Game {
        blizzards,
        bounds: (height, width),
        occupation: Default::default(),
    };
    game.fill_occupation(1500);
    game
}

pub fn part1(game: &Game) -> usize {
    let mut game = game.clone();
    let target = (game.bounds.0, game.bounds.1 - 1);
    solve(&mut game, &[target])
}
pub fn part2(game: &Game) -> usize {
    let mut game = game.clone();
    let target = (game.bounds.0, game.bounds.1 - 1);
    solve(&mut game, &[target, (0, 0), target])
}

pub fn solve(game: &mut Game, targets: &[Pos]) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(Reverse((0, (0, 0), 0)));

    let mut opts = Vec::with_capacity(5);

    while let Some(Reverse(key @ (time, pos, mut target))) = heap.pop() {
        if visited.contains(&key) {
            continue;
        }

        // dbg!(time);
        if pos == targets[target] {
            // println!("Reached {target:?}");
            if target == targets.len() - 1 {
                return time;
            }
            target += 1;
        }
        visited.insert((time, pos, target));
        let time = time + 1;
        opts.push(pos);
        opts.push((pos.0 + 1, pos.1));
        if pos.0 > 0 {
            opts.push((pos.0 - 1, pos.1));
        };
        if pos.1 > 0 {
            opts.push((pos.0, pos.1 - 1));
        };
        if pos.1 < targets[target].1 {
            opts.push((pos.0, pos.1 + 1))
        }

        for pos in opts.drain(..) {
            let key = (time, pos);
            if !game.occupation.contains(&key) && game.inbounds(pos) {
                heap.push(Reverse((time, pos, target)));
            }
        }
    }
    panic!("no solution found");
}
