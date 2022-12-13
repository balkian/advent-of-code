use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Rule {
    Input(usize, usize),
    Give(usize, Entity, Entity),
}

#[derive(Debug, Clone)]
pub enum Entity {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Clone)]
pub struct Input {
    chips: HashMap<usize, Vec<usize>>,
    rules: HashMap<usize, (Entity, Entity)>,
    outs: HashMap<usize, usize>,
}

fn entity(input: &str) -> IResult<&str, Entity> {
    map_res(
        tuple((alt((tag("bot "), tag("output "))), digit1)),
        |(t, d): (&str, &str)| -> Result<Entity, std::num::ParseIntError> {
            let i = d.parse::<usize>()?;
            let o = match t {
                "bot " => Entity::Bot(i),
                "output " => Entity::Output(i),
                _ => panic!("unknown entity"),
            };
            Ok(o)
        },
    )(input)
}
fn digit(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse::<usize>())(input)
}

pub fn parse(input: &str) -> Input {
    let value = map(
        tuple((
            preceded(tag("value "), digit),
            preceded(tag(" goes to bot "), digit),
        )),
        |(v, b)| Rule::Input(v, b),
    );
    let giving = map(
        tuple((
            delimited(tag("bot "), digit, tag(" gives ")),
            preceded(tag("low to "), entity),
            preceded(tag(" and high to "), entity),
        )),
        |(b, low, high)| Rule::Give(b, low, high),
    );
    let (rest, lines) = separated_list1(newline, alt((value, giving)))(input).unwrap();
    assert!(rest.trim().is_empty());
    let mut chips: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut rules = HashMap::new();
    for rule in lines {
        match rule {
            Rule::Input(value, bot) => chips.entry(bot).or_default().push(value),
            Rule::Give(bot, low, high) => {
                rules.insert(bot, (low, high));
            }
        }
    }
    Input {
        chips,
        rules,
        outs: Default::default(),
    }
}

pub fn solve(input: &Input, first: bool) -> usize {
    let mut chips = input.chips.clone();
    let rules = input.rules.clone();
    let mut out = input.outs.clone();
    let mut doubled: Vec<_> = chips
        .iter()
        .filter(|(_i, v)| v.len() == 2)
        .map(|(i, _v)| i)
        .copied()
        .collect();
    while !doubled.is_empty() {
        let target = doubled.pop().unwrap();

        let these = chips.get_mut(&target).unwrap();
        let a1 = these.pop().unwrap();
        let a2 = these.pop().unwrap();
        let (a, b) = if a1 < a2 { (a1, a2) } else { (a2, a1) };
        if first && a == 17 && b == 61 {
            return target;
        }
        let (low, high) = rules.get(&target).unwrap();
        match low {
            Entity::Output(x) => {
                out.insert(*x, a);
            }
            Entity::Bot(x) => {
                chips.entry(*x).or_default().push(a);
                if chips.get(x).unwrap().len() > 1 {
                    doubled.push(*x);
                }
            }
        }
        match high {
            Entity::Output(x) => {
                out.insert(*x, b);
            }
            Entity::Bot(x) => {
                chips.entry(*x).or_default().push(b);
                if chips.get(x).unwrap().len() > 1 {
                    doubled.push(*x);
                }
            }
        }
    }
    out[&0] * out[&1] * out[&2]
}

pub fn part1(input: &Input) -> usize {
    solve(input, true)
}
pub fn part2(input: &Input) -> usize {
    solve(input, false)
}
