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
    let mut min_x = 0;
    let mut max_x = ml;
    let mut min_y = 0;
    let mut max_y = rl;

    while (min_x <= max_x) && (min_y <= max_y) {
        let mut nmin_x = usize::MAX;
        let mut nmin_y = usize::MAX;

        let mut nmax_x = usize::MIN;
        let mut nmax_y = usize::MIN;

        let mut removed = 0;

        for i in min_x..max_x {
            for j in min_y..max_y {
                let Some(count) = counts[i][j] else {
                    continue;
                };
                if count < 4 {
                    counts[i][j] = None;
                    removed += 1;

                    for (nx, ny) in neighbors((i, j), (ml, rl)) {
                        match counts[nx][ny].as_mut() {
                            Some(count @ 1..) => {
                                *count -= 1;
                                nmin_x = nmin_x.min(nx);
                                nmin_y = nmin_y.min(ny);

                                nmax_x = nmax_x.max(nx + 1);
                                nmax_y = nmax_y.max(ny + 1);
                            }
                            Some(0) => {
                                panic!("This should never happen");
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
        if removed == 0 {
            break;
        }
        total += removed;
        max_x = nmax_x;
        max_y = nmax_y;
        min_x = nmin_x;
        min_y = nmin_y;
    }
    total
}
