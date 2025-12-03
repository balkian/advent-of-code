pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines().filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            Some(line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
        }
    }).collect()
}

pub fn part1(banks: &[Vec<usize>]) -> usize {
    banks.iter()
        .map(|bank| {
            let mut a = bank[0];
            if bank.len() == 1 {
                return a;
            }
            let mut max = a;
            for b in &bank[1..bank.len()] {
                max = max.max(a * 10 + b);
                if *b > a {
                    a = *b;
                }
            }
            max
        }).sum()
}

fn joltage(positions: &[usize], batteries: &[usize]) -> usize {
    positions.iter().fold(0, |acc, jolts| {
        acc * 10 + batteries[*jolts]
    })
}

pub fn part2(banks: &[Vec<usize>]) -> usize {
    banks.iter()
        .map(|bank| {
            let mut chosen: [usize; 12] = std::array::from_fn(|i| i);
            for start in 1..=bank.len()-chosen.len() {
                let mut left = chosen[0];
                let mut right = start;

                for cur in chosen.iter_mut() {
                    *cur = left;
                    for option in left+1..=right {
                        if bank[option] > bank[*cur] {
                            *cur = option;
                        }
                    }
                    left = *cur + 1;
                    right += 1;
                }
            }
            //let chosenbank: Vec<usize> = chosen.iter().map(|c| bank[*c]).collect();
            joltage(&chosen, bank)
        }).sum()
}
