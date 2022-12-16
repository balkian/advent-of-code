use itertools::Itertools;
use regex::Regex;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
use std::collections::{BTreeMap, HashSet, VecDeque};

type Edges<'a> = BTreeMap<&'a str, usize>;
type Graph<'a> = BTreeMap<&'a str, Edges<'a>>;
type Valves<'a> = Vec<(&'a str, usize)>;

#[derive(Debug)]
pub struct Input<'a> {
    edges: Graph<'a>,
    flows: BTreeMap<&'a str, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State<'a, const N: usize> {
    edges: &'a Graph<'a>,
    flows: &'a BTreeMap<&'a str, usize>,
    valves: Valves<'a>,
    remaining: usize,
    total: usize,
    positions: [&'a str; N],
    last_move: [usize; N],
}

impl<'a, const N: usize> State<'a, N> {
    fn new(input: &'a Input, remaining: usize) -> Self {
        State {
            flows: &input.flows,
            edges: &input.edges,
            valves: vec![],
            remaining: remaining,
            last_move: [remaining; N],
            total: 0,
            positions: [input.edges.keys().find(|k| *k == &"AA").unwrap(); N],
        }
    }

    fn max_expected(&self) -> usize {
        self.total
            + self
                .flows
                .iter()
                .filter_map(|(name, flow)| {
                    if self.valves.iter().any(|(v, _)| v == name) {
                        None
                    } else {
                        Some(flow)
                    }
                })
                .sum::<usize>()
                * self.last_move.iter().min().copied().unwrap_or_default()
    }
}

// This was required by the BinaryHeap. But it's even faster without it.
// impl<'a> Ord for State<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.max_expected().cmp(&other.max_expected())
//     }
// }

// impl<'a> PartialOrd for State<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

impl<'a, const N: usize> State<'a, N> {
    fn explore<'b>(&'b self, idx: usize) -> Vec<State<'a, N>>
    where
        'a: 'b,
    {
        let mut opts = vec![];
        for (other, distance) in &self.edges[self.positions[idx]] {
            // if !self.flows.contains_key(other)
            if other == &self.positions[idx]
                || self.last_move[idx] <= distance + 1
                || self.valves.iter().any(|(i, _)| i == other)
            {
                continue;
            }

            let Some((_, delta)) = self.flows.iter().find(|(i, _)| i == &other) else {
                continue;
            };
            let mut out = self.clone();
            out.positions[idx] = other;
            out.last_move[idx] = self.last_move[idx] - distance - 1;
            if out.last_move[idx] < self.remaining {
                out.remaining = out.last_move[idx];
            }
            out.total += out.last_move[idx] * delta;
            out.valves.push((other, out.last_move[idx]));
            opts.push(out);
        }
        opts
    }
}

fn bfs<'a>(graph: &Graph<'a>) -> Graph<'a> {
    let mut out: Graph = Default::default();

    for node in graph.keys() {
        let mut edges: Edges = Default::default();
        let mut opts = VecDeque::new();
        opts.push_front((node, 0));
        while edges.len() < graph.len() {
            let (curr, dist) = opts.pop_front().unwrap();
            for (hop, delta) in graph.get(curr).expect("node not in the graph!") {
                let d = delta + dist;
                let v = edges.entry(hop).or_insert(usize::MAX);
                *v = std::cmp::min(*v, d);
                opts.push_back((hop, d));
            }
        }
        out.insert(node, edges);
    }
    out
}

// Valve EF has flow rate=22; tunnels lead to valves FK, HT, DE
pub fn parse(input: &str) -> Input {
    let re = Regex::new(
        r"Valve (?P<valve>\w+) has flow rate=(?P<flow>\d+); tunnel[s]? lead[s]? to valve[s]? (?P<others>.*)",
    )
    .expect("wrong regex");
    let mut edges: BTreeMap<&str, BTreeMap<_, _>> = BTreeMap::new();
    let mut flows: BTreeMap<&str, usize> = BTreeMap::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let cap = re
            .captures(line)
            .expect(&format!("no match for line {line}"));
        let valve = cap.get(1).unwrap().as_str();
        let flow = cap[2].parse().unwrap();
        let others: Vec<_> = cap.get(3).unwrap().as_str().split(", ").collect();
        if flow != 0 {
            flows.insert(valve, flow);
        }
        edges.insert(valve, others.into_iter().map(|v| (v, 1)).collect());
    }
    let edges = bfs(&edges);
    Input { edges, flows }
}

pub fn solve<const N: usize>(input: &Input, remaining: usize) -> usize {
    // let mut opts = BinaryHeap::new();
    let mut opts = vec![];

    opts.push(State::new(input, remaining));
    let mut best: Option<State<N>> = None;
    let mut visited: HashSet<(Vec<(&str, usize)>, Vec<_>)> = HashSet::new();
    while let Some(state) = opts.pop() {
        // dbg!((state.remaining, opts.len()));
        let key = (
            state
                .positions
                .iter()
                .copied()
                .zip(state.last_move.iter().copied())
                .sorted()
                .collect_vec()
                .try_into()
                .unwrap(),
            state
                .valves
                .iter()
                .map(|(i, _)| i.clone())
                .sorted()
                .collect(),
        );
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
