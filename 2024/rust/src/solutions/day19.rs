use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Problem {
    tokens: Vec<String>,
    towels: Vec<String>,
}

pub fn parse(i: &str) -> Problem {
    let mut lines = i.lines();
    let tokens = lines
        .next()
        .expect("cannot find")
        .trim()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    lines.by_ref().next();
    let mut towels = vec![];
    for towel in lines {
        towels.push(towel.trim().to_string());
    }
    Problem { tokens, towels }
}

pub fn matches(i: &str, tokens: &[String]) -> bool {
    if i.is_empty() {
        return true;
    }
    for t in tokens {
        if i.starts_with(t) {
            let (_, newrem) = i.split_at(t.len());
            if matches(newrem, tokens) {
                return true;
            }
        }
    }
    false
}

pub fn count_matches<'a: 'b, 'b>(
    i: &'a str,
    tokens: &[String],
    cache: &'b mut HashMap<&'a str, usize>,
) -> usize {
    if i.is_empty() {
        return 1;
    }
    if let Some(already) = cache.get(i) {
        return *already;
    }
    let mut found = 0;
    for t in tokens {
        if i.starts_with(t) {
            let (_, newrem) = i.split_at(t.len());
            found += count_matches(newrem, tokens, cache);
        }
    }
    cache.insert(i, found);
    found
}

pub fn part1(p: &Problem) -> usize {
    let mut possible = vec![];
    for towel in &p.towels {
        if matches(towel, &p.tokens) {
            possible.push(towel);
        }
    }
    possible.len()
}

pub fn part2(p: &Problem) -> usize {
    let mut possible = 0;
    let cache = &mut HashMap::new();
    for towel in &p.towels {
        possible += count_matches(towel, &p.tokens, cache);
    }
    possible
}
