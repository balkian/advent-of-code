use std::fmt::{Debug, Error, Formatter};
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum Cell {
    Right,
    Down,
    Empty,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: isize,
    height: isize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f)?;
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .map(|row| -> String {
                    row.iter()
                        .map(|cell| match cell {
                            Cell::Right => ">",
                            Cell::Down => "v",
                            Cell::Empty => ".",
                        })
                        .collect::<String>()
                        + "\n"
                })
                .collect::<String>()
                + "\n"
        )?;
        Ok(())
    }
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '>' => Cell::Right,
                        'v' => Cell::Down,
                        '.' => Cell::Empty,
                        _ => panic!("unknown character {}", c),
                    })
                    .collect()
            })
            .collect();
        Grid {
            height: cells.len() as isize,
            width: cells[0].len() as isize,
            cells,
        }
    }

    fn move_herd(&self, herd: Cell) -> Self {
        let delta = match herd {
            Cell::Down => (0, 1),
            Cell::Right => (1, 0),
            _ => panic!("invalid cell type to move"),
        };
        let mut result = self.clone();

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell != &herd {
                    continue;
                }
                let (nx, ny) = (
                    ((x as isize) + delta.0).rem_euclid(self.width) as usize,
                    ((y as isize) + delta.1).rem_euclid(self.height) as usize,
                );
                // dbg!((x, y, nx, ny, delta));
                if self.cells[ny][nx] == Cell::Empty {
                    result.cells[y][x] = Cell::Empty;
                    result.cells[ny][nx] = herd;
                }
            }
        }
        result
    }

    fn evolve(&self) -> Self {
        let out = self.move_herd(Cell::Right);
        out.move_herd(Cell::Down)
    }
}

pub fn parse(input: &str) -> Grid {
    Grid::from_str(input)
}

pub fn part1(input: &Grid) -> usize {
    let mut last = input.clone();
    for i in 1.. {
        let current = last.evolve();
        // dbg!(&current);
        if current == last {
            return i;
        }
        last = current;
    }
    panic!("No solution found");
}

pub fn part2(_input: &Grid) -> &str {
    "There is no part 2. Congratulations!"
}
