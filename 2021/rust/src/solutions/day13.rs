type Dot = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fold {
    X(usize),
    Y(usize),
}

pub fn parse(input: &str) -> (Vec<Dot>, Vec<Fold>) {
    let mut lines = input.lines();
    let dots = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds = lines
        .map(|line| {
            let (txt, coord) = line.split_once('=').unwrap();
            if txt == "fold along x" {
                Fold::X(coord.parse().unwrap())
            } else {
                Fold::Y(coord.parse().unwrap())
            }
        })
        .collect();
    (dots, folds)
}

pub fn fold(dots: &[Dot], fold: &Fold) -> Vec<Dot> {
    let mut folds: Vec<Dot> = dots
        .iter()
        .copied()
        .map(|(x, y)| match fold {
            Fold::X(fx) => {
                if x > *fx {
                    (2 * fx - x, y)
                } else {
                    (x, y)
                }
            }
            Fold::Y(fy) => {
                if y > *fy {
                    (x, 2 * fy - y)
                } else {
                    (x, y)
                }
            }
        })
        .collect();
    folds.sort();
    folds.dedup();
    folds
}

pub fn part1((dots, folds): &(Vec<Dot>, Vec<Fold>)) -> usize {
    let newdots = fold(dots, &folds[0]);
    newdots.len()
}

fn print(dots: &[Dot]) {
    let mx = *dots.iter().map(|(x, _y)| x).max().unwrap();
    let my = *dots.iter().map(|(_x, y)| y).max().unwrap();
    println!();
    for j in 0..=my {
        for i in 0..=mx {
            let c = if dots.contains(&(i, j)) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn part2((dots, folds): &(Vec<Dot>, Vec<Fold>)) -> &str {
    let mut newdots = dots.clone();
    for f in folds {
        newdots = fold(&newdots, f);
    }
    print(&newdots);
    "^^^ look up ^^^"
}

#[test]
fn test_example() {
    let input = &parse(
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
    );
    assert_eq!(part1(input), 17);
}
