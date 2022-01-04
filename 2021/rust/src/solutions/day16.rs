use crate::aoc_sample;

pub fn part1(input: &Packet) -> usize {
    input.versionsum()
}
pub fn part2(input: &Packet) -> usize {
    input.calculate()
}

#[derive(Debug, Clone)]
pub enum Packet {
    Binary(usize, usize),
    Operator(usize, usize, Vec<Packet>),
}

#[derive(Debug, Clone)]
enum Stop {
    Nbits(usize),
    Npackets(usize),
    End,
}

use Packet::*;

impl Packet {
    fn push(&mut self, pkt: Packet) {
        match self {
            Packet::Binary(_, _) => panic!("trying to add to a binary packet"),
            Packet::Operator(_, _, v) => v.push(pkt),
        }
    }
    fn versionsum(&self) -> usize {
        match self {
            Packet::Binary(version, _) => *version,
            Packet::Operator(version, _, v) => {
                *version + v.iter().map(|p| p.versionsum()).sum::<usize>()
            }
        }
    }
    fn calculate(&self) -> usize {
        match self {
            Packet::Binary(_, num) => *num,
            Packet::Operator(_, otype, v) => match *otype {
                0 => v.iter().map(Packet::calculate).sum::<usize>(),
                1 => v.iter().map(Packet::calculate).product::<usize>(),
                2 => v.iter().map(Packet::calculate).min().unwrap(),
                3 => v.iter().map(Packet::calculate).max().unwrap(),
                5 => (v[0].calculate() > v[1].calculate()) as usize,
                6 => (v[0].calculate() < v[1].calculate()) as usize,
                7 => (v[0].calculate() == v[1].calculate()) as usize,
                _ => panic!("unknown operator"),
            },
        }
    }
    fn len(&self) -> usize {
        match self {
            Packet::Binary(_, _) => panic!("trying to get the len of a binary packet"),
            Packet::Operator(_, _, v) => v.len(),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    current: Option<Packet>,
    stop: Stop,
}

/// I'm forcing myself to do it iteratively. A recursive implementation would be
/// much cleaner.
pub fn parse(input: &str) -> Packet {
    let mut input: String = input
        .trim()
        .chars()
        .flat_map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<_>>()
        })
        .collect();

    let mut stack: Vec<State> = vec![];
    let mut state = State {
        current: None,
        stop: Stop::End,
    };

    loop {
        if match state.stop {
            Stop::End => input.is_empty() || input.len() < 7,
            Stop::Nbits(until) if until > input.len() => {
                panic!("consumed more bits than required")
            }
            Stop::Npackets(until) if until < state.current.as_ref().unwrap().len() => {
                panic!("consumed more subpackets than required")
            }
            Stop::Npackets(until) => state.current.as_ref().unwrap().len() == until,
            Stop::Nbits(until) => until == input.len(),
        } {
            if let Some(mut st) = stack.pop() {
                if st.current.is_none() {
                    break;
                }
                let read = state.current.unwrap();
                st.current.as_mut().unwrap().push(read);
                state = st;
                continue;
            } else {
                break;
            }
        }
        let pversion: String = input.drain(..3).collect();
        let ptype: String = input.drain(..3).collect();
        let pversion = usize::from_str_radix(&pversion, 2).unwrap();
        let ptype = usize::from_str_radix(&ptype, 2).unwrap();

        if ptype == 4 {
            let mut payload = String::from("");
            loop {
                let s: Vec<char> = input.drain(..5).collect();
                payload.extend(&s[1..]);
                if s[0] == '0' {
                    break;
                }
            }
            let payload = usize::from_str_radix(&payload, 2).unwrap();
            let bin = Binary(pversion, payload);
            if let Some(st) = state.current.as_mut() {
                st.push(bin);
            } else {
                state.current = Some(bin);
                break;
            }
        } else {
            stack.push(state);
            let stop = if input.remove(0) == '0' {
                let delta =
                    usize::from_str_radix(&input.drain(..15).collect::<String>(), 2).unwrap();
                Stop::Nbits(input.len() - delta)
            } else {
                let delta =
                    usize::from_str_radix(&input.drain(..11).collect::<String>(), 2).unwrap();
                Stop::Npackets(delta)
            };
            state = State {
                current: Some(Operator(pversion, ptype, vec![])),
                stop,
            };
        }
    }
    debug_assert_eq!(stack.len(), 0);
    state.current.unwrap()
}

aoc_sample!(day16sample1part1, "../../day16.sample1", part1, 16);
aoc_sample!(day16sample4part1, "../../day16.sample4", part1, 31);
