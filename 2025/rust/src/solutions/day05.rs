use nom::{
    IResult, Parser,
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::{many0, many1},
    sequence::{separated_pair, terminated},
};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Input {
    ranges: Vec<RangeInclusive<usize>>,
    ids: Vec<usize>,
}

fn nusize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, (start_num, end_num)) = separated_pair(nusize, char('-'), nusize).parse(input)?;

    Ok((input, start_num..=end_num))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, (ranges, ids)) = separated_pair(
        many0(terminated(parse_range, newline)),
        newline,
        many1(terminated(nusize, newline)),
    )
    .parse(input)?;

    Ok((input, Input { ranges, ids }))
}

pub fn parse(input: &str) -> Input {
    parse_input(input).unwrap().1
}

pub fn part1(i: &Input) -> usize {
    let mut i = i.clone();
    i.ranges.sort_by_key(|r| *r.start());
    i.ids
        .iter()
        .filter(|id| {
            let mut idx = i.ranges.partition_point(|range| range.start() < id);
            while idx > 0 {
                idx -= 1;
                let rng = &i.ranges[idx];
                if rng.start() <= id && rng.end() >= id {
                    return true;
                }
            }
            false
        })
        .count()
}

pub fn part2(i: &Input) -> usize {
    let mut i = i.clone();
    i.ranges.sort_by_key(|r| *r.start());

    let mut idx = 1;

    while idx < i.ranges.len() {
        let r1 = &i.ranges[idx - 1];
        let r2 = &i.ranges[idx];

        if r1.contains(r2.end()) {
            i.ranges.remove(idx);
            continue;
        } else if r1.end() >= r2.start() {
            i.ranges[idx - 1] = *r1.start()..=r2.start() - 1;
        }
        idx += 1;
    }

    i.ranges.into_iter().map(|range| range.count()).sum()
}
