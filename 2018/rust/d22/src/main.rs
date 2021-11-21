use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Material {
    Rocky,
    Narrow,
    Wet,
}

use Material::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tool {
    Climbing,
    Torch,
    Neither,
}

use Tool::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Explorer {
    pos: (usize, usize),
    tool: Tool,
    time: usize,
    // Keep track of past cells, for debugging
    trace: Vec<(usize, usize)>,
    distance: usize,
}

fn distance(exp: (usize, usize), tool: &Tool, target: (usize, usize), ttool: &Tool) -> usize {
    let dist =
        max(exp.0, target.0) - min(exp.0, target.0) + max(exp.1, target.1) - min(exp.1, target.1);
    if tool != ttool {
        dist + 7
    } else {
        dist
    }
}

impl Ord for Explorer {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.distance + other.time)
            .cmp(&(self.distance + self.time))
            .then_with(|| other.time.cmp(&self.time))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Explorer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Material>]) {
    for row in map {
        for r in row {
            let c = match r {
                Rocky => '.',
                Wet => '=',
                Narrow => '|',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn solve(depth: usize, target: (usize, usize)) {
    let mut levels = vec![vec!(0; target.1); target.1 * 2];

    for y in 0..levels.len() {
        for x in 0..levels[y].len() {
            let level = match (x, y) {
                a if (a == (0, 0)) | (a == target) => 0,
                (0, y) => y * 48271,
                (x, 0) => x * 16807,
                (x, y) => (levels[y - 1][x] * levels[y][x - 1]) % 20183,
            };
            levels[y][x] = (level + depth) % 20183;
        }
    }
    let tiles: Vec<Vec<Material>> = levels
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| match x % 3 {
                    0 => Rocky,
                    1 => Wet,
                    2 => Narrow,
                    _ => panic!("this should never happen"),
                })
                .collect()
        })
        .collect();

    let risk: usize = tiles
        .iter()
        .take(target.1 + 1)
        .flat_map(|row| {
            row.iter().take(target.0 + 1).map(|x| match x {
                Rocky => 0,
                Wet => 1,
                Narrow => 2,
            })
        })
        .sum();
    println!("Solution 1: {}", risk);

    let mut explorers = BinaryHeap::from(vec![Explorer {
        pos: (0, 0),
        tool: Torch,
        time: 0,
        trace: vec![],
        distance: target.0 + target.1,
    }]);

    let mut distances: HashMap<((usize, usize), Tool), usize> = HashMap::new();

    let try_move = |expl: &Explorer, delta: (isize, isize)| {
        let y = delta.1 + (expl.pos.1 as isize);
        let x = delta.0 + (expl.pos.0 as isize);
        if y < 0 || x < 0 {
            return None;
        }
        let (y, x) = (y as usize, x as usize);

        match (&expl.tool, &tiles[y][x]) {
            (Neither, Rocky) | (Torch, Wet) | (Climbing, Narrow) => None,
            _ => {
                let mut new_trace = expl.trace.clone();
                new_trace.push((x, y));
                Some(Explorer {
                    pos: (x, y),
                    tool: expl.tool.clone(),
                    trace: new_trace,
                    time: expl.time + 1,
                    distance: distance((x, y), &expl.tool, target, &Torch),
                })
            }
        }
    };

    loop {
        let explorer = explorers.pop().unwrap();
        if explorer.pos == target && explorer.tool == Torch {
            println!("Solution 2: {}", explorer.time);
            break;
        }
        let key = (explorer.pos, explorer.tool.clone());
        if let Some(val) = distances.get_mut(&key) {
            if *val <= explorer.time {
                continue;
            }
            *val = explorer.time;
        } else {
            distances.insert(key, explorer.time);
        }

        for delta in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            if let Some(exp) = try_move(&explorer, delta) {
                explorers.push(exp);
            }
        }
        for tool in [Torch, Climbing, Neither] {
            if explorer.tool == tool {
                continue;
            }
            let mut new_exp = explorer.clone();
            new_exp.tool = tool;
            new_exp.time += 6;
            if let Some(exp) = try_move(&new_exp, (0, 0)) {
                explorers.push(exp);
            }
        }
    }
}

fn main() {
    let depth = 11991;
    let target = (6, 797);
    // let depth = 510;
    // let target = (10,10);
    solve(depth, target);
}
