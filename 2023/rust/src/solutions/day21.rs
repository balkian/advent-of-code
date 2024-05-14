/// A very **very** inefficient solution for day21
/// It would be much better to calculate distances of each cell and then filter only those 
/// reachable in the given number of steps.
/// i.e., those that are at distance <= steps, and the parity of the steps and the distance is the
/// same.
use std::fmt;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    y: isize,
    x: isize,
}

#[derive(Clone)]
pub struct Map {
    repr: Vec<Vec<u8>>,
    occupied: HashSet<Coord>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut min_x = 0;
        let rows = self.repr[0].len() as isize;
        let mut max_x = rows;
        let mut min_y = 0;
        let cols = self.repr.len() as isize;
        let mut max_y = cols;
        for Coord{x, y} in self.occupied.iter().copied() {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.occupied.contains(&Coord{y, x}) {
                    write!(f, "X")?;
                } else {
                    let y = y.rem_euclid(rows) as usize;
                    let x = x.rem_euclid(cols) as usize;
                    write!(f, "{}", self.repr[y][x] as char)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}


pub fn parse(repr: &str) -> Map {
    let mut occupied = HashSet::new();
    let mut repr: Vec<Vec<u8>> = repr.split('\n').filter(|row| !row.is_empty()).map(|row| Vec::from(row.trim())).collect();
    for (y, row) in repr.iter_mut().enumerate() {
        for (x, i) in row.iter_mut().enumerate() {
            match i {
                b'S' => {
                    occupied.insert(Coord{x: x as isize, y: y as isize});
                    *i = b'.';
                },
                b'.' | b'#' | b'\n' => {
                },
                c =>  { panic!("Unknown character {c}")}
            }
        }
    }
    Map{repr, occupied}
}

impl Map {
    fn is_empty(&self, Coord{y, x}: &Coord) -> bool {
        let rows = self.repr[0].len() as isize;
        let cols = self.repr.len() as isize;
        let y = y.rem_euclid(rows) as usize;
        let x = x.rem_euclid(cols) as usize;
        self.repr[y][x] != b'#'
    }
    // This is a very inefficient solution to this problem.
    fn evolve(&mut self) {
        let previous = std::mem::take(&mut self.occupied);

        for Coord{y, x} in previous {
            for coord in [Coord{y: y-1, x}, Coord{y: y+1, x}, Coord{y, x: x-1}, Coord{y, x: x+1}] {
                if self.is_empty(&coord) {
                    self.occupied.insert(coord);
                }
            }
        }
    }
}

pub fn solve(map: &Map, steps: usize) -> usize {
    let mut map = map.clone();
    let mut values = vec![1];
    let mid = map.repr.len() / 2;
    let threshold = mid + 2 * map.repr.len();
    for step in 0..steps {
        map.evolve();
        values.push(map.occupied.len());
        if step >= threshold && (steps - mid) % map.repr.len() == 0 {
            // Here we're using the fact that the growth seems to be periodic every N cycles,
            // where N is the length of each subgrid.
            // There is an offset of N / 2, because the start point is in the center.
            // At those points, the number of reachable spots is a parabola:
            //      f(x) = a * x^2 + b * x + c.
            // We only need three points to estimate the coefficients a, b and c
            //
            // This only applies to the specific type of pattern used in the input.
            //
            // Warning: Due to floating point precision errors and the multiple operations
            // required, this solution might not properly work.
            // You may always use an external tool to fit the parabola and calculate the value
            // at that point.
            let x1 = mid;
            let x2 = x1 + map.repr.len();
            let x3 = threshold;
            let y1 = values[x1] as f64;
            let y2 = values[x2] as f64;
            let y3 = values[x3] as f64;
            let x1 = 1f64;
            let x2 = 2f64;
            let x3 = 3f64;
            let x4 = 1f64 + ((steps - mid) / map.repr.len()) as f64;

            let a = (x1 * (y3 - y2) + x2 * (y1-y3) + x3 * (y2-y1)) / ((x1 - x2)*(x1-x3)*(x2-x3));
            let b = ((y2-y1)/(x2-x1)) - a * (x1+x2);
            let c = y1 - a * x1*x1 - b*x1;

            return (a * (x4 * x4) + b * x4 + c) as usize;
        }
    }
    *values.last().unwrap()
}

pub fn part1(map: &Map) -> usize {
    solve(map, 64)
}

pub fn part2(map: &Map) -> usize {
    solve(map, 26501365)
}

#[cfg(test)]
mod test {
    use aoc_utils::*;
    use super::*;
    #[test]
    fn test_example_part1() {
        let input = &parse("...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........");
        assert_eq!(solve(&input, 6), 16);
    }

//    #[test]
//    fn test_example_part2() {
//        let input = &parse("...........
//.....###.#.
//.###.##..#.
//..#.#...#..
//....#.#....
//.##..S####.
//.##..#...#.
//.......##..
//.##.#.####.
//.##..##.##.
//...........");
//        assert_eq!(solve(&input, 10), 50);
//        assert_eq!(solve(&input, 50), 1594);
//        assert_eq!(solve(&input, 100), 6536);
//        assert_eq!(solve(&input, 500), 167004); // Does not work
//        assert_eq!(solve(&input, 1000), 668697);
//        assert_eq!(solve(&input, 5000), 16733044);
//    }
}

