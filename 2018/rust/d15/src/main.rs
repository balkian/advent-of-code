// use std::ops::Add;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::{fmt, fs};

fn main() {
    let input = fs::read_to_string("input").expect("could not read file");
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Race {
    Goblin,
    Elf,
}

/// In this problem, we'll store (y, x) for a change
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Position(usize, usize);

impl Position {
    fn distance(&self, other: Self) -> usize {
        (((self.0 as isize) - (other.0 as isize)).abs()
            + ((self.1 as isize) - (other.1 as isize)).abs()) as usize
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Character {
    race: Race,
    pos: Position,
    hp: usize,
}

impl Character {
    fn new(race: Race, pos: Position) -> Self {
        Self { race, pos, hp: 200 }
    }

    fn attack(&mut self, delta: usize) {
        self.hp = self.hp.saturating_sub(delta);
    }
}

#[derive(Clone)]
struct Game {
    grid: Vec<Vec<char>>,
    characters: Vec<Character>,
    elf_hp: usize,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (ix, row) in self.grid.iter().enumerate() {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "\t\t")?;
            for c in self.characters.iter() {
                if c.pos.0 != ix {
                    continue;
                }
                let s = match c.race {
                    Race::Goblin => "G",
                    Race::Elf => "E",
                };
                write!(f, "{}({})  ", s, c.hp)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Game {
    fn from_str(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let characters = grid
            .iter()
            .enumerate()
            .flat_map(move |(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, cell)| match cell {
                        'G' => Some(Character::new(Race::Goblin, Position(i, j))),
                        'E' => Some(Character::new(Race::Elf, Position(i, j))),
                        _ => None,
                    })
            })
            .collect();

        Game {
            grid,
            characters,
            elf_hp: 3,
        }
    }
    fn step(&mut self) -> bool {
        self.characters.retain(|c| c.hp > 0);
        self.characters.sort_by_key(|x| x.pos);

        let mut finished = false;

        for ix in 0..self.characters.len() {
            if self.characters[ix].hp == 0 {
                continue;
            }
            if self.attack(ix) {
                continue;
            } else if self.get_closer(ix) {
                self.attack(ix);
                continue;
            }
            finished = true;
        }
        finished
    }
    fn attack(&mut self, ix: usize) -> bool {
        let this = &self.characters[ix];

        let mut toattack = vec![];

        for (ix, other) in self.characters.iter().enumerate() {
            if other == this || this.race == other.race || other.hp == 0 {
                continue;
            }
            if this.pos.distance(other.pos) == 1 {
                toattack.push(ix);
            }
        }

        if toattack.is_empty() {
            return false;
        }
        toattack.sort_by_key(|c| {
            let c = &self.characters[*c];
            (c.hp, c.pos)
        });
        let target = toattack[0];
        let c2 = self.characters.get_mut(target).unwrap();
        let hp = match c2.race {
            Race::Elf => 3,
            Race::Goblin => self.elf_hp,
        };
        c2.attack(hp);
        if c2.hp == 0 {
            self.grid[c2.pos.0][c2.pos.1] = '.';
        }
        true
    }

    fn get_closer(&mut self, ix: usize) -> bool {
        let this = &self.characters[ix];
        let mut targets = HashSet::new();
        let mut any_alive = false;
        let c1 = this;
        for ox in 0..self.characters.len() {
            let c2 = &self.characters[ox];
            if c2.hp == 0 || c2.race == c1.race {
                continue;
            }
            any_alive = true;

            let y = self.characters[ox].pos.0 as isize;
            let x = self.characters[ox].pos.1 as isize;

            for (i, j) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].into_iter() {
                if i < 0
                    || i as usize > self.grid.len()
                    || j < 0
                    || j as usize > self.grid[i as usize].len()
                {
                    continue;
                }
                let i = i as usize;
                let j = j as usize;
                let target = Position(i, j);
                if target == c1.pos {
                    continue;
                }
                if self.grid[i][j] == '.' {
                    targets.insert(target);
                }
            }
        }
        if let Some(target) = self.find_dijkstra(c1.pos, &targets) {
            assert_eq!(target.distance(c1.pos), 1);
            assert_ne!(target, c1.pos);

            let symbol = self.grid[c1.pos.0][c1.pos.1];
            assert_ne!(symbol, '.');
            assert_eq!(self.grid[target.0][target.1], '.');

            self.grid[c1.pos.0][c1.pos.1] = '.';
            self.grid[target.0][target.1] = symbol;
            self.characters[ix].pos = target;
        }
        any_alive
    }

    fn find_dijkstra(&self, pos: Position, positions: &HashSet<Position>) -> Option<Position> {
        let mut missing: HashSet<Position> = positions.clone();
        let mut candidates = BinaryHeap::from(vec![Reverse((0, pos))]);
        let mut distances: HashMap<Position, (usize, Position)> = HashMap::new();
        distances.insert(pos, (0, pos));
        while let Some(Reverse((cost, pos))) = candidates.pop() {
            let y = pos.0;
            let x = pos.1;
            let cost = cost + 1;
            for (i, j) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].into_iter() {
                if self.grid[i][j] == '.' {
                    let next = Position(i, j);
                    let new_value = (cost, pos);
                    if !distances.contains_key(&next) || distances.get(&next).unwrap() > &new_value
                    {
                        distances.insert(next, new_value);
                        candidates.push(Reverse((cost, next)));
                        missing.remove(&next);
                    }
                }
            }
        }

        distances
            .iter()
            .filter(|(x, _)| positions.contains(x))
            .min_by_key(|(pos, (dist, _))| (*dist, *pos))
            .map(|(target, _)| {
                let mut target = *target;
                while pos.distance(target) > 1 {
                    target = distances.get(&target).unwrap().1;
                }
                target
            })
    }
    #[allow(dead_code)]
    fn print_distances(&self, distances: &HashMap<Position, (usize, Position)>) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                print!(
                    "{} ",
                    distances
                        .get(&Position(i, j))
                        .map(|(dist, _)| format!("{:03}", *dist))
                        .or_else(|| Some(" . ".into()))
                        .unwrap()
                );
            }
            println!();
        }
        println!();
    }
}

