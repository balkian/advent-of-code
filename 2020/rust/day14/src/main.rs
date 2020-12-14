use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::num::ParseIntError;

lazy_static! {
    static ref COMMAND: Regex =
        Regex::new(r"mask = (?P<mask>.*)|mem\[(?P<memory>\d+)\] = (?P<address>\d+)").unwrap();
}

#[derive(Debug)]
enum Command {
    Mask(u64, u64, String),
    Load(u64, u64),
}

#[derive(Debug)]
struct Mask(u64, u64, String);

#[derive(Debug)]
struct Computer {
    memory: HashMap<u64, u64>,
    mask: Mask,
}

impl Computer {
    fn new() -> Self {
        Computer {
            memory: HashMap::new(),
            mask: Mask(std::u64::MAX, 0, "".into()),
        }
    }

    fn apply(&mut self, c: &Command) {
        match c {
            Command::Mask(zeroes, ones, v) => {
                self.mask = Mask(*zeroes, *ones, v.clone());
            }
            Command::Load(m, a) => {
                self.memory.insert(*m, (a & self.mask.0) | self.mask.1);
            }
        }
    }

    fn apply_v2(&mut self, c: &Command) {
        match c {
            Command::Mask(zeroes, ones, v) => {
                self.mask = Mask(*zeroes, *ones, v.clone());
            }
            Command::Load(m, a) => {
                let mut masks: Vec<u64> = vec![0];
                for (idx, v) in self.mask.2.chars().rev().enumerate() {
                    if v == 'X' {
                        for i in 0..masks.len() {
                            let val = masks[i] + (2u64.pow(idx as u32)) as u64;
                            masks.push(val);
                        }
                    }
                }
                let keep = !self.mask.0 | self.mask.1;
                let mem0 = (m | self.mask.1) & keep;
                for m in masks {
                    let mem1 = mem0 | m;
                    self.memory.insert(mem1, *a);
                }
            }
        }
    }
}

enum ParseCmdError {
    ParseIntError(ParseIntError),
    WrongStructure(String),
}

impl From<ParseIntError> for ParseCmdError {
    fn from(e: ParseIntError) -> Self {
        ParseCmdError::ParseIntError(e)
    }
}

impl std::str::FromStr for Command {
    type Err = ParseCmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = COMMAND
            .captures(&s)
            .ok_or_else(|| ParseCmdError::WrongStructure("no results found".to_string()))?;
        match (reg.name("mask"), reg.name("memory"), reg.name("address")) {
            (Some(v), _, _) => {
                let zeroes = u64::from_str_radix(&v.as_str().to_string().replace("X", "1"), 2)?;
                let ones = u64::from_str_radix(&v.as_str().to_string().replace("X", "0"), 2)?;
                Ok(Command::Mask(zeroes, ones, v.as_str().to_string()))
            }
            (None, Some(m), Some(a)) => {
                let m: u64 = m.as_str().parse()?;
                let a: u64 = a.as_str().parse()?;
                Ok(Command::Load(m, a))
            }
            _ => Err(Self::Err::WrongStructure(format!(
                "invalid instruction: {:?}",
                s
            ))),
        }
    }
}

fn main() {
    let args = aoc_utils::app("14").get_matches();
    let cmds: Vec<Command> = aoc_utils::file_iter_clap(&args)
        .filter_map(|x| x.parse().ok())
        .collect();
    let pc = &mut Computer::new();
    for c in &cmds {
        pc.apply(&c);
    }
    println!("Part 1: {}", pc.memory.values().sum::<u64>());

    let pc = &mut Computer::new();
    for c in &cmds {
        pc.apply_v2(&c);
    }
    println!("Part 2: {}", pc.memory.values().sum::<u64>());
}
