type Input = Vec<Vec<isize>>;
type InputRef<'a> = &'a [Vec<isize>];

pub fn parse(input: &str) -> Input {
    input.lines().filter(|line| !line.is_empty())
        .map(|line| {
            line.trim().split_whitespace().map(|n| n.parse::<isize>().expect("could not convert number")).collect()
        }).collect()
}

pub fn part1(input: InputRef<'_>) -> usize {
    input.iter().filter(|reg| {
        if reg.len() > 1 {
            let s = (reg[1] - reg[0]).signum();
            reg.windows(2).all(|win| {
                let diff = (win[1] - win[0]);
                (1 <= diff.abs()) & (diff.abs() <= 3) & (diff.signum() == s)
            })
        } else {
            true
        }
    }).count()
}

pub fn part2(input: InputRef<'_>) -> usize {
    todo!();
}


