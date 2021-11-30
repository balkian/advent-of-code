use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<stamina>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.").unwrap();
}

#[derive(Debug, Clone)]
enum State {
    Rested(usize),
    Recharging(usize),
}

#[derive(Debug, Clone)]
pub struct Reindeer<'a> {
    name: &'a str,
    speed: usize,
    stamina: usize,
    rest: usize,
    state: State,
    advanced: usize,
    points: usize,
}

impl<'a> Reindeer<'a> {
    fn advance(&mut self) {
        match self.state {
            State::Rested(0) => {
                self.state = State::Recharging(self.rest - 1);
            }
            State::Rested(a) => {
                self.advanced += self.speed;
                self.state = State::Rested(a - 1);
            }
            State::Recharging(1) => {
                self.state = State::Rested(self.stamina);
            }
            State::Recharging(a) => {
                self.state = State::Recharging(a - 1);
            }
        }
    }
}

pub fn parse(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            let stamina = cap.name("stamina").unwrap().as_str().parse().unwrap();
            Reindeer {
                name: cap.name("name").unwrap().as_str(),
                speed: cap.name("speed").unwrap().as_str().parse().unwrap(),
                rest: cap.name("rest").unwrap().as_str().parse().unwrap(),
                state: State::Rested(stamina),
                advanced: 0,
                points: 0,
                stamina,
            }
        })
        .collect()
}

pub fn part1(input: &[Reindeer]) -> usize {
    let input = &mut input.to_owned();
    for _ in 0..2503 {
        for r in input.iter_mut() {
            r.advance();
        }
    }
    input.iter().map(|c| c.advanced).max().unwrap()
}

pub fn part2(input: &[Reindeer]) -> usize {
    let input = &mut input.to_owned();
    for _ in 0..2503 {
        for r in input.iter_mut() {
            r.advance();
        }
        let max = input.iter().map(|c| c.advanced).max().unwrap();
        input
            .iter_mut()
            .filter(|c| c.advanced == max)
            .for_each(|r| r.points += 1);
    }
    input.iter().map(|c| c.points).max().unwrap()
}
