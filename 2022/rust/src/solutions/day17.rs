use itertools::Itertools;
use once_cell::sync::OnceCell;
use std::collections::HashSet;

static ROCKS_STR: &str = r"
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

static W: usize = 7;
// static DEBUG: bool = true;
static DEBUG: bool = false;
type Input<'a> = Vec<Move>;

#[derive(Debug, Clone)]
pub enum Move {
    Left,
    Right,
    Down,
}

type Template = HashSet<(usize, usize)>;
#[derive(Debug)]
pub struct Rock<'a> {
    template: &'a Template,
    pos: (usize, usize),
}
impl<'a> Rock<'a> {
    fn new(template: &'a Template, height: usize) -> Self {
        Self {
            template,
            pos: (height, 2),
        }
    }
    fn occupies(&self, pos: (usize, usize)) -> bool {
        self.iter().any(|point| point == pos)
    }
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.bits(self.pos)
    }

    fn bits(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.template
            .iter()
            .map(move |(ix, jx)| (ix + pos.0, jx + pos.1))
    }
}

#[derive(Default)]
struct Map<'a> {
    fixed: Vec<[bool; 7]>,
    rock: Option<Rock<'a>>,
    max_height: usize,
}

impl<'a> Map<'a> {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn len(&self) -> usize {
        self.fixed.len()
    }
    fn insert(&mut self) {
        let rock = self.rock.take().unwrap();
        assert!(rock.iter().any(|(i, j)| i == 1 || self.fixed[i - 1][j]));
        dbg!("Inserting", &rock);
        for (i, j) in rock.iter() {
            while i >= self.fixed.len() {
                self.fixed.push(Default::default());
            }
            assert!(!self.fixed[i][j]);
            self.fixed[i][j] = true;
        }
        self.max_height = std::cmp::max(
            self.max_height,
            rock.iter()
                .map(|(i, _)| i)
                .max()
                .unwrap_or_else(|| usize::MAX),
        );
    }
    fn update(&mut self, dir: &Move) -> bool {
        let Some(ref rock) = self.rock else {
            return true;
        };
        let pos = match dir {
            Move::Right if rock.pos.1 < 6 => (rock.pos.0, rock.pos.1 + 1),
            Move::Left if rock.pos.1 > 0 => (rock.pos.0, rock.pos.1 - 1),
            Move::Down => {
                if rock.pos.0 > 1 && self.can_move(rock, (rock.pos.0 - 1, rock.pos.1)) {
                    (rock.pos.0 - 1, rock.pos.1)
                } else {
                    return true;
                }
            }
            _ => {
                return false;
            }
        };
        if self.can_move(rock, pos) {
            self.rock.as_mut().unwrap().pos = pos;
        }
        false
    }

    fn begin(&mut self, tpl: &'a Template) {
        let rock = Rock::new(tpl, dbg!(self.max_height) + 4);
        while self.len() < rock.pos.0 + rock.template.len() {
            self.fixed.push(Default::default());
        }
        self.rock = Some(rock);
    }

    fn can_move(&self, rock: &Rock, pos: (usize, usize)) -> bool {
        if pos.0 == 0 {
            return false;
        }
        for (i, j) in rock.bits(pos) {
            if j > 6 {
                return false;
            }
            if let Some(row) = self.fixed.get(i) {
                if let Some(val) = row.get(j) {
                    if *val {
                        return false;
                    }
                }
            };
        }
        true
    }

    fn draw(&self) {
        if !DEBUG {
            return;
        }
        println!();
        if let Some(rock) = &self.rock {
            println!("Rock @ {}, {}", rock.pos.0, rock.pos.1);
        }
        for (i, row) in self.fixed.iter().enumerate().rev() {
            for (j, cell) in row.iter().enumerate() {
                let c = match (&self.rock, *cell) {
                    (Some(rock), _) if rock.occupies((i, j)) => '@',
                    (_, true) => '#',
                    _ => '.',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn parse_rocks(input: &str) -> Vec<Template> {
    input
        .lines()
        .rev()
        .batching(|it| {
            match it
                .take_while(|l| !l.is_empty())
                .enumerate()
                .flat_map(|(ix, l)| {
                    l.trim()
                        .chars()
                        .enumerate()
                        .filter_map(move |(idx, c)| match c {
                            '#' => Some((ix, idx)),
                            '.' => None,
                            _ => panic!("unknown character {c}"),
                        })
                })
                .collect::<HashSet<_>>()
            {
                rs if rs.is_empty() => None,
                rs => Some(rs),
            }
        })
        .collect()
}

static ROCKS: OnceCell<Vec<Template>> = OnceCell::new();

pub fn parse(input: &str) -> Input {
    let mut rocks = parse_rocks(ROCKS_STR);
    debug_assert_eq!(rocks.len(), 5);
    rocks.reverse();
    ROCKS.set(rocks).unwrap();

    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Unknown wind direction {c}"),
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let rocks = ROCKS.get().unwrap();
    let mut map = Map::new();
    let n_rocks = 2022;
    // let n_rocks = 11;
    let mut wind = input.iter().cycle().intersperse(&Move::Down);
    // let mut wind = input.iter().cycle();
    for tpl in rocks.iter().cycle().take(n_rocks) {
        map.begin(tpl);
        // println!("NEW ROCK");
        let mut last = wind.next().unwrap();
        map.draw();
        while !map.update(dbg!(last)) {
            last = wind.next().unwrap();
        }
        // assert!(matches!(last, Move::Down));
        // loop {
        //     map.update(dbg!(wind.next().unwrap()));
        //     if map.update(&Move::Down) {
        //         break;
        //         // dir = wind.next().unwrap();
        //     }
        //     map.draw();
        // }
        map.insert();
    }
    map.draw();

    map.max_height
}

pub fn part2(input: &Input) -> usize {
    0
}
