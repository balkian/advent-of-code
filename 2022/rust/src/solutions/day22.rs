use nalgebra::{Point3, Vector2, Vector3};
use std::collections::{HashMap, HashSet};
#[derive(Clone, Debug)]
enum Tile {
    Void,
    Empty,
    Wall,
}

type Dir = Vector2<isize>;

#[derive(Clone, Debug)]
enum Command {
    Move(usize),
    Right,
    Left,
}

static NORTH: Vector2<isize> = Vector2::new(-1, 0);
static EAST: Vector2<isize> = Vector2::new(0, 1);
static SOUTH: Vector2<isize> = Vector2::new(1, 0);
static WEST: Vector2<isize> = Vector2::new(0, -1);

type Pos2 = (usize, usize);

#[derive(Clone, Debug)]
pub struct Map {
    grid: Vec<Vec<Tile>>,
    cmds: Vec<Command>,
    dir: Dir,
    pos: Pos2,
}

impl Map {
    #[allow(dead_code)]
    fn draw(&self) {
        println!();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if (i, j) == self.pos {
                    print!("@");
                    debug_assert!(matches!(self.grid[self.pos.0][self.pos.1], Tile::Empty));
                    continue;
                }
                let c = match cell {
                    Tile::Void => ' ',
                    Tile::Wall => '#',
                    Tile::Empty if (i, j) == self.pos => '@',
                    _ => '.',
                };
                print!("{}", c);
            }
            println!();
        }
    }
    fn get_pos(&self, pos: (usize, usize), delta: Dir) -> (usize, usize) {
        let mut pos = pos;
        loop {
            let y = ((pos.0 + self.grid.len()) as isize + delta[0]) as usize % self.grid.len();
            pos = (
                y,
                ((pos.1 + self.grid[y].len()) as isize + delta[1]) as usize % self.grid[y].len(),
            );
            if !matches!(self.grid[pos.0][pos.1], Tile::Void) {
                return pos;
            }
        }
    }
    fn walk(&self, steps: usize) -> ((usize, usize), usize) {
        let delta = self.dir;
        let mut pos = self.pos;
        let mut i = 0;
        while i < steps {
            let next_pos = self.get_pos(pos, delta);
            match self.grid[next_pos.0][next_pos.1] {
                Tile::Wall => break,
                Tile::Void => panic!("get_pos should not return a void"),
                _ => {}
            }
            i += 1;
            pos = next_pos;
        }
        (pos, i)
    }
    fn score(&self) -> usize {
        let dir_score = match self.dir {
            d if d == EAST => 0,
            d if d == SOUTH => 1,
            d if d == WEST => 2,
            d if d == NORTH => 3,
            _ => panic!(),
        };
        1000 * (self.pos.0 + 1) + 4 * (self.pos.1 + 1) + dir_score
    }
    fn run(&mut self) {
        for d in self.cmds.iter() {
            match d {
                Command::Move(i) => {
                    let pos = self.walk(*i).0;
                    self.pos = pos;
                }
                Command::Right => {
                    self.dir = Vector2::new(self.dir[1], -self.dir[0]);
                }
                Command::Left => {
                    self.dir = Vector2::new(-self.dir[1], self.dir[0]);
                }
            }
            // self.draw();
        }
    }
}

pub fn parse(input: &str) -> Map {
    let mut it = input.lines();
    let mut grid: Vec<Vec<_>> = it
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.trim_end_matches('\n')
                .chars()
                .map(move |c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    ' ' => Tile::Void,
                    _ => panic!("wrong character {c}"),
                })
                .collect()
        })
        .collect();
    let max_width = grid.iter().map(|row| row.len()).max().unwrap_or_default();
    for row in grid.iter_mut() {
        row.extend(
            std::iter::once(Tile::Void)
                .cycle()
                .take(max_width - row.len()),
        );
    }

    debug_assert!(!grid
        .iter()
        .any(|row| row.iter().all(|c| matches!(c, Tile::Void))));
    debug_assert!(
        !(0..grid[0].len()).any(|idx| grid.iter().all(|row| matches!(row[idx], Tile::Void)))
    );
    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, t)| ((i, j), t.clone()))
        })
        .filter(|(_pos, t)| matches!(t, Tile::Empty))
        .next()
        .unwrap()
        .0;

    let mut cmds = vec![];
    let mut buff: String = String::new();
    for c in it.next().unwrap().chars() {
        if ('0'..='9').contains(&c) {
            buff.push(c);
            continue;
        }
        if buff.len() > 0 {
            cmds.push(Command::Move(buff.parse().unwrap()));
            buff.clear();
        }
        let d = match c {
            'R' => Command::Right,
            'L' => Command::Left,
            '\n' => continue,
            _ => panic!("unexpected char {c}"),
        };
        cmds.push(d);
    }
    if !buff.is_empty() {
        cmds.push(Command::Move(buff.parse().unwrap()));
    }
    Map {
        grid,
        pos: start,
        dir: EAST,
        cmds,
    }
}

