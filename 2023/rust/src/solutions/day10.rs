use std::collections::HashSet;
use std::mem;

// Row, Column
type Coord = [usize; 2];
type Delta = [isize; 2];

/// Map is a representation of the tiles in this problem, which represent a set
/// of tubes. Instead of parsing the input, as I usually do, I've chosen to keep
/// the input as a string (or rather, an array of bytes) and use byte values in
/// the logic.
#[derive(Debug, Clone)]
pub struct Map<'a> {
    start: Coord,
    lines: Vec<&'a [u8]>,
}

impl<'a> TryFrom<&'a str> for Map<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let lines = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes())
            .collect();
        let mut m = Map {
            lines,
            start: [0, 0],
        };
        m.set_start()?;
        Ok(m)
    }
}

impl<'a> Map<'a> {
    fn set_start(&mut self) -> Result<(), String> {
        let start = self
            .lines
            .iter()
            .enumerate()
            .filter_map(|(ix, line)| Some([ix, line.iter().position(|c| c == &b'S')?]))
            .next()
            .ok_or_else(|| String::from("start not found"))?;
        self.start = start;
        Ok(())
    }

    fn convert_deltas<'b>(
        &'b self,
        coord: &'b Coord,
        deltas: Vec<Delta>,
    ) -> impl Iterator<Item = Coord> + 'b {
        deltas.into_iter().filter_map(|delta| {
            let y = coord[0] as isize + delta[0];
            let x = coord[1] as isize + delta[1];
            if y >= 0
                && y < (self.lines.len() as isize)
                && x >= 0
                && x < self.lines[y as usize].len() as isize
            {
                Some([y as usize, x as usize])
            } else {
                None
            }
        })
    }

    fn connections<'b>(&'b self, coord: &'b Coord) -> impl Iterator<Item = Coord> + 'b {
        let c = self.lines[coord[0]][coord[1]];

        let neighbors = match c {
            b'|' => vec![[-1, 0], [1, 0]],
            b'-' => vec![[0, -1], [0, 1]],
            b'L' => vec![[-1, 0], [0, 1]],
            b'J' => vec![[-1, 0], [0, -1]],
            b'7' => vec![[0, -1], [1, 0]],
            b'F' => vec![[0, 1], [1, 0]],
            b'.' => vec![],
            b'S' => vec![[0, 1], [0, -1], [1, 0], [-1, 0]],
            c => panic!("invalid character {c}"),
        };
        self.convert_deltas(coord, neighbors)
    }

    fn find_loop(&self) -> Vec<Coord> {
        let mut heads: Vec<Coord> = self.connections(&self.start).collect();
        let mut paths: Vec<Vec<Coord>> = vec![vec![self.start]; heads.len()];
        while !heads.is_empty() {
            // dbg!(&heads);
            // dbg!(&paths);
            let mut ix = 0;

            while ix < heads.len() {
                for jx in 0..heads.len() {
                    if ix == jx {
                        continue;
                    }
                    if heads[ix] == heads[jx] {
                        let mut v1 = paths[ix].clone();
                        v1.push(heads[ix]);
                        v1.extend(paths[jx].iter().skip(1).copied().rev());
                        return v1;
                    }
                }

                // get directions
                let mut d: Vec<Coord> = self.connections(&heads[ix]).collect();
                let previous = paths[ix].last().expect("empty path");
                if let Some(next_head) = d.iter().position(|c| c == previous).and_then(|prev| {
                    d.remove(prev);
                    d.pop()
                        .and_then(|next_head| d.is_empty().then_some(next_head))
                }) {
                    let old_head = mem::replace(&mut heads[ix], next_head);
                    paths[ix].push(old_head);
                    ix += 1;
                } else {
                    paths.remove(ix);
                    heads.remove(ix);
                }
            }
        }
        // dbg!(&heads);
        // dbg!(&paths);
        panic!("Could not find a loop");
    }
}

pub fn parse(input: &str) -> Map {
    input.try_into().expect("could not convert string to Map")
}

pub fn part1(map: &Map) -> usize {
    let path = map.find_loop();
    (&path.len() + 1) / 2
}
pub fn part2(map: &Map) -> usize {
    let path = map.find_loop();
    let prev = *path.last().expect("loop is too small (<2 elements)");
    let next = path[1];
    // HashSet is around 2x as fast in this specific case
    let path: HashSet<Coord> = path.into_iter().collect();
    let mut inside = 0;

    // S should count as a boundary if it can be replaced by |, L or J. i.e.,
    // if the path includes a pipe to the north of the start.
    let include_s = [prev, next].contains(&[map.start[0].saturating_sub(1), map.start[1]]);

    for i in 0..map.lines.len() {
        let mut crossings = 0;
        for (j, tile) in map.lines[i].iter().enumerate() {
            if path.contains(&[i, j]) {
                // print!("{}", char::try_from(*tile).unwrap());
                match tile {
                    &b'|' | &b'L' | &b'J' => {
                        crossings += 1;
                    }
                    &b'S' if include_s => {
                        crossings += 1;
                    }
                    _ => {}
                }
            } else if crossings % 2 != 0 {
                inside += 1;
                // print!("I");
                // } else {
                // print!("O");
            }
        }
        // println!();
    }
    inside
}