fn solve2(input: &str) -> usize {
    let orig_game = Game::from_str(input);
    let count_elves = |x: &Game| x.characters.iter().filter(|c| c.race == Race::Elf).count();
    let mut game: Game;

    let num_elves = count_elves(&orig_game);
    'outer: for hp in 3.. {
        game = orig_game.clone();
        game.elf_hp = hp;
        for i in 1.. {
            let finished = game.step();
            if count_elves(&game) != num_elves {
                continue 'outer;
            }
            if finished {
                return (i - 2) * game.characters.iter().map(|c| c.hp).sum::<usize>();
            }
        }
    }
    unreachable!();
}

fn solve1(input: &str) -> usize {
    let mut game = Game::from_str(input);
    for i in 0.. {
        let finished = game.step();
        println!("Round {}", i);
        println!("{}", &game);
        if finished {
            println!("Answer: {}", i);
            return i * game.characters.iter().map(|c| c.hp).sum::<usize>();
        }
    }
    unreachable!();
}

#[test]
fn text_example2() {
    assert_eq!(
        solve1(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
        ),
        27730
    );
}

#[test]
fn text_example3() {
    assert_eq!(
        solve1(
            "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
        ),
        36334
    );
}

#[test]
fn text_example4() {
    assert_eq!(
        solve1(
            "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
        ),
        39514
    );
}

#[test]
fn text_example5() {
    assert_eq!(
        solve1(
            "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
        ),
        27755
    );
}

#[test]
fn text_example6() {
    assert_eq!(
        solve1(
            "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
        ),
        28944
    );
}

#[test]
fn text_example7() {
    assert_eq!(
        solve1(
            "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
"
        ),
        18740
    );
}
