use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref OUTER: Regex =
        Regex::new(r"(?P<bag>[\w\s]+) bag[s]? contain (?P<others>.*).").unwrap();
    static ref INNER: Regex = Regex::new(r"(?P<number>\d+) (?P<bag>[\s\w]+) bag[s]?").unwrap();
}

fn calculate(tree: &HashMap<String, Vec<String>>) {
    let mut opts: Vec<&String> = tree
        .get("shiny gold")
        .expect("Shiny gold is not contained in any other bag")
        .iter()
        .collect();
    let mut idx = 0;

    while idx < opts.len() {
        let candidate = opts[idx];

        if let Some(others) = tree.get(candidate) {
            for o in others {
                if opts.contains(&o) {
                    continue;
                }
                opts.push(&o);
            }
        }
        idx += 1;
    }
    println!("Solution to part 1: {:}", opts.len());
}

fn calculate2(contains: &HashMap<String, Vec<(usize, String)>>) {
    let target = &"shiny gold".to_string();
    let mut missing: VecDeque<&String> = VecDeque::new();
    missing.push_front(target);

    let mut calculated = HashMap::<&String, usize>::new();

    while let Some(candidate) = missing.pop_front() {
        let mut count = 0;
        let mut found = true;
        let next = match contains.get(candidate) {
            Some(c) => c,
            None => {
                calculated.insert(&candidate, 1);
                continue;
            }
        };
        for (times, inner) in next {
            match calculated.get(inner) {
                Some(num) => count += times * num,
                None => {
                    missing.push_back(&inner);
                    found = false;
                }
            }
            // dbg!{&candidate,&inner,&found,&missing};
        }
        if found {
            calculated.insert(candidate, count + 1);
        } else {
            missing.push_back(candidate);
        }
    }
    println!("Solution part2: {:}", calculated.get(target).unwrap() - 1);
}

fn main() {
    let mut deps = HashMap::<String, Vec<String>>::new();
    let mut contains = HashMap::<String, Vec<(usize, String)>>::new();
    for line in aoc_utils::file_iter() {
        let caps = OUTER.captures(line.as_str()).unwrap();
        let outer = caps.name("bag").unwrap().as_str().to_string();
        let others_str = caps.name("others").unwrap().as_str();
        // dbg!{&others_str};
        for i in INNER.captures_iter(others_str) {
            let num: usize = i.name("number").unwrap().as_str().parse().unwrap();
            let inner = i.name("bag").unwrap().as_str().to_string();

            deps.entry(inner.clone())
                .or_insert_with(Vec::new)
                .push(outer.clone());

            contains
                .entry(outer.clone())
                .or_insert_with(Vec::new)
                .push((num, inner));
        }
    }
    calculate(&deps);
    calculate2(&contains);
}
