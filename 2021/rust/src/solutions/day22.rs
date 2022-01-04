use sscanf::scanf;
use std::ops::Range;

type Range1 = Range<isize>;

type Xyz = (Range1, Range1, Range1);

type Rule = (bool, Xyz);

/// This assumes that all rules that affect the [-50,50] interval
/// are contained in that region, which might not be true in general.
pub fn part1(input: &[Rule]) -> isize {
    count_on(input.iter().filter(|(_, (x, y, z))| {
        x.start >= -50
            && x.end <= 51
            && y.start >= -50
            && y.end <= 51
            && z.start >= -50
            && z.end <= 51
    }))
}

pub fn part2(input: &[Rule]) -> isize {
    count_on(input.iter())
}

fn intersect(r1: &Rule, r2: &Rule) -> Option<Rule> {
    intersect1d(&r1.1 .0, &r2.1 .0).and_then(|x| {
        intersect1d(&r1.1 .1, &r2.1 .1).and_then(|y| {
            intersect1d(&r1.1 .2, &r2.1 .2)
                .map(|z| (r2.0, (x.start..x.end, y.start..y.end, z.start..z.end)))
        })
    })
}

fn intersect1d(r1: &Range1, r2: &Range1) -> Option<Range1> {
    if r1.start >= r2.end || r2.start >= r1.end {
        return None;
    }
    let s1 = if r2.start < r1.start {
        r1.start
    } else {
        r2.start
    };
    let s2 = if r2.end > r1.end { r1.end } else { r2.end };
    Some(s1..s2)
}

pub fn parse(input: &str) -> Vec<Rule> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (state, xmin, xmax, ymin, ymax, zmin, zmax) = scanf!(
                line.trim(),
                "{} x={}..{},y={}..{},z={}..{}",
                String,
                isize,
                isize,
                isize,
                isize,
                isize,
                isize
            )
            .unwrap();
            let state = if state == "on" {
                true
            } else if state == "off" {
                false
            } else {
                panic!("unknown state {}", state);
            };
            (state, (xmin..xmax + 1, ymin..ymax + 1, zmin..zmax + 1))
        })
        .rev()
        .collect()
}

pub fn count_on<'a>(rules: impl Iterator<Item = &'a Rule>) -> isize {
    rules
        .scan(Vec::<Rule>::new(), |acc, r2| {
            if !r2.0 {
                acc.push(r2.clone());
                return Some(0);
            }
            let inter: Vec<_> = acc.iter().filter_map(|r1| intersect(r1, r2)).collect();
            acc.push(r2.clone());
            let intersize = count_on(inter.iter());
            Some(sumsizes(std::iter::once(r2)) - intersize)
        })
        .sum::<isize>()
}

pub fn sumsizes<'a>(rules: impl Iterator<Item = &'a Rule>) -> isize {
    rules
        .map(|(st, r)| {
            let size: isize = ((r.0).len() * (r.1).len() * (r.2).len()) as isize;
            if *st {
                size
            } else {
                -size
            }
        })
        .sum::<isize>()
}
#[test]
fn test_intersection() {
    let inter = intersect(
        &(true, (10..13, 10..13, 10..13)),
        &(true, (11..14, 11..14, 11..14)),
    );
    dbg!(&inter);
    // Overlap of 2x2x2 = 8 light
    // Size: 3x3x3 = 27 lights
    assert_eq!(sumsizes(inter.iter()), 8);

    let inter = intersect(
        &(true, (11..14, 11..14, 11..14)),
        &(true, (10..13, 10..13, 10..13)),
    );
    dbg!(&inter);

    assert_eq!(sumsizes(inter.iter()), 8);
}

#[test]
fn test_part1_manual() {
    let inter = vec![
        (true, (10..13, 10..13, 10..13)),
        (true, (11..14, 11..14, 11..14)),
    ];
    // Overlap of 2x2x2 = 8 light
    // Size: 3x3x3 = 27 lights
    assert_eq!(part1(&inter), 2 * 27 - 8);
}

#[test]
fn test_intersection_example() {
    let input = parse(
        "on x=10..12,y=10..12,z=10..12
         on x=11..13,y=11..13,z=11..13",
    );
    assert_eq!(part1(&input), 27 * 2 - 8)
}

#[test]
fn test_example() {
    let input = parse(include_str!("../../day22.sample"));
    assert_eq!(part2(&input), 2758514936282235);
}
