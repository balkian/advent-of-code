use std::fs;


macro_rules! rel {
    ($reg:ident $index:expr) => {{
        let index = $index;
        if (index < 0) || (index as usize >= $reg.len()) {
            return None
        } else {
            $reg[index as usize]
        }
    }}
}

macro_rules! command {
    ($regs:ident $op:ident $code:block) => {{
        let mut res = $regs.clone();
        res[$op.3] = $code;
        Some(res)
    }};
}


#[derive(Debug, PartialEq)]
struct Op(String, isize, isize, usize);

#[derive(Debug)]
struct PC {
    regs: Registers,
    code: Vec<Op>,
    ip: usize,
}

impl Iterator for PC {
    type Item = isize;
    
    fn next(&mut self) -> Option<Self::Item> {
        let ip = self.regs[self.ip];
        if ip < 0 || ip >= self.code.len() as isize {
            return None;
        }
        let inst = &self.code[ip as usize];
        if let Some(mut reg) = self.exec_one(inst){
            reg[self.ip] += 1;
            if cfg!(debug_assertions){
                println!("{}\t{:2?}\t{:2?} -> {:2?}", self.regs[self.ip as usize], inst, &self.regs, &reg);
            }
            self.regs = reg;
            Some(self.regs[self.ip])
        } else {
            println!("Instruction failed: {:?}", inst);
            None
        }
    }
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


    fn exec_one(&self, op: &Op) -> Option<Registers>{
        let regs = self.regs;
        let res = match op.0.as_str() {
            "addi" => command!(regs op {rel!(regs op.1) + op.2}),
            "addr" => command!(regs op {rel!(regs op.1) + rel!(regs op.2)}),
            "muli" => command!(regs op {rel!(regs op.1) * op.2}),
            "mulr" => command!(regs op {rel!(regs op.1) * rel!(regs op.2)}),

            "bani" => command!(regs op {rel!(regs op.1) & op.2}),
            "banr" => command!(regs op {rel!(regs op.1) & rel!(regs op.2)}),

            "bori" => command!(regs op {rel!(regs op.1) | op.2}),
            "borr" => command!(regs op {rel!(regs op.1) | rel!(regs op.2)}),

            "gtri" => command!(regs op {if (rel!(regs op.1) > op.2) {1} else {0}}),
            "gtrr" => command!(regs op {if (rel!(regs op.1) > rel!(regs op.2)) {1} else {0}}),
            "gtir" => command!(regs op {if op.1 > rel!(regs op.2) {1} else {0}}),

            "eqri" => command!(regs op {if rel!(regs op.1) == op.2 {1} else {0}}),
            "eqrr" => command!(regs op {if rel!(regs op.1) == rel!(regs op.2) {1} else {0}}),
            "eqir" => command!(regs op {if op.1 == rel!(regs op.2) {1} else {0}}),

            "seti" => command!(regs op {op.1}),
            "setr" => command!(regs op {rel!(regs op.1)}),
            _ => panic!("unknown command"),
        };
        res
    }
    fn run(&mut self) {
        let mut break_next = false;
        let breakpoints = [];
        while self.next().is_some() {
            let ip = &self.regs[self.ip as usize];
            if break_next || breakpoints.contains(ip) {
                let mut input_string = String::new();
                std::io::stdin().read_line(&mut input_string)
                    .expect("Failed to read line");
                match input_string.trim() {
                    "c" => break_next=false,
                    "n" => break_next=true,
                    _ => continue,
                }
            }
        }
    }

    #[allow(dead_code)]
    fn run2(&mut self) -> isize {
        let mut break_next = false;
        let breakpoints = [];
        let mut values: Vec<isize> = Vec::new();
        while self.next().is_some() {
            let ip = &self.regs[self.ip as usize];
            if *ip == 28 {
                if values.contains(&self.regs[3]){
                    return values.into_iter().last().unwrap()
                }
                values.push(self.regs[3]);
            }
            if break_next || breakpoints.contains(ip) {
                let mut input_string = String::new();
                std::io::stdin().read_line(&mut input_string)
                    .expect("Failed to read line");
                match input_string.trim() {
                    "c" => break_next=false,
                    "n" => break_next=true,
                    _ => continue,
                }
            }
        }
        panic!("program halted");
    }
}

/// This is a rust implementation of the ELF program
fn program2() -> usize {
    let mut values = vec!();
    let mut r = [0usize; 6];
    let mask24 = 1 << 24;
    loop {
        r[4] = r[3] | 65536;
        r[3] = 2176960;
        loop {
            r[1] = r[4] & 255;
            r[3] = (((r[3] + r[1]) % mask24) * 65899) % mask24;
            if 256 > r[4]{
                break
            }
            r[4] /= 256
        }
        if values.contains(&r[3]) {
            return *values.last().unwrap()
        }
        values.push(r[3]);
    }
}


type Registers = [isize; 6];

#[allow(clippy::many_single_char_names)]
fn parse_op(s: &str) -> Op {
    let caps: Vec<&str> = s.split(' ').collect();
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
    println!("Solution 2: {}", program2());
}


fn solve1(input: &str) -> isize {
    let mut pc = PC::parse(input);
    pc.regs[0] = 11474091;
    pc.run();
    pc.regs[0]
}

#[allow(dead_code)]
fn solve2(input: &str) -> isize {
    let mut pc = PC::parse(input);
    pc.regs[0] = 0;
    pc.run2()
}

#[test]
fn test_example() {
    let example = &std::fs::read_to_string("example").unwrap();
    assert_eq!(solve1(example), 6);
}
