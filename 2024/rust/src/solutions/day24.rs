use std::collections::{HashSet, BTreeMap};

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
enum State<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

#[derive(Debug,Clone)]
pub struct Circuit<'a> {
    states: BTreeMap<&'a str, State<'a>>,
    xs: Vec<&'a str>,
    ys: Vec<&'a str>,
    zs: Vec<&'a str>,
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
    fn bits2usize(&mut self, bits: &[&'a str]) -> usize {
        bits.into_iter().fold(0, |acc, o| {
            (acc << 1) + (self.calculate(o) as usize)
        })
    }
    fn set_x(&mut self, mut value: usize) {
        for x in self.xs.iter_mut().rev() {
            self.states.insert(x, State::Value(value % 2 == 1));
            value = value >> 1;
        }
        assert_eq!(value, 0);
    }
    fn set_y(&mut self, mut value: usize) {
        for y in self.ys.iter_mut().rev() {
            self.states.insert(y, State::Value(value % 2 == 1));
            value = value >> 1;
        }
        assert_eq!(value, 0);
    }
    fn get_z(&mut self) -> usize {
        let bits = self.zs.clone();
        self.bits2usize(&bits)
    }
    fn diff(&self, mut expected: usize) -> Vec<&'a str> {
        let mut diff = vec![];
        for i in self.zs.iter().rev() {
            let val = (expected % 2) == 1;
            expected = expected / 2;
            match self.states.get(i).unwrap() {
                State::Value(v) if *v == val => {
                    continue;
                }
                _ => {
                    diff.push(*i);
                }
            }
        }
        diff
    }
}

pub fn parse<'a>(i: &'a str) -> Circuit<'a> {
    let mut states: BTreeMap<&str, State> = Default::default();
    let mut zs = vec![];
    let mut ys = vec![];
    let mut xs = vec![];
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
            zs.push(input);
        } else if input.starts_with('x') {
            xs.push(input);
        } else if input.starts_with('y') {
            ys.push(input);
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
            zs.push(output);
        }
    }
    zs.sort();
    zs.reverse();
    xs.sort();
    xs.reverse();
    ys.sort();
    ys.reverse();
    Circuit{states, xs, zs, ys}
}


pub fn part1<'a: 'b, 'b>(i: &'b Circuit<'a>) -> usize {
    let mut circuit = i.clone();
    let out = circuit.get_z();
    out
}

fn count_defects<'a>(c: &'a Circuit) -> BTreeMap<Vec<&'a str>, Vec<(usize, usize)>> {
    let mut counts: BTreeMap<Vec<&str>, Vec<(usize, usize)>> = Default::default();
    for t in 0..=c.xs.len() {
        let t = (1 << t) / 2;
        for (x, y) in [(t, 0), (0, t), (t, t)] {
            let mut c = c.clone();
            c.set_x(x);
            c.set_y(y);
            c.get_z();
            let expected = x + y;
            let faulty = c.diff(expected);
            if !faulty.is_empty() {
                counts.entry(faulty).or_default().push((x, y));
            }
        }
    }
    counts
}

pub fn part2<'a: 'b, 'b>(input: &'b Circuit<'a>) -> String {
    let mut c = input.clone();

    let swaps = [
        ["z07", "rts"],
        ["z12", "jpj"],
        ["z26", "kgj"],
        ["chv", "vvw"] ];
    for [t1, t2] in swaps {
        let s1 = c.states.remove(t1).unwrap();
        let s2 = c.states.remove(t2).unwrap();
        c.states.insert(t1, s2);
        c.states.insert(t2, s1);
    }
    
    // I've used this code to generate a graph of dependencies for every 
    // output and a list of faulty outputs for every input pin.
    //
    // The list of faulty outputs can be used to detect which two pins are affected
    // by a given output. Then, the graph is used to select two candidate pins to 
    // swap.
    //
    // This whole process could be automated, but some pin swaps generate loops, which end
    // up crashing the program.
    let debugging = false;
    if debugging {
        let mut graph: BTreeMap<&str, Vec<&str>> = Default::default();
        for (output, state) in &c.states {
            match state {
                State::Value(_) => {},
                State::And(a, b) | State::Or(a, b) | State::Xor(a, b) => {
                    graph.entry(output).or_default().extend([a, b]);
                }
            }
        }
        let mut outgraph: BTreeMap<&str, Vec<&str>> = Default::default();
        let mut seen: HashSet<&str> = Default::default();
        for o in input.zs.iter().rev() {
            eprintln!("*** {o} {:?}", c.states.get(o).unwrap());
            let mut pending = vec![*o];
            while let Some(nxt) = pending.pop() {
                for n in graph.get(nxt).cloned().unwrap_or_default() {
                    eprintln!("{nxt} -> {n} {:?}", c.states.get(n).unwrap());
                    outgraph.entry(o).or_default().push(n);
                    if seen.contains(n) {
                        continue;
                    }
                    seen.insert(n);
                    pending.push(n);
                }
            }
        }

        let outs = count_defects(&c);

        for (out, points) in outs {
            for (x, y) in points {
                eprintln!("({x}, {y}) -> {out:?}");
            }
        }
    }
    let mut wires: Vec<_> = swaps.iter().flatten().cloned().collect();
    wires.sort();
    wires.join(",")
}
