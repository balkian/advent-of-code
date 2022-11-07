#[derive(Debug, Default, Clone)]
pub struct PC {
    program: Vec<Instruction>,
    regs: [usize; 2],
    offset: usize,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

use Instruction::*;

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let tokens: Vec<_> = input.split(' ').filter(|t| t != &",").collect();
        match tokens[0] {
            "hlf" => Hlf(tokens[1].chars().next().unwrap()),
            "tpl" => Tpl(tokens[1].chars().next().unwrap()),
            "inc" => Inc(tokens[1].chars().next().unwrap()),
            "jmp" => Jmp(tokens[1].parse().unwrap()),
            "jio" => Jio(
                tokens[1].chars().next().unwrap(),
                tokens[2].parse().unwrap(),
            ),
            "jie" => Jie(
                tokens[1].chars().next().unwrap(),
                tokens[2].parse().unwrap(),
            ),
            _ => panic!("unknown instruction"),
        }
    }
}

fn reg_index(c: char) -> usize {
    match c {
        'a' => 0,
        'b' => 1,
        _ => panic!("invalid register"),
    }
}

impl PC {
    fn parse(input: &str) -> Self {
        PC {
            program: input.lines().map(|line| Instruction::parse(line)).collect(),
            ..Default::default()
        }
    }

    fn step(&mut self) {
        let inst = self.program[self.offset];
        self.apply_inst(inst);
    }

    fn apply_inst(&mut self, i: Instruction) {
        match i {
            Hlf(reg) => {
                let idx = reg_index(reg);
                self.regs[idx] /= 2;
            }
            Tpl(reg) => {
                let idx = reg_index(reg);
                self.regs[idx] *= 3;
            }
            Inc(reg) => {
                let idx = reg_index(reg);
                self.regs[idx] += 1;
            }
            Jmp(off) => {
                self.offset = ((self.offset as isize) + off) as usize;
                return;
            }
            Jie(reg, off) => {
                let idx = reg_index(reg);
                if self.regs[idx] % 2 == 0 {
                    self.offset = ((self.offset as isize) + off) as usize;
                    return;
                }
            }
            Jio(reg, off) => {
                let idx = reg_index(reg);
                if self.regs[idx] == 1 {
                    self.offset = ((self.offset as isize) + off) as usize;
                    return;
                }
            }
        }
        self.offset += 1;
    }
}

pub fn parse(input: &str) -> PC {
    PC::parse(input)
}

pub fn part1(pc: &PC) -> usize {
    let mut pc = pc.clone();
    loop {
        if pc.offset >= pc.program.len() {
            break;
        }
        pc.step()
    }
    pc.regs[1]
}

pub fn part2(pc: &PC) -> usize {
    let mut pc = pc.clone();
    pc.regs[0] = 1;
    part1(&pc)
}
