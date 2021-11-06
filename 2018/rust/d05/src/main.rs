use std::cmp;
use std::collections::HashSet;
use std::fs;

fn solve2(input: &str) -> usize {
    let units: HashSet<char> = input.to_lowercase().chars().collect();

    let mut shortest = usize::MAX;

    for unit in units {
        let removed = input
            .replace(unit, "")
            .replace(unit.to_ascii_uppercase(), "");
        let length = solve1(&removed);
        shortest = cmp::min(shortest, length);
    }
    shortest
}

fn solve1(input: &str) -> usize {
    let mut result: Vec<char> = input.trim().chars().collect();
    loop {
        let mut ix = 0;
        let mut next_string = vec![];
        let s = result.len();

        while ix < result.len() {
            let c = result[ix];
            if ix == result.len() - 1 {
                next_string.push(c);
                break;
            }

            let n = result[ix + 1];

            let reaction = if c.is_uppercase() {
                c.to_ascii_lowercase() == n
            } else {
                c.to_ascii_uppercase() == n
            };
            if reaction {
                ix += 1;
            } else {
                next_string.push(c);
            }
            ix += 1;
        }

        result = next_string;
        if s == result.len() {
            break;
        }
    }
    result.len()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

#[test]
fn test_first() {
    assert_eq!(solve1("dabAcCaCBAcCcaDA"), 10);
}

#[test]
fn test_simple() {
    assert_eq!(solve1("cC"), 0);
    assert_eq!(solve1("Cc"), 0);
    assert_eq!(solve1("aCcA"), 0);
    assert_eq!(solve1("ACca"), 0);
    assert_eq!(solve1("aaAcCaAA"), 0);
    assert_eq!(solve1("acCa"), 2);
    assert_eq!(solve1("aCca"), 2);
    assert_eq!(solve1("ACcA"), 2);
}

#[test]
fn test_shortest() {
    assert_eq!(solve2("dabAcCaCBAcCcaDA"), 4);
}
