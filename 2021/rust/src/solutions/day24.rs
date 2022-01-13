use crate::dbg;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::ops::{Index, IndexMut};

pub fn part1(pc: &PC) -> Regtype {
    pc.find(false)
}

pub fn part2(pc: &PC) -> Regtype {
    pc.find(true)
}

pub fn parse(input: &str) -> PC {
    PC::parse(input)
}

#[derive(Debug, Clone)]
pub struct PC<'a> {
    program: Vec<Instruction<'a>>,
}

impl<'a> PC<'a> {
    /// Find the biggest input that results in regs["z"] == 0 after running
    /// the program.
    fn find(&self, reverse: bool) -> Regtype {
        let mut states = StateMap::new(reverse);
        states.push(State::new());

        while let Some(mut st) = states.pop() {
            dbg!(
                "Inst: {} States: {}. Seen: {}. Ignored: {}. Input: {:?}",
                st.inst,
                states.len(),
                states.seen.len(),
                states.ignored,
                &st.input
            );

            // process the state until an input comes along
            while st.inst < self.program.len() {
                let inst = &self.program[st.inst];
                st.inst += 1;
                if matches!(inst, Single("inp", _)) {
                    for inp in 1..10 {
                        let mut st = st.clone();
                        st.input.push(inp);
                        st.apply(inst);
                        states.push(st);
                    }
                    break;
                }
                st.apply(inst);
            }

            if st.inst == self.program.len() && st["z"] == 0 {
                assert!(self.confirm(&st.input));
                return st.input.into_iter().reduce(|acc, b| acc * 10 + b).unwrap();
            }
        }
        panic!("solution not found");
    }

    fn confirm(&self, input: &[Regtype]) -> bool {
        let mut state = State::new();
        state.input.extend(input.iter());
        for inst in self.program.iter() {
            state.apply(inst);
        }
        state["z"] == 0
    }

    fn parse(input: &'a str) -> Self {
        let program: Vec<Instruction> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                match line
                    .trim()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    [cmd, a] => Single(cmd, a),
                    [cmd, a, b] => {
                        if let Ok(b) = b.parse::<Regtype>() {
                            Direct(cmd, a, b)
                        } else {
                            Indirect(cmd, a, b)
                        }
                    }
                    _ => panic!("unknown instruction type"),
                }
            })
            .collect();
        PC { program }
    }
}

type Regtype = i64;

#[derive(Debug, Clone)]
enum Instruction<'a> {
    Single(&'a str, &'a str),
    Direct(&'a str, &'a str, Regtype),
    Indirect(&'a str, &'a str, &'a str),
}

use Instruction::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    regs: [Regtype; 4],
    input: Vec<Regtype>,
    inst: usize,
    ninputs: usize,
}

// Lower input first, then lower instruction
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.input
                .cmp(&other.input)
                .then_with(|| self.inst.cmp(&other.inst)),
        )
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.input
            .cmp(&other.input)
            .then_with(|| self.inst.cmp(&other.inst))
    }
}

impl State {
    fn new() -> Self {
        State {
            regs: [0; 4],
            input: vec![],
            inst: 0,
            ninputs: 0,
        }
    }

    fn apply(&mut self, inst: &Instruction) {
        let (cmd, rout, b) = match inst {
            Single("inp", i) => {
                self[i] = self.input[self.ninputs];
                self.ninputs += 1;
                return;
            }
            Direct(cmd, a, b) => (cmd, a, b),
            Indirect(cmd, a, b) => (cmd, a, &self[b]),
            _ => panic!("invalid instruction"),
        };

        let a = self[rout];
        let out = match *cmd {
            "add" => a + b,
            "mul" => a * b,
            "div" => a / b,
            "mod" => a % b,
            "eql" => {
                if a == *b {
                    1
                } else {
                    0
                }
            }
            _ => panic!("invalid command"),
        };
        self[rout] = out;
    }
}

impl Index<&str> for State {
    type Output = Regtype;
    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "w" => &self.regs[0],
            "x" => &self.regs[1],
            "y" => &self.regs[2],
            "z" => &self.regs[3],
            _ => panic!("invalid register name"),
        }
    }
}

impl IndexMut<&str> for State {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "w" => &mut self.regs[0],
            "x" => &mut self.regs[1],
            "y" => &mut self.regs[2],
            "z" => &mut self.regs[3],
            _ => panic!("invalid register name"),
        }
    }
}

/// A structure that keeps track of states that have already been visited.
/// If a state is already tracked, that means it was added through an input of higher priority.
#[derive(Clone, Debug)]
struct StateMap {
    heap: BTreeSet<State>,
    reverse: bool,
    ignored: usize,
    // Keep track of reg[w] and reg[z] (they others are zeroed out), to avoid re-computing
    seen: HashSet<(usize, (Regtype, Regtype))>,
}

impl StateMap {
    fn new(reverse: bool) -> Self {
        StateMap {
            heap: BTreeSet::new(),
            seen: HashSet::new(),
            ignored: 0,
            reverse,
        }
    }

    /// Add to the list only if the value is better than what we already have
    fn push(&mut self, state: State) {
        if self.seen.contains(&(state.inst, (state["w"], state["z"]))) {
            self.ignored += 1;
            return;
        }
        self.seen.insert((state.inst, (state["w"], state["z"])));
        self.heap.insert(state);
    }

    /// Remove the best candidate
    fn pop(&mut self) -> Option<State> {
        if self.heap.is_empty() {
            return None;
        }
        let key = if self.reverse {
            self.heap.iter().next().unwrap()
        } else {
            self.heap.iter().rev().next().unwrap()
        }
        .clone();
        self.heap.take(&key)
    }

    fn len(&mut self) -> usize {
        self.heap.len()
    }
}
