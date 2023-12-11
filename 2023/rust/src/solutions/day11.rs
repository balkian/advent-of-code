type Coord = [usize; 2];

#[derive(Debug,Clone)]
pub struct Universe {
    // grid: Vec<Vec<bool>>,
    galaxies: Vec<Coord>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Universe {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input
            .lines()
            .filter(|line| !line
            .is_empty())
            .map(|line| line.chars().map(|c| {
                    match c { 
                        '#' => true,
                        '.' => false,
                        c => panic!("unknown character {c}")
                    }
                }).collect()
            ).collect();
        let galaxies = grid.iter().enumerate().flat_map(|(ix, row)| row.iter().enumerate().filter(|(_, cell)| **cell).map(move |(jx, _)| [ix, jx])).collect();
        let mut empty_rows = vec![];
        let mut empty_cols = vec![];
        //begin expansion 
        if grid.is_empty() {
            panic!("Empty galaxy input");
        }
        for (ix, row) in grid.iter().enumerate() {
            if row.iter().all(|c| !c) {
                empty_rows.push(ix);
            }
        }

        // Assume all rows have the same length
        for jx in 0..grid[0].len() {
            let mut all = true;
            for row in grid.iter() {
                if row[jx] {
                    all = false;
                    break;
                }
            }
            if all {
                empty_cols.push(jx);
            }
        }
        Universe{galaxies, empty_rows, empty_cols}
    }

    fn distances(&self, age: usize) -> usize {
        let mut total = 0;
        for (ix, gal) in self.galaxies.iter().enumerate().skip(1) {
            for (_jx, gal2) in self.galaxies.iter().enumerate().take(ix) {
                let (y0, y1) = if gal[1] < gal2[1] {
                    (gal[1], gal2[1])
                } else {
                    (gal2[1], gal[1])
                };
                let (x0, x1) = if gal[0] < gal2[0] {
                    (gal[0], gal2[0])
                } else {
                    (gal2[0], gal[0])
                };
                let r_x = x0..x1;
                let r_y = y0..y1;
                let mut dist = r_x.len() + r_y.len();
                // dbg!([ix, jx], &r_x, &r_y);
                // dbg!(&dist);
                for row in r_x {
                    if self.empty_rows.contains(&row) {
                        dist += age;
                    }
                }
                for col in r_y {
                    if self.empty_cols.contains(&col) {
                        dist += age;
                    }
                }
                // dbg!(&dist);
                total += dist;
            }
        }
        total
    }
}

pub fn parse(input: &str) -> Universe {
    Universe::new(input)
}

pub fn part1(universe: &Universe) -> usize {
    universe.distances(1)
}

pub fn part2(universe: &Universe) -> usize {
    universe.distances(1000000-1)
}
