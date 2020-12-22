/// I realized that everything in this problem is binary: 1) two branches, 2) two values (a|b)...
/// This could be solved with a binary tree.
/// However, I wanted to implement something more general, and use it as an excuse to learn pest
///
use std::collections::HashMap;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rules.pest"] // relative to project `src`
struct RuleParser;

fn parse(input: &str) -> Result<(Vec<&str>, MsgRules), Error<Rule>> {
    let structure = RuleParser::parse(Rule::structure, input)?.next().unwrap();

    let mut messages: Vec<&str> = vec![];
    let mut rules = MsgRules::new();

    for part in structure.into_inner() {
        match part.as_rule() {
            Rule::rules => {
                for rule in part.into_inner() {
                    let mut name: Option<usize> = None;
                    let mut branches = vec![];
                    for attr in rule.into_inner() {
                        match attr.as_rule() {
                            Rule::rule_name => {
                                name = Some(attr.as_str().parse().expect("invalid name"))
                            }
                            Rule::branch => {
                                let branch_vec = attr
                                    .into_inner()
                                    .map(|branch| match branch.as_rule() {
                                        Rule::rule_name => {
                                            let value =
                                                branch.as_str().parse().expect("invalid value");
                                            Value::Other(value)
                                        }
                                        Rule::value => {
                                            Value::Simple(branch.as_str().chars().next().unwrap())
                                        }
                                        _ => panic!("unknown node in branch"),
                                    })
                                    .collect();
                                branches.push(branch_vec);
                            }
                            part => panic!("unknown part: {:?}", part),
                        }
                    }
                    rules.add(name.expect("No name given"), branches);
                }
            }
            Rule::messages => part
                .into_inner()
                .for_each(|msg| messages.push(msg.as_str())),
            el => panic!("unknown element {:?}", el),
        }
    }
    Ok((messages, rules))
}

#[derive(Debug)]
enum Value {
    Simple(char),
    Other(usize),
}

type Branch = Vec<Value>;

type Branches = Vec<Branch>;

#[derive(Debug)]
struct MsgRules {
    list: HashMap<usize, Branches>,
}

impl MsgRules {
    fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
    fn add(&mut self, name: usize, branches: Vec<Branch>) {
        self.list.insert(name, branches);
    }

    fn matches_partial(&self, name: usize, msg: &str) -> Vec<usize> {
        let root = self.list.get(&name).unwrap();
        let mut options = vec![];
        for branch in root {
            let mut branch_opts = vec![0];
            for rule in branch.iter() {
                let mut valid = vec![];
                for idx in branch_opts {
                    if msg.len() <= idx {
                        continue;
                    }
                    match rule {
                        Value::Simple(c) => {
                            if msg.chars().nth(idx).unwrap() == *c {
                                valid.push(idx + 1)
                            }
                        }
                        Value::Other(uid) => {
                            for i in self.matches_partial(*uid, &msg[idx..]) {
                                valid.push(idx + i);
                            }
                        }
                    }
                }
                branch_opts = valid;
            }
            options.extend(branch_opts.drain(0..));
        }
        options
    }
    fn matches(&self, name: usize, msg: &str) -> bool {
        self.matches_partial(name, msg).contains(&msg.len())
    }
}

fn solve(input: &str) -> usize {
    let (msgs, rules) = parse(input).unwrap();
    msgs.iter().filter(|msg| rules.matches(0, msg)).count()
}

fn main() {
    let args = aoc_utils::app(env!("CARGO_PKG_NAME")).get_matches();
    let input = std::fs::read_to_string(args.value_of("input").expect("no input specified"))
        .expect("invalid file");
    let input2 = std::fs::read_to_string("input2.txt").expect("invalid file");
    println!("Part 1: {}", solve(&input));
    println!("Part 2: {}", solve(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solve(include_str!("example_1.txt")), 2);
    }
    #[test]
    fn test_example2() {
        assert_eq!(solve(include_str!("example_2.txt")), 3);
    }

    #[test]
    fn test_example3() {
        assert_eq!(solve(include_str!("example_3.txt")), 12);
    }
}
