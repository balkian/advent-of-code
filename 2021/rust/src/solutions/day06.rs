use std::collections::HashMap;

pub fn part1(input: &[usize]) -> usize {
    part1_times(input, 80)
}

pub fn part2(input: &[usize]) -> usize {
    part1_times(input, 256)
}

pub fn part1_times(input: &[usize], times: usize) -> usize {
    let mut fish: HashMap<usize, usize> = HashMap::new();
    for day in input {
        *fish.entry(*day).or_insert(0) += 1;
    }

    for _ in 0..times {
        let mut nufish: HashMap<usize, usize> = HashMap::new();
        for day in fish.keys() {
            let next_day = if day > &0 { day - 1 } else { 6 };
            *nufish.entry(next_day).or_insert(0) += *fish.get(day).unwrap();
        }
        nufish.insert(8, *fish.get(&0).unwrap_or(&0));
        fish = nufish;
    }
    fish.values().sum()
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect()
}

#[test]
fn test_example() {
    let input = &parse("3,4,3,1,2");
    assert_eq!(part1_times(input, 18), 26);
    assert_eq!(part1(input), 5934);
}
