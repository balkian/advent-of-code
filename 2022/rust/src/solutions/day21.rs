use std::collections::HashMap;

type Input<'a> = HashMap<&'a str, Rule<'a>>;

pub enum Rule<'a> {
    Number(isize),
    Op(&'a str, &'a str, &'a str),
}

impl<'a> Rule<'a> {
    fn value(
        &self,
        calculated: &HashMap<&'a str, isize>,
    ) -> Result<isize, (Option<&'a str>, Option<&'a str>)> {
        match self {
            Rule::Number(i) => Ok(*i),
            Rule::Op(op, a, b) => match (calculated.get(a), calculated.get(b)) {
                (Some(a), Some(b)) => {
                    let res = match *op {
                        "*" => a * b,
                        "+" => a + b,
                        "-" => a - b,

                        "/" => a / b,
                        _ => panic!("unsupported operation {op}"),
                    };
                    Ok(res)
                }
                (Some(_), None) => Err((None, Some(b))),
                (None, None) => Err((Some(a), Some(b))),
                (None, Some(_)) => Err((Some(a), None)),
            },
        }
    }
}

pub fn parse(input: &str) -> Input {
    let mut out: Input = Default::default();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (l, r) = line.trim().split_once(": ").unwrap();
        if let Ok(num) = r.trim().parse::<isize>() {
            out.insert(l, Rule::Number(num));
        } else {
            let toks: Vec<_> = r.trim().split_whitespace().collect();
            out.insert(l, Rule::Op(toks[1], toks[0], toks[2]));
        }
    }
    out
}

pub fn part1(input: &Input) -> isize {
    let mut stack = vec![];
    let mut calculated: HashMap<&str, isize> = Default::default();
    let root = "root";
    stack.push(root.clone());
    while !stack.is_empty() {
        let key = stack.last().unwrap();
        match input[key].value(&calculated) {
            Ok(i) => {
                let key = stack.pop().unwrap();
                calculated.insert(key, i);
            }
            Err((a, b)) => {
                if let Some(a) = a {
                    stack.push(a);
                }
                if let Some(b) = b {
                    stack.push(b);
                }
            }
        }
    }
    calculated[root]
}

pub fn part2(input: &Input) -> isize {
    todo!();
}
