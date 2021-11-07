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

fn algo2(input: &str, nworkers: usize, offset: usize) -> (String, usize) {
    let mut order = String::new();
    let mut reqs = parse(input);
    let mut missing: Vec<char> = reqs
        .iter()
        .flat_map(|tup| once(tup.0).chain(once(tup.1)))
        .collect();
    missing.sort_unstable();
    missing.dedup();

    let mut workers = vec![State::Idle; nworkers];
    let mut clock = 0;
    let offset = ('A' as usize) - 1 - offset;

    'outer: while !missing.is_empty() {
        if let Some(worker) = workers
            .iter_mut()
            .filter(|x| matches!(x, State::Idle))
            .last()
        {
            let mut ix = 0;
            while ix < missing.len() {
                let t = *missing.get(ix).unwrap();
                if !reqs.iter().any(|r| r.1 == t) {
                    missing.remove(ix);
                    let when =
                        clock + (t.to_ascii_uppercase() as usize) - offset;
                    *worker = State::Working(when, t);
                    workers.sort_by_key(|s| match s {
                        State::Idle => 0,
                        State::Working(time, _) => -(*time as isize),
                    });
                    continue 'outer;
                }
                ix += 1;
            }
        }
        let last = workers
            .iter_mut()
            .filter(|x| matches!(x, State::Working(_, _)))
            .last()
            .unwrap();
        if let State::Working(when, task) = last {
            clock = *when;
            let mut i = 0;
            while i < reqs.len() {
                if reqs[i].0 == *task {
                    reqs.remove(i);
                } else {
                    i += 1;
                }
            }
            order.push(*task);
        }
        *last = State::Idle;
    }
    for worker in workers.into_iter().rev() {
        if let State::Working(time, t) = worker {
            clock = time;
            order.push(t);
        }
    }
    (order, clock)
}

fn solve1(input: &str) -> String {
    algo2(input, 1, 60).0
}

fn solve2(input: &str, nworkers: usize, offset: usize) -> usize {
    algo2(input, nworkers, offset).1
}

#[test]
fn test_example1() {
    assert_eq!(solve1(include_str!("../example")), "CABDFE");
}
#[test]
fn test_example2() {
    assert_eq!(solve2(include_str!("../example"), 2, 0), 15);
}
