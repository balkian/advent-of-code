use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nomi64, line_ending, multispace1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone)]
pub struct Equation {
    res: i64,
    ops: Vec<i64>,
}

impl Equation {
    fn is_valid(&self, concat: bool) -> bool {
        let mut res = vec![self.ops[0]];
        for op in self.ops[1..].iter() {
            let mut newres = vec![];
            for r in res.drain(..) {
                newres.push(r * op);
                newres.push(r + op);
                if concat {
                    let digits = op.ilog10() + 1;
                    newres.push(r * 10i64.pow(digits) + op);
                }
            }
            res = newres;
        }

        res.into_iter().any(|r| r == self.res)
    }
}

pub fn parse(i: &str) -> Vec<Equation> {
    let equation = map(
        separated_pair(nomi64, tag(": "), separated_list1(space1, nomi64)),
        |(res, ops)| Equation { res, ops },
    );
    let res: IResult<_, _> = terminated(separated_list1(line_ending, equation), multispace1)(i);
    let (i, eqs) = res.expect("could not parse input");
    assert!(i.is_empty());
    eqs
}

pub fn part1(eqs: &[Equation]) -> i64 {
    eqs.iter()
        .filter(|e| e.is_valid(false))
        .map(|e| e.res)
        .sum()
}

pub fn part2(eqs: &[Equation]) -> i64 {
    eqs.iter().filter(|e| e.is_valid(true)).map(|e| e.res).sum()
}
