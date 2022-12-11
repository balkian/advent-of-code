use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum Op {
    Old,
    Number(u64),
}

impl Op {
    fn value(&self, other: u64) -> u64 {
        match self {
            Op::Old => other,
            Op::Number(x) => *x,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(Op, Op),
    Sum(Op, Op),
}

impl Operation {
    fn value(&self, other: u64) -> u64 {
        match self {
            Operation::Mul(op1, op2) => op1.value(other) * op2.value(other),
            Operation::Sum(op1, op2) => op1.value(other) + op2.value(other),
        }
    }
}

#[derive(Debug, Clone)]
struct Test(u64, usize, usize);

#[derive(Debug, Clone)]
pub struct Game {
    rounds: usize,
    monkeys: Vec<Monkey>,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    position: usize,
    inspections: usize,
    operation: Operation,
    test: Test,
    items: Vec<u64>,
}

impl Game {
    fn round(&mut self, reducer: impl Fn(u64) -> u64) {
        self.rounds += 1;
        for ix in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[ix];
            let discards = monkey.round(&reducer);
            for (target, val) in discards {
                self.monkeys[target].items.push(val);
            }
        }
    }
}

impl Monkey {
    fn round(&mut self, reducer: impl Fn(u64) -> u64) -> Vec<(usize, u64)> {
        let size = self.items.len();
        let items = std::mem::replace(&mut self.items, Vec::with_capacity(size));
        self.inspections += size;
        items
            .iter()
            .map(|item| {
                let item = reducer(self.operation.value(*item));
                if item % self.test.0 == 0 {
                    (self.test.1, item)
                } else {
                    (self.test.2, item)
                }
            })
            .collect()
    }
}

// Parsing code

fn mydigit(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    let (rest, op) = alt((tag("old"), digit1))(input)?;
    let op = match op {
        "old" => Op::Old,
        op => Op::Number(op.parse::<u64>().unwrap()),
    };
    Ok((rest, op))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (rest, tup) = preceded(
        tag("new = "),
        tuple((
            parse_op,
            delimited(multispace0, alt((tag("*"), tag("+"))), multispace0),
            parse_op,
        )),
    )(input)?;
    let res = match tup.1 {
        "+" => Operation::Sum(tup.0, tup.2),
        "*" => Operation::Mul(tup.0, tup.2),
        _ => panic!("unknown operation"),
    };
    Ok((rest, res))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (rest, tup) = tuple((
        delimited(tag("divisible by "), mydigit, newline),
        delimited(
            tuple((multispace1, tag("If true: throw to monkey "))),
            mydigit,
            newline,
        ),
        delimited(
            tuple((multispace1, tag("If false: throw to monkey "))),
            mydigit,
            newline,
        ),
    ))(input)?;
    Ok((rest, Test(tup.0 as u64, tup.1, tup.2)))
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (rest, tup) = tuple((
        delimited(tag("Monkey "), mydigit, tuple((tag(":"), newline))),
        delimited(
            tuple((multispace1, tag("Starting items: "))),
            separated_list1(tag(", "), map_res(digit1, |s: &str| s.parse::<u64>())),
            newline,
        ),
        delimited(
            tuple((multispace1, tag("Operation: "))),
            parse_operation,
            newline,
        ),
        preceded(tuple((multispace1, tag("Test: "))), parse_test),
    ))(input)?;
    let monkey = Monkey {
        inspections: 0,
        position: tup.0,
        items: tup.1,
        operation: tup.2,
        test: tup.3,
    };
    Ok((rest, monkey))
}

pub fn parse(input: &str) -> Game {
    let (rest, monkeys) =
        // terminated(
        separated_list1(newline,
                        parse_monkey
    )(input)
    .unwrap();
    debug_assert!(rest.is_empty());
    for (ix, m) in monkeys.iter().enumerate() {
        assert_eq!(ix, m.position);
    }
    Game { rounds: 0, monkeys }
}

// End of parsing code

pub fn solve(input: &Game, times: usize, reducer: impl Fn(u64) -> u64) -> usize {
    let mut game = input.clone();
    for _ in 0..times {
        game.round(&reducer);
    }
    game.monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

pub fn part1(input: &Game) -> usize {
    solve(input, 20, |u: u64| u / 3)
}

pub fn part2(input: &Game) -> usize {
    let modulus: u64 = input.monkeys.iter().map(|m| m.test.0).product();
    solve(input, 10000, |u: u64| u % modulus)
}
