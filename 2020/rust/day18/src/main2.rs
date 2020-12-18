type Operation<'a> = &'a dyn Fn(isize, isize) -> isize;

fn parse(number: &mut String, buffer: &mut Vec<isize>) {
    if !number.is_empty() {
        let temp = number.parse().unwrap();
        buffer.push(temp);
        *number = String::new();
    }
}

struct Op

fn apply(operations: &mut Vec<Operation>, buffer: &mut Vec<isize>) {
    let level = buffer.len()-1;
    buffer[level-1] = operations[level-1](buffer[level-1], buffer[level]);
    operations.pop().expect("closing without opening");
    buffer.pop().expect("closing without opening");
}


fn up(operations: &mut Vec<Operation>, buffer: &mut Vec<isize>) {
    buffer.push(0);
    operations.push(&|x,y| x+y);
}

fn swap(operations: &mut Vec<Operation>, buffer: &mut Vec<isize>) {
    let l1 = operations.len();
    let l2 = buffer.len();
    operations.swap(l1-1, l1-2);
    buffer.swap(l2-1, l2-2);
}

fn calculate(input: &str) -> isize {
    let sum = &|x: isize, y: isize| x + y;

    dbg!{&input};
    let mut buffer: Vec<isize> = vec!(0);
    let mut operations: Vec<&dyn Fn(isize, isize) -> isize> = vec!(sum);
    let mut number = String::new();

    for c in input.chars().chain(std::iter::once(' ')) {
        dbg!{&c, &number, &buffer};
        match c {
            '(' => {
                parse(&mut number, &mut buffer);
                up(&mut operations, &mut buffer);
            }
            ')' => {
                parse(&mut number, &mut buffer);
                apply(&mut operations, &mut buffer);
            }
            '+' => {
                parse(&mut number, &mut buffer);
                apply(&mut operations, &mut buffer);
                operations.push(&|x,y| x+y);
            }
            '-' => {
                parse(&mut number, &mut buffer);
                apply(&mut operations, &mut buffer);
                operations.push(&|x,y| x-y);
            }
            '*' => {
                parse(&mut number, &mut buffer);
                apply(&mut operations, &mut buffer);
                operations.push(&|x,y| x*y);
            }
            ' ' => {
                parse(&mut number, &mut buffer);
            }
            _ => { number.push(c)}
        }
    }
    for _i in 0..buffer.len()-1 {
        apply(&mut operations, &mut buffer);
        dbg!{&buffer};
    }
    buffer.pop().unwrap()
}

fn calculate2(input: &str) -> isize {
    let sum = &|x: isize, y: isize| x + y;

    dbg!{&input};
    let mut buffer: Vec<isize> = vec!(0);
    let mut operations: Vec<&dyn Fn(isize, isize) -> isize> = vec!(sum);
    let mut number = String::new();
    let mut fold = false;

    for c in input.chars().chain(std::iter::once(' ')) {
        dbg!{&c, &number, &buffer};
        match c {
            '(' => {
                parse(&mut number, &mut buffer);
                up(&mut operations, &mut buffer);
            }
            ')' => {
                parse(&mut number, &mut buffer);
                apply(&mut operations, &mut buffer);
            }
            '+' => {
                parse(&mut number, &mut buffer);
                if fold {
                    apply(&mut operations, &mut buffer);
                    fold = false;
                }
                apply(&mut operations, &mut buffer);
                operations.push(&|x,y| x+y);
                up(&mut operations, &mut buffer);
                swap(&mut operations, &mut buffer);
                fold = true;
            }
            '*' => {
                parse(&mut number, &mut buffer);
                if fold {
                    apply(&mut operations, &mut buffer);
                    fold = false;
                }
                apply(&mut operations, &mut buffer);
                operations.push(&|x,y| x*y);
            }
            ' ' => {}
            _ => { number.push(c)}
        }
    }
    if fold {
        apply(&mut operations, &mut buffer);
        fold = false;
    }
    apply(&mut operations, &mut buffer);
    assert_eq!(buffer.len(), 1);
    buffer.pop().unwrap()
}

// fn calculate2(input: &str) -> isize {
//     let sum = &|x: isize, y: isize| x + y;

//     dbg!{&input};
//     let mut buffer: Vec<isize> = vec!(0);
//     let mut level = 0;
//     let mut operations: Vec<&dyn Fn(isize, isize) -> isize> = vec!(sum);
//     let mut number = String::new();

//     for c in input.chars().chain(std::iter::once(' ')) {
//         dbg!{&c, &number, &buffer};
//         match c {
//             '(' => {
//                 buffer.push(0);
//                 operations.push(sum);
//                 level += 1;
//             }
//             ')' => {
//                 parse_and_apply(&mut number, &mut operations, &mut buffer);
//                 buffer[level-1] = operations[level-1](buffer[level-1], buffer[level]);
//                 operations.pop().expect("closing without opening");
//                 buffer.pop().expect("closing without opening");
//                 level -= 1;
//             }
//             '+' => {
//                 operations[level] = &|x,y| x+y;
//             }
//             '-' => {
//                 operations[level] = &|x,y| x-y;
//             }
//             '*' => {
//                 operations[level] = &|x,y| x*y;
//             }
//             ' ' => {}
//             _ => { number.push(c)}
//         }
//     }
//     buffer[level]
// }


fn main() {
    let args = aoc_utils::app(env!("CARGO_PKG_NAME")).get_matches();
    let mut res1 = 0;
    let mut res2 = 0;
    for line in aoc_utils::file_iter_clap(&args) {
        res1 += calculate(&line);
        // res2 += calculate2(&line);

    }
    println!("Part 1: {}", res1);
    // println!("Part 2: {}", res2);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part1() {
        let examples = &[
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)];
        for (input, expected) in examples {
            assert_eq!(calculate(input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let examples = &[
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)];
        for (input, expected) in examples {
            assert_eq!(calculate2(input), *expected);
        }
    }
}
