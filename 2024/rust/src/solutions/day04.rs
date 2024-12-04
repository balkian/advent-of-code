pub fn parse(i: &str) -> Vec<Vec<char>> {
    i.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect()
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part1(i: &[Vec<char>]) -> usize {
    let rangey = 0isize..i.len() as isize;
    if i.is_empty() {
        return 0;
    }
    let rangex = 0isize..i[0].len() as isize;
    let mut found = 0;
    let target: &str = "XMAS";
    for dir in DIRS {
        for ix in rangey.clone() {
            for jx in rangex.clone() {
                let matched = target
                    .chars()
                    .scan((ix, jx), |pos, c| {
                        let (ix, jx) = &pos;
                        if rangey.contains(ix) && rangex.contains(jx) {
                            let matched = i[*ix as usize][*jx as usize] == c;
                            *pos = (dir.0 + ix, dir.1 + jx);
                            Some(matched)
                        } else {
                            Some(false)
                        }
                    })
                    .all(|f| f);
                if matched {
                    found += 1;
                }
            }
        }
    }
    found
}

/// Specific search for two MAS words in an X shape.
/// This could be generalized to words of odd length by splitting the word in two and iterating
/// over the two sides similar to how we did it in part1.
pub fn part2(i: &[Vec<char>]) -> usize {
    let mut found = 0;
    for (ix, row) in i.iter().enumerate().skip(1).rev().skip(1) {
        for (jx, c) in row.iter().enumerate().skip(1).rev().skip(1) {
            if c != &'A' {
                continue;
            }
            let left = (i[ix - 1][jx - 1] == 'M' && i[ix + 1][jx + 1] == 'S')
                || (i[ix - 1][jx - 1] == 'S' && i[ix + 1][jx + 1] == 'M');
            let right = (i[ix + 1][jx - 1] == 'M' && i[ix - 1][jx + 1] == 'S')
                || (i[ix + 1][jx - 1] == 'S' && i[ix - 1][jx + 1] == 'M');
            if left && right {
                found += 1;
            }
        }
    }
    found
}
