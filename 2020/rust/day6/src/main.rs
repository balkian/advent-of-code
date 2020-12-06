use std::collections::HashSet;


fn main() {
    let counts: (usize, usize) = aoc_utils::file_iter_blocks(
        |line| {
            line.chars().collect::<HashSet<char>>()
        },
        |mut block| {
            let mut p1 = block.pop().unwrap();
            let mut p2 = p1.clone();
            for line in &block{
                p1.extend(line);
                let after = &p2 & line;
                p2 = after;
            }
            (p1.len(), p2.len())
        })
        .fold((0, 0), |c, x| {
            (c.0+x.0, c.1+x.1)
        });
    // let counts: (usize, usize) = counts.iter().sum::<usize>();
    println!("Part 1: {:?}", counts.0);
    println!("Part 2: {:?}", counts.1);
}
