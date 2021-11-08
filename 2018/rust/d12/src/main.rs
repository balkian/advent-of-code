use std::collections::VecDeque;
use std::fmt;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("could not read file");
    println!("Solution 1: {}", solve1(&input, 20));
    println!("Solution 2: {}", solve1(&input, 50000000000));
}

const MASK_SIZE: usize = 5;
const MASK_OFF: usize = MASK_SIZE / 2;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Pot {
    Empty = b'.',
    Plant = b'#',
}

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Pot::Empty => '.',
            Pot::Plant => '#',
        };
        write!(f, "{}", c)
    }
}

impl From<char> for Pot {
    fn from(c: char) -> Self {
        match c {
            '#' => Pot::Plant,
            '.' => Pot::Empty,
            _ => panic!("unknown state: {}", c),
        }
    }
}

#[derive(Clone)]
struct Pots {
    pots: VecDeque<Pot>,
    zero: isize,
}

impl Pots {
    /// Get a position in the vector. Return Empty for out of bounds
    fn get_pot(&self, ix: isize) -> Pot {
        let ix = ix + self.zero;
        if ix < 0 || (ix as usize) >= self.pots.len() {
            Pot::Empty
        } else {
            self.pots[ix as usize]
        }
    }

    /// Set the value of an index.
    /// If out of bounds and value is not Empty,
    /// extend the vector to allow it
    fn set_pot(&mut self, ix: isize, value: Pot) {
        let ix = ix + self.zero;
        if ix < 0 {
            if value == Pot::Empty {
                return;
            }
            for _ in ix..-1 {
                self.pots.push_front(Pot::Empty);
                self.zero += 1;
            }
            self.pots.push_front(value);
            self.zero += 1;
            return;
        }
        if (ix as usize) >= self.pots.len() {
            if value == Pot::Empty {
                return;
            }
            for _ in (self.pots.len() as isize)..=ix {
                self.pots.push_back(Pot::Empty);
            }
            self.pots[ix as usize] = value;
            return;
        }
        self.pots[ix as usize] = value;
    }

    fn len(&self) -> usize {
        self.pots.len()
    }

    fn score(&self) -> isize {
        let mut score: isize = 0;
        for (ix, pot) in self.pots.iter().enumerate() {
            if *pot == Pot::Plant {
                score += (ix as isize) - self.zero
            }
        }
        score
    }
    fn prune(&mut self) {
        if let Some(first) = self
            .pots
            .iter()
            .enumerate()
            .find(|(_, x)| **x == Pot::Plant)
            .map(|(ix, _)| ix)
        {
            if first > 0 {
                self.zero -= first as isize;
                self.pots = self.pots.split_off(first);
            }
        }
        let tochop = self
            .pots
            .iter()
            .rev()
            .take_while(|x| **x == Pot::Empty)
            .count();
        if tochop > 0 {
            self.pots.truncate(self.pots.len() - tochop);
        }
    }

    fn apply_rules(&self, rules: &[Rule]) -> Pots {
        let mut newpots = Pots {
            pots: self.pots.iter().map(|_| Pot::Empty).collect(),
            zero: self.zero,
        };
        for rule in rules {
            'eachmask: for pos in 0..self.len() + 2 * MASK_SIZE {
                let pos = pos as isize - self.zero - (MASK_SIZE as isize);
                for delta in 0..MASK_SIZE {
                    if self.get_pot(pos + (delta as isize) - (MASK_OFF as isize))
                        != rule.mask[delta]
                    {
                        continue 'eachmask;
                    }
                }
                newpots.set_pot(pos, rule.result);
            }
        }
        newpots.prune();
        newpots
    }
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "First #{:04} ", self.zero)?;
        for i in self.pots.iter() {
            let c = match i {
                Pot::Empty => '.',
                Pot::Plant => '#',
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

struct Rule {
    mask: Mask,
    result: Pot,
}

type Mask = [Pot; MASK_SIZE];

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.mask {
            write!(f, "{}", i)?;
        }
        write!(f, " ==> ")?;
        write!(f, "{}", self.result)
    }
}

fn parse(input: &str) -> (Pots, Vec<Rule>) {
    let mut lines = input.lines();
    let pts: VecDeque<Pot> = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .chars()
        .map(Pot::from)
        .collect();
    let pots = Pots { pots: pts, zero: 0 };
    lines.next();
    let mut rules = vec![];
    for line in lines {
        let tokens: Vec<_> = line.split(" => ").take(2).collect();
        let pattern: [Pot; MASK_SIZE] = tokens[0]
            .chars()
            .map(Pot::from)
            .collect::<Vec<Pot>>()
            .try_into()
            .expect("could not convert");
        let result = tokens[1].chars().next().unwrap().into();
        rules.push(Rule {
            mask: pattern,
            result,
        });
    }

    (pots, rules)
}

fn solve1(input: &str, gens: usize) -> isize {
    let (pots, rules) = parse(input);

    if cfg!(debug_assertions) {
        println!("{:03}: {}", 0, &pots);
    }
    let mut pots = pots;
    let mut newpots;
    for gen in 1..=gens {
        newpots = pots.apply_rules(&rules);
        if cfg!(debug_assertions) {
            println!("{:03}: {}", gen, &newpots);
        }
        if newpots.pots == pots.pots {
            let s2 = newpots.score();
            let s1 = pots.score();
            return s2 + (s2 - s1) * ((gens - gen) as isize);
        }
        pots = newpots;
    }

    pots.score()
}

#[test]
fn text_example() {
    assert_eq!(
        solve1(
            &fs::read_to_string("example").expect("example file could not be read"),
            20
        ),
        325
    );
}
#[test]
fn test_part1() {
    assert_eq!(
        solve1(
            &fs::read_to_string("input").expect("example file could not be read"),
            20
        ),
        3793
    );
}
