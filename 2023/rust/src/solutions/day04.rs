use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, space0, space1},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Card {
    #[allow(dead_code)]
    id: usize,
    winning: BTreeSet<usize>,
    present: BTreeSet<usize>,
}

impl Card {
    fn n_win(&self) -> usize {
        self.winning.intersection(&self.present).count()
    }
}

fn unsigned(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(space0, separated_list1(space1, unsigned), space0)(input)
}

pub fn parse(input: &str) -> Vec<Card> {
    let (rest, cards) = separated_list1(
        multispace1,
        tuple((
            delimited(tuple((tag("Card"), space1)), unsigned, tag(":")),
            separated_pair(numbers, tag("|"), numbers),
        )),
    )(input)
    .expect("could not parse cards");
    assert!(rest.trim().is_empty());
    cards
        .into_iter()
        .map(|(id, (winning, present))| Card {
            id,
            winning: winning.into_iter().collect(),
            present: present.into_iter().collect(),
        })
        .collect()
}

pub fn part1(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|card| match card.n_win() {
            0 => 0,
            a => 1 << a,
        })
        .sum()
}

pub fn part2(cards: &[Card]) -> usize {
    let mut counts = vec![1; cards.len()];

    for (ix, card) in cards.iter().enumerate() {
        let n_won = card.n_win();
        for other in ((ix + 1)..).take(n_won) {
            counts[other] += counts[ix];
        }
    }

    counts.into_iter().sum()
}
