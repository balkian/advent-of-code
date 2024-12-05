use std::collections::HashSet;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::character::complete::multispace1;
use nom::character::complete;
use nom::bytes::complete::tag;

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashSet<(u64, u64)>,
    updates: Vec<Vec<u64>>,

}
impl Input {
    fn correct(&self, update: &[u64]) -> bool {
        let mut seen: HashSet<_> = Default::default();
        for page in update.iter().copied() {
            for other in seen.iter().copied() {
                if self.rules.contains(&(page, other)) {
                    return false
                }
            }
            seen.insert(page);
        }
        true
    }
}

pub fn parse(i: &str) -> Input {
    let rulelist = separated_list1(multispace1::<&str, ()>, separated_pair(complete::u64, tag("|"), complete::u64));
    let updates = separated_list1(multispace1, separated_list1(tag(","), complete::u64));
    let (_, (rulelist, updates)) = separated_pair(rulelist, multispace1, updates)(i).expect("parsing error");
    let rules = rulelist.into_iter().collect();
    Input{rules, updates}
}

pub fn part1(i: &Input) -> usize {
    dbg!(i);
    i.updates.iter().filter(|rule| {
        i.correct(rule)
    }).map(|rule| {
        rule[rule.len()/2]
    }).sum::<u64>().try_into().expect("invalid number")
}
pub fn part2(i: &Input) -> usize {
    todo!();
}
