use std::collections::HashMap;
pub struct Input<'a> {
    template: &'a str,
    rules: Vec<((char, char), char)>,
}

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let template = lines.by_ref().next().unwrap();
    let rules = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (chain, res) = line.split_once(" -> ").unwrap();
            (
                (chain.chars().next().unwrap(), chain.chars().nth(1).unwrap()),
                res.chars().next().unwrap(),
            )
        })
        .collect();
    Input { rules, template }
}

pub fn part1(input: &Input) -> usize {
    part(input, 10)
}

pub fn part2(input: &Input) -> usize {
    part(input, 40)
}

pub fn part(input: &Input, iters: usize) -> usize {
    let polymer: Vec<char> = input.template.chars().collect();
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    polymer
        .windows(2)
        .for_each(|p| *pairs.entry((p[0], p[1])).or_default() += 1);
    for _i in 0..iters {
        let mut new_pairs = pairs.clone();
        for (pair, old) in pairs.iter() {
            if let Some((_, res)) = input.rules.iter().find(|(cs, _res)| cs == pair) {
                *new_pairs.entry(*pair).or_default() -= old;
                *new_pairs.entry((pair.0, *res)).or_default() += old;
                *new_pairs.entry((*res, pair.1)).or_default() += old;
            }
        }
        pairs = new_pairs;
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    pairs.iter().for_each(|((c1, c2), count)| {
        *counts.entry(*c1).or_default() += count;
        *counts.entry(*c2).or_default() += count;
    });
    *counts.get_mut(&polymer[0]).unwrap() += 1;
    *counts.get_mut(&polymer[polymer.len() - 1]).unwrap() += 1;
    let most = counts.iter().max_by_key(|(_, v)| *v).unwrap().1;
    let least = counts.iter().min_by_key(|(_, v)| *v).unwrap().1;
    (most - least) / 2
}

#[test]
fn test_example() {
    let input = &parse(
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
    );
    assert_eq!(part1(input), 1588);
}
