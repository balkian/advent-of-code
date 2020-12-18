use std::collections::HashMap;

fn main() {
    let args = aoc_utils::app("15").get_matches();
    let mut last = 0;
    let mut round = 1;
    let hm: &mut HashMap<usize, usize> = &mut HashMap::new();
    for line in aoc_utils::file_iter_clap(&args) {
        for token in line.split(',').into_iter() {
            hm.insert(last, round);
            last = token.parse::<usize>().unwrap();
            round += 1;
        }
    }
    for round in round..=30_000_000 {
        let last_said = *hm.get(&last).unwrap_or(&round);
        hm.insert(last, round);
        last = round - last_said;
        if round == 2020 {
            println!("Num {:}: {}", round, last);
        }
    }
    println!("Part 2: {}", last);
}
