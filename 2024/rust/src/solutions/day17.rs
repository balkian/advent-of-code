use nom::bytes::complete::tag;
use nom::character::complete::i64 as ni64;
use nom::character::complete::u64 as nu64;
use nom::sequence::delimited;
use nom::character::complete::{alpha1, multispace1, space1, line_ending};
use nom::branch::alt;
use nom::sequence::{terminated, preceded, separated_pair};
use nom::multi::separated_list1;
use nom::{IResult, Parser};


use std::collections::HashMap;

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
    ///Combo operands 0 through 3 represent literal values 0 through 3.
    ///Combo operand 4 represents the value of register A.
    ///Combo operand 5 represents the value of register B.
    ///Combo operand 6 represents the value of register C.
    ///Combo operand 7 is reserved and will not appear in valid programs.
    fn combo(&self, pos: usize) -> i64 {
        let c = self.program[pos].0;
        match c {
            0..=3 => {
                c as i64
            },
            4 => { self.regs.a },
            5 => { self.regs.b },
            6 => { self.regs.c },
            7 => { panic!("reserved!") },
            _ => { panic!("wrong operand") },
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
                let den = 2i64.pow(self.combo(pos+1).try_into().expect("could not convert combo to u64"));
                self.regs.a = num / den;
            },
            //bxl
            1 => {
                self.regs.b = self.regs.b ^ self.literal(pos+1);
            },
            // bst
            2 => {
                self.regs.b = self.combo(pos + 1) % 8;
            },
            // jnz
            3 => {
                if self.regs.a != 0 {
                    self.ptr = self.literal(pos + 1) as usize;
                    return Ok(out);
                }
            },
            //bxc
            4 => {
                self.regs.b = self.regs.b ^ self.regs.c;
                self.literal(pos+1);
            }
            //out
            5 => {
                let val = self.combo(pos+1) % 8;
                out = Some(format!("{}", val));
            }
            // bdv
            6 => {
                let num = self.regs.a;
                let den = 2i64.pow(self.combo(pos+1).try_into().expect("could not convert combo to u64"));
                self.regs.b = num / den;
            },
            // cdv
            7 => {
                let num = self.regs.a;
                let den = 2i64.pow(self.combo(pos+1).try_into().expect("could not convert combo to u64"));
                self.regs.c = num / den;
            },
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
        ni64);
    let mut regs = Registers{a: 0, b: 0, c: 0};
    let mut value = terminated(reg, line_ending);
    for _ in 0..3 {
        let (ni, (r, v)) = value(i)?;
        match r {
            "A" => { regs.a = v;},
            "B" => { regs.b = v;},
            "C" => { regs.c = v;},
            _ => { unreachable!()},
        }
        i = ni;
    }
    let (i, _) = multispace1(i)?;
    let (i, program) = preceded(tag("Program: "), separated_list1(tag(","), nu64.map(|nu| Word(nu))))(i)?;
    let (i, _) = multispace1(i)?;
    assert!(i.is_empty());
    Ok((i, Computer{program: program, ptr: 0, regs}))
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
    dbg!(&pc);
    outs.join(",")
}

pub fn part2(i: &Computer) -> usize {
    todo!();
}
