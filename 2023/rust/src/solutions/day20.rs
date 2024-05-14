use aoc_utils::lcm;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
    sequence::tuple,
    IResult,
};

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::{BTreeMap, VecDeque};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

use Pulse::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Gate<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(BTreeMap<&'a str, Pulse>),
    /// A Gate that wraps around another Gate and records whenever it receives
    /// a given type of Pulse.
    Breaker(Pulse, Rc<RefCell<bool>>, Box<Gate<'a>>),
}

use Gate::*;

impl<'a> Gate<'a> {
    fn receive(&mut self, input: Pulse, gate: &'a str) -> Option<Pulse> {
        match self {
            Broadcaster => Some(input),
            FlipFlop(_) if input == High => None,
            FlipFlop(ref mut state) if *state => {
                *state = false;
                Some(Low)
            }
            FlipFlop(ref mut state) if !*state => {
                *state = true;
                Some(High)
            }
            Breaker(target, ref mut state, ref mut proxy) => {
                if *target == input {
                    state.replace(true);
                }
                proxy.receive(input, gate)
            }
            Conjunction(ref mut memory) => {
                memory.insert(gate, input);
                if memory.values().all(|p| *p == High) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Circuit<'a> {
    gates: BTreeMap<&'a str, Gate<'a>>,
    outputs: BTreeMap<&'a str, Vec<&'a str>>,
}

impl<'a> Circuit<'a> {
    fn push_button(&mut self) -> (usize, usize, bool) {
        let mut low_count = 0;
        let mut high_count = 0;
        let mut low_rx = false;

        let mut pulses = VecDeque::from(vec![("button", "broadcaster", Low)]);
        // println!();

        while let Some((from, name, pulse)) = pulses.pop_front() {
            // println!("{from} -{pulse:?}-> {name}");
            if pulse == High {
                high_count += 1;
            } else {
                if name == "rx" {
                    low_rx = true;
                }
                low_count += 1;
            }
            let Some(gate) = self.gates.get_mut(name) else {
                continue;
            };
            let pulse = gate.receive(pulse, from);
            if let Some(pulse) = pulse {
                if let Some(outs) = self.outputs.get_mut(name) {
                    for out in outs {
                        pulses.push_back((name, *out, pulse));
                    }
                }
            }
        }
        (low_count, high_count, low_rx)
    }
}

fn line(input: &str) -> IResult<&str, (&str, Gate, Vec<&str>)> {
    let (input, ((gtype, gname), outputs)) = separated_pair(
        tuple((opt(alt((tag("&"), tag("%")))), alpha1)),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(input)?;
    let gate = match gtype {
        None if gname == "broadcaster" => Broadcaster,
        Some("%") => FlipFlop(false),
        Some("&") => Conjunction(Default::default()),
        _ => panic!("invalid gate type {gtype:?}"),
    };
    Ok((input, (gname, gate, outputs)))
}

pub fn parse(input: &str) -> Circuit<'_> {
    let input = input.trim();
    let (_rest, lines) = separated_list1(newline, line)(input).expect("empty description");
    let mut gates: BTreeMap<_, _> = Default::default();
    let mut outputs: BTreeMap<_, Vec<_>> = Default::default();
    for (name, gate, out) in lines {
        outputs.entry(name).or_default().extend(out);
        gates.insert(name, gate);
    }
    for (name, gate) in gates.iter_mut() {
        if let Conjunction(state) = gate {
            for (inname, outs) in outputs.iter() {
                if outs.contains(name) {
                    state.insert(inname, Low);
                }
            }
        }
    }
    Circuit { gates, outputs }
}

pub fn part1(input: &Circuit<'_>) -> usize {
    let mut circuit = input.clone();
    let (lc, hc) = (0..1000).fold((0, 0), |acc, _| {
        let (lc, hc, _) = circuit.push_button();
        (acc.0 + lc, acc.1 + hc)
    });
    lc * hc
}

pub fn part2(input: &Circuit<'_>) -> usize {
    let mut circuit = input.clone();

    let mut target = &"rx";

    let watchlist = loop {
        let mut adjacent = vec![];
        for (name, outputs) in circuit.outputs.iter() {
            if outputs.contains(target) {
                adjacent.push(name);
            }
        }
        match adjacent.len() {
            0 => panic!("no dependencies found"),
            1 => {
                target = adjacent.pop().unwrap();
            }
            _a => {
                break adjacent;
            }
        }
    };

    let mut signals = Vec::with_capacity(watchlist.len());

    for name in watchlist.iter() {
        let original = circuit.gates.remove(*name).expect("unknown gate");
        let signal = Rc::new(RefCell::new(false));
        signals.push(signal.clone());
        let proxy = Breaker(Low, signal, Box::new(original));
        circuit.gates.insert(name, proxy);
    }

    let mut cycles = vec![0; watchlist.len()];

    for times in 1.. {
        circuit.push_button();
        for (ix, signal) in signals.iter().enumerate() {
            if *signal.as_ref().borrow() && cycles[ix] == 0 && times > 1 {
                cycles[ix] = times;
            }
        }
        if cycles.iter().all(|i| *i > 0) {
            break;
        }
    }
    cycles.into_iter().fold(1, lcm)

    // As an alternative:
    //
    // Generate network to visually analyze the graph.
    //
    // let filtered = ["broadcaster", "kt", "mt", "zp", "rc", "hj", "vc", "jv", "hf", "nm", "dh", "mc", "lv", "tg", "output"];
    // circuit.outputs.retain(|k, v| filtered.contains(k));
    // for (i, outs) in circuit.outputs.iter() {
    //     for out in outs {
    //         println!("{i}>{out}");
    //     }
    // }
    // The graph shows 4 distinct structures that
    // 243221023462303
}
