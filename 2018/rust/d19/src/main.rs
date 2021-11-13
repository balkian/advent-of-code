use std::collections::{HashMap};

use lazy_static::lazy_static;

use std::fs;


macro_rules! operations {
    ($($tts:tt)*) => {{
        let mut vect: Vec<(&str, Operation)> = vec![];
        operations_inner!(vect $($tts)*);
        let hm: HashMap<&str, Operation> = HashMap::from_iter(vect);
        hm
    }}
}

/// This is a very ugly macro, but I'm still learning
macro_rules! operations_inner {
    (@direct $vect:ident $name:ident |$a:ident, $b:ident| $op:block ) => {{
        fn $name(reg: &Registers, op: &Op) -> Option<Registers> {
            if op.1 < 0 || op.1 as usize >= reg.len() {
                return None
            }
            let mut res = reg.clone();
            let $a = reg[op.1 as usize];
            let $b = op.2;
            res[op.3] = $op;
            Some(res)
        }
        $vect.push((stringify!($name), $name));

    }};
    (@indirect $vect:ident $name:ident |$a:ident, $b:ident| $op:block) => {{
        fn $name(reg: &Registers, op: &Op) -> Option<Registers> {
            if op.2 < 0 || op.2 as usize >= reg.len() {
                return None
            }
            let mut res = reg.clone();
            let $a = reg[op.1 as usize];
            let $b = reg[op.2 as usize];
            res[op.3] = $op;
            Some(res)
        }
        $vect.push((stringify!($name), $name));
    }};
    (@inverse $vect:ident $name:ident |$a:ident, $b:ident| $op:block ) => {{
        fn $name(reg: &Registers, op: &Op) -> Option<Registers> {
            if op.1 < 0 || op.2 < 0 || op.1 as usize > reg.len() || op.2 as usize >= reg.len() {
                return None
            }
            let mut res = reg.clone();
            let $a = op.1;
            let $b = reg[op.2 as usize];
            res[op.3] = $op;
            Some(res)
        }
        $vect.push((stringify!($name), $name));
    }};
    ($vect:ident ($($dir:ident)?, $($indir:ident)?, $($inv:ident)?) => |$a:ident, $b:ident| $op:block
     $(;$(($($odir:ident)?, $($oindir:ident)?, $($oinv:ident)?) => |$oa:ident, $ob:ident| $oop:block)*)* ;)
     => {{
        $(operations_inner!(@direct $vect $dir |$a, $b| $op);)?
        $(operations_inner!(@indirect $vect $indir |$a, $b| $op);)?
        $(operations_inner!(@inverse $vect $inv |$a, $b| $op);)?
        $(operations_inner!($vect $(($($odir)?, $($oindir)?, $($oinv)?) => |$oa ,$ob| $oop;)*);)?
    }}
}

#[derive(Debug, PartialEq)]
struct Op(String, isize, isize, usize);


impl Op {
    fn apply(&self, reg: &Registers) -> Option<Registers> {
        (OPS.get(&*self.0).unwrap())(reg, self)
    }
}

#[derive(Debug)]
struct PC {
    regs: Registers,
    code: Vec<Op>,
    ip: usize,
}

lazy_static! {
    static ref OPS: HashMap<&'static str, Operation> = operations! {
        (addi, addr,     ) => |a, b| {a+b};
        (muli, mulr,     ) => |a, b| {a*b};
        (bani, banr,     ) => |a, b| {a&b};
        (bori, borr,     ) => |a, b| {a|b};
        (    ,     , seti) => |a,_b| {a};
        (setr,     ,     ) => |a,_b| {a};
        (gtri, gtrr, gtir) => |a, b| {if a>b {1} else {0}};
        (eqri, eqrr, eqir) => |a, b| {if a==b {1} else {0}};
    };
}

impl PC {
    fn parse(input: &str) -> Self {
        let mut code = vec!();
        let mut it = input.lines();
        let ip: usize = it.next().unwrap().chars().skip(4).collect::<String>().parse().unwrap();
        for line in it {
            code.push(parse_op(line));
        }
        let regs = [0;6];
        PC{regs, code, ip}
    }
    fn execute(&mut self) -> Option<isize> {
        let ip = self.regs[self.ip];
        if ip < 0 || ip >= self.code.len() as isize {
            dbg!(&ip);
            return None;
        }
        let inst = &self.code[ip as usize];
        if let Some(mut reg) = inst.apply(&self.regs){
            reg[self.ip] += 1;
            println!("{}\t{:2?}\t{:2?} -> {:2?}", self.regs[self.ip as usize], inst, &self.regs, &reg);
            self.regs = reg;
            Some(self.regs[self.ip])
        } else {
            println!("Instruction failed: {:?}", inst);
            None
        }
    }

    fn run(&mut self) {
        while let Some(a) = self.execute() {
        }
    }
}



type Registers = [isize; 6];

type Operation = fn(&Registers, &Op) -> Option<Registers>;


#[allow(clippy::many_single_char_names)]
fn parse_op(s: &str) -> Op {
    let caps: Vec<&str> = s.split(" ").collect();
    let op = caps[0].to_string();
    let a = caps[1].parse().unwrap();
    let b = caps[2].parse().unwrap();
    let c = caps[3].parse().unwrap();
    Op(op, a, b, c)
}


fn main() {
    let input = fs::read_to_string("input").expect("could not read file");
    let sol1 = solve1(&input);
    println!("Solution 1: {}", sol1);
}


fn solve1(input: &str) -> isize {

    let mut pc = PC::parse(input);
    dbg!(&pc);
    pc.run();
    pc.regs[pc.ip]
}

#[test]
fn test_example() {
    let example = &std::fs::read_to_string("example").unwrap();
    assert_eq!(solve1(example), 6);
}
