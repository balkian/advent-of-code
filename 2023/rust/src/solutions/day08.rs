use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Game<'a> {
    directions: Vec<usize>,
    jumps: HashMap<&'a str, [&'a str; 2]>,
}

pub fn parse<'a>(input: &'a str) -> Game<'a> {
    let mut lines = input.lines();
    let directions = lines.next().expect("less than one line").chars().map(|c| {
        match c{
        'L' => 0,
        'R' => 1,
        c => panic!("unknown direction {c}"),
            }
    } ).collect();
    let mut jumps = HashMap::new();
    lines.next().expect("no empty line after directions");
    for line in lines {
        let (name, sides)= line.split_once(" = ").expect("did not find equal");
        let sides = sides.trim_matches(&['(', ')']).split_once(", ").expect("did not find comma");
        jumps.insert(name, [sides.0, sides.1]);
    }

    Game{directions, jumps}
    
}

pub fn part1(game: &Game) -> usize {
    let mut current = "AAA";
    let jumps = &game.jumps;
    let mut directions = game.directions.iter().cycle();
    for i in 0.. {
        if current == "ZZZ" {
            return i
        }
        let d = directions.next().unwrap();
        current = jumps.get(current).expect("current not found {current}")[*d];
    }
    unreachable!();
}
pub fn part2(game: &Game) -> usize {
    let mut current: Vec<&str>  = game.jumps.keys().filter(|k| k.ends_with("A")).copied().collect();
    dbg!(current.len());
    let jumps = &game.jumps;
    let mut cycles = vec![0; current.len()];
    let mut directions = game.directions.iter().cycle();
    for i in 0.. {
        for (ix, c) in current.iter().enumerate() {
            if cycles[ix] == 0 && c.ends_with('Z') {
                cycles[ix] = i;
            }
        }

        let d = directions.next().unwrap();
        let mut done = true;
        for (ix, c) in current.iter_mut().enumerate() {
            if cycles[ix] == 0 {
                *c = jumps.get(c).expect("current not found {current}")[*d];
                done = false;
            }
        }
        if done {
            break
        }
    }
    dbg!(&cycles, game.directions.len());
    cycles.push(game.directions.len());

    let solution = cycles.into_iter().fold(1, |acc, b| lcm(acc, b));
    solution
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					
