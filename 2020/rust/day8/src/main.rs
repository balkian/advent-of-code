use lazy_static::lazy_static;
use regex::Regex;

type Command = (String, isize);

lazy_static! {
    static ref COMMAND: Regex =
        Regex::new(r"(?P<instruction>(acc|jmp|nop)) (?P<value>[+-]\d*)").unwrap();
}

/// Go through every instruction and update both the ACC and the IDX
/// Return as soon as an instruction is repeated.
fn calculate(program: &[Command]) -> (isize, bool, Vec<usize>) {
    let mut acc: isize = 0;
    let mut idx = 0;
    let mut visited = Vec::<usize>::new();

    while idx < program.len() {
        let (inst, value) = &program[idx];
        if visited.contains(&idx) {
            break
        }
        visited.push(idx);
        match inst.as_str() {
            "acc" => {
                acc += value;
                idx += 1;
            },
            "jmp" => {
                idx = ((idx as isize) + value) as usize;
            },
            "nop" => {
                idx += 1;
            },
            _ => panic!("invalid instruction")
        }
    }
    (acc, idx == program.len(), visited)
}

/// We only need to change one of the JMP instructions to NOP, or vice versa.
/// Instead of going through all the instructions, we limit ourselves to the ones
/// that have actually been executed.
fn calculate2(program: &[Command], visited: &[usize]) {
    for idx in visited {
        let (inst, value) = &program[*idx];
        let n_inst = match inst.as_str() {
            "nop" => "jmp",
            "jmp" => "nop",
            _ => continue,
        };
        let mut next: Vec<Command> = program.to_vec().clone();
        next[*idx] = (n_inst.to_string(), *value);

        let (acc, finished, _) = calculate(&next);
        if finished {
            println!("Part 2: Accumulator: {:}", acc);
            break
        }
    }
}

fn main() {

    let mut program: Vec<Command>= vec![];

    for line in aoc_utils::file_iter() {
        let cmd = COMMAND.captures(line.as_str()).unwrap();
        let instruction = cmd.name("instruction").unwrap().as_str().to_string();
        let value: isize = cmd.name("value").unwrap().as_str().parse().unwrap();
        program.push((instruction, value));
    }
    let (acc, _, visited) = calculate(&program);
    println!("Part 1: Accumulator: {:}", &acc);
    calculate2(&program, &visited);
}
