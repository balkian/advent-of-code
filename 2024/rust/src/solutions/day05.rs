use nom::{
    bytes::complete::tag,
    character::{complete, complete::multispace1},
    multi::separated_list1,
    sequence::separated_pair,
};
use std::collections::HashSet;

type Update = Vec<u64>;

#[derive(Debug, Clone)]
pub struct Input {
    rules: HashSet<(u64, u64)>,
    updates: Vec<Vec<u64>>,
}

impl Input {
    #[inline]
    fn is_correct(&self, update: &Update) -> bool {
        let mut seen: Vec<_> = Vec::with_capacity(update.len());
        for page in update.iter().copied() {
            for other in seen.iter().copied() {
                if self.rules.contains(&(page, other)) {
                    return false;
                }
            }
            seen.push(page);
        }
        true
    }

    /// If the rule is wrong, return it in the right order.
    /// Note: We assume that a solution is always possible
    #[inline]
    fn fix<'a>(&self, update: &'a Update) -> Result<&'a Update, Update> {
        // We assume seen pages are in the correct order
        let mut seen: Vec<_> = Vec::with_capacity(update.len());
        let mut updated = false;

        'outer: for page in update.iter().copied() {
            for (ix, other) in seen.iter().copied().enumerate() {
                if self.rules.contains(&(page, other)) {
                    // Adding page after other would break a rule,
                    // so it has to come before it.
                    seen.insert(ix, page);
                    updated = true;
                    // In theory, we should check if the new order (page, other) is
                    // correct for the remaining seen pages. But, if it isn't, there
                    // would not be a valid solution, which cannot happen in AoC.
                    continue 'outer;
                }
            }
            seen.push(page);
        }
        if updated {
            Err(seen)
        } else {
            Ok(update)
        }
    }
}

pub fn parse(i: &str) -> Input {
    let rulelist = separated_list1(
        multispace1::<&str, ()>,
        separated_pair(complete::u64, tag("|"), complete::u64),
    );
    let updates = separated_list1(multispace1, separated_list1(tag(","), complete::u64));
    let (_, (rulelist, updates)) =
        separated_pair(rulelist, multispace1, updates)(i).expect("parsing error");
    let rules = rulelist.into_iter().collect();
    Input { rules, updates }
}

pub fn part1(i: &Input) -> usize {
    i.updates
        .iter()
        .filter(|rule| i.is_correct(rule))
        .map(|rule| rule[rule.len() / 2])
        .sum::<u64>()
        .try_into()
        .expect("invalid u64 to usize conversion")
}

pub fn part2(i: &Input) -> usize {
    i.updates
        .iter()
        .filter_map(|rule| i.fix(rule).err())
        .map(|rule| rule[rule.len() / 2])
        .sum::<u64>()
        .try_into()
        .expect("invalid u64 to usize conversion")
}
