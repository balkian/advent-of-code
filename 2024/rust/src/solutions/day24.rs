use std::collections::HashMap;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
enum State<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

#[derive(Debug,Clone)]
pub struct Circuit<'a> {
    states: HashMap<&'a str, State<'a>>,
    outputs: Vec<&'a str>,
}

impl<'a> Circuit<'a> {
    fn calculate(&mut self, input: &'a str) -> bool {
        let out = match self.states.get_mut(input).expect("invalid input").clone() {
            State::Value(b) => { return b; },
            State::And(a, b) => {
                let a = self.calculate(a);
                let b = self.calculate(b);
                a & b
            }
            State::Or(a, b) => {
                let a = self.calculate(a);
                let b = self.calculate(b);
                a || b
            }
            State::Xor(a, b) => {
                let a = self.calculate(a);
                let b = self.calculate(b);
                a ^ b
            }
        };
        self.states.insert(input, State::Value(out));
        out
    }
}

pub fn parse<'a>(i: &'a str) -> Circuit<'a> {
    let mut states: HashMap<&str, State> = Default::default();
    let mut outputs = vec![];
    let mut lines = i.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let (input, value) = line.trim().split_once(": ").expect("could not find input");
        let val = match value {
            "0" => {false},
            "1" => {true},
            _ => panic!("invalid input value"),
        };
        states.insert(input, State::Value(val));
        if input.starts_with('z') {
            outputs.push(input);
        }
    }
    for line in lines.by_ref() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let state = match tokens[1] {
            "XOR" => State::Xor(tokens[0], tokens[2]),
            "OR" => State::Or(tokens[0], tokens[2]),
            "AND" => State::And(tokens[0], tokens[2]),
            _ => panic!("invalid operation {line}"),
        };
        let output = tokens[4];
        states.insert(output, state);
        if output.starts_with('z') {
            outputs.push(output);
        }
    }
    outputs.sort();
    Circuit{states, outputs}
}

pub fn part1<'a: 'b, 'b>(i: &'b Circuit<'a>) -> usize {
    let mut circuit = i.clone();
    let out: usize = i.outputs.iter().rev().fold(0, |acc, o| {
        (acc << 1) + (circuit.calculate(o) as usize)
    });
    dbg!(&circuit.outputs);
    out
}
pub fn part2<'a: 'b, 'b>(i: &'b Circuit<'a>) -> usize {
    todo!();
}
