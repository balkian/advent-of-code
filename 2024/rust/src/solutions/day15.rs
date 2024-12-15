use std::ops::Add;
use nalgebra::{Point2, Vector2};
use std::collections::VecDeque;

use std::fmt;

type Pos = Point2<usize>;

#[derive(Clone,PartialEq,Eq,Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match &self {
            Dir::Up => {'^'},
            Dir::Down => {'v'},
            Dir::Right => {'>'},
            Dir::Left => {'<'},
        };
        write!(f, "{c}")
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, other: Dir) -> Pos {
        match other {
            Dir::Up => {
                Point2::new(self.coords.x, self.coords.y - 1)
            },
            Dir::Down => {
                Point2::new(self.coords.x, self.coords.y + 1)
            },
            Dir::Right => {
                Point2::new(self.coords.x + 1, self.coords.y)
            },
            Dir::Left => {
                Point2::new(self.coords.x - 1, self.coords.y)
            },
        }
    }

}

#[derive(Clone,PartialEq,Eq,Copy)]
enum Tile {
    Box,
    BoxLeft,
    BoxRight,
    Empty,
    Wall,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match &self {
            Tile::Box => {'O'},
            Tile::BoxLeft => {'['},
            Tile::BoxRight => {']'},
            Tile::Empty => {'.'},
            Tile::Wall => {'#'},
        };
        write!(f, "{c}")
    }
}

#[derive(Clone)]
pub struct Warehouse {
    robot: Pos,
    grid: Vec<Vec<Tile>>,
    instructions: VecDeque<Dir>,
}

impl fmt::Debug for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if y == self.robot.coords.y && x == self.robot.coords.x {
                    write!(f, "@")?;
                }else {
                    write!(f, "{c:?}")?;
                }
            }
            writeln!(f)?;
        }
        for i in &self.instructions {
            write!(f, "{i:?}")?;
        }
        Ok(())
    }
}

impl Warehouse {
    fn advance(&mut self) -> Option<bool> {
        let dir = self.instructions.pop_front()?;
        let n = self.robot + dir;
        if self.push(n, dir) {
            self.robot = n;
            Some(true)
        } else {
            Some(false)
        }
    }

    fn push(&mut self, pos: Pos, dir: Dir) -> bool {
        match (self.grid[pos.coords.y][pos.coords.x], dir) {
            (Tile::Wall, _) => {
                false
            },
            (Tile::Empty, _) => {
                true
            },
            (Tile::Box, _) | (Tile::BoxLeft | Tile::BoxRight, Dir::Left | Dir::Right) => {
                let n = pos + dir;
                if self.push(n, dir) {
                    self.swap(pos, n);
                    return true;
                } else {
                    return false;
                }
            }
            (Tile::BoxLeft, Dir::Up | Dir::Down)  => {
                let n = pos + dir;
                let twinpos = pos + Dir::Right;
                let twinneigh = twinpos + dir;
                if !self.canmove(n, dir) || !self.canmove(twinneigh, dir) {
                    return false;
                }
                self.push(n, dir);
                self.push(twinneigh, dir);
                self.swap(pos, n);
                self.swap(twinpos, twinneigh);
                return true;
            }
            (Tile::BoxRight, Dir::Up | Dir::Down)  => {
                let n = pos + dir;
                let twinpos = pos + Dir::Left;
                let twinneigh = twinpos + dir;
                if !self.canmove(n, dir) || !self.canmove(twinneigh, dir) {
                    return false;
                }
                self.push(n, dir);
                self.push(twinneigh, dir);
                self.swap(pos, n);
                self.swap(twinpos, twinneigh);
                return true;
            }
        }
    }

    fn swap(&mut self, p1: Pos, p2: Pos) {
        let t1 = self.grid[p1.coords.y][p1.coords.x];
        let t2 = self.grid[p2.coords.y][p2.coords.x];
        self.grid[p1.coords.y][p1.coords.x] = t2;
        self.grid[p2.coords.y][p2.coords.x] = t1;
    }

    fn canmove(&mut self, pos: Pos, dir: Dir) -> bool {
        match (self.grid[pos.coords.y][pos.coords.x], dir) {
            (Tile::Wall, _) => {
                false
            },
            (Tile::Empty, _) => {
                true
            },
            (Tile::Box, _) | (Tile::BoxLeft | Tile::BoxRight, Dir::Left | Dir::Right) => {
                let n = pos + dir;
                self.canmove(n, dir)
            }
            (Tile::BoxLeft, Dir::Up | Dir::Down)  => {
                let n = pos + dir;
                self.canmove(n, dir) && self.canmove(n + Dir::Right, dir)
            }
            (Tile::BoxRight, Dir::Up | Dir::Down)  => {
                let n = pos + dir;
                self.canmove(n, dir) && self.canmove(n + Dir::Left, dir)
            }
        }
    }

    fn gps(&self) -> usize {
        let mut score = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if matches!(c, Tile::Box | Tile::BoxLeft) {
                    score += 100 * y + x;
                }
            }
        }
        score
    }

    fn widen(&self) -> Warehouse{
        let mut newgrid = vec![];
        for row in self.grid.iter() {
            let mut newrow = vec![];
            for c in row {
                match c {
                    Tile::Wall => {
                        newrow.push(Tile::Wall);
                        newrow.push(Tile::Wall);
                    },
                    Tile::Empty => {
                        newrow.push(Tile::Empty);
                        newrow.push(Tile::Empty);
                    },
                    Tile::Box => {
                        newrow.push(Tile::BoxLeft);
                        newrow.push(Tile::BoxRight);
                    },
                    _ => {
                        panic!("trying to widen twice!!!!");
                    }
                }
            }
            newgrid.push(newrow);
        }
        let newrobot = Point2::new(self.robot.coords.x * 2, self.robot.coords.y);
        Warehouse{grid: newgrid, robot: newrobot, instructions: self.instructions.clone()}
    }
}

pub fn parse(i: &str) -> Warehouse {
    let mut robot = None;
    let mut grid = vec![];
    let mut lines = i.trim().lines().enumerate();
    for (y, line) in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    robot = Some(Point2::new(x, y));
                    row.push(Tile::Empty);
                },
                'O' => {
                    row.push(Tile::Box);
                },
                '#' => {
                    row.push(Tile::Wall);
                },
                '.' => {
                    row.push(Tile::Empty);
                },
                '\n' | '\r' => {
                    break;
                },
                _ => {panic!("unknown char {c}")}
            }

        }
        grid.push(row);
    }
    let robot = robot.expect("robot not found");

    let mut instructions: VecDeque<_> = Default::default();
    for (_, line) in lines.by_ref() {
        for c in line.trim().chars() {
            match c {
                '<' => {
                    instructions.push_back(Dir::Left);
                },
                '^' => {
                    instructions.push_back(Dir::Up);
                },
                '>' => {
                    instructions.push_back(Dir::Right);
                },
                'v' => {
                    instructions.push_back(Dir::Down);
                },
                _ => {panic!("unknown direction");}
            }
        }
    }

    Warehouse{ robot, grid, instructions }
}

pub fn part1(w: &Warehouse) -> usize {
    let mut w = w.clone();
    while let Some(could) = w.advance() {
        //println!("{w:?}");
    }
    w.gps()
}

pub fn part2(w: &Warehouse) -> usize {
    let mut w = w.widen();
    while let Some(could) = w.advance() {
        //println!("{w:?}");
    }
    w.gps()
}

