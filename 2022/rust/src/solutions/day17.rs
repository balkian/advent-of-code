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

const W: usize = 7;
static DEBUG: bool = false;
// static DEBUG: bool = true;

type Input<'a> = Vec<Move>;
type Key<'a> = (Vec<[bool; W]>, Option<&'a Template>, usize);
type Template = HashSet<(usize, usize)>;

#[derive(Debug, Clone)]
pub enum Move {
    Left,
    Right,
    Down,
}

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
            fixed: vec![[true; W]],
            ..Default::default()
        }
    }

    fn len(&self) -> usize {
        self.fixed.len()
            - self
                .fixed
                .iter()
                .rev()
                .take_while(|row| row.iter().all(|c| !*c))
                .count()
            - 1
    }

    fn to_key(&self, wind: usize) -> Key<'a> {
        let top: Vec<_> = self
            .fixed
            .iter()
            .rev()
            .skip_while(|a| a.iter().all(|cell| !cell))
            .take_while(|a| a.iter().any(|cell| !cell))
            .copied()
            .collect();
        (top, self.rock.as_ref().map(|r| r.template), wind)
    }

    fn insert(&mut self) {
        let rock = self.rock.take().unwrap();
        assert!(rock.iter().any(|(i, j)| i == 1 || self.fixed[i - 1][j]));
        // dbg!("Inserting", &rock);
        for (i, j) in rock.iter() {
            while i >= self.fixed.len() {
                self.fixed.push(Default::default());
            }
            assert!(!self.fixed[i][j]);
            self.fixed[i][j] = true;
        }
        self.max_height = std::cmp::max(
            self.max_height,
            rock.iter().map(|(i, _)| i).max().unwrap_or(usize::MAX),
        );
        self.simplify();
    }

    fn simplify(&mut self) {
        // println!("SIMPLIFYING");
        self.draw();
        let mut blocked = [false; W];
        for row in self.fixed.iter_mut().rev() {
            let mut accessible: [bool; W] = blocked
                .iter()
                .map(|b| !b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            for ix in 1..W {
                accessible[ix] |= !row[ix - 1] & accessible[ix - 1];
            }
            for ix in (0..W - 1).rev() {
                accessible[ix] |= !row[ix + 1] & accessible[ix + 1];
            }
            for ix in 0..W {
                row[ix] = row[ix] || !accessible[ix];
            }
            blocked = *row;
        }
        self.draw();
    }

    fn update(&mut self, dir: &Move) -> bool {
        let Some(ref rock) = self.rock else {
            return true;
        };
        let pos = match dir {
            Move::Right if rock.pos.1 < 6 => (rock.pos.0, rock.pos.1 + 1),
            Move::Left if rock.pos.1 > 0 => (rock.pos.0, rock.pos.1 - 1),
            Move::Down => {
                if self.can_move(rock, (rock.pos.0 - 1, rock.pos.1)) {
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
        let rock = Rock::new(tpl, self.max_height + 4);
        while self.fixed.len() < rock.pos.0 + rock.template.len() {
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
            if row.iter().all(|c| *c) {
                return;
            }
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

pub fn solve(input: &Input, n_rocks: usize) -> usize {
    let rocks = ROCKS.get().unwrap();
    let mut map = Map::new();
    let mut cache: Vec<(Key, usize)> = vec![];
    let mut wind =
        Itertools::intersperse(input.iter().enumerate().cycle(), (input.len(), &Move::Down));

    for (i, (_tpl_nr, tpl)) in rocks.iter().enumerate().cycle().enumerate().take(n_rocks) {
        let (wind_nr, mut last_wind) = wind.next().unwrap();
        // wind_hist.push(last.clone());
        map.begin(tpl);
        let key = map.to_key(wind_nr);

        if DEBUG {
            println!("ROCK #{i}");
        }
        map.draw();

        if let Some((idx, (_, last_subtotal))) =
            cache.iter().enumerate().find(|(_idx, (k, _v))| *k == key)
        {
            let cycle_len = cache.len() - idx;
            let diff = map.len() - last_subtotal;
            let remaining = n_rocks - i;

            let mut total = map.len() + (remaining / cycle_len) * diff;
            total += cache[idx + (remaining % cycle_len)].1 - last_subtotal;

            // println!("Match with Rock #{idx}");
            return total;
        }
        cache.push((key, map.len()));

        loop {
            if map.update(last_wind) {
                break;
            }
            last_wind = wind.next().unwrap().1;
        }
        map.insert();
    }
    map.draw();
    println!("Cache size: {}", cache.len());

    map.max_height
}

pub fn part1(input: &Input) -> usize {
    solve(input, 2022)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 1000000000000)
}
