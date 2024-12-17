use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64 as ni64;
use nom::character::complete::u64 as nu64;
use nom::character::complete::{line_ending, multispace1};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{IResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Word(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Computer {
    regs: Registers,
    ptr: usize,
    program: Vec<Word>,
}

impl Computer {
    fn combo(&self, pos: usize) -> i64 {
        let c = self.program[pos].0;
        match c {
            0..=3 => c as i64,
            4 => self.regs.a,
            5 => self.regs.b,
            6 => self.regs.c,
            7 => {
                panic!("reserved!")
            }
            _ => {
                panic!("wrong operand")
            }
        }
    }

    fn literal(&self, pos: usize) -> i64 {
        self.program[pos].0 as i64
    }

    fn execute(&mut self) -> Result<Option<String>, ()> {
        let pos = self.ptr;
        if pos >= self.program.len() - 1 {
            return Err(());
        }
        let c = self.program[pos].0;
        let mut out = None;
        match c {
            // adv
            0 => {
                let num = self.regs.a;
                let den = 2i64.pow(
                    self.combo(pos + 1)
                        .try_into()
                        .expect("could not convert combo to u64"),
                );
                self.regs.a = num / den;
            }
            //bxl
            1 => {
                self.regs.b ^= self.literal(pos + 1);
            }
            // bst
            2 => {
                self.regs.b = self.combo(pos + 1) % 8;
            }
            // jnz
            3 => {
                if self.regs.a != 0 {
                    self.ptr = self.literal(pos + 1) as usize;
                    return Ok(out);
                }
            }
            //bxc
            4 => {
                self.regs.b ^= self.regs.c;
                self.literal(pos + 1);
            }
            //out
            5 => {
                let val = self.combo(pos + 1) % 8;
                out = Some(format!("{}", val));
            }
            // bdv
            6 => {
                let num = self.regs.a;
                let den = 2i64.pow(
                    self.combo(pos + 1)
                        .try_into()
                        .expect("could not convert combo to u64"),
                );
                self.regs.b = num / den;
            }
            // cdv
            7 => {
                let num = self.regs.a;
                let den = 2i64.pow(
                    self.combo(pos + 1)
                        .try_into()
                        .expect("could not convert combo to u64"),
                );
                self.regs.c = num / den;
            }
            _ => {
                panic!("invalid code");
            }
        }
        self.ptr += 2;
        Ok(out)
    }
}

pub fn computer(mut i: &str) -> IResult<&str, Computer> {
    let reg = separated_pair(
        preceded(tag("Register "), alt((tag("A"), tag("B"), tag("C")))),
        tag(": "),
        ni64,
    );
    let mut regs = Registers { a: 0, b: 0, c: 0 };
    let mut value = terminated(reg, line_ending);
    for _ in 0..3 {
        let (ni, (r, v)) = value(i)?;
        match r {
            "A" => {
                regs.a = v;
            }
            "B" => {
                regs.b = v;
            }
            "C" => {
                regs.c = v;
            }
            _ => {
                unreachable!()
            }
        }
        i = ni;
    }
    let (i, _) = multispace1(i)?;
    let (i, program) = preceded(tag("Program: "), separated_list1(tag(","), nu64.map(Word)))(i)?;
    let (i, _) = multispace1(i)?;
    assert!(i.is_empty());
    Ok((
        i,
        Computer {
            program,
            ptr: 0,
            regs,
        },
    ))
}

pub fn parse(i: &str) -> Computer {
    let (i, c) = computer(i).expect("could not parse computer");
    assert!(i.is_empty());
    c
}

pub fn part1(i: &Computer) -> String {
    let mut pc = i.clone();
    let mut outs = vec![];
    while let Ok(opt) = pc.execute() {
        if let Some(out) = opt {
            outs.push(out);
        }
    }
    outs.join(",")
}

/// I did some manual analysis of the code, and MY program is a loop
/// that updates the B and C registers based on the previous value of A
/// and sets A to A / 8.
/// Then, it prints the value of C % 8 until A == 0.
/// This solution works from the last state and keeps track of any possible
/// remainder that would satisfy the output.
pub fn part2(pc: &Computer) -> u64 {
    let mut opts = vec![0u64];
    for Word(res) in pc.program.iter().rev() {
        let mut newopts = vec![];
        for opt in opts {
            for rem in 0..8 {
                let a = opt * 8 + rem;
                let b = (a % 8) ^ (a / 2u64.pow(((a % 8) ^ 5) as u32)) ^ 3;
                if (b % 8) == *res {
                    newopts.push(a);
                }
            }
        }
        opts = newopts;
    }
    let sol = opts.into_iter().min().expect("no solution found");

    let mut nc = pc.clone();
    nc.regs.a = sol as i64;
    let out = part1(&nc);
    let expected = pc
        .program
        .iter()
        .map(|Word(r)| format!("{r}"))
        .collect::<Vec<String>>()
        .join(",");
    assert_eq!(expected, out);
    sol
}
