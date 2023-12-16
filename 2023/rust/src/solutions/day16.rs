use std::ops::Add;

#[derive(Debug,Clone,Copy)]
struct Position { 
    x: isize,
    y: isize,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

impl Direction {
    fn right(&self) -> Self {
        match self {
            North => West, 
            South => East,
            East => South,
            West => North,
        }
    }
    fn left(&self) -> Self {
        match self {
            North => East, 
            South => West,
            East => North,
            West => South,
        }
    }
}

impl Add<&Direction> for Position {
    type Output = Self;

    fn add(self, other: &Direction) -> Self {
        match other {
            North => Self{x: self.x, y: self.y-1},
            South => Self{x: self.x, y: self.y+1},
            East => Self{x: self.x-1, y: self.y},
            West => Self{x: self.x+1, y: self.y},
        }
    }
}


#[derive(Debug,Clone,Copy)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Right,
    Left,
}

use Tile::*;


#[derive(Debug,Clone,Copy)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {

    fn left(&mut self) {
        self.direction = self.direction.left();
        self.next();
    }
    fn right(&mut self) {
        self.direction = self.direction.right();
        self.next();
    }
    fn next(&mut self) {
        self.position = self.position + &self.direction;
    }

    fn reflect(&mut self, tile: Tile) -> Option<Beam> {
        match (tile, self.direction) {
            (Tile::Empty, _) | (Tile::Horizontal, West | East) | (Tile::Vertical, North | South) => {
                self.next();
                None
            },
            (Tile::Right, _) => {
                self.right();
                None
            },
            (Tile::Left, _) => {
                self.left();
                None
            },
            (Tile::Horizontal, North | South ) | (Tile::Vertical, West | East) => {
                let mut twin = *self;
                self.left();
                twin.right();
                Some(twin)
            },
        }
    }
}

#[derive(Debug,Clone,Default)]
struct EnergyState {
    directions: Vec<Direction>,
}


#[derive(Debug,Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    energy: Vec<Vec<EnergyState>>,
}

impl Grid {
    fn get_xy(&self, Position{x, y}: Position) -> Option<(usize, usize)> {
        if y >= 0 && (y as usize) < self.tiles.len() {
            let y = y as usize;
            let row = &self.tiles[y];
            if x >=0 && (x as usize) < row.len() {
               return Some((y, x as usize))
            }
        }
        None
    }

    fn energize(&mut self, beam: Beam) {
        let mut beams = vec![beam];

        while let Some(mut beam) = beams.pop() {
            let Some((y, x)) = self.get_xy(beam.position) else {
                continue;
            };
            let energy = &mut self.energy[y][x];
            if energy.directions.contains(&beam.direction) {
                continue
            }
            energy.directions.push(beam.direction);
            let tile = self.tiles[y][x];
            if let Some(reflection) = beam.reflect(tile) {
                beams.push(reflection);
            }
            beams.push(beam)
        }
    }
}



pub fn parse(input: &str) -> Grid {
    let tiles: Vec<Vec<_>> = input.lines().filter(|line| !line.is_empty())
    .map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => Empty,
                    '/' => Right,
                    '\\' => Left,
                    '-' => Horizontal,
                    '|' => Vertical,
                    _ => panic!("unknown tile {c}")
                }
            }).collect()
        }).collect();
    let energy = vec![vec![Default::default(); tiles[0].len()]; tiles.len()];
    Grid{tiles, energy}
}

pub fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.energize(Beam{position: Position{x: 0, y:0}, direction: West});
    grid.energy.iter().flatten().filter(|e| !e.directions.is_empty()).count()
}

pub fn part2(grid: &Grid) -> usize {
    let mut max = 0;
    let mut beams = vec![];

    
    for x in 0..grid.tiles[0].len() {
        beams.push(Beam{direction: South, position: Position{y: 0, x: x as isize} });
        beams.push(Beam{direction: North, position: Position{y: (grid.tiles.len()-1) as isize, x: x as isize} });
    }
    for y in 0..grid.tiles.len() {
        beams.push(Beam{direction: West, position: Position{y: y as isize, x: 0} });
        beams.push(Beam{direction: East, position: Position{y: y as isize, x: (grid.tiles[y].len()-1) as isize} });
    }

    for beam in beams {
        let mut grid = grid.clone();
        grid.energize(beam);
        let count = grid.energy.iter().flatten().filter(|e| !e.directions.is_empty()).count();
        if count > max {
            max = count;
        }
    }
    max
}
