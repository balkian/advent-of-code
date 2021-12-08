use std::collections::HashSet;

pub type Panel = HashSet<char>;
pub type Combs = Vec<Panel>;

#[derive(Debug, Clone)]
pub struct Case {
    configs: Combs,
    output: Combs,
}

pub fn parse(input: &str) -> Vec<Case> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<_> = line.split(" | ").collect();
            let configs = parts[0]
                .split_whitespace()
                .map(|t| t.chars().collect())
                .collect();
            let output = parts[1]
                .split_whitespace()
                .map(|t| t.chars().collect())
                .collect();
            Case { configs, output }
        })
        .collect()
}

fn find(case: &Case, f: impl FnMut(&&Panel) -> bool) -> &Panel {
    case.configs.iter().find(f).unwrap()
}

fn decode(case: &Case) -> usize {
    let one = find(case, |c| c.len() == 2);
    let four = find(case, |c| c.len() == 4);
    let seven = find(case, |c| c.len() == 3);
    let eight = find(case, |c| c.len() == 7);

    let nine = find(case, |c| {
        (c.len() == 6) && c.intersection(four).count() == 4
    });

    let zero = find(case, |c| {
        (c.len() == 6) && c != &nine && c.intersection(one).count() == 2
    });

    let six = find(case, |c| {
        (c.len() == 6) && c != &nine && c.intersection(zero).count() == 5
    });

    let five = find(case, |c| (c.len() == 5) && c.intersection(six).count() == 5);

    let three = find(case, |c| {
        (c.len() == 5) && c != &five && c.intersection(four).count() == 3
    });
    let two = find(case, |c| (c.len() == 5) && c != &five && c != &three);

    case.output
        .iter()
        .map(|out| match out {
            c if c == one => 1,
            c if c == two => 2,
            c if c == three => 3,
            c if c == four => 4,
            c if c == five => 5,
            c if c == six => 6,
            c if c == seven => 7,
            c if c == eight => 8,
            c if c == nine => 9,
            c if c == zero => 0,
            c => {
                panic!("unknown digit {:?}", c)
            }
        })
        .fold(0, |acc, a| acc * 10 + a)
}

pub fn part1(input: &[Case]) -> usize {
    input
        .iter()
        .flat_map(|case| case.output.iter())
        .filter(|o| [2, 3, 4, 7].contains(&o.len()))
        .count()
}

pub fn part2(input: &[Case]) -> usize {
    input.iter().map(decode).sum::<usize>()
}

#[test]
fn test_example() {
    let input = &parse(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    );
    assert_eq!(part1(input), 0);
    assert_eq!(part2(&input[0..1]), 5353);

    let input = &parse(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    );
    assert_eq!(part2(&input[0..1]), 8394);
    // assert_eq!(part2(input), 61229);
}
