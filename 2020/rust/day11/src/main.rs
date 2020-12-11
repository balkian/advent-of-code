/// Solution that convers the string into a Vec<Vec<char>>
/// It might be better to simply use the string and work on it,
/// but rust makes it a bit harder to index chars within a string.
///

type Row = Vec<char>;
type Map = Vec<Row>;
type MapRef<'a> = &'a [Row];

#[allow(unused)]
fn print(m: MapRef) {
    for row in m {
        println!("{:}", row.iter().collect::<String>());
    }
}

const OCCUPIED: char = '#';
const EMPTY: char = 'L';

#[allow(unused)]
fn pause(m: MapRef) {
    print(m);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
}

fn count(m: MapRef, x: usize, y: usize, dx: usize, dy: usize) -> usize {
    let mut occupied = 0;

    let x0 = x.saturating_sub(dx);
    let y0 = y.saturating_sub(dy);
    let x1 = std::cmp::min(x + dx, m[0].len() - 1);
    let y1 = std::cmp::min(y + dy, m.len() - 1);

    let mut s = String::new();
    for row in m[y0..=y1].iter() {
        for c in row[x0..=x1].iter() {
            s.push(*c);
            if c == &OCCUPIED {
                occupied += 1;
            }
        }
    }
    occupied
}

fn counter1(m: MapRef, x: usize, y: usize) -> usize {
    count(m, x, y, 1, 1)
}
fn checked_add(m: MapRef, x: usize, y: usize, dx: &isize, dy: &isize) -> Option<(usize, usize)> {
    let x = (x as isize) + dx;
    let y = (y as isize) + dy;
    if x < 0 || y < 0 || y >= (m.len() as isize) || x >= (m[y as usize].len() as isize) {
        return None;
    }
    Some((x as usize, y as usize))
}

fn count_linear(m: MapRef, x: usize, y: usize) -> usize {
    let mut occupied = 0;

    for dy in &[-1isize, 0, 1] {
        for dx in &[-1isize, 0, 1] {
            if *dx == 0isize && *dy == 0isize {
                continue;
            }
            let mut y = y;
            let mut x = x;
            while let Some((nx, ny)) = checked_add(m, x, y, dx, dy) {
                y = ny;
                x = nx;
                match m[y][x] {
                    value if value == OCCUPIED => {
                        occupied += 1;
                        break;
                    }
                    value if value == EMPTY => break,
                    _ => continue,
                }
            }
        }
    }
    occupied
}

fn update(m: MapRef, counter: impl Fn(MapRef, usize, usize) -> usize) -> (bool, Vec<Row>) {
    let mut next = Vec::with_capacity(m.len());
    let mut changed = false;
    for (y, row) in m.iter().enumerate() {
        next.push(Vec::with_capacity(row.len()));
        for (x, seat) in row.iter().enumerate() {
            let occupied = counter(m, x, y);
            let nc = match *seat {
                OCCUPIED if occupied > 4 => {
                    changed = true;
                    EMPTY
                }
                EMPTY if occupied == 0 => {
                    changed = true;
                    OCCUPIED
                }
                nc => nc,
            };
            next[y].push(nc);
        }
    }
    (changed, next)
}

fn solve(title: &str, m: MapRef, counter: impl Fn(MapRef, usize, usize) -> usize) {
    let mut changed: bool;
    let mut map = m.to_owned();
    loop {
        // pause(&map);
        let res = update(&map, &counter);
        changed = res.0;
        map = res.1;
        if !changed {
            break;
        }
    }
    let occupied = count(&map, 0, 0, map.len(), map[0].len());
    println!("{}: {:?}", title, occupied);
}

fn main() {
    let args = aoc_utils::app("11").get_matches();
    let map: MapRef = &aoc_utils::file_iter_clap(&args)
        .map(|x| x.chars().collect())
        .collect::<Map>();
    solve("Part 1", map, counter1);
    solve("Part 2", map, count_linear);
}
