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
            dbg!(&ip);
            return None;
        }
        let inst = &self.code[ip as usize];
        if let Some(mut reg) = self.exec_one(inst){
            reg[self.ip] += 1;
            println!("{}\t{:2?}\t{:2?} -> {:2?}", self.regs[self.ip as usize], inst, &self.regs, &reg);
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
        // dbg!{&res};
        res
    }

    fn run(&mut self) {
        for _i in self {
        }
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
    // let sol1 = solve1(&input);
    // println!("Solution 1: {}", sol1);
    let sol2 = solve2(&input);
    println!("Solution 2: {}", sol2);
}


fn solve1(input: &str) -> isize {
    let mut pc = PC::parse(input);
    pc.run();
    pc.regs[0]
}

fn solve2(input: &str) -> isize {
    let mut pc = PC::parse(input);
    dbg!(&pc);
    pc.regs[0] = 1;
    pc.run();
    pc.regs[pc.ip]
}

#[test]
fn test_example() {
    let example = &std::fs::read_to_string("example").unwrap();
    assert_eq!(solve1(example), 6);
}
