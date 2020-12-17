use std::cmp;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coord(isize, isize, isize, isize);

impl Coord {
    fn neighbors3_d(&self) -> impl IntoIterator<Item = Coord> {
        let center = *self;
        (center.2 - 1..=center.2 + 1).flat_map(move |nz| {
            (center.1 - 1..=center.1 + 1).flat_map(move |ny| {
                (center.0 - 1..=center.0 + 1).map(move |nx| {
                    Coord(nx, ny, nz, center.3)
                })
            })
        }).filter(move |neigh| *neigh != center)
    }

    fn neighbors4_d(&self) -> impl IntoIterator<Item = Coord> {
        {
            let center = *self;
            (center.3 - 1..=center.3 + 1).flat_map(move |nw| {
                center
                    .neighbors3_d()
                    .into_iter()
                    .chain(std::iter::once(center))
                    .filter_map(move |Coord(nx, ny, nz, _)| {
                        let ncord = Coord(nx, ny, nz, nw);
                        if ncord == center {
                            None
                        } else {
                            Some(ncord)
                        }
                    })
            })
        }
    }
}

#[derive(Clone)]
struct Map {
    active: HashMap<Coord, bool>,
}

impl Map {
    fn new() -> Self {
        Self {
            active: HashMap::new(),
        }
    }

    fn from_string(s: &str) -> Self {
        let mut m = Self::new();
        for (y, row) in s.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == ACTIVE {
                    let c = Coord(x as isize, y as isize, 0isize, 0isize);
                    m.set(c, true);
                }
            }
        }
        m
    }

    fn update<T>(&mut self, neigh_func: impl Fn(&Coord) -> T)
    where
        T: IntoIterator<Item = Coord>,
    {
        let mut conv: HashMap<Coord, u64> = HashMap::new();
        for (coord, value) in &self.active {
            if !value {
                continue;
            }
            for neighbor in neigh_func(&coord) {
                let counter = conv.entry(neighbor).or_insert(0);
                *counter += 1;
            }
        }
        self.apply_conv(conv);
    }

    fn apply_conv(&mut self, conv: HashMap<Coord, u64>) {
        let old_active = std::mem::replace(&mut self.active, HashMap::new());
        for (coord, counter) in conv {
            match old_active.get(&coord) {
                Some(true) if (2..=3).contains(&counter) => {
                    self.set(coord, true);
                }
                None if 3 == counter => {
                    self.set(coord, true);
                }
                _ => {
                    self.set(coord, false);
                }
            }
        }
    }

    fn is_active(&self, coord: &Coord) -> bool {
        self.active.get(coord).unwrap_or(&false).to_owned()
    }

    fn set(&mut self, coord: Coord, value: bool) {
        if value {
            self.active.insert(coord, value);
        } else {
            self.active.remove(&coord);
        }
    }

    #[allow(unused)]
    fn print_active(&self) {
        let mut keys: Vec<_> = self.active.keys().collect();
        keys.sort();
        for coord in keys {
            println!("{}, {}, {}", coord.0, coord.1, coord.2);
        }
    }

    #[allow(unused)]
    fn print(&self) {
        let mut keys = self.active.keys();
        let mut min = *keys.next().expect("there are no active cells");
        let mut max = min;
        for coord in keys {
            min = Coord(
                cmp::min(min.0, coord.0),
                cmp::min(min.1, coord.1),
                cmp::min(min.2, coord.2),
                0,
            );
            max = Coord(
                cmp::max(max.0, coord.0),
                cmp::max(max.1, coord.1),
                cmp::max(max.2, coord.2),
                0,
            );
        }
        println!(
            "Active: {}. Min: {:?}. Max: {:?}",
            self.active.len(),
            &min,
            &max
        );
        for z in min.2..=max.2 {
            println!("Z={}", z);
            println!();
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    let c = if self.is_active(&Coord(x, y, z, 0)) {
                        ACTIVE
                    } else {
                        INACTIVE
                    };
                    print!("{}", c);
                    if x == 0 {
                        print!("|");
                    }
                }
                println!();
                if y == 0 {
                    for x in min.0..=max.0 {
                        print!("-");
                        if x == 0 {
                            print!("|");
                        }
                    }
                    println!();
                }
            }
            println!();
        }
    }
}

const ACTIVE: char = '#';
const INACTIVE: char = '.';

#[allow(unused)]
fn pause(m: &Map) {
    m.print();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
}

fn main() {
    let args = aoc_utils::app("17").get_matches();
    let input: String = aoc_utils::file_iter_clap(&args)
        .collect::<Vec<String>>()
        .join("\n");
    // println!("{}", &input);
    let map = &mut Map::from_string(&input);
    let mut map2 = map.clone();

    for _i in 0..6 {
        // pause(&map);
        map.update(|coord| coord.neighbors3_d())
    }

    println!("Part 1: {}", map.active.len());

    for _i in 0..6 {
        map2.update(|coord| coord.neighbors4_d())
        // map.print();
    }
    println!("Part 2: {}", map2.active.len());
}
