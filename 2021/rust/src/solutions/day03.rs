pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn vec2number(input: &[usize]) -> usize {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(ix, val)| val << ix)
        .sum()
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    let input: Vec<&Vec<usize>> = input.iter().collect();
    let (min, max) = minmax(&input);
    vec2number(&max) * vec2number(&min)
}

pub fn minmax<'a>(input: &'a [&'a Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    let counts = input.iter().fold(vec![0; input[0].len()], |mut acc, a| {
        for ix in 0..a.len() {
            acc[ix] += a[ix] as usize;
        }
        acc
    });
    let mid = (input.len() + 1) / 2;
    let max: Vec<usize> = counts
        .iter()
        .map(|c| if *c >= mid { 1 } else { 0 })
        .collect();
    let min: Vec<usize> = counts
        .iter()
        .map(|c| if *c >= mid { 0 } else { 1 })
        .collect();

    (min, max)
}

fn filter<'a, 'b>(input: &'b [&'a Vec<usize>], max: bool) -> &'a Vec<usize> {
    let mut valid: Vec<_> = input.to_vec();
    for ix in 0..valid[0].len() {
        let minmax = minmax(&valid);
        let mask = if max { minmax.0 } else { minmax.1 };
        valid = valid.into_iter().filter(|v| v[ix] == mask[ix]).collect();
        if valid.len() == 1 {
            break;
        }
    }
    if valid.len() == 1 {
        valid[0]
    } else {
        panic!("solution not found");
    }
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    let input: &Vec<&Vec<usize>> = &input.iter().collect();
    let vmax = filter(input, true);
    let vmin = filter(input, false);

    vec2number(vmax) * vec2number(vmin)
}

#[test]
fn test_example() {
    let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(part1(&parse(input)), 198);
}

#[test]
fn test_example2() {
    let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(part2(&parse(input)), 230);
}
