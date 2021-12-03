pub fn part1(input: &[Vec<usize>]) -> usize {
    let input: Vec<_> = input.iter().collect();
    let (min, max): (Vec<_>, Vec<_>) = (0..input[0].len()).map(|ix| minmax(&input, ix)).unzip();
    it2num(min.iter()) * it2num(max.iter())
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    let input: &Vec<_> = &input.iter().collect();
    let vmax = filter(input, true);
    let vmin = filter(input, false);

    it2num(vmax) * it2num(vmin)
}

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn it2num<'a, I>(input: I) -> usize
where
    I: IntoIterator<Item = &'a usize>,
{
    input.into_iter().fold(0, |mut acc, num| {
        acc = (acc << 1) + num;
        acc
    })
}

pub fn minmax<'a>(input: &'a [&'a Vec<usize>], ix: usize) -> (usize, usize) {
    let count = input.iter().map(|v| v[ix]).sum::<usize>();
    let mid = (input.len() + 1) / 2;
    (
        if count < mid { 0 } else { 1 },
        if count >= mid { 0 } else { 1 },
    )
}

fn filter<'a, 'b>(input: &'b [&'a Vec<usize>], max: bool) -> &'a Vec<usize> {
    let mut valid: Vec<_> = input.to_vec();
    for ix in 0..valid[0].len() {
        let minmax = minmax(&valid, ix);
        let mask = if max { minmax.0 } else { minmax.1 };
        valid = valid.into_iter().filter(|v| v[ix] == mask).collect();
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
