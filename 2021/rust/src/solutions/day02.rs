pub enum Instruction {
    Up(usize),
    Down(usize),
    Forward(usize),
}

use Instruction::*;

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut tok = line.split(' ');
            let inst = tok.next().unwrap();
            let val = tok.next().unwrap().parse::<usize>().unwrap();
            match inst {
                "forward" => Forward(val),
                "down" => Down(val),
                "up" => Up(val),
                ins => panic!("invalid instruction: {}", ins),
            }
        })
        .collect()
}
pub fn part1(input: &[Instruction]) -> usize {
    let mut pos = (0, 0);
    for i in input {
        match i {
            Forward(a) => pos.1 += a,
            Down(a) => pos.0 += a,
            Up(a) => pos.0 -= a,
        }
    }
    pos.0 * pos.1
}
pub fn part2(input: &[Instruction]) -> usize {
    let mut pos = (0, 0);
    let mut aim = 0;
    for i in input {
        match i {
            Forward(a) => {
                pos.1 += a;
                pos.0 += a * aim;
            }
            Down(a) => aim += a,
            Up(a) => aim -= a,
        }
    }
    pos.0 * pos.1
}
