/// I realized that everything in this problem is binary: 1) two branches, 2) two values (a|b)...
/// This could be solved with a binary tree.
/// However, I wanted to implement something more general, and use it as an excuse to learn pest
///
use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar="rules.pest"] // relative to project `src`
struct RuleParser;

fn parse(input: &str) -> Result<(Vec<&str>, MsgRules), Error<Rule>> {
    let structure = RuleParser::parse(Rule::structure, input)?.next().unwrap();

    let mut messages: Vec<&str> = vec!();
    let mut rules = MsgRules::new();

    for part in structure.into_inner() {
        match part.as_rule() {
            Rule::rules=> {
                for rule in part.into_inner() {
                    let mut name = 0usize; 
                    let mut branches = vec!();
                    for attr in rule.into_inner() {
                        match attr.as_rule() {
                            Rule::rule_name => { name = attr.as_str().parse().expect("invalid name") }
                            Rule::branch => {
                                let mut branch_vec = vec!();
                                for branch in attr.into_inner() {
                                    match branch.as_rule() {
                                        Rule::rule_name => {
                                            let value = branch.as_str().parse().expect("invalid value");
                                            branch_vec.push(Value::Other(value));
                                        }
                                        Rule::value => branch_vec.push(Value::Simple(branch.as_str().chars().next().unwrap())),
                                        _ => panic!("unknown node in branch"),
                                    }
                                }
                                branches.push(branch_vec);
                            }
                            part => panic!("unknown part: {:?}", part)
                        }
                    }
                    rules.add(name, branches);
                }
            }
            Rule::messages => {
                part.into_inner().for_each(|msg| {
                    messages.push(msg.as_str())
                })
            }
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
        Self{list: HashMap::new()}
    }
    fn add(&mut self, name: usize, branches: Vec<Branch>) {
        self.list.insert(name, branches);
    }

    fn matches_partial(&self, name: usize, msg: &str) -> Vec<usize> {
        let root = self.list.get(&name).unwrap();
        let mut options = vec!();
        for branch in root {
            let mut branch_opts = vec!(0);
            for rule in branch.iter() {
                let mut valid = vec!();
                for idx in branch_opts {
                    if msg.len() <= idx {
                        continue
                    }
                    match rule {
                        Value::Simple(c) => {
                            if msg.chars().nth(idx).unwrap() == *c  {
                                valid.push(idx+1)
                            }
                        }
                        Value::Other(uid) => {
                            for i in self.matches_partial(*uid, &msg[idx..]) {
                                valid.push(idx+i);
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
        let matched = self.matches_partial(name, msg);
        matched.contains(&msg.len())
    }
}


fn solve(input: &str) -> usize {
    let (msgs, rules) = parse(input).unwrap();
    let mut counter = 0;
    for msg in msgs {
        if rules.matches(0, msg) {
            counter += 1
        }
    }
    counter
}

fn main() {
    let args = aoc_utils::app(env!("CARGO_PKG_NAME")).get_matches();
    let input = std::fs::read_to_string(args.value_of("input").expect("no input specified")).expect("invalid file");
    let input2 = std::fs::read_to_string("input2.txt").expect("invalid file");
    println!("Part 1: {}", solve(&input));
    println!("Part 2: {}", solve(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solve(r#"0: 4 1 5
                         1: 2 3 | 3 2
                         2: 4 4 | 5 5
                         3: 4 5 | 5 4
                         4: "a"
                         5: "b"

                         ababbb
                         bababa
                         abbbab
                         aaabbb
                         aaaabbb
                         "#), 2);
    }
    #[test]
    fn test_example2() {
     assert_eq!(solve(r#"42: 9 14 | 10 1
                        9: 14 27 | 1 26
                        10: 23 14 | 28 1
                        1: "a"
                        11: 42 31
                        5: 1 14 | 15 1
                        19: 14 1 | 14 14
                        12: 24 14 | 19 1
                        16: 15 1 | 14 14
                        31: 14 17 | 1 13
                        6: 14 14 | 1 14
                        2: 1 24 | 14 4
                        0: 8 11
                        13: 14 3 | 1 12
                        15: 1 | 14
                        17: 14 2 | 1 7
                        23: 25 1 | 22 14
                        28: 16 1
                        4: 1 1
                        20: 14 14 | 1 15
                        3: 5 14 | 16 1
                        27: 1 6 | 14 18
                        14: "b"
                        21: 14 1 | 1 14
                        25: 1 1 | 1 14
                        22: 14 14
                        8: 42
                        26: 14 22 | 1 20
                        18: 15 15
                        7: 14 5 | 1 21
                        24: 14 1

                        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
                        bbabbbbaabaabba
                        babbbbaabbbbbabbbbbbaabaaabaaa
                        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
                        bbbbbbbaaaabbbbaaabbabaaa
                        bbbababbbbaaaaaaaabbababaaababaabab
                        ababaaaaaabaaab
                        ababaaaaabbbaba
                        baabbaaaabbaaaababbaababb
                        abbbbabbbbaaaababbbbbbaaaababb
                        aaaaabbaabaaaaababaa
                        aaaabbaaaabbaaa
                        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
                        babaaabbbaaabaababbaabababaaab
                        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
                        "#), 3);
     }

    #[test]
    fn test_example3() {
     assert_eq!(solve(r#"42: 9 14 | 10 1
                        9: 14 27 | 1 26
                        10: 23 14 | 28 1
                        1: "a"
                        5: 1 14 | 15 1
                        19: 14 1 | 14 14
                        12: 24 14 | 19 1
                        16: 15 1 | 14 14
                        31: 14 17 | 1 13
                        6: 14 14 | 1 14
                        2: 1 24 | 14 4
                        0: 8 11
                        13: 14 3 | 1 12
                        15: 1 | 14
                        17: 14 2 | 1 7
                        23: 25 1 | 22 14
                        28: 16 1
                        4: 1 1
                        20: 14 14 | 1 15
                        3: 5 14 | 16 1
                        27: 1 6 | 14 18
                        14: "b"
                        21: 14 1 | 1 14
                        25: 1 1 | 1 14
                        22: 14 14
                        8: 42 | 42 8
                        11: 42 31 | 42 11 31
                        26: 14 22 | 1 20
                        18: 15 15
                        7: 14 5 | 1 21
                        24: 14 1

                        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
                        bbabbbbaabaabba
                        babbbbaabbbbbabbbbbbaabaaabaaa
                        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
                        bbbbbbbaaaabbbbaaabbabaaa
                        bbbababbbbaaaaaaaabbababaaababaabab
                        ababaaaaaabaaab
                        ababaaaaabbbaba
                        baabbaaaabbaaaababbaababb
                        abbbbabbbbaaaababbbbbbaaaababb
                        aaaaabbaabaaaaababaa
                        aaaabbaaaabbaaa
                        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
                        babaaabbbaaabaababbaabababaaab
                        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
                        "#), 12);
     }
}
