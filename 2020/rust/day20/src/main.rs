#![feature(hash_drain_filter)]
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Image {
    name: usize,
    rows: Vec<Vec<char>>,
    borders: Vec<Border>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Border {
    value: Vec<char>,
    orientation: Orientation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Orientation {
    North,
    East,
    South,
    West,
}

use Orientation::*;

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\n{}", self))
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Tile: {}", self.name)?;
        for row in &self.rows {
            for c in row {
                write!(f, "{}", c)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Image {
    fn new(name: usize, rows: Vec<&str>) -> Self {
        let mut img = Image {
            name,
            rows: rows.iter().map(|row| row.chars().collect()).collect(),
            borders: vec![],
        };
        img.calculate_borders();
        img
    }

    fn from_collection(coll: &Collection) -> Self {
        let example = coll.images.get(&coll.grid[0][0]).unwrap();
        let len = coll.grid.len();
        let h = example.rows.len() - 2;
        let w = example.rows[0].len() - 2;
        let mut rows = vec![vec!['.'; w * len]; h * len];
        for (y, row) in coll.grid.iter().enumerate() {
            for (x, img_id) in row.iter().enumerate() {
                let img = &coll.images[img_id];
                for (j, row) in img.rows.iter().skip(1).take(h).enumerate() {
                    for (i, value) in row.iter().skip(1).take(w).enumerate() {
                        let y = y * h + j;
                        let x = x * w + i;
                        if x >= rows[0].len() || y >= rows.len() {
                            continue;
                        }
                        rows[y][x] = *value;
                    }
                }
            }
        }
        let mut img = Image {
            name: 0,
            rows,
            borders: vec![],
        };
        img.calculate_borders();
        img
    }

    fn calculate_borders(&mut self) {
        let rows = &mut self.rows;
        let (start, end) = (0, rows[0].len() - 1);

        let mut borders: Vec<Border> = vec![];
        borders.push(Border {
            orientation: North,
            value: rows[start].clone(),
        });
        borders.push(Border {
            orientation: East,
            value: rows.iter().map(|row| row[end]).collect(),
        });
        borders.push(Border {
            orientation: South,
            value: rows[end].iter().rev().copied().collect(),
        });
        borders.push(Border {
            orientation: West,
            value: rows.iter().rev().map(|row| row[start]).collect(),
        });
        self.borders = borders;
    }

    fn matchings(&self) -> Vec<Vec<char>> {
        let mut matchings: Vec<Vec<char>> = vec![];

        let rows = &self.rows;
        let start = 0;
        let end = rows[0].len() - 1;
        matchings.push(rows[end].clone());
        matchings.push(rows.iter().map(|row| row[start]).collect());
        matchings.push(rows[0].iter().rev().copied().collect());
        matchings.push(rows.iter().rev().map(|row| row[end]).collect());

        matchings.push(matchings[0].iter().rev().copied().collect());
        matchings.push(matchings[3].iter().rev().copied().collect());
        matchings.push(matchings[2].iter().rev().copied().collect());
        matchings.push(matchings[1].iter().rev().copied().collect());

        matchings
    }

    fn is_aligned(&self, border: &Border) -> bool {
        let orientation = match border.orientation {
            North => South,
            South => North,
            West => East,
            East => West,
        };
        let value = border.value.iter().rev().copied().collect();
        self.borders.contains(&Border { value, orientation })
    }

    fn flip(&mut self) {
        for row in self.rows.iter_mut() {
            *row = row.iter().rev().copied().collect();
        }
        self.calculate_borders();
    }

    fn rotate(&mut self, times: usize) {
        for _ in 0..times {
            let mut rot = Vec::with_capacity(self.rows[0].len());
            for row in self.rows.iter().rev() {
                for (idx, c) in row.iter().enumerate() {
                    if rot.len() <= idx {
                        rot.push(Vec::with_capacity(self.rows.len()));
                    }
                    rot[idx].push(*c);
                }
            }
            self.rows = rot;
        }
        for border in self.borders.iter_mut() {
            border.orientation = match border.orientation {
                North => East,
                East => South,
                South => West,
                West => North,
            }
        }
    }

    fn mask_at(&self, x: usize, y: usize, mask: &[Vec<char>]) -> bool {
        // println!("Searching at {} {}", x, y);
        for (j, row) in mask.iter().enumerate() {
            for (i, value) in row.iter().enumerate() {
                if *value == ' ' {
                    continue;
                }
                if value != &self.rows[y + j][x + i] {
                    return false;
                }
            }
        }
        true
    }

    fn mask(&self, mask: &[Vec<char>]) -> usize {
        // let mut x = 0;
        // let mut y = 0;
        let limit_y = self.rows.len() - mask.len() + 1;
        let limit_x = self.rows[0].len() - mask[0].len() + 1;
        let mut count = 0;
        // dbg!{limit_x, limit_y};
        // while y <= limit_y {
        for y in 0..limit_y {
            // while x <= limit_x {
            for x in 0..limit_x {
                if self.mask_at(x, y, &mask) {
                    count += 1;
                }
                // x += 1;
            }
            // y += 1;
        }
        count
    }
}

type Grid = Vec<Vec<usize>>;

struct Collection {
    images: HashMap<usize, Image>,
    grid: Vec<Vec<usize>>,
}

impl Collection {
    fn new() -> Self {
        Collection {
            images: HashMap::new(),
            grid: Vec::new(),
        }
    }

    fn from_string(input: &str) -> Self {
        let mut images = Self::new();
        let it = &mut input.lines();
        loop {
            let mut it = it.skip_while(|x| x.is_empty());
            let name = match it.next() {
                Some(x) => x,
                None => break,
            };
            let name = name[5..name.len() - 1].parse().unwrap();
            let rows = it.take_while(|line| !line.is_empty()).collect();
            let img = Image::new(name, rows);
            images.images.insert(img.name, img);
        }
        images
    }

    fn corners(&self) -> HashSet<usize> {
        let mut res = HashSet::new();
        for &y in &[0, self.grid.len() - 1] {
            for &x in &[0, self.grid[y].len() - 1] {
                res.insert(self.grid[y][x]);
            }
        }
        res
    }

    fn part1(&mut self) -> usize {
        self.matchall();
        self.corners().iter().product()
    }

    fn part2(&mut self) -> usize {
        let monster = parse_seamonster();

        let mut big = Image::from_collection(&self);

        let mut count = 0;

        'all: for _rot in 0..4 {
            for _inv in 0..2 {
                count = big.mask(&monster);
                if count > 0 {
                    break 'all;
                }
                // Ideally I could rotate the monster, but I'm not parsing it as an image
                big.flip();
            }
            big.rotate(1);
        }

        let monster_roughness = monster.iter().flatten().filter(|x| **x == '#').count();
        let image_roughness = big.rows.iter().flatten().filter(|x| **x == '#').count();
        image_roughness - count * monster_roughness
    }

    fn normalize(&self, done: &HashMap<usize, (isize, isize)>) -> Result<Grid, Grid> {
        let (xs, ys): (Vec<isize>, Vec<isize>) = done.values().cloned().unzip();

        let find_bounds = |vs: Vec<isize>| {
            vs.iter()
                .fold((isize::MAX, isize::MIN), |(min_v, max_v), v| {
                    (min(min_v, *v), max(max_v, *v))
                })
        };

        let (min_x, max_x) = find_bounds(xs);
        let (min_y, max_y) = find_bounds(ys);

        let lx = 1 + (max_x - min_x) as usize;
        let ly = 1 + (max_y - min_y) as usize;

        let mut grid = Vec::with_capacity(ly);
        grid.resize_with(ly, || {
            let mut v = Vec::with_capacity(lx);
            v.resize_with(lx, || 99);
            v
        });

        for (id, (mut x, mut y)) in done {
            x -= min_x;
            y -= min_y;
            grid[y as usize][x as usize] = *id;
        }
        if done.len() == self.images.len() && lx * ly != done.len() {
            println!("Done, but not in the right shape");
            return Err(grid);
        }
        Ok(grid)
    }

    #[allow(unused)]
    fn print_done(&self, done: &HashMap<usize, (isize, isize)>) {
        let value = match self.normalize(done) {
            Ok(value) => value,
            Err(value) => value,
        };
        println!("Printing DONE so far: ");
        for row in value {
            for c in row {
                match c {
                    99 => print!("     "),
                    c => print!("{:5}", c),
                }
            }
            println!();
        }
    }

    fn backtrack(
        &mut self,
        current_id: usize,
        x: isize,
        y: isize,
        matchings: &HashMap<Vec<char>, HashSet<usize>>,
        done: &HashMap<usize, (isize, isize)>,
    ) -> Option<Grid> {
        // println!("BACKTRACKING!!: {} /{}", done.len(), self.images.keys().len());

        if (x + 1).pow(2) > (self.images.len() as isize)
            || (y + 1).pow(2) > (self.images.len() as isize)
        {
            return None;
        }

        let last_row = (y + 1).pow(2) == (self.images.len() as isize);
        let even_column = (x % 2) == 0;

        let mut done = done.clone();
        done.insert(current_id, (x, y));

        // self.print_done(&done);

        match (self.images.keys().len() as isize) - (done.len() as isize) {
            0 => return self.normalize(&done).ok(),
            c if c > 0 => {}
            c => panic!("overshot: {}", c),
        }

        let current = self.images.get(&current_id).unwrap().clone();
        let borders: Vec<_> = current.borders.clone();

        for border in borders {
            // Optimization using matchings
            let others = match matchings.get(&border.value) {
                Some(x) => x,
                None => continue,
            };
            // let others: Vec<usize> = self.images.keys().copied().collect();

            let (x, y) = match border.orientation {
                South if even_column => (x, y + 1),
                East if (even_column && last_row) || (!even_column && y == 0) => (x + 1, y),
                North if !even_column => (x, y - 1),
                // West if ((x %2) != 0) && y = 0 => (x-1, y),
                _ => continue,
            };

            // println!("Trying to match {} at {:?} ({}, {}): {}", &current.name , &border.orientation, &x, &y, &current);

            if let Some((other_id, _)) = done.iter().find(|(_k, &val)| val == (x, y)) {
                let other = self.images.get(other_id).unwrap();
                if !other.is_aligned(&border) {
                    // println!("ILLEGAL");
                    return None;
                }
                continue;
            }

            for &next_id in others {
                if next_id == current.name {
                    continue;
                }

                if done.contains_key(&next_id) {
                    continue;
                }

                for _rot in 0..4 {
                    for _flip in 0..2 {
                        if self.images.get(&next_id).unwrap().is_aligned(&border) {
                            if let Some(grid) = self.backtrack(next_id, x, y, &matchings, &done) {
                                return Some(grid);
                            }
                        }
                        self.images.get_mut(&next_id).unwrap().flip();
                    }
                    self.images.get_mut(&next_id).unwrap().rotate(1);
                }
            }
        }
        None
    }

    fn matchall(&mut self) {
        // let start = (0..=4).rev().filter_map(|i| self.find_connected(i).pop()).next().unwrap();

        let done: HashMap<usize, (isize, isize)> = HashMap::new();
        // let stack: Vec<_> = Vec::new();

        let matchings = self.matching_borders();

        let keys: Vec<usize> = self.images.keys().copied().collect();
        let mut found = false;
        for start in keys {
            // println!("Starting with {}", &start);
            if let Some(grid) = self.backtrack(start, 0, 0, &matchings, &done) {
                self.grid = grid;
                found = true;
                break;
            }
        }
        if !found {
            panic!("no solution found");
        }
    }

    fn matching_borders(&self) -> HashMap<Vec<char>, HashSet<usize>> {
        let mut borders: HashMap<Vec<char>, HashSet<usize>> = HashMap::new();
        for img in self.images.values() {
            for border in &img.matchings() {
                borders.entry(border.clone()).or_default().insert(img.name);
            }
        }
        borders
    }
}

fn parse_seamonster() -> Vec<Vec<char>> {
    let monster = include_str!("sea_monster.txt");
    monster.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let args = aoc_utils::app(env!("CARGO_PKG_NAME")).get_matches();
    let input = std::fs::read_to_string(args.value_of("input").expect("no input specified"))
        .expect("wrong file");
    let mut collection = Collection::from_string(input.as_ref());
    // collection.matchall();
    println!("Part 1: {}", collection.part1());
    println!("Part 2: {}", collection.part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_rotate() {
        let mut img = Image::new(1, vec!["#.", ".#"]);
        dbg!(&img.rows);
        img.rotate(1);
        dbg!(&img.rows);
        assert_eq!(img.rows[0][0], '.');
        assert_eq!(img.rows[0][1], '#');
        assert_eq!(img.rows[1][0], '#');
        assert_eq!(img.rows[1][1], '.');
    }

    #[test]
    fn test_flip() {
        let mut img = Image::new(1, vec!["..", ".#"]);
        dbg!(&img.rows);
        img.flip();
        dbg!(&img.rows);
        assert_eq!(img.rows[0][0], '.');
        assert_eq!(img.rows[0][1], '.');
        assert_eq!(img.rows[1][0], '#');
        assert_eq!(img.rows[1][1], '.');
    }

    #[test]
    fn test_rotate2() {
        let mut img = Image::new(1, vec!["##", ".#"]);
        dbg!(&img.rows);
        img.rotate(1);
        dbg!(&img.rows);
        assert_eq!(img.rows[0][0], '.');
    }

    #[test]
    fn test_alignment() {
        let img = Image::new(1, vec!["##", ".."]);
        let mut img2 = Image::new(1, vec![".#", ".#"]);

        assert_eq!(img2.is_aligned(&img.borders[0]), false);
        img2.rotate(1);
        assert_eq!(img2.is_aligned(&img.borders[0]), true);
    }
    #[test]
    fn test_alignment_flip() {
        let img = Image::new(1, vec![".#", ".#"]);

        let mut img2 = img.clone();
        img2.flip();

        assert_eq!(img2.is_aligned(&img.borders[0]), false);
        img2.flip();
        assert_eq!(img2.is_aligned(&img.borders[0]), true);
    }

    #[test]
    fn test_create() {
        let collection = Collection::from_string(include_str!("../example-1.txt"));
        assert_eq!(collection.images.len(), 9);
    }

    #[test]
    fn test_custom() {
        let mut collection = Collection::from_string(include_str!("../example-custom.txt"));
        assert_eq!(collection.images.len(), 4);
        collection.matchall();

        dbg! {&collection.grid};
        assert_eq!(
            collection.corners(),
            HashSet::from_iter([0usize, 1, 2, 3].iter().cloned())
        );
    }

    #[test]
    fn test_custom2() {
        let collection = Collection::from_string(include_str!("../example-custom2.txt"));
        assert_eq!(collection.images.len(), 2);

        let img1 = collection.images.get(&1).unwrap();
        let img2 = collection.images.get(&2).unwrap();

        assert_eq!(img2.is_aligned(&img1.borders[0]), true);
    }
    #[test]
    fn test_custom3() {
        let collection = Collection::from_string(include_str!("../example-custom3.txt"));
        assert_eq!(collection.images.len(), 2);

        let img1 = collection.images.get(&0).unwrap();
        let img2 = collection.images.get(&1).unwrap();

        assert_eq!(img2.is_aligned(&img1.borders[3]), true);
    }

    #[test]
    fn test_part1() {
        let mut collection = Collection::from_string(include_str!("../example-1.txt"));
        dbg!(&collection.grid);
        assert_eq!(collection.part1(), 20899048083289);
    }
}
