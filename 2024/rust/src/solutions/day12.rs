use std::collections::HashMap;

type Input = Vec<Vec<char>>;

pub fn parse(i: &str) -> Input {
    i.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn regions_area(input: &Input) -> (Vec<Vec<usize>>, HashMap<usize, usize>) {
    let mut regions: Vec<Vec<usize>> = vec![vec![0; input[0].len()]; input.len()];
    let mut area: HashMap<usize, usize> = Default::default();
    let mut nregions = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, target) in row.iter().enumerate() {
            if regions[i][j] != 0 {
                continue;
            }
            nregions += 1;
            let mut pending = vec![(i, j)];
            while let Some(nxt) = pending.pop() {
                let (ni, nj) = nxt;
                if regions[ni][nj] != 0 {
                    continue;
                }
                area.entry(nregions).and_modify(|v| *v += 1).or_insert(1);
                regions[ni][nj] = nregions;
                for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let (pi, underi) = ni.overflowing_add_signed(di);
                    let (pj, underj) = nj.overflowing_add_signed(dj);
                    if underi
                        || underj
                        || pi >= input.len()
                        || pj >= input[pi].len()
                        || input[pi][pj] != *target
                    {
                        continue;
                    }
                    pending.push((pi, pj));
                }
            }
        }
    }
    (regions, area)
}

/// Check whether the content of two elements in a grid are different.
/// The positions are determined by a valid position and the offset of the other position.
/// An element outside of the grid is always different
fn changes<T: Eq>(grid: &[Vec<T>], (i, j): (usize, usize), (di, dj): (isize, isize)) -> bool {
    let (pi, underi) = i.overflowing_add_signed(di);
    let (pj, underj) = j.overflowing_add_signed(dj);
    underi || underj || pi >= grid.len() || pj >= grid[pi].len() || grid[pi][pj] != grid[i][j]
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub fn part1(input: &Input) -> usize {
    let (regions, area) = regions_area(input);
    let mut fences: HashMap<usize, usize> = Default::default();
    for (i, row) in regions.iter().enumerate() {
        for (j, target) in row.iter().enumerate() {
            for (di, dj) in DIRS {
                if changes(&regions, (i, j), (di, dj)) {
                    fences.entry(*target).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }

    fences
        .into_iter()
        .map(|(k, v)| {
            let a = area.get(&k).expect("char not found in area");
            v * a
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let (regions, area) = regions_area(input);
    let mut fences: HashMap<usize, usize> = Default::default();
    for (i, row) in regions.iter().enumerate() {
        for (j, target) in row.iter().enumerate() {
            let corners = [
                changes(&regions, (i, j), DIRS[0]),
                changes(&regions, (i, j), DIRS[1]),
                changes(&regions, (i, j), DIRS[2]),
                changes(&regions, (i, j), DIRS[3]),
            ];
            let f = fences.entry(*target).or_default();
            for pos in 0..4 {
                let c1 = corners[pos];
                let c2 = corners[(pos + 1) % 4];
                let d1 = DIRS[pos];
                let d2 = DIRS[(pos + 1) % 4];
                let accross = (d1.0 + d2.0, d1.1 + d2.1);
                if c1 && c2 || (!c1 && !c2 && changes(&regions, (i, j), accross)) {
                    *f += 1;
                }
            }
        }
    }
    fences
        .into_iter()
        .map(|(k, v)| {
            let a = area.get(&k).expect("char not found in area");
            v * a
        })
        .sum()
}
