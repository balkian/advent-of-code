use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<name>\w+):(?: capacity (?P<capacity>-?\d+),?| durability (?P<durability>-?\d+),?| flavor (?P<flavor>-?\d+),?| texture (?P<texture>-?\d+),?| calories (?P<calories>-?\d+),?)*").unwrap();
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    qualities: [isize; 5],
}

pub fn parse(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Ingredient {
                qualities: [
                    cap.name("capacity").unwrap().as_str().parse().unwrap(),
                    cap.name("durability").unwrap().as_str().parse().unwrap(),
                    cap.name("flavor").unwrap().as_str().parse().unwrap(),
                    cap.name("texture").unwrap().as_str().parse().unwrap(),
                    cap.name("calories").unwrap().as_str().parse().unwrap(),
                ],
            }
        })
        .collect()
}

fn calculate_combs(budget: usize, remaining: usize, size: usize) -> Option<Vec<Vec<usize>>> {
    match (budget, remaining) {
        (0, 0) => Some(vec![vec![0; size]]),
        (0, _) | (_, 0) => None,
        _ => {
            let mut results = vec![];
            for i in 0..=budget {
                for mut res in calculate_combs(budget - i, remaining - 1, size)
                    .into_iter()
                    .flatten()
                {
                    res[size - remaining] = i;
                    results.push(res);
                }
            }
            Some(results)
        }
    }
}

pub fn part(input: &[Ingredient], with_calories: bool) -> usize {
    let mut max_score = 0;
    'outer: for quantities in calculate_combs(100, input.len(), input.len()).unwrap() {
        let mut qualities = [0; 5];
        for i in 0..input.len() {
            qualities[4] += input[i].qualities[4] * quantities[i] as isize;
        }
        if with_calories && qualities[4] != 500 {
            continue 'outer;
        }
        let mut score = 1;
        for (qx, qual) in qualities.iter_mut().enumerate().take(4) {
            for i in 0..input.len() {
                *qual += input[i].qualities[qx] * quantities[i] as isize;
            }
            if *qual < 0 {
                continue 'outer;
            }
            score *= *qual;
        }
        if score > max_score {
            max_score = score;
        }
    }
    max_score as usize
}

pub fn part1(input: &[Ingredient]) -> usize {
    part(input, false)
}

pub fn part2(input: &[Ingredient]) -> usize {
    part(input, true)
}
