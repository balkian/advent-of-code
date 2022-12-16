use itertools::Itertools;
use regex::Regex;
use std::collections::{BTreeMap, HashSet, VecDeque};

type Edges = Vec<usize>;
type Graph = Vec<Edges>;
type Valves = Vec<usize>;

#[derive(Debug)]
pub struct Input<'a> {
    edges: Graph,
    names: Vec<&'a str>,
    flows: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State<'a, const N: usize> {
    edges: &'a Graph,
    flows: &'a Vec<usize>,
    valves: Valves,
    remaining: usize,
    total: usize,
    positions: [usize; N],
    last_move: [usize; N],
}

impl<'a, const N: usize> State<'a, N> {
    fn new(input: &'a Input, remaining: usize) -> Self {
        State {
            flows: &input.flows,
            edges: &input.edges,
            valves: vec![usize::MAX; input.names.len()],
            remaining,
            last_move: [remaining; N],
            total: 0,
            positions: [0; N],
        }
    }

    fn to_key(&self) -> (Vec<(usize, usize)>, Valves) {
        (
            self.positions
                .iter()
                .copied()
                .zip(self.last_move.iter().copied())
                .sorted()
                .collect_vec(),
            self.valves.clone(),
        )
    }

    fn max_expected(&self) -> usize {
        self.total
            + self
                .flows
                .iter()
                .enumerate()
                .filter_map(|(name, flow)| {
                    if self.valves[name] != usize::MAX {
                        None
                    } else {
                        Some(flow)
                    }
                })
                .sum::<usize>()
                * self.last_move.iter().min().copied().unwrap_or_default()
    }
}

impl<'a, const N: usize> State<'a, N> {
    fn explore<'b>(&'b self, idx: usize) -> Vec<State<'a, N>>
    where
        'a: 'b,
    {
        let mut opts = vec![];
        for (other, distance) in self.edges[self.positions[idx]].iter().enumerate() {
            // if !self.flows.contains_key(other)
            if other == self.positions[idx]
                || self.last_move[idx] <= distance + 1
                || (&self.valves)[other] != usize::MAX
            {
                continue;
            }

            let delta = self.flows[other];
            let mut out = self.clone();
            out.positions[idx] = other;
            out.last_move[idx] = self.last_move[idx] - distance - 1;
            if out.last_move[idx] < self.remaining {
                out.remaining = out.last_move[idx];
            }
            out.total += out.last_move[idx] * delta;
            out.valves[other] = out.last_move[idx];
            opts.push(out);
        }
        opts
    }
}

static ROOT: &str = "AA";
// Valve EF has flow rate=22; tunnels lead to valves FK, HT, DE
pub fn parse(input: &str) -> Input {
    let re = Regex::new(
        r"Valve (?P<valve>\w+) has flow rate=(?P<flow>\d+); tunnel[s]? lead[s]? to valve[s]? (?P<others>.*)",
    )
    .expect("wrong regex");
    let mut names = vec![ROOT];
    let mut graph: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut flows: BTreeMap<&str, usize> = BTreeMap::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let cap = re
            .captures(line)
            .unwrap_or_else(|| panic!("no match for line {line}"));
        let valve = cap.get(1).unwrap().as_str();
        let flow = cap[2].parse().unwrap();
        if flow != 0 || valve == ROOT {
            flows.insert(valve, flow);
            if valve != ROOT {
                names.push(valve);
            }
        }
        graph.insert(
            valve,
            cap.get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|valve| (valve, 1))
                .collect(),
        );
    }

    let mut out: Graph = vec![Default::default(); names.len()];

    for (idx, node) in names.iter().enumerate() {
        let mut edges: Edges = vec![usize::MAX; names.len()];
        let mut opts = VecDeque::new();
        opts.push_front((node, 0));
        while edges.iter().any(|v| *v == usize::MAX) {
            let (curr, dist) = opts.pop_front().unwrap();
            for (hop, delta) in graph.get(curr).expect("node not in the graph!") {
                let d = delta + dist;
                if let Some(pos) = names.iter().position(|n| n == hop) {
                    let v = &mut edges[pos];
                    *v = std::cmp::min(*v, d);
                }

                opts.push_back((hop, d));
            }
        }
        out[idx] = edges;
    }

    Input {
        edges: out,
        flows: names.iter().map(|name| *flows.get(name).unwrap()).collect(),
        names,
    }
}

pub fn solve<const N: usize>(input: &Input, remaining: usize) -> usize {
    let mut opts = vec![State::new(input, remaining)];
    let mut best: Option<State<N>> = None;
    let mut visited = HashSet::new();
    while let Some(state) = opts.pop() {
        let key = state.to_key();
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);
        let max = match (&best, &state) {
            (None, s) => {
                best = Some(s.clone());
                s.total
            }
            (Some(State { total: b, .. }), s) if s.total > *b => {
                let max = *b;
                best = Some(s.clone());
                max
            }
            (Some(State { total: b, .. }), _) if b > &state.max_expected() => {
                continue;
            }
            (Some(State { total: b, .. }), _) => *b,
        };
        for idx in 0..N {
            for opt in state.explore(idx) {
                if opt.max_expected() >= max {
                    opts.push(opt);
                }
            }
        }
    }
    let best = best.unwrap();
    // dbg!(best.valves);
    best.total
}

pub fn part1(input: &Input) -> usize {
    solve::<1>(input, 30)
}
pub fn part2(input: &Input) -> usize {
    solve::<2>(input, 26)
}
