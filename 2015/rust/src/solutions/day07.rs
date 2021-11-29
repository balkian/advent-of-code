use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?:(?P<left>[a-z0-9]+) )?(?:(?P<instr>[A-Z]+) )?(?:(?P<right>(?:[a-z]+)|(?:[0-9]+)))?").unwrap();
}

#[derive(Debug, Clone)]
enum Signal {
    Wire(String),
    Value(usize),
}
use Signal::*;

impl FromStr for Signal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let v = if let Ok(v) = s.trim().parse::<usize>() {
            Value(v)
        }else {
            Wire(s.to_string())
        };
        Ok(v)
    }
}


#[derive(Debug,Clone)]
enum Instruction {
    Set(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    Lshift(Signal, Signal),
    Rshift(Signal, Signal),
    Not(Signal),
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let cap = RE.captures(s).unwrap();
        let val = match (cap.name("left").map(|s| s.as_str().parse::<Signal>().unwrap()),
                         cap.name("instr").map(|s| s.as_str().to_string()),
                         cap.name("right").map(|s| s.as_str().parse::<Signal>().unwrap())) {
            (Some(v), None, None ) => {
                Set(v)
            },
            (Some(left), Some(instr), Some(right)) if instr == "OR" => {
                Or(left, right)
            },
            (Some(left), Some(instr), Some(right)) if instr == "AND" => {
                And(left, right)
            },
            (Some(left), Some(instr), Some(right)) if instr == "LSHIFT" => {
                Lshift(left, right)
            },
            (Some(left), Some(instr), Some(value)) if instr == "RSHIFT" => {
                Rshift(left, value)
            },
            (None, Some(instr), Some(right)) if instr == "NOT" => {
                Not(right)
            },
            a => {dbg!(a); panic!("invalid function")},
        };
        Ok(val)
    }
}

#[derive(Debug,Clone)]
pub struct Grid {
    map: HashMap<String, Instruction>,
    wires: HashMap<String, usize>,
}

impl Grid {
    fn calculate(&mut self, g: &Signal) -> usize {
        match g {
            Value(u) => *u,
            Wire(g) => {
                if let Some(v) = self.wires.get(g) {
                    return *v
                }
                let res = &self.map.get_mut(g).map(|s| s.clone());
                match res {
                    None => {panic!("unknown gate")},
                    Some(inst) => {
                        let val = match inst {
                            Set(i) => self.calculate(i),
                            Not(i) => !self.calculate(i),
                            And(l, r) => self.calculate(l) & self.calculate(r),
                            Or(l, r) => self.calculate(l) | self.calculate(r),
                            Lshift(l, Value(r)) => self.calculate(l) << *r,
                            Rshift(l, Value(r)) => self.calculate(l) >> *r,
                            _ => panic!("invalid instruction"),
                        };
                        self.wires.insert(g.clone(), val);
                        val
                    }
                }
            }
        }
    }
}

pub fn parse(input: &str) -> Grid {
    let map: HashMap<String, Instruction> = input.lines().filter(|line| !line.is_empty()).map(|line| {
        let mut cap = line.split("->");
        let inst = cap.next().unwrap().parse::<Instruction>().unwrap();
        let output = cap.next().unwrap().trim().to_string();
        (output, inst)
    }).collect();
    let mut g = Grid{map, wires: HashMap::new()};
    g.calculate(&Wire("a".to_string()));
    g
}

pub fn part1(grid: &Grid) -> usize {
    *grid.wires.get("a").unwrap()
}

pub fn part2(grid: &Grid) -> usize {
    let mut grid = (*grid).clone();
    let value_a = *grid.wires.get("a").unwrap();
    grid.wires = HashMap::new();
    grid.wires.insert("b".into(), value_a);
    grid.calculate(&Wire("a".to_string()));
    *grid.wires.get("a").unwrap()
}
