use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::cmp::{
    Ordering,
    Ordering::{Equal, Less},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Integer(usize),
    List(Vec<Item>),
}

use Item::*;

fn parse_integer(input: &str) -> IResult<&str, Item> {
    map(digit1, |num: &str| {
        Item::Integer(num.parse::<usize>().unwrap())
    })(input)
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    map(
        delimited(tag("["), separated_list0(tag(","), parse_item), tag("]")),
        Item::List,
    )(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((parse_list, parse_integer))(input)
}

fn parse_line(input: &str) -> IResult<&str, Item> {
    terminated(parse_item, opt(newline))(input)
}

type Input = Vec<(Item, Item)>;

pub fn parse(input: &str) -> Input {
    let parse_pair = tuple((parse_line, parse_line));
    let (rest, pairs) = separated_list0(newline, parse_pair)(input).unwrap();
    assert!(rest.is_empty());
    pairs
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Integer(x), Integer(y)) => x.cmp(y),
            (Integer(_), y) => List(vec![self.clone()]).cmp(y),
            (x, Integer(_)) => x.cmp(&List(vec![other.clone()])),
            (List(v1), List(v2)) => {
                for (i1, other) in v1.iter().zip(v2.iter()) {
                    match i1.cmp(other) {
                        Equal => continue,
                        a => return a,
                    }
                }
                v1.len().cmp(&v2.len())
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sorted(i1: &Item, i2: &Item) -> bool {
    i1.cmp(i2) == Less
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(
            |(ix, (l1, l2))| {
                if sorted(l1, l2) {
                    Some(ix + 1)
                } else {
                    None
                }
            },
        )
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut pkts = vec![
        List(vec![List(vec![Integer(2)])]),
        List(vec![List(vec![Integer(6)])]),
    ];
    let dividers = pkts.clone();
    for (i1, i2) in input {
        pkts.push(i1.clone());
        pkts.push(i2.clone());
    }
    pkts.sort();
    pkts.iter()
        .enumerate()
        .filter(|(_ix, i)| dividers.contains(i))
        .map(|(ix, _)| ix + 1)
        .product()
}
