use std::collections::HashMap;

type Input = Vec<Vec<char>>;

pub fn parse(i: &str) -> Input {
    i.trim().lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut regions: Vec<Vec<usize>> = vec![vec![0; input[0].len()]; input.len()];
    let mut area: HashMap<usize, usize> = Default::default();
    let mut nregions = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, target) in row.iter().enumerate() {
            if regions[i][j] != 0  {
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
                    if underi || underj || pi >= input.len() || pj >= input[pi].len() || input[pi][pj] != *target {
                        continue;
                    }
                    pending.push((pi, pj));
                }
            }
        }
    }
    let mut fences: HashMap<usize, usize> = Default::default();
    for (i, row) in regions.iter().enumerate() {
        for (j, target) in row.iter().enumerate() {
                for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    let (pi, underi) = i.overflowing_add_signed(di);
                    let (pj, underj) = j.overflowing_add_signed(dj);
                    if underi || underj || pi >= input.len() || pj >= input[pi].len() || regions[pi][pj] != *target{
                        fences.entry(*target).and_modify(|v| *v += 1).or_insert(1);
                    }
                }
        }
    }

   // dbg!(&area);
   // dbg!(&fences);

    fences.into_iter().map(|(k, v)| { 
        let a = area.get(&k).expect("char not found in area");
        let total = v * a;
        //eprintln!("{k} => {v} {a} = {total}");
        total
    }).sum()
}

pub fn part2(i: &Input) -> usize {
    todo!();
}
