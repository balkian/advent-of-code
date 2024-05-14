use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, digit1, multispace1, newline};
use nom::combinator::opt;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;

use std::collections::BTreeMap;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Gift {
    attrs: [u16; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MagicGift {
    attrs: [RangeInclusive<u16>; 4],
}

impl MagicGift {
    fn count(&self) -> usize {
        self.attrs.iter().map(|r| r.len()).product()
    }
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    condition: Condition,
    output: &'a str,
}

impl<'a> Rule<'a> {
    fn magic_process(&self, gift: &MagicGift) -> (&'a str, Vec<MagicGift>, Vec<MagicGift>) {
        let (pass, reject) = self.condition.split(gift);
        (self.output, pass, reject)
    }

    fn process(&self, gift: &Gift) -> Option<&'a str> {
        if self.condition.matches(gift) {
            Some(self.output)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    Less(usize, u16),
    Equal(usize, u16),
    Greater(usize, u16),
    Always,
}

impl Condition {
    fn split(&self, gift: &MagicGift) -> (Vec<MagicGift>, Vec<MagicGift>) {
        let (passed, rejected) = match self {
            Condition::Less(attr, a) => {
                let val = &gift.attrs[*attr];
                let mut passed = vec![];
                let mut rejected = vec![];
                let r1 = *val.start()..=((*a - 1).min(*val.end()));

                if !r1.is_empty() {
                    let mut p1 = gift.clone();
                    p1.attrs[*attr] = r1;
                    passed.push(p1);
                }
                let r2 = *a..=*val.end();
                if !r2.is_empty() {
                    let mut p2 = gift.clone();
                    p2.attrs[*attr] = r2;
                    rejected.push(p2);
                }
                (passed, rejected)
            }
            Condition::Equal(attr, a) => {
                let val = &gift.attrs[*attr];
                if !val.contains(a) || val.is_empty() {
                    (vec![], vec![gift.clone()])
                } else {
                    let mut passed = gift.clone();
                    passed.attrs[*attr] = *a..=*a;
                    let mut rejected = vec![];

                    let r1 = *val.start()..=(*a - 1).min(*val.end());
                    if !r1.is_empty() {
                        let mut mg1 = gift.clone();
                        mg1.attrs[*attr] = r1;
                        rejected.push(mg1);
                    }
                    let r2 = (a + 1)..=*val.end();
                    if !r2.is_empty() {
                        let mut rej2 = gift.clone();
                        rej2.attrs[*attr] = r2;
                        rejected.push(rej2);
                    }
                    (vec![passed], rejected)
                }
            }
            Condition::Greater(attr, a) => {
                let val = &gift.attrs[*attr];
                let mut passed = vec![];
                let mut rejected = vec![];
                let r1 = *val.start()..=(*a.min(val.end()));
                if !r1.is_empty() {
                    let mut p1 = gift.clone();
                    p1.attrs[*attr] = r1;
                    rejected.push(p1);
                }
                let r2 = (a + 1)..=*val.end();
                if !r2.is_empty() {
                    let mut p2 = gift.clone();
                    p2.attrs[*attr] = r2;
                    passed.push(p2);
                }
                (passed, rejected)
            }
            Condition::Always => (vec![gift.clone()], vec![]),
        };

        debug_assert_eq!(
            passed
                .iter()
                .chain(rejected.iter())
                .map(|mp| mp.count())
                .sum::<usize>(),
            gift.count()
        );
        (passed, rejected)
    }

    fn matches(&self, gift: &Gift) -> bool {
        match self {
            Condition::Less(attr, a) => {
                let value = gift.attrs[*attr];
                value < *a
            }
            Condition::Equal(attr, a) => {
                let value = gift.attrs[*attr];
                value == *a
            }
            Condition::Greater(attr, a) => {
                let value = gift.attrs[*attr];
                value > *a
            }
            Condition::Always => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Processor<'a> {
    workflow: BTreeMap<&'a str, Vec<Rule<'a>>>,
    buckets: BTreeMap<&'a str, Vec<Gift>>,
    accepted: Vec<Gift>,
    // rejected: Vec<Gift>,
}

impl<'a> Processor<'a> {
    fn process(&mut self) {
        while let Some(key) = self.buckets.keys().next() {
            let key = *key;
            let gifts = self.buckets.remove(&key).unwrap();

            for gift in gifts {
                let mut out = None;
                for rule in self.workflow.get(&key).expect("unknown workflow {key}") {
                    out = rule.process(&gift);
                    if out.is_some() {
                        break;
                    }
                }
                let outbox = match out.expect("did not find a destination") {
                    "A" => &mut self.accepted,
                    "R" => continue,
                    name => self.buckets.entry(name).or_default(),
                };
                outbox.push(gift);
            }
        }
    }

    // fn get_mut(&mut self, queue: &str) -> &mut Vec<Gift> {
    //     match queue {
    //         "A" => &mut self.accepted,
    //         "R" => &mut self.rejected,
    //         _ => self.buckets.get_mut(queue).expect("queue does not exist"),
    //     }
    // }

    fn value(&self) -> usize {
        self.accepted
            .iter()
            .map(|gift| gift.attrs.iter().sum::<u16>() as usize)
            .sum::<usize>()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MagicProcessor<'a> {
    workflow: BTreeMap<&'a str, Vec<Rule<'a>>>,
    buckets: BTreeMap<&'a str, Vec<MagicGift>>,
    accepted: Vec<MagicGift>,
    // rejected: Vec<MagicGift>,
}

impl<'a> MagicProcessor<'a> {
    fn new(p: &'a Processor) -> Self {
        let mut mp = MagicProcessor {
            workflow: p.workflow.clone(),
            ..Default::default()
        };
        let mb = MagicGift {
            attrs: [1..=4000, 1..=4000, 1..=4000, 1..=4000],
        };
        mp.buckets.insert("in", vec![mb]);
        mp
    }
    fn magic_process(&mut self) {
        while let Some(key) = self.buckets.keys().next() {
            let key = *key;
            let gifts = self.buckets.remove(&key).unwrap();

            for gift in gifts {
                let mut pending = vec![gift];
                for rule in self.workflow.get(&key).expect("unknown workflow {key}") {
                    let this_batch = std::mem::take(&mut pending);
                    for gift in this_batch.iter() {
                        let (out, pass, reject) = rule.magic_process(gift);
                        assert_eq!(
                            pass.iter()
                                .chain(reject.iter())
                                .map(|mp| mp.count())
                                .sum::<usize>(),
                            gift.count()
                        );

                        pending.extend(reject);
                        let outbox = match out {
                            "A" => &mut self.accepted,
                            "R" => continue,
                            name if name == key => panic!("loop detected"),
                            name => self.buckets.entry(name).or_default(),
                        };
                        outbox.extend(pass);
                    }
                }
            }
        }
    }
}

fn index_for_char(c: char) -> usize {
    match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("invalid char"),
    }
}

fn parse_gift(input: &str) -> IResult<&str, Gift> {
    let (rest, keyvalues) = delimited(
        tag("{"),
        separated_list1(tag(","), separated_pair(anychar, tag("="), digit1)),
        tag("}"),
    )(input)?;
    let mut attrs = [0; 4];
    for (k, v) in keyvalues {
        attrs[index_for_char(k)] = v.parse::<u16>().expect("could not parse number {v}");
    }
    Ok((rest, Gift { attrs }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (rest, (cond, output)) = tuple((
        opt(terminated(
            tuple((anychar, alt((tag("="), tag(">"), tag("<"))), digit1)),
            tag(":"),
        )),
        alpha1,
    ))(input)?;
    let condition = match cond {
        None => Condition::Always,
        Some((att, "<", val)) => Condition::Less(
            index_for_char(att),
            val.parse::<u16>().expect("could not parse {val}"),
        ),
        Some((att, ">", val)) => Condition::Greater(
            index_for_char(att),
            val.parse::<u16>().expect("could not parse {val}"),
        ),
        Some((att, "=", val)) => Condition::Equal(
            index_for_char(att),
            val.parse::<u16>().expect("could not parse {val}"),
        ),
        Some((_, op, _val)) => panic!("invalid operator: {op}"),
    };
    Ok((rest, Rule { condition, output }))
}

fn parse_branch(input: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    let (rest, id) = alpha1(input)?;
    let (rest, rules) = delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(rest)?;
    Ok((rest, (id, rules)))
}

fn parse_workflow(input: &str) -> IResult<&str, BTreeMap<&str, Vec<Rule>>> {
    let (rest, rulepairs) = separated_list1(newline, parse_branch)(input)?;
    let hm = BTreeMap::from_iter(rulepairs);
    Ok((rest, hm))
}

pub fn parse(input: &str) -> Processor {
    let (input, workflow) = parse_workflow(input).expect("could not parse workflow");
    let (input, _) = multispace1::<_, Error<_>>(input).unwrap(); //("no empty line afterwards");
    let (input, gifts) = separated_list1(newline, parse_gift)(input).expect("no gifts");
    assert!(input.trim().is_empty());

    let mut buckets = BTreeMap::new();
    buckets.insert("in", gifts);

    Processor {
        workflow,
        buckets,
        accepted: vec![],
    } //, rejected: vec![]}
}

pub fn part1(proc: &Processor) -> usize {
    let mut proc = proc.clone();
    proc.process();
    proc.value()
}

pub fn part2(input: &Processor) -> usize {
    let mut mp = MagicProcessor::new(input);
    mp.magic_process();
    mp.accepted.into_iter().map(|mg| mg.count()).sum::<usize>()
}
