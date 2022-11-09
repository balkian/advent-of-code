pub fn parse(input: &str) -> Vec<usize> {
    let mut v = input.lines().flat_map(str::parse).collect::<Vec<usize>>();
    v.sort();
    v.reverse();
    v
}

fn valid_groups(weights: &[usize], target: usize) -> Vec<Vec<usize>> {
    if target == 0 {
        return vec![vec![]];
    }
    if weights.is_empty() {
        return vec![];
    }
    let (&head, rest) = weights.split_first().unwrap();
    let mut opts = valid_groups(rest, target);
    if target >= head {
        let mut others = valid_groups(rest, target - head);
        for o in others.iter_mut() {
            o.push(head);
        }
        opts.extend(others);
    }
    opts
}

pub fn solve(weights: &[usize], n: usize) -> Option<usize> {
    if n == 0 {
        if weights.is_empty() {
            return Some(0);
        } else {
            return None;
        }
    }
    let target = weights.iter().sum::<usize>() / n;
    let groups = valid_groups(weights, target);
    let mut tups: Vec<_> = groups
        .iter()
        .map(|g| {
            (
                g.len(),
                g.iter().fold(1usize, |a, b| a.saturating_mul(*b)),
                g,
            )
        })
        .collect();
    tups.sort();
    for (_, qe, g) in tups {
        let missing: Vec<usize> = weights.iter().filter(|u| !g.contains(u)).copied().collect();
        if solve(&missing, n - 1).is_some() {
            return Some(qe);
        }
    }
    None
}

pub fn part1(weights: &[usize]) -> usize {
    solve(weights, 3).unwrap()
}
pub fn part2(weights: &[usize]) -> usize {
    solve(weights, 4).unwrap()
}
