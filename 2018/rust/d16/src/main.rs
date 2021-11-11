use anyhow::{anyhow, Result as AResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

use std::fs;
use std::{error::Error, fmt as efmt};

#[derive(Debug, PartialEq)]
struct Op(usize, isize, isize, usize);

#[derive(Debug)]
struct PC {
    regs: [isize; 4],
}

type Registers = [isize; 4];

type Operation = fn(&Registers, &Op) -> Option<Registers>;

#[derive(Debug)]
struct Case {
    before: Registers,
    op: Op,
    after: Registers,
}

#[derive(Debug, Clone)]
struct ParseError;

impl Error for ParseError {}

impl efmt::Display for ParseError {
    fn fmt(&self, f: &mut efmt::Formatter) -> efmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

lazy_static! {
    static ref OP: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    static ref REGS: Regex = Regex::new(r"(\d+), (\d+), (\d+), (\d+)").unwrap();
    static ref CASES: Regex =
        Regex::new(r"Before: \[(?P<before>[^\]]+)\]\n(?P<code>.*)\nAfter:  \[(?P<after>[^\]]+)\]")
            .unwrap();
}

#[allow(clippy::many_single_char_names)]
fn parse_op(s: &str) -> AResult<Op> {
    let caps = OP.captures(s).ok_or_else(|| anyhow!("oops"))?;
    let op = caps[1].parse::<usize>()?;
    let a = caps[2].parse::<isize>()?;
    let b = caps[3].parse::<isize>()?;
    let c = caps[4].parse::<usize>()?;
    Ok(Op(op, a, b, c))
}

#[allow(clippy::many_single_char_names)]
fn parse_regs(s: &str) -> AResult<Registers> {
    let caps = REGS.captures(s).ok_or_else(|| anyhow!("ohno"))?;
    let a = caps[1].parse::<isize>()?;
    let b = caps[2].parse::<isize>()?;
    let c = caps[3].parse::<isize>()?;
    let d = caps[4].parse::<isize>()?;
    Ok([a, b, c, d])
}

fn parse_cases(s: &str) -> AResult<(Vec<Case>, Vec<Op>)> {
    let mut cases = vec![];
    let mut s = s;
    let mut last_b = 0;
    for case in CASES.captures_iter(s) {
        let before = parse_regs(&case[1])?;
        let op = parse_op(&case[2])?;
        let after = parse_regs(&case[3])?;
        cases.push(Case { before, op, after });
        last_b = case.get(3).unwrap().end();
    }
    s = s.split_at(last_b + 1).1;
    let ops: Vec<Op> = s
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|line| parse_op(line))
        .collect::<AResult<Vec<Op>>>()?;
    Ok((cases, ops))
}

macro_rules! operations {
    ($($tts:tt)*) => {{
        let mut vect: Vec<(&str, Operation)> = vec![];
        operations_inner!(vect $($tts)*);
        let hm: HashMap<&str, Operation> = HashMap::from_iter(vect);
        hm
    }}
}

/// This is a very ugly macro, but I'm still learning
macro_rules! operations_inner {
    (@direct $vect:ident $name:ident |$a:ident, $b:ident| $op:block ) => {{
        fn $name(reg: &Registers, op: &Op) -> Option<Registers> {
            if op.1 < 0 || op.1 as usize >= reg.len() {
                return None
            }
            let mut res = reg.clone();
            let $a = reg[op.1 as usize];
            let $b = op.2;
            res[op.3] = $op;
            Some(res)
        }
        $vect.push((stringify!($name), $name));

    }};
    (@indirect $vect:ident $name:ident |$a:ident, $b:ident| $op:block) => {{
        fn $name(reg: &Registers, op: &Op) -> Option<Registers> {
            if op.2 < 0 || op.2 as usize >= reg.len() {
                return None
            }
            let mut res = reg.clone();
            let $a = reg[op.1 as usize];
            let $b = reg[op.2 as usize];
            res[op.3] = $op;
            Some(res)
        }
        $vect.push((stringify!($name), $name));
    }};
    (@inverse $vect:ident $name:ident |$a:ident, $b:ident| $op:block ) => {{
        fn $name(reg: &Registers, op: &Op) -> Option<Registers> {
            if op.1 < 0 || op.2 < 0 || op.1 as usize > reg.len() || op.2 as usize >= reg.len() {
                return None
            }
            let mut res = reg.clone();
            let $a = op.1;
            let $b = reg[op.2 as usize];
            res[op.3] = $op;
            Some(res)
        }
        $vect.push((stringify!($name), $name));
    }};
    ($vect:ident ($($dir:ident)?, $($indir:ident)?, $($inv:ident)?) => |$a:ident, $b:ident| $op:block
     $(;$(($($odir:ident)?, $($oindir:ident)?, $($oinv:ident)?) => |$oa:ident, $ob:ident| $oop:block)*)* ;)
     => {{
        $(operations_inner!(@direct $vect $dir |$a, $b| $op);)?
        $(operations_inner!(@indirect $vect $indir |$a, $b| $op);)?
        $(operations_inner!(@inverse $vect $inv |$a, $b| $op);)?
        $(operations_inner!($vect $(($($odir)?, $($oindir)?, $($oinv)?) => |$oa ,$ob| $oop;)*);)?
    }}
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read file");
    let (sol1, sol2) = solve(&input);
    println!("Solution 1: {}", sol1);
    println!("Solution 2: {}", sol2);
}

fn solve(input: &str) -> (usize, isize) {
    let (cases, code) = parse_cases(input).unwrap();

    let ops = operations! {
        (addi, addr,     ) => |a, b| {a+b};
        (muli, mulr,     ) => |a, b| {a*b};
        (bani, banr,     ) => |a, b| {a&b};
        (bori, borr,     ) => |a, b| {a|b};
        (    ,     , seti) => |a,_b| {a};
        (setr,     ,     ) => |a,_b| {a};
        (gtri, gtrr, gtir) => |a, b| {if a>b {1} else {0}};
        (eqri, eqrr, eqir) => |a, b| {if a==b {1} else {0}};

    };
    let mut discarded: HashMap<&str, HashSet<usize>> = HashMap::new();
    let mut count = 0;
    for case in cases.iter() {
        let mut matches = 0;
        for (k, func) in ops.iter() {
            if let Some(res) = func(&case.before, &case.op) {
                if res == case.after {
                    matches += 1;
                } else {
                    let e = discarded.entry(k).or_default();
                    e.insert(case.op.0);
                }
            }
        }
        if matches >= 3 {
            count += 1;
        }
    }
    let allpossible: HashSet<usize> = (0..16).collect();
    let mut found: HashMap<usize, &str> = HashMap::new();
    let mut missing: VecDeque<&str> = discarded.keys().copied().collect();
    while !missing.is_empty() {
        let k = missing.pop_front().unwrap();
        let disc = discarded.get_mut(k).unwrap();
        for v in found.keys() {
            disc.insert(*v);
        }
        if disc.len() == 15 {
            let f = allpossible.difference(disc).next().unwrap();
            found.insert(*f, k);
        } else {
            missing.push_back(k);
        }
    }
    let _regs = &[0, 0, 0, 0];
    let regs = code.iter().fold([0, 0, 0, 0], |acc, op| {
        let name = found.get(&op.0).expect("opcode not found");
        let func = ops.get(name).unwrap();
        func(&acc, op).unwrap()
    });
    (count, regs[0])
}

#[test]
fn test_example() {
    let case = &"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
    let (sol1, _sol2) = solve(case);
    assert_eq!(sol1, 1);
}
