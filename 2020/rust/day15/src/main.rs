use std::collections::HashMap;
use std::collections::VecDeque;

fn keep_two(hm: &mut HashMap<usize, VecDeque<usize>>, num: usize, current: usize) -> usize {
    // dbg!{&hm};
    let indices = hm.entry(num).or_default();
    indices.push_front(current);
    if indices.len() > 1 {
        indices[0] - indices.pop_back().unwrap()
    } else {
        0
    }
}

fn main() {
    let args = aoc_utils::app("15").get_matches();
    let mut last = 0;
    let mut round = 1;
    let hm: &mut HashMap<usize, VecDeque<usize>> = &mut HashMap::new();
    for line in aoc_utils::file_iter_clap(&args) {
        for token in line.split(',').into_iter() {
            let num = token.parse::<usize>().unwrap();
            last = keep_two(hm, num, round);
            round += 1;
        }
    }
    for round in round + 1..=30000000 {
        // println!("Turn {}. Checking {}", &round, last);
        last = keep_two(hm, last, round - 1);
        if round == 2020 {
            println!("Num {:}: {}", round, last);
        }
    }
    println!("Part 2: {}", last);
}
