use lazy_static::lazy_static;
use std::collections::HashMap;

type Sue<'a> = HashMap<&'a str, usize>;

lazy_static! {
    static ref CRIME: Sue<'static> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1)
    ]
    .into_iter()
    .collect();
}

fn agrees(sue: &Sue) -> bool {
    CRIME
        .iter()
        .all(|(k, v)| sue.get(k).map(|v2| v2 == v).unwrap_or(true))
}

fn agrees_ranges(sue: &Sue) -> bool {
    CRIME.iter().all(|(k, vt)| {
        sue.get(k)
            .map(|vi| match *k {
                "cats" | "trees" => vi > vt,
                "pomerians" | "goldfish" => vi < vt,
                _ => vt == vi,
            })
            .unwrap_or(true)
    })
}

pub fn parse(input: &str) -> Vec<Sue> {
    input
        .lines()
        .enumerate()
        .filter_map(|(ix, line)| {
            if line.is_empty() {
                return None;
            }
            let mut sue = HashMap::new();
            sue.insert("name", ix + 1);
            for arg in line.split_once(": ").unwrap().1.split(", ") {
                let mut toks = arg.split(": ");
                sue.insert(toks.next().unwrap(), toks.next().unwrap().parse().unwrap());
            }
            Some(sue)
        })
        .collect()
}
pub fn part1(input: &[Sue]) -> usize {
    *input
        .iter()
        .find(|s| agrees(s))
        .unwrap()
        .get("name")
        .unwrap()
}
pub fn part2(input: &[Sue]) -> usize {
    *input
        .iter()
        .find(|s| agrees_ranges(s))
        .unwrap()
        .get("name")
        .unwrap()
}
