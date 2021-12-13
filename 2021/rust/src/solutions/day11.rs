pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.chars()
                        .map(|c| c.to_string().parse().unwrap())
                        .collect(),
                )
            }
        })
        .collect()
}

fn evolve(input: &[Vec<usize>]) -> (Vec<Vec<usize>>, usize) {
    let mut ninput = input.to_vec();
    let mut flashes = 0;
    ninput.iter_mut().flatten().for_each(|c| *c += 1);
    let mut flashed = true;
    while flashed {
        flashed = false;
        for j in 0..ninput.len() {
            for i in 0..ninput[j].len() {
                if ninput[j][i] > 9 {
                    flashed = true;
                    flashes += 1;
                    ninput[j][i] = 0;
                    let j0 = j.saturating_sub(1);
                    let i0 = i.saturating_sub(1);
                    for (nj, nrow) in ninput.iter_mut().enumerate().take(j + 2).skip(j0) {
                        for (ni, ncell) in nrow.iter_mut().enumerate().take(i + 2).skip(i0) {
                            if ni == i && nj == j || *ncell == 0 {
                                continue;
                            }
                            *ncell += 1;
                        }
                    }
                }
            }
        }
    }
    (ninput, flashes)
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    let mut input = input.to_vec();
    let mut flashes = 0;
    for _ in 0..100 {
        let (ninput, nflashes) = evolve(&input);
        flashes += nflashes;
        input = ninput;
    }
    flashes
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    let mut input = input.to_vec();
    for i in 1.. {
        let (ninput, _nflashes) = evolve(&input);
        if ninput.iter().flatten().copied().sum::<usize>() == 0 {
            return i;
        }
        input = ninput;
    }
    unreachable!();
}
