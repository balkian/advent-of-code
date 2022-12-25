pub fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.trim().chars().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!("unknown symbol {c }"),
                    }
            })
        })
        .collect()
}

fn to_snafu(mut num: isize) -> String {
    if num == 0 {
        return "".into();
    }
    let rem = num % 5;
    let mut out = String::new();
    match rem {
        0..=2 => {
            out = rem.to_string();
            num -= rem;
        }
        3 => {
            out = "=".into();
            num += 2;
        }
        4 => {
            out = "-".into();
            num += 1;
        }
        _ => unreachable!(),
    }
    to_snafu(num / 5) + &out
}

pub fn part1(input: &[isize]) -> String {
    to_snafu(input.iter().sum::<isize>())
}

pub fn part2(input: &[isize]) -> String {
    todo!();
}