pub fn part1(input: &Map) -> usize {
    let mut map = input.clone();
    map.run();
    map.score()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos3D {
    point: Point3<isize>,
    normal: Vector3<isize>,
}

#[derive(Debug, Clone)]
struct Cube {
    side: usize,
    map: Map,
    walls: HashSet<Pos3D>,
    mapping: HashMap<Pos3D, Pos2>,
    cursor: Pos3D,
    dir: Vector3<isize>,
    cmds: Vec<Command>,
}

impl Cube {
    fn continue_or_fold(&self, pos: &Pos3D, dir: &Vector3<isize>) -> (Pos3D, Vector3<isize>) {
        let new_point = pos.point + dir;
        if !new_point.iter().any(|&a| a < 0 || a >= self.side as isize) {
            (
                Pos3D {
                    point: new_point,
                    normal: pos.normal,
                },
                dir.clone(),
            )
        } else {
            let new_dir = -pos.normal;
            let new_pos = Pos3D {
                normal: dir.clone(),
                point: pos.point,
            };
            (new_pos, new_dir)
        }
    }

    fn from_map(map: &Map) -> Self {
        let tiles: HashSet<_> = map
            .grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, t)| {
                    if !matches!(t, Tile::Void) {
                        None
                    } else {
                        Some((i, j))
                    }
                })
            })
            .collect();
        // map.draw();
        // dbg!(map.grid.len(), Pos3Dap.grid[0].len());
        let side = ((tiles.len() / 6) as f64).sqrt() as usize;
        let walls: HashSet<_> = Default::default();
        let mapping = HashMap::new();

        // Start from 1,1 to avoid folding this time.
        let start3d = Pos3D {
            point: Point3::new(1, 1, 0),
            normal: Vector3::new(0, 0, -1),
        };
        let startdir = Vector3::new(0, 1, 0);
        let start2d = (map.pos.0 + 1, map.pos.1 + 1);
        let start2ddir = EAST;

        let mut cube = Cube {
            map: map.clone(),
            side,
            walls,
            mapping,
            cursor: Pos3D {
                point: Default::default(),
                normal: start3d.normal.clone(),
            },
            dir: startdir,
            cmds: map.cmds.clone(),
        };

        let mut queue = vec![];
        queue.push((start2d, start2ddir, start3d.clone(), startdir.clone()));
        let mut walls_2d = HashSet::new();

        while let Some((pos2, dir2, pos3, dir3)) = queue.pop() {
            let key = pos3.clone();
            if cube.mapping.contains_key(&key) {
                continue;
            }
            match map.grid[pos2.0][pos2.1] {
                Tile::Void => continue,
                Tile::Wall => {
                    cube.walls.insert(pos3.clone());
                    walls_2d.insert(pos2.clone());
                }
                _ => {}
            }
            cube.mapping.insert(key, pos2);
            let four_directions = [
                (dir2, dir3.clone()),
                (-dir2, -dir3),
                (Vector2::new(dir2[1], -dir2[0]), pos3.normal.cross(&dir3)),
                (Vector2::new(-dir2[1], dir2[0]), -(pos3.normal.cross(&dir3))),
            ];
            for (d2, d3) in four_directions {
                let new_pos2 = (pos2.0 as isize + d2[0], pos2.1 as isize + d2[1]);
                if new_pos2.0 < 0
                    || new_pos2.0 >= map.grid.len() as isize
                    || new_pos2.1 < 0
                    || new_pos2.1 >= map.grid[new_pos2.0 as usize].len() as isize
                {
                    continue;
                }
                let new_pos2 = (new_pos2.0 as usize, new_pos2.1 as usize);
                // let mut new_pos3 = pos3.clone();
                // new_pos3.point += d3;
                let (new_pos3, d3) = cube.continue_or_fold(&pos3, &d3);
                queue.push((new_pos2, d2, new_pos3, d3));
            }
        }
        // dbg!(&cube.walls);
        // dbg!(&cube.walls.len());
        // dbg!(orig_walls.len(), walls_2d.len());
        // dbg!(orig_walls.difference(&walls_2d));
        // dbg!(walls_2d.difference(&orig_walls));
        // dbg!(tiles.difference(&visited).count());
        // dbg!(visited.difference(&tiles).count());
        debug_assert_eq!((cube.side * cube.side * 6), (cube.mapping.len()));
        cube
    }

    fn run(&mut self) {
        for d in self.cmds.iter() {
            // dbg!(d);
            match d {
                Command::Move(steps) => {
                    let mut i = 0;
                    while i < *steps {
                        let (next_cursor, next_dir) =
                            self.continue_or_fold(&self.cursor, &self.dir);
                        if self.walls.contains(&next_cursor) {
                            break;
                        }
                        i += 1;
                        self.cursor = next_cursor;
                        self.dir = next_dir;
                        self.map.pos = self.mapping[&self.cursor];
                        // self.map.draw();
                    }
                }
                Command::Right => {
                    self.dir = self.cursor.normal.cross(&self.dir);
                }
                Command::Left => {
                    self.dir = -self.cursor.normal.cross(&self.dir);
                }
            }
            // self.draw();
        }
    }
    fn score(&self) -> usize {
        let next_3d = self.continue_or_fold(&self.cursor, &self.dir).0;
        let this_2d = self.mapping[&self.cursor];
        let next_2d = self.mapping[&next_3d];
        let diff = Vector2::new(
            next_2d.0 as isize - this_2d.0 as isize,
            next_2d.1 as isize - this_2d.1 as isize,
        );

        let dir_score = match diff {
            d if d == EAST => 0,
            d if d == SOUTH => 1,
            d if d == WEST => 2,
            d if d == NORTH => 3,
            _ => panic!("wrong direction {diff}"),
        };
        1000 * (this_2d.0 + 1) + 4 * (this_2d.1 + 1) + dir_score
    }
}

pub fn part2(input: &Map) -> usize {
    let mut cube = Cube::from_map(input);
    cube.run();
    cube.score()
}
