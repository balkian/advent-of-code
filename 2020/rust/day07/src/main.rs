use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

type BagCounter = HashMap<String, Vec<(usize, String)>>;
type Fits = HashMap<String, Vec<String>>;

lazy_static! {
    static ref OUTER: Regex =
        Regex::new(r"(?P<bag>[\w\s]+) bag[s]? contain (?P<others>.*).").unwrap();
    static ref INNER: Regex = Regex::new(r"(?P<number>\d+) (?P<bag>[\s\w]+) bag[s]?").unwrap();
}

fn calculate(fits: &Fits) {
    let mut opts: Vec<&String> = fits
        .get("shiny gold")
        .expect("Shiny gold is not contained in any other bag")
        .iter()
        .collect();

    // We have processed 0..ix  options
    let mut idx = 0;

    while idx < opts.len() {
        if let Some(others) = fits.get(opts[idx]) {
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

fn calculate2(counter: &BagCounter) {
    let target = &"shiny gold".to_string();
    let mut missing: VecDeque<&String> = VecDeque::new();
    missing.push_front(target);

    let mut calculated = HashMap::<&String, usize>::new();

    while let Some(candidate) = missing.pop_front() {
        let mut count = 0;
        let mut found = true;
        if let Some(next) = counter.get(candidate) {
            for (times, inner) in next {
                match calculated.get(inner) {
                    Some(num) => count += times * num,
                    None => {
                        found = false;
                        if !missing.contains(&inner) {
                            missing.push_back(&inner);
                        }
                    }
                }
            }
        };
        if found {
            calculated.insert(candidate, count + 1);
        } else {
            missing.push_back(candidate);
        }
    }
    println!("Solution part2: {:}", calculated.get(target).unwrap() - 1);
}

fn main() {
    let mut fits = Fits::new();
    let mut contains = BagCounter::new();

    for line in aoc_utils::file_iter() {
        let caps = OUTER.captures(line.as_str()).unwrap();
        let outer = caps.name("bag").unwrap().as_str().to_string();
        let others_str = caps.name("others").unwrap().as_str();
        for i in INNER.captures_iter(others_str) {
            let num: usize = i.name("number").unwrap().as_str().parse().unwrap();
            let inner = i.name("bag").unwrap().as_str().to_string();

            fits.entry(inner.clone())
                .or_default()
                .push(outer.clone());

            contains
                .entry(outer.clone())
                .or_default()
                .push((num, inner));
        }
    }
    calculate(&fits);
    calculate2(&contains);
}
