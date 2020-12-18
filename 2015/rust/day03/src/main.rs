use std::collections::HashSet;
use std::env;

fn solve1(input: &str) -> usize {
    solve(input).len()
}

fn solve2(input: &str) -> usize {
    let santa = solve(&input.chars().step_by(2).collect::<String>());
    let robo = solve(&input.chars().skip(1).step_by(2).collect::<String>());
    santa.union(&robo).count()
}

fn solve(input: &str) -> HashSet<(isize, isize)> {
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    let mut current = (0isize, 0isize);
    positions.insert(current);
    for line in input.lines() {
        for c in line.chars() {
            match c {
                '>' => {
                    current.0 += 1;
                }
                '<' => {
                    current.0 -= 1;
                }
                '^' => {
                    current.1 -= 1;
                }
                'v' => {
                    current.1 += 1;
                }
                _ => panic!("invalid position"),
            }
            positions.insert(current);
        }
    }
    positions
}

fn main() {
    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = std::fs::read_to_string(file).expect("invalid file");
    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve1(">"), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(solve1("^>v<"), 4);
    }
    #[test]
    fn test3() {
        assert_eq!(solve1("^v^v^v^v^v"), 2);
    }
}
