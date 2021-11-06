use std::fs;
use std::cmp::{max,min};
use std::collections::{HashMap, HashSet};
use counter::Counter;
use std::cmp::Ordering;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input, 10000));
}

type Position = (usize, usize);

type ID = usize;


fn get_locations(input: &str) -> (Vec<Position>, Position, Position) {

    let locations: Vec<Position> = input.lines().map(|line| {
        let mut it = line.split(", ").map(|i| i.parse().unwrap());
        let x = it.next().unwrap();
        let y = it.next().unwrap();
        (x, y)
    }
    ).collect();
    
    let mins = locations.iter().fold((usize::MAX, usize::MAX), |acc, pos| {
        (min(acc.0, pos.0), min(acc.1, pos.1))
    });
    let dims = locations.iter().fold((0, 0), |acc, pos| {
        (max(acc.0, pos.0), max(acc.1, pos.1))
    });
    (locations, mins, dims)
}


/// Calculate the distance from every point to each location ID (index in locations).
/// Store both the closes location ID, the location to that ID, and the sum of distances
/// to every other location.
///
/// Return the bounds (minumum and maximum locations) as well.
fn distances(input: &str) -> (HashMap<Position, (ID, usize, usize)>, Position, Position) {
    let (locations, mins, dims) = get_locations(input);

    let mut dists: HashMap<Position, (ID, usize, usize)> = HashMap::new();
    for (ix, loc) in locations.iter().enumerate() {
        for i in mins.0..=dims.0 {
            for j in mins.0..=dims.1 {
                let dist = max(i, loc.0) + max(j, loc.1) - min(i, loc.0) - min(j, loc.1);

                if let Some(value) = dists.get_mut(&(i, j)){
                    match value.1.cmp(&dist) {
                        Ordering::Greater => *value = (ix, dist, value.2+dist),
                        Ordering::Equal => *value = (usize::MAX, dist, value.2+dist),
                        Ordering::Less => *value = (value.0, value.1, value.2+dist),
                    }
                }
                else {
                    dists.insert((i, j), (ix, dist, dist));
                }
            }
        }
    }
    (dists, mins, dims)
}

fn solve1(input: &str) -> usize {
    let (dists, mins, dims) = distances(input);


    let outer: HashSet<usize> = dists.iter().filter(|(pos, _)|
        pos.0 == dims.0 || pos.1 == dims.1 || pos.0 == mins.0 || pos.1 == mins.1
     ).map(|(_, (id, _, _))| *id).collect();

    let counter: Counter<usize> = dists.iter().map(|(_, (id, _, _))| *id).filter(|id| !outer.contains(id)).collect();
    counter.most_common()[0].1
}

fn solve2(input: &str, radius: usize) -> usize {
    let (dists, _, _) = distances(input);

    let solution2 = dists.values().filter(|(_, _, dist)| *dist < radius).count();
    solution2
}


#[test]
fn test_example(){
    assert_eq!(solve1(include_str!("../example")), 17);
    assert_eq!(solve2(include_str!("../example"), 32), 16);
}
