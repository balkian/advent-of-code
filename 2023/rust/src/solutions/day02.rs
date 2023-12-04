use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    character::complete::{multispace0, newline},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};
use std::cmp::max;

type Draw = [usize; 3];

#[derive(Debug)]
pub struct Game {
    id: usize,
    draws: Vec<Draw>,
}

fn unsigned(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

impl Game {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (id, draws)) = tuple((
            delimited(tag("Game "), unsigned, tag(":")),
            separated_list1(tag(";"), parse_draw),
        ))(input)?;
        Ok((input, Game { id, draws }))
    }

    fn matches(&self, r: usize, g: usize, b: usize) -> bool {
        self.draws
            .iter()
            .all(|[sr, sg, sb]| *sr <= r && *sg <= g && *sb <= b)
    }

    fn power(&self) -> usize {
        let mut minimum = [0; 3];
        for draw in &self.draws {
            minimum[0] = max(minimum[0], draw[0]);
            minimum[1] = max(minimum[1], draw[1]);
            minimum[2] = max(minimum[2], draw[2]);
        }
        minimum.into_iter().product()
    }
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (input, colors) = separated_list1(
        tag(","),
        delimited(
            space0,
            separated_pair(
                unsigned,
                space0,
                alt((tag("green"), tag("red"), tag("blue"))),
            ),
            space0,
        ),
    )(input)?;
    let mut draw = [0; 3];
    for (count, color) in colors {
        match color {
            "red" => draw[0] += count,
            "green" => draw[1] += count,
            "blue" => draw[2] += count,
            _ => panic!("unknown color"),
        }
    }
    Ok((input, draw))
}

pub fn parse(input: &str) -> Vec<Game> {
    let (rest, games) = terminated(separated_list1(newline, Game::parser), multispace0)(input)
        .expect("coult not parse games");
    assert!(rest.is_empty());
    games
}

pub fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .filter_map(|game| game.matches(12, 13, 14).then_some(game.id))
        .sum::<usize>()
}

pub fn part2(input: &[Game]) -> usize {
    input.iter().map(|game| game.power()).sum()
}
