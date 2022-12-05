use regex::Regex;

#[derive(Debug)]
pub struct Def {
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

pub fn parse(input: &str) -> Def {
    let mut it = input.lines();
    #[allow(clippy::needless_collect)]
    let top: Vec<_> = it.by_ref().take_while(|l| !l.is_empty()).collect();
    let stacks: Vec<Vec<char>> = top
        .into_iter()
        .rev()
        .skip(1)
        .map(|line| {
            line.as_bytes()
                .chunks(4)
                .map(|c| {
                    if c[0] == b'[' {
                        Some(c[1] as char)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .fold(vec![], |mut acc, new| {
            while acc.len() < new.len() {
                acc.push(vec![]);
            }
            for (ix, col) in new.into_iter().enumerate() {
                if let Some(c) = col {
                    acc[ix].push(c);
                }
            }
            acc
        });
    let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    // let bottom = it.
    let moves = it
        .into_iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap() - 1,
                caps[3].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();
    Def { stacks, moves }
}

pub fn part1(input: &Def) -> String {
    let mut stacks = input.stacks.clone();
    // dbg!(&stacks);
    for &(qty, from, to) in input.moves.iter() {
        // dbg!(qty, from, to);
        for _i in 0..qty {
            let last = stacks[from].pop().unwrap();
            stacks[to].push(last);
        }
        // dbg!(&stacks);
    }
    stacks
        .into_iter()
        .map(|s| s.last().copied().unwrap_or('?'))
        .collect::<String>()
}

pub fn part2(input: &Def) -> String {
    let mut stacks = input.stacks.clone();
    for &(qty, from, to) in input.moves.iter() {
        let from = &mut stacks[from];
        let last = from.split_off(from.len() - qty);
        let to = &mut stacks[to];
        to.extend(last);
    }
    stacks
        .into_iter()
        .map(|s| s.last().copied().unwrap_or('?'))
        .collect::<String>()
}
