use std::cmp::min;
type Grid = Vec<Vec<usize>>;

fn print_grid(grid: &Grid) {
    println!();
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            match grid[j][i] {
                0 => print!("."),
                1 => print!("#"),
                _ => panic!("unknown char"),
            }
        }
        println!();
    }
}
fn change_stuck(grid: &mut Grid) {
    for y in [0, grid.len() - 1] {
        for x in [0, grid[y].len() - 1] {
            grid[y][x] = 1;
        }
    }
}

fn mutate(grid: &Grid) -> Grid {
    let mut next = grid.clone();
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            let ys = (j.saturating_sub(1), min(grid.len() - 1, j + 1));
            let xs = (i.saturating_sub(1), min(grid[j].len() - 1, i + 1));
            let around = (ys.0..=ys.1)
                .flat_map(|y| {
                    (xs.0..=xs.1)
                        .filter(move |x| (*x, y) != (i, j))
                        .map(move |x| grid[y][x])
                })
                .sum::<usize>();

            match (grid[j][i], around) {
                (1, a) if a < 2 || a > 3 => next[j][i] = 0,
                (0, 3) => next[j][i] = 1,
                _ => {} // DO nothing, value did not change
            }
        }
    }
    next
}

pub fn parse(input: &str) -> Grid {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &Grid) -> usize {
    evolve(input, 100, false)
}

pub fn evolve(input: &Grid, times: usize, stuck: bool) -> usize {
    let mut input = input.clone();
    stuck.then(|| change_stuck(&mut input));
    for step in 0..times {
        if cfg!(debug_assertions) {
            println!("Step: {}", step);
            print_grid(&input);
        }
        input = mutate(&input);
        stuck.then(|| change_stuck(&mut input));
    }
    input.iter().flatten().sum::<usize>()
}

pub fn part2(input: &Grid) -> usize {
    evolve(input, 100, true)
}

#[test]
fn test_example() {
    let input = parse(
        ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
    );
    assert_eq!(evolve(&input, 4, false), 4);
}
