type Record = isize;

pub fn parse(input: &str) -> Vec<Record> {
    let mut vs = Vec::with_capacity(1000);
    let mut v = 1;
    vs.push(v);
    for line in input.lines().filter(|line| !line.is_empty()) {
        let toks: Vec<&str> = line.split_whitespace().collect();
        match toks[..] {
            ["noop"] => {}
            ["addx", x] => {
                let x: isize = x.parse().unwrap();
                vs.push(v);
                v += x;
            }
            _ => panic!("wrong line"),
        }
        vs.push(v);
    }
    vs
}

pub fn part1(input: &[Record]) -> isize {
    input
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(ix, v)| v * ((ix + 1) as isize))
        .sum()
}

pub fn part2(input: &[Record]) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    for (ix, v) in input.iter().enumerate() {
        let px = (ix % 40) as isize;
        if px == 0 {
            out += "\n";
        }
        out += if (v - 1..=v + 1).contains(&px) {
            "#"
        } else {
            "."
        };
    }
    out
}
