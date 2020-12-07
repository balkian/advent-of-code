use std::collections::HashSet;

fn parse_num(mut s: String) -> isize {
    s = s.replace("B", "1");
    s = s.replace("F", "0");
    s = s.replace("R", "1");
    s = s.replace("L", "0");
    isize::from_str_radix(&s, 2).unwrap()
}

fn main() {
    let mut set: HashSet<isize> = HashSet::new();
    for i in aoc_utils::file_iter().map(parse_num) {
        set.insert(i);
    }
    println!("The highest seat is: {:?}", set.iter().max().unwrap());
    let mut candidates = HashSet::new();
    for i in &set {
        if !set.contains(&(i + 1)) {
            candidates.insert(i + 1);
        }
        if !set.contains(&(i - 1)) {
            candidates.insert(i - 1);
        }
    }
    let mut candidates: Vec<isize> = candidates.into_iter().collect();
    candidates.sort_unstable();
    println!("Candidates: {:?}", candidates);
    let your_seat = candidates[1];
    println!("Your seat is: {:?}", your_seat);
}
