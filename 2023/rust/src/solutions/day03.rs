use std::cmp::min;
use std::collections::BTreeSet;

#[derive(Debug)]
struct NumberIndex {
    number: usize,
    nline: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Tile {
    Dot,
    Symbol(char),
    Number(usize),
}

#[derive(Debug)]
pub struct Board {
    numbers: Vec<NumberIndex>,
    grid: Vec<Vec<Tile>>,
}

impl Board {
    fn part_number_sum(&self) -> usize {
        self.numbers
            .iter()
            .filter_map(
                |NumberIndex {
                     number,
                     nline,
                     start,
                     end,
                 }| {
                    let ymin = nline.saturating_sub(1);
                    let ymax = min(self.grid.len(), nline + 2);
                    for row in &self.grid[ymin..ymax] {
                        let xmin = start.saturating_sub(1);
                        let xmax = min(row.len(), end + 1);
                        for tile in &row[xmin..xmax] {
                            if let Tile::Symbol(_) = tile {
                                return Some(number);
                            }
                        }
                    }
                    None
                },
            )
            .sum()
    }

    fn gear_ratio_sum(&self) -> usize {
        let mut sum: usize = 0;
        for (ix, row) in self.grid.iter().enumerate() {
            for (jx, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Symbol('*')) {
                    let ymin = ix.saturating_sub(1);
                    let ymax = min(ix + 2, self.grid.len());
                    let xmin = jx.saturating_sub(1);
                    let xmax = min(jx + 2, self.grid.len());

                    let mut adjacent = BTreeSet::new();

                    for y in ymin..ymax {
                        for x in xmin..xmax {
                            if let Tile::Number(a) = self.grid[y][x] {
                                adjacent.insert(a);
                            }
                        }
                    }
                    if adjacent.len() == 2 {
                        sum += adjacent
                            .into_iter()
                            .map(|ix| self.numbers[ix].number)
                            .product::<usize>();
                    }
                }
            }
        }
        sum
    }
}

pub fn parse(input: &str) -> Board {
    let mut grid = vec![];
    let mut numbers = vec![];
    for (nline, line) in input.lines().enumerate() {
        let mut row = vec![];
        let mut start = None;
        let add_number = |numbers: &mut Vec<_>, start: &mut Option<usize>, end: usize| {
            if let Some(start) = start.take() {
                let number = line[start..end]
                    .parse::<usize>()
                    .expect("could not parse number");
                numbers.push(NumberIndex {
                    number,
                    nline,
                    start,
                    end,
                });
            }
        };

        for (ix, c) in line.char_indices() {
            match c {
                '0'..='9' => {
                    row.push(Tile::Number(numbers.len()));
                    start.get_or_insert(ix);
                    continue; // Only case where number may still continue
                }
                '\n' => {}
                '.' => row.push(Tile::Dot),
                a => row.push(Tile::Symbol(a)),
            }
            add_number(&mut numbers, &mut start, ix);
        }
        add_number(&mut numbers, &mut start, line.len());
        grid.push(row);
    }
    Board { numbers, grid }
}

pub fn part1(input: &Board) -> usize {
    input.part_number_sum()
}

pub fn part2(input: &Board) -> usize {
    input.gear_ratio_sum()
}
