use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::combinator::map;
use nom::character::complete::i64 as nomi64;
use nom::character::complete::{space1, multispace1};
use nom::sequence::separated_pair;
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug, Clone)]
pub struct Equation {
    res: i64,
    ops: Vec<i64>,

}

impl Equation {
    fn is_valid(&self) -> bool {
        let mut res = vec![self.ops[0]];
        for op in self.ops[1..].iter() {
            let mut newres = vec![];
            for r in res.drain(..){
                newres.push(r * op);
                newres.push(r + op);
            }
            res = newres;
        }

        res.into_iter().any(|r| r == self.res)
    }
}

pub fn parseall(i: &str) -> IResult<&str, Vec<Equation>> {
    let equation = map(separated_pair(nomi64, tag(": "), separated_list1(space1, nomi64)), |(res, ops)| Equation{res, ops});
    separated_list1(line_ending, equation)(i)
}
pub fn parse(i: &str) -> Vec<Equation> {
    let (i, eqs) = parseall(i).expect("could not parse input");
    eqs
}

pub fn part1(eqs: &[Equation]) -> i64 {
    eqs.iter().filter(|e| {
        e.is_valid()
    }).map(|e| e.res).sum()
}

pub fn part2(eqs: &[Equation]) -> i64 {
    todo!();
}
