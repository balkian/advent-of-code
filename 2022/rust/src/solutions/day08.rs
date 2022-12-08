pub fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &[Vec<isize>]) -> usize {
    let mut visible: Vec<Vec<bool>> = vec![vec![false; input.len()]; input.len()];
    let mut max_v = vec![-1; input.len()];
    let mut max_v_rev = vec![-1; input.len()];
    for (ix, row) in input.iter().enumerate() {
        let mut max = -1;
        for (jx, &cell) in row.iter().enumerate() {
            if cell > max {
                visible[ix][jx] = true;
                max = cell;
            }
            if cell > max_v[jx] {
                visible[ix][jx] = true;
                max_v[jx] = cell;
            }
        }
        max = -1;
        for (jx, &cell) in row.iter().enumerate().rev() {
            if cell > max {
                visible[ix][jx] = true;
                max = cell;
            }
        }
    }
    for (ix, row) in input.iter().enumerate().rev() {
        for (jx, &cell) in row.iter().enumerate() {
            if cell > max_v_rev[jx] {
                visible[ix][jx] = true;
                max_v_rev[jx] = cell;
            }
        }
    }
    visible.iter().flatten().filter(|c| **c).count()
}

pub fn part2(input: &[Vec<isize>]) -> usize {
    let mut score = vec![vec![0; input.len()]; input.len()];
    for (i, row) in input.iter().enumerate() {
        for (j, &tree) in row.iter().enumerate() {
            let count_x = (0..i)
                .rev()
                .enumerate()
                .find(|&(_x, di)| input[di][j] >= tree)
                .map(|(ix, _)| ix + 1)
                .unwrap_or(i);
            let count_x_rev = ((i + 1)..input.len())
                .enumerate()
                .find(|&(_ix, di)| input[di][j] >= tree)
                .map(|(ix, _)| ix + 1)
                .unwrap_or(input.len() - i - 1);
            let count_y = (0..j)
                .rev()
                .enumerate()
                .find(|&(_, dj)| input[i][dj] >= tree)
                .map(|(ix, _)| ix + 1)
                .unwrap_or(j);
            let count_y_rev = (j + 1..row.len())
                .enumerate()
                .find(|&(_, dj)| input[i][dj] >= tree)
                .map(|(ix, _)| ix + 1)
                .unwrap_or(row.len() - j - 1);
            score[i][j] = count_x * count_x_rev * count_y * count_y_rev;
        }
    }
    score
        .into_iter()
        .flat_map(|row| row.into_iter())
        .max()
        .unwrap_or_default()
}
