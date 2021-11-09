use std::ops::Add;
use std::{fmt, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Solution 1: {:?}", solve1(&input));
    println!("Solution 2: {:?}", solve2(&input));
}

fn solve1(input: &str) -> Position {
    let mut map = parse(input);
    for _ in 0.. {
        if map.step() {
            return map.carts.iter().find(|c| !c.alive).unwrap().pos;
        }
    }
    unreachable!();
}

fn solve2(input: &str) -> Position {
    let mut map = parse(input);
    for i in 0.. {
        if i % 100 == 0 {
            print!(".");
        }
        if map.step() {
            let mut ix = 0;
            while ix < map.carts.len() {
                if !map.carts[ix].alive {
                    map.carts.remove(ix);
                } else {
                    ix += 1;
                }
            }
            if map.carts.len() == 1 {
                println!();
                return map.carts[0].pos;
            }
        }
    }
    unreachable!();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(usize, usize);
struct Direction(isize, isize);

impl Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, other: &Direction) -> Self::Output {
        Position(
            ((self.0 as isize) + other.0) as usize,
            ((self.1 as isize) + other.1) as usize,
        )
    }
}

struct Cart {
    pos: Position,
    alive: bool,
    dir: Direction,
    cross: usize,
}

struct Map {
    grid: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut overlay = self.grid.clone();

        for (ix, cart) in self.carts.iter().enumerate() {
            writeln!(f, "Cart {}: <{:3},{:3}>", ix, cart.pos.0, cart.pos.1)?;
            overlay[cart.pos.1][cart.pos.0] = '@';
        }
        writeln!(f)?;
        for row in overlay.iter() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Map {
    fn connected(&self, x: usize, y: usize) -> (bool, bool, bool, bool) {
        (
            x > 0 && matches!(self.grid[y][x - 1], '/' | '\\' | '-' | '+'),
            y > 0 && matches!(self.grid[y - 1][x], '\\' | '/' | '|' | '+'),
            x < self.grid[y].len() - 1 && matches!(self.grid[y][x + 1], '\\' | '/' | '-' | '+'),
            y < self.grid.len() - 1 && matches!(self.grid[y + 1][x], '\\' | '/' | '|' | '+'),
        )
    }

    fn step(&mut self) -> bool {
        self.carts.sort_by_key(|c| (c.pos.1, c.pos.0));
        let mut new_carts = Vec::with_capacity(self.carts.len());
        let mut crashed = false;
        for _ in 0..self.carts.len() {
            let mut cart = self.carts.remove(0);
            if !cart.alive {
                new_carts.push(cart);
                continue;
            }
            let next = &cart.pos + &cart.dir;
            cart.pos = next;
            match self.grid[cart.pos.1][cart.pos.0] {
                '+' => {
                    match cart.cross % 3 {
                        0 => cart.dir = Direction(cart.dir.1, -cart.dir.0),
                        1 => {}
                        2 => cart.dir = Direction(-cart.dir.1, cart.dir.0),
                        _ => unreachable!(),
                    };
                    cart.cross += 1;
                }
                ' ' => panic!("derailed"),
                '/' => {
                    cart.dir = Direction(-cart.dir.1, -cart.dir.0);
                }
                '\\' => {
                    cart.dir = Direction(cart.dir.1, cart.dir.0);
                }
                '-' if cart.dir.0 == 1 || cart.dir.0 == -1 => {}
                '|' if cart.dir.1 == 1 || cart.dir.1 == -1 => {}
                _ => panic!("unknown state"),
            }
            self.carts
                .iter_mut()
                .chain(new_carts.iter_mut())
                .for_each(|other| {
                    if cart.pos == other.pos {
                        crashed = true;
                        cart.alive = false;
                        other.alive = false;
                    }
                });
            new_carts.push(cart);
        }
        self.carts = new_carts;
        crashed
    }
}

fn parse(input: &str) -> Map {
    let carts = vec![];
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut map = Map { carts, grid };
    for j in 0..map.grid.len() {
        for i in 0..map.grid[j].len() {
            let dir = match map.grid[j][i] {
                '>' => Direction(1, 0),
                '<' => Direction(-1, 0),
                'v' => Direction(0, 1),
                '^' => Direction(0, -1),
                _ => continue,
            };
            map.carts.push(Cart {
                pos: Position(i, j),
                dir,
                alive: true,
                cross: 0,
            });
            let replacement = match map.connected(i, j) {
                (true, true, false, false) | (false, false, true, true) => '/',
                (false, true, true, false) | (true, false, false, true) => '\\',
                (true, false, true, false) => '-',
                (false, true, false, true) => '|',
                _ => '+',
            };
            map.grid[j][i] = replacement;
        }
    }
    map
}

#[test]
fn test_example() {
    let track = r"
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
";
    assert_eq!(solve1(&track), Position(7, 3));
}
