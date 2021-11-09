// use std::ops::Add;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fmt;

use std::{thread, time};

fn main() {
    println!("Hello, world!");
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

// impl Add for &Position {
//     type Output = Position;

//     fn add(self, other: Self) -> Self::Output {
//         Position(
//             ((self.0 as isize) + other.0) as usize,
//             ((self.1 as isize) + other.1) as usize,
//         )
//     }
// }

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

    fn attack(&mut self) {
        self.hp = self.hp.saturating_sub(3);
    }
}

struct Game {
    grid: Vec<Vec<char>>,
    characters: Vec<Character>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        for c in self.characters.iter() {
            let s = match c.race {
                Race::Goblin => "G",
                Race::Elf => "E",
            };
            writeln!(f, "{}({})", s, c.hp);
        }
        Ok(())
    }
}

impl Game {
    fn print_distances(&self, distances: &HashMap<Position, (usize, Position)>) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                print!(
                    "{} ",
                    distances
                        .get(&Position(i, j))
                        .map(|(dist, _)| format!("{:03}", *dist))
                        .or(Some(" . ".into()))
                        .unwrap()
                );
            }
            println!();
        }
        println!();
    }
    fn find_dijkstra<'a, 'b>(
        &self,
        pos: Position,
        positions: &'b HashSet<Position>,
    ) -> Option<Position> {
        let mut missing: HashSet<Position> = positions.clone();
        let mut candidates = BinaryHeap::from(vec![Reverse((0, pos))]);
        let mut distances: HashMap<Position, (usize, Position)> = HashMap::new();
        distances.insert(pos, (0, pos));
        // dbg!{&missing};
        while let Some(Reverse((cost, pos))) = candidates.pop() {
            // dbg!{&candidates,&distances};

            if missing.is_empty() {
                break;
            }
            let y = pos.0;
            let x = pos.1;
            let cost = cost + 1;
            for (i, j) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].into_iter() {
                // println!("checking one {},{} - {}", i, j, self.grid[pos.0][pos.1]);
                // println!("{}", &self);
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
                // println!("{:?}", distances.get(&target));
                // print!(
                //     "Distance {:?} to {:?}: {}. ",
                //     pos,
                //     target,
                //     pos.distance(target)
                // );
                while pos.distance(target) > 1 {
                    target = distances.get(&target).unwrap().1;
                }
                // println!("Next: {:?}", target);
                // self.print_distances(&distances);
                target
            })
    }

    fn attack(&mut self, ix: usize) -> bool {
        let this = &self.characters[ix];

        let mut toattack = vec![];

        for (ix, other) in self.characters.iter().enumerate() {
            if other == this || this.race == other.race {
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
        c2.attack();
        if c2.hp == 0 {
            self.grid[c2.pos.0][c2.pos.1] = '.';
            println!("Agent died");
            println!("{}", self);
        }
        return true;
    }

    fn get_closer(&mut self, ix: usize) -> bool {
        let this = &self.characters[ix];
        let mut targets = HashSet::new();
        let c1 = this;
        for ox in 0..self.characters.len() {
            if ix == ox {
                continue;
            }

            let c2 = &self.characters[ox];
            if c2.hp == 0 || c2.race == c1.race {
                continue;
            }

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
            // println!("Moving {:?} to {:?}", c1.pos, target);

            let symbol = self.grid[c1.pos.0][c1.pos.1];
            assert_ne!(symbol, '.');
            assert_eq!(self.grid[target.0][target.1], '.');

            self.grid[c1.pos.0][c1.pos.1] = '.';
            self.grid[target.0][target.1] = symbol;
            self.characters[ix].pos = target;
            return true;
        }
        return false;
    }

    fn step(&mut self) -> bool {
        let mut played = false;

        self.characters.sort_by_key(|x| x.pos);
        self.characters.retain(|c| c.hp > 0);

        for ix in 0..self.characters.len() {
            thread::sleep(time::Duration::from_millis(200));
            if self.attack(ix) {
                played = true
            } else if self.get_closer(ix) {
                played = true;
                self.attack(ix);
            }
        }
        !played
    }

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

        Game { grid, characters }
    }
}

fn solve1(input: &str) -> usize {
    let mut game = Game::from_str(input);
    let mut rounds: usize = 0;
    for i in 1.. {
        println!("{}", &game);
        if game.step() {
            println!("{}", &game);
            rounds = i;
            break;
        }
    }
    rounds * game.characters.iter().map(|c| c.hp).sum::<usize>()
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
