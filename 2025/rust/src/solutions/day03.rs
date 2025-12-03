pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines().filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            Some(line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        }
    }).collect()
}

pub fn part1(packs: &[Vec<usize>]) -> usize {

    packs.iter()
        .map(|pack| {
            let mut a = pack[0];
            if pack.len() == 1 {
                return a;
            }
            let mut max = a;
            for b in &pack[1..pack.len()] {
                max = max.max(a * 10 + b);
                if *b > a {
                    a = *b;
                }
            }
            max
        }).sum()
}

pub fn part2(i: &[Vec<usize>]) -> usize {
    todo!()
}
