use std::collections::BTreeSet;

pub fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().map(|c| c == '@').collect())
        .collect()
}

pub fn neighbors(
    (x, y): (usize, usize),
    (lenx, leny): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let xrange = x.saturating_sub(1)..(x + 2).min(lenx);
    let yrange = y.saturating_sub(1)..(y + 2).min(leny);
    xrange
        .flat_map(move |nx| yrange.clone().map(move |ny| (nx, ny)))
        .filter(move |(nx, ny)| *nx != x || *ny != y)
}

pub fn part1(map: &[Vec<bool>]) -> usize {
    let mut total = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }
            if neighbors((i, j), (map.len(), row.len()))
                .filter(|(nx, ny)| map[*nx][*ny])
                .take(4)
                .count()
                < 4
            {
                //eprintln!("({i}, {j}) {filled}");
                total += 1;
            }
        }
    }
    total
}

pub fn part2(map: &[Vec<bool>]) -> usize {
    let mut total = 0;
    let ml = map.len();
    let rl = if ml > 0 { map[0].len() } else { 0 };
    let mut counts: Vec<Vec<_>> = map
        .iter()
        .enumerate()
        .map(|(ix, row)| {
            row.iter()
                .enumerate()
                .map(|(jx, cell)| {
                    cell.then(|| {
                        neighbors((ix, jx), (ml, rl))
                            .filter(|(nx, ny)| map[*nx][*ny])
                            .count()
                    })
                })
                .collect()
        })
        .collect();

    let mut pending: BTreeSet<(usize, usize)> = counts
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, cell)| cell.map(|_| (x, y)))
        })
        .collect();

    while let Some((i, j)) = pending.pop_first() {
        let Some(count) = counts[i][j] else {
            continue;
        };
        if count < 4 {
            counts[i][j] = None;
            total += 1;
            for (nx, ny) in neighbors((i, j), (ml, rl)) {
                match counts[nx][ny].as_mut() {
                    Some(count @ 1..) => {
                        *count -= 1;
                        pending.insert((nx, ny));
                    }
                    Some(0) => {
                        panic!("we should never get under 0");
                    }
                    _ => {}
                };
            }
        }
    }
    total
}
