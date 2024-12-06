use regex::Regex;

pub enum Inst {
    Mul(i64, i64),
    Set(bool),
}

pub fn parse(i: &str) -> Vec<Inst> {
    let re =
        Regex::new(r"mul\((?<m1>[0-9]+),(?<m2>[0-9]+)\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();

    let mut results = vec![];
    for cap in re.captures_iter(i) {
        if cap.name("do").is_some() {
            results.push(Inst::Set(true));
        } else if cap.name("dont").is_some() {
            results.push(Inst::Set(false));
        } else {
            results.push(Inst::Mul(
                cap.name("m1")
                    .expect("m1 is wrong")
                    .as_str()
                    .parse::<i64>()
                    .expect("invalid first mult"),
                cap.name("m2")
                    .expect("m2 is wrong")
                    .as_str()
                    .parse::<i64>()
                    .expect("invalid second mult"),
            ));
        }
    }
    results
}

pub fn part1(input: &[Inst]) -> i64 {
    input.iter().fold(0, |acc, i| {
        if let Inst::Mul(a, b) = i {
            acc + a * b
        } else {
            acc
        }
    })
}

pub fn part2(input: &[Inst]) -> i64 {
    let mut enabled = true;
    let mut total = 0;
    for inst in input {
        match inst {
            Inst::Set(val) => {
                enabled = *val;
            }
            Inst::Mul(m1, m2) if enabled => {
                total += m1 * m2;
            }
            _ => {}
        }
    }
    total
}
