use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Blueprint {
    blueprint_id: usize,
    costs: [[usize; 4]; 4],
}

#[derive(Clone, Debug, Eq, Hash)]
struct State<'a> {
    blueprint: &'a Blueprint,
    remaining: usize,
    robots: [usize; 4],
    pending_robots: [usize; 4],
    stock: [usize; 4],
}

impl<'a> State<'a> {
    fn estimate(&self) -> usize {
        self.stock[3] + self.robots[3] * (self.remaining) + (self.remaining).pow(2) / 2
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .remaining
            .cmp(&self.remaining)
            .then_with(|| self.estimate().cmp(&other.estimate()))
        // self.estimate(32).cmp(&other.estimate(32))
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.remaining == other.remaining && self.estimate() == other.estimate()
    }
}

impl<'a> State<'a> {
    fn from_blueprint(blueprint: &'a Blueprint, remaining: usize) -> Self {
        State {
            blueprint,
            remaining,
            robots: [1, 0, 0, 0],
            pending_robots: [0; 4],
            stock: [0; 4],
        }
    }

    fn tick(&mut self) {
        self.remaining -= 1;
        self.robots
            .iter()
            .zip(self.stock.iter_mut())
            .for_each(|(r, s)| {
                *s += r;
            });
        self.robots
            .iter_mut()
            .zip(self.pending_robots.iter_mut())
            .for_each(|(r, pr)| {
                *r += *pr;
                *pr = 0;
            });
    }

    fn explore(&self) -> Vec<Self> {
        let mut opts = Vec::with_capacity(5);
        // opts.push(self.clone());

        // opts.push(self.clone());
        let opt = self;
        let mut could_buy = 0;
        for idx in 0..4 {
            if opt
                .stock
                .iter()
                .zip(opt.blueprint.costs[idx].iter())
                .all(|(a, b)| a >= b)
            {
                could_buy += 1; //idx+1;
                let mut new = opt.clone();
                new.stock
                    .iter_mut()
                    .zip(opt.blueprint.costs[idx].iter())
                    .for_each(|(s, c)| {
                        *s -= c;
                    });
                new.pending_robots[idx] += 1;
                opts.push(new);
            }
        }
        if could_buy < 4 {
            opts.push(self.clone());
        }
        opts.iter_mut().for_each(|b| b.tick());

        opts
    }
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (?P<bp>\d+): Each ore robot costs (?P<ore_cost>\d+) ore. Each clay robot costs (?P<clay_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_cost_ore>\d+) ore and (?P<obsidian_cost_clay>\d+) clay. Each geode robot costs (?P<geode_cost_ore>\d+) ore and (?P<geode_cost_obsidian>\d+) obsidian.").unwrap();
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let cap = re.captures(l.trim()).unwrap();

            Blueprint {
                blueprint_id: cap["bp"].parse().unwrap(),
                costs: [
                    [cap["ore_cost"].parse().unwrap(), 0, 0, 0],
                    [cap["clay_cost"].parse().unwrap(), 0, 0, 0],
                    [
                        cap["obsidian_cost_ore"].parse().unwrap(),
                        cap["obsidian_cost_clay"].parse().unwrap(),
                        0,
                        0,
                    ],
                    [
                        cap["geode_cost_ore"].parse().unwrap(),
                        0,
                        cap["geode_cost_obsidian"].parse().unwrap(),
                        0,
                    ],
                ],
            }
        })
        .collect()
}

pub fn solve(blueprints: &[Blueprint], limit: usize) -> Vec<(usize, usize)> {
    // dbg!(&blueprints);

    blueprints
        .iter()
        .map(|b| {
            let mut opts = BinaryHeap::new();
            let mut visited = HashSet::new();
            let mut max_score = 0;

            let root = State::from_blueprint(b, limit);
            let mut max = root.clone();
            opts.push(root);

            // dbg!(b.blueprint_id);
            while let Some(next) = opts.pop() {
                let estimate = next.estimate();
                if estimate <= max_score || visited.contains(&next) {
                    // if estimate <= max_score {
                    continue;
                }
                visited.insert(next.clone());

                if next.remaining > 0 {
                    opts.extend(
                        next.explore(), // .into_iter()
                                        // .filter(|b| max_score == 0 || b.estimate() >= max_score),
                    );
                    continue;
                }
                // println!("FINISHED ONE {estimate}");
                max_score = next.stock[3];
                max = next;
            }
            max
        })
        .map(|b| (b.blueprint.blueprint_id, b.stock[3]))
        .collect()
}

pub fn part1(blueprints: &[Blueprint]) -> usize {
    solve(blueprints, 24)
        .iter()
        .map(|(id, max)| id * max)
        .sum::<usize>()
}

pub fn part2(blueprints: &[Blueprint]) -> usize {
    solve(&blueprints[0..3], 32)
        .iter()
        .map(|(_id, max)| max)
        .product::<usize>()
}
