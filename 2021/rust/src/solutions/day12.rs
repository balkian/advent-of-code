use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
    Start,
    End,
}

use Cave::*;

impl<'a> From<&'a str> for Cave<'a> {
    fn from(s: &'a str) -> Self {
        match s.chars().all(char::is_lowercase) {
            true if s == "start" => Cave::Start,
            true if s == "end" => Cave::End,
            true => Cave::Small(s),
            false => Cave::Big(s),
        }
    }
}

type Input<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;
pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(k, v)| (k.into(), v.into()))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<Cave, Vec<Cave>>, (k, v): (Cave, Cave)| {
                acc.entry(k.clone()).or_default().push(v.clone());
                acc.entry(v).or_default().push(k);
                acc
            },
        )
}

#[allow(clippy::nonminimal_bool)]
pub fn part(input: &Input, twice: bool) -> usize {
    let mut finished = vec![];
    let mut paths = vec![vec![Start]];
    while let Some(path) = paths.pop() {
        let last = path.last().unwrap();
        if *last == End {
            finished.push(path);
            continue;
        }
        let mut small: Vec<_> = path.iter().filter(|n| matches!(n, Small(_))).collect();
        // check every possible new path
        let allowed = twice && {
            let num_small = small.len();
            small.sort();
            small.dedup();
            debug_assert!(small.len().saturating_sub(num_small) <= 1);
            num_small == small.len()
        };

        for opt in input.get(last).unwrap().iter() {
            match opt {
                Small(_) if small.contains(&opt) => {
                    if !allowed {
                        continue;
                    }
                }
                Small(_) => {}
                Start => continue,
                _ => {}
            };
            let mut np = path.clone();
            np.push(opt.clone());
            paths.push(np);
        }
    }
    finished.len()
}
pub fn part1(input: &Input) -> usize {
    part(input, false)
}

pub fn part2(input: &Input) -> usize {
    part(input, true)
}

#[allow(dead_code)]
fn print(input: &[Cave]) {
    for c in input {
        let pp = match c {
            Start => "start",
            End => "end",
            Small(p) | Big(p) => p,
        };
        print!("{} -> ", pp);
    }
    println!();
}

#[test]
fn test_example_short() {
    let input = &parse(
        "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    );
    assert_eq!(part1(input), 10);
    assert_eq!(part2(input), 36);
}
#[test]
fn test_example() {
    let input = &parse(
        "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    );
    assert_eq!(part1(input), 19);
    assert_eq!(part2(input), 103);
}
