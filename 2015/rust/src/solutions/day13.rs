use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<who>\w+) would (?P<sign>lose|gain) (?P<amount>\d+) happiness units by sitting next to (?P<other>\w+)").unwrap();
}

pub fn parse(input: &str) -> HashMap<(&str, &str), isize> {
    input
        .lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            let who = cap.name("who").unwrap().as_str();
            let other = cap.name("other").unwrap().as_str();
            let mut amount = cap
                .name("amount")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap();
            if cap.name("sign").unwrap().as_str() == "lose" {
                amount = -amount;
            }
            ((who, other), amount)
        })
        .collect()
}

pub fn part1(rules: &HashMap<(&str, &str), isize>) -> isize {
    let mut people: HashSet<&str> = HashSet::new();
    people.extend(rules.iter().map(|((a, _), _)| a));
    people.extend(rules.iter().map(|((_, b), _)| b));

    let mut configs: Vec<Vec<&str>> = people.iter().map(|a| vec![*a]).collect();

    let mut finished: Vec<_> = vec![];
    while let Some(cfg) = configs.pop() {
        if cfg.len() == people.len() {
            finished.push(cfg);
            continue;
        }
        for i in &people {
            if !cfg.contains(i) {
                let mut c = cfg.clone();
                c.push(i);
                configs.push(c);
            }
        }
    }
    finished
        .iter()
        .map(|pp| {
            let mut score = 0;
            for ix in 0..pp.len() {
                score += rules
                    .get(&(pp[ix], pp[(ix + pp.len() - 1) % pp.len()]))
                    .unwrap_or(&0);
                score += rules.get(&(pp[ix], pp[(ix + 1) % pp.len()])).unwrap_or(&0);
            }
            score
        })
        .max()
        .unwrap()
}

pub fn part2(rules: &HashMap<(&str, &str), isize>) -> isize {
    let mut new_rules = rules.clone();
    new_rules.insert(("me", "me"), 0);
    part1(&new_rules)
}
