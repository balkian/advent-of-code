use std::env;

fn main() {
    let file = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = std::fs::read_to_string(file).expect("could not read the file");
    // let part1: isize = contents.chars().map(|x| match x{ '(' => 1, ')' => -1, _ => panic!("unexpected")}).sum();
    // println!("Part 1: {}", part1);

    let mut floor = 0;
    let mut found = false;
    for (idx, c) in contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected character"),
        }
        if floor == -1 && !found {
            println!("Part 2: {}", idx + 1);
            found = true;
        }
    }
    println!("Part 1: {}", floor);
}
