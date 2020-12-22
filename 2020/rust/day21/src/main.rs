use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").unwrap();

    let (part1, part2) = solve(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn clean_dangerous<'a>(dangerous: &'a mut HashMap<&'a str, HashSet<&'a str>>) -> String {
    let mut confirmed: Vec<(&str, &str)> = Vec::new();

    while confirmed.len() != dangerous.len() {
        for (allergen, options) in dangerous.iter_mut() {
            if options.len() > 1 {
                for (_, w) in &confirmed {
                    options.remove(*w);
                }
            }

            if options.len() == 1 {
                confirmed.push((allergen, options.drain().next().unwrap()));
            }
        }
    }
    cleaned_str(&confirmed)
}

fn cleaned_str<'a>(cleaned: &'a [(&'a str, &'a str)]) -> String {
    let mut cleaned: Vec<(&str, &str)> = cleaned.iter().copied().collect();
    cleaned.sort_by(|a, b| (a.0).partial_cmp(b.0).unwrap());
    let output: String = cleaned
        .into_iter()
        .map(|(_, w)| w)
        .collect::<Vec<&str>>()
        .join(",");
    output
}

fn solve(input: &str) -> (usize, String) {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut dangerous: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split('(').collect();

        let ingredients = tokens[0].split_terminator(' ').collect();

        for allergen in tokens[1][9..tokens[1].len() - 1]
            .split_terminator(',')
            .map(|x| x.trim())
        {
            if dangerous.contains_key(allergen) {
                dangerous.insert(
                    allergen,
                    dangerous[allergen]
                        .intersection(&ingredients)
                        .copied()
                        .collect(),
                );
            } else {
                dangerous.insert(allergen, ingredients.clone());
            }
        }

        for word in ingredients {
            *counts.entry(word).or_default() += 1;
        }
    }

    let suspicious: HashSet<_> = dangerous.values().flatten().copied().collect();

    let cleaned = clean_dangerous(&mut dangerous);

    let total: usize = counts
        .iter()
        .filter(|(elem, _)| !suspicious.contains(*elem))
        .map(|(_, times)| times)
        .sum();
    (total, cleaned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (part1, part2) = solve(include_str!("../example.txt"));
        assert_eq!(part1, 5);
        assert_eq!(part2, "mxmxvkd,sqjhc,fvjkl");
    }
}
