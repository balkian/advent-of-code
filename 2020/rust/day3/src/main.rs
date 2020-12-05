use aoc_utils;

fn part1(map: &Vec<String>, slope_x: usize, slope_y: usize) -> i64 {
    let mut pos_x = 0;
    let mut trees = 0;
    for i in (0..map.len()).step_by(slope_y) {
        // dbg!(&map[i], &map[i].chars(), &pos_x);
        if map[i].chars().nth(pos_x).unwrap() == '#' {
            trees += 1;
        }
        pos_x = (pos_x+slope_x) % map[i].len();
    }
    trees
}

fn part2(map: &Vec<String>) -> i64 {
    let slopes = vec!((1,1), (3,1), (5,1), (7,1), (1,2));
    let mut count = 1;
    for (slope_x, slope_y) in slopes {
        // dbg!(slope_x, slope_y);
        count = count * part1(&map, slope_x, slope_y);
    }
    // println!("{:}", count);
    count
}

fn main() {
    let it: Vec<String> = aoc_utils::file_iter().collect();
    let p1 = part1(&it, 3, 1);
    println!("P1: Encountered {:} trees", p1);
    let p2 = part2(&it);
    println!("P2: Encountered {:} trees", p2);
}
