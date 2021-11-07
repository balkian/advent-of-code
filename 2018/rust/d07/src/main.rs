use std::collections::HashSet;
use std::fs;
use std::iter::once;

fn main() {
    let input = fs::read_to_string("input").expect("file could not be read");
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input, 5, 60));
}

fn parse(input: &str) -> Vec<(char, char)> {
    let mut reqs = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let prev = line.chars().nth(5).unwrap();
        let this = line.chars().nth(36).unwrap();
        reqs.push((prev, this));
    }
    reqs
}

#[derive(Clone, Debug)]
enum State {
    Idle,
    Working(usize, char),
}

fn solve2(input: &str, nworkers: usize, offset: usize) -> usize {
    let mut order = String::new();
    let mut done: HashSet<char> = HashSet::new();
    let mut working: HashSet<char> = HashSet::new();
    let mut reqs = parse(input);
    let mut total: Vec<char> = reqs
        .iter()
        .flat_map(|tup| once(tup.0).chain(once(tup.1)))
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();
    total.sort_unstable();

    let mut workers = vec![State::Idle; nworkers];
    let mut clock = 0;

    'outer: while total.len() != done.len() {
        let mut completed = vec![];
        for (ix, req) in reqs.iter().enumerate().rev() {
            if done.contains(&req.0) {
                completed.push(ix);
            }
        }
        for c in completed {
            reqs.remove(c);
        }

        for t in total.iter() {
            if done.contains(t) || working.contains(t) {
                continue;
            }
            if !reqs.iter().any(|r| r.1 == *t) {
                let state = workers.last_mut().unwrap();
                if let State::Working(time, task) = state {
                    clock = *time;
                    done.insert(*task);
                    order.push(*task);
                }
                let when = clock + (t.to_ascii_uppercase() as usize) - ('A' as usize) + 1 + offset;

                *state = State::Working(when, *t);
                working.insert(*t);

                workers.sort_by_key(|s| match s {
                    State::Idle => 0,
                    State::Working(time, _) => -(*time as isize),
                });
                continue 'outer;
            }
        }
        let last = workers
            .iter_mut()
            .filter(|x| matches!(x, State::Working(_, _)))
            .last()
            .unwrap();
        if let State::Working(when, what) = last {
            clock = *when;
            working.remove(what);
            done.insert(*what);
            order.push(*what);
        }
        *last = State::Idle;
    }
    if let State::Working(time, _) = workers[0] {
        clock += time;
    }
    clock
}

fn solve1(input: &str) -> String {
    let mut order = String::new();
    let mut done: HashSet<char> = HashSet::new();
    let mut reqs = parse(input);
    let mut total: Vec<char> = reqs
        .iter()
        .flat_map(|tup| once(tup.0).chain(once(tup.1)))
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();
    total.sort_unstable();

    'outer: while total.len() != done.len() {
        let mut completed = vec![];
        for (ix, req) in reqs.iter().enumerate().rev() {
            if done.contains(&req.0) {
                completed.push(ix);
            }
        }
        for c in completed {
            reqs.remove(c);
        }
        for t in total.iter() {
            if done.contains(t) {
                continue;
            }
            if !reqs.iter().any(|r| r.1 == *t) {
                done.insert(*t);
                order.push(*t);
                continue 'outer;
            }
        }
    }
    order
}

#[test]
fn test_example() {
    assert_eq!(solve1(include_str!("../example")), "CABDFE");
}
#[test]
fn test_example2() {
    assert_eq!(solve2(include_str!("../example"), 2, 0), 15);
}
