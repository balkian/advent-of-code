use std::cmp::{max,min};
use std::collections::HashSet;

type Coord = [isize; 4];

fn dist(p1: &Coord, p2: &Coord) -> usize {
    p1.iter().zip(p2.iter()).map(|(a,b)| max(a,b) - min(a,b)).sum::<isize>() as usize
}

fn solve1(input: &str) -> usize {
    let coords = parse(input);
    let mut group: Vec<usize> = (0..coords.len()).collect();
    for ix in 0..coords.len()-1 {
        let c1 = &coords[ix];
        for jx in ix+1..coords.len() {
            let c2 = &coords[jx];
            if dist(c1, c2) <= 3 {
                let g1 = group[ix];
                let g2 = group[jx];
                if g2 == jx {
                    group[jx] = g1;
                } else {
                    for g in group.iter_mut(){
                        if g == &g2 {
                            *g = g1;
                        }
                    }
                }
            }
        }
    }

    let uniq: HashSet<usize> = group.iter().copied().collect();
    uniq.len()
}

fn parse(input: &str) -> Vec<Coord> {
   input.lines().filter(|l| !l.is_empty()).map(|l| l.split(',').map(|p| p.parse::<isize>().unwrap()).collect::<Vec<isize>>().as_slice().try_into().unwrap()).collect()
}

fn main() {
    let input = &std::fs::read_to_string("input").unwrap();
    println!("Solution 1: {}", solve1(input));
}

#[test]
fn test_example(){
    let ex4 = "
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
    assert_eq!(solve1(ex4), 4);
}
