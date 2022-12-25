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
    let out = match rem {
        0..=2 => {
            num -= rem;
            rem.to_string()
        }
        3 => {
            num += 2;
            "=".into()
        }
        4 => {
            num += 1;
            "-".into()
        }
        _ => unreachable!(),
    };
    to_snafu(num / 5) + &out
}

pub fn part1(input: &[isize]) -> String {
    to_snafu(input.iter().sum::<isize>())
}

pub fn part2(_input: &[isize]) -> String {
    String::from("AoC done for the year! Weee!")
}
