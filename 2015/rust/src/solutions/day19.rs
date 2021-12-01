use std::collections::{BinaryHeap, HashSet};
type Molecule = String;
type RefMolecule<'a> = &'a str;
type Reaction = (String, String);

type Input = (Vec<Reaction>, Molecule);

pub fn parse(input: &str) -> (Vec<Reaction>, Molecule) {
    let it = &mut input.lines();
    let reactions = it
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let toks: Vec<&str> = line.split(" => ").collect();
            (toks[0].to_string(), toks[1].to_string())
        })
        .collect();

    (reactions, it.next().unwrap().to_string())
}

fn combinations(rules: &[Reaction], input: RefMolecule) -> HashSet<String> {
    rules
        .iter()
        .flat_map(|(from, to)| {
            input.match_indices(from).map(|(idx, chunk)| {
                let mut result = input.to_string();
                result.replace_range(idx..(idx + chunk.len()), to);
                result
            })
        })
        .collect::<HashSet<String>>()
}

fn reverse_rules(rules: &[Reaction]) -> Vec<Reaction> {
    rules.iter().map(|(a, b)| (b.clone(), a.clone())).collect()
}

pub fn part1((rules, target): &Input) -> usize {
    combinations(rules, target).len()
}

pub fn part2((rules, target): &Input) -> usize {
    let mut options: BinaryHeap<(usize, String)> = BinaryHeap::new();
    let mut visited: HashSet<String> = HashSet::new();

    let rules = &reverse_rules(rules);

    let input = target;
    options.push((0, input.clone()));

    let target = String::from("e");

    while let Some(opt) = options.pop() {
        let i = opt.0 + 1;
        let input = opt.1;

        for c in combinations(rules, &input) {
            if c == target {
                return i;
            }
            if c.len() > input.len() {
                continue;
            }
            if visited.insert(c.clone()) {
                options.push((i, c));
            }
        }
    }
    unreachable!("could not find a solution");
}

#[test]
fn test_example() {
    let input = parse(
        "H => HO
H => OH
O => HH

HOH",
    );
    assert_eq!(part1(&input), 4);
}

#[test]
fn test_example2() {
    let input = parse(
        "e => H
e => O
H => HO
H => OH
O => HH

HOH",
    );
    assert_eq!(part2(&input), 3);
}
