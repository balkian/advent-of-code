#[derive(Debug,Clone)]
pub struct Input {
    keys: Vec<[usize; 5]>,
    locks: Vec<[usize; 5]>,
}
pub fn parse(i: &str) -> Input {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut lines = i.lines().map(|line| line.trim());
    while let Some(nxt) = lines.by_ref().next() {
        let mut current: [usize; 5] = nxt.chars()
            .map(|c| match c { '#' => 1, '.' => 0, _ => panic!("invalid char")})
            .collect::<Vec<usize>>()
            .try_into().expect("could not transform to array");
        let iskey = current == [0; 5];
        while let Some(nxt) = lines.by_ref().next() {
            if nxt.is_empty() {
                break;
            }
            for (c, v) in current.iter_mut().zip(nxt.chars()
                .map(|c| match c { '#' => 1, '.' => 0, _ => panic!("invalid char")})
            ) {
                *c += v;
            }
        }
        if iskey {
            keys.push(current);
        } else {
            locks.push(current);
        }
    }
    Input{keys, locks}
}

pub fn part1(i: &Input) -> usize {
    let mut total_pairs = 0;
    for k in &i.keys {
        for l in &i.locks {
            //dbg!(&k, &l);
            if k.iter().zip(l.iter()).all(|(a, b)| (a + b) <= 7) {
                total_pairs += 1;
            }
            
        }
    }
    total_pairs
}
pub fn part2(_i: &Input) -> usize {
    todo!();
}
