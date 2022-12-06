use itertools::Itertools;

type Groups<'a> = (&'a str, Vec<&'a [u8]>, Vec<&'a [u8]>);

pub fn parse(input: &str) -> Vec<Groups> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut squared: Vec<&[u8]> = vec![];
            let mut unsquared: Vec<&[u8]> = vec![];
            line.trim()
                .as_bytes()
                .iter()
                .enumerate()
                .peekable()
                .batching(|it| match it.next()? {
                    (from, b'[') => {
                        let to = it.find(|(_, &c)| c == b']').expect("unclosed bracket").0;
                        Some(&line[from..to])
                    }
                    (from, _) => {
                        let to = it
                            .peeking_take_while(|(_, &c)| c != b'[')
                            .last()
                            .expect("unterminated string")
                            .0;
                        Some(&line[from..=to])
                    }
                })
                .for_each(|w| {
                    if w.as_bytes()[0] == b'[' {
                        squared.push(&w.as_bytes()[1..w.len()]);
                    } else {
                        unsquared.push(w.as_bytes())
                    }
                });
            (line, unsquared, squared)
        })
        .collect()
}

pub fn chunks<const N: usize>(chunk: &[u8]) -> Vec<&[u8]> {
    let mid: usize = (N + 1) / 2;
    chunk
        .windows(N)
        .filter(|win| {
            !win.iter().take(mid).duplicates().any(|_x| true)
                && win.iter().take(mid).eq(win.iter().rev().take(mid))
        })
        .map(|win| &win[..mid])
        .collect()
}

pub fn part1(input: &[Groups]) -> usize {
    input
        .iter()
        .filter(|(_line, unsq, sq)| {
            unsq.iter().any(|chunk| !chunks::<4>(chunk).is_empty())
                && sq.iter().all(|chunk| chunks::<4>(chunk).is_empty())
        })
        .count()
}

pub fn part2(input: &[Groups]) -> usize {
    input
        .iter()
        .filter(|(_line, unsq, sq)| {
            let unsq = unsq.iter().flat_map(|c| chunks::<3>(c));
            let sq = sq.iter().flat_map(|c| chunks::<3>(c));
            unsq.cartesian_product(sq)
                .any(|(s, u)| s.iter().eq(u.iter().rev()))
        })
        .count()
}
