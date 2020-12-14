/// Solution that convers the string into a Vec<Vec<char>>
/// It might be better to simply use the string and work on it,
/// but rust makes it a bit harder to index chars within a string.
///

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    // Angle east
    angle: isize,
    waypoint: Waypoint,
}

#[derive(Debug)]
struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    fn rotate(&mut self, v: isize) {
        let (nx, ny) = match (360 + v) % 360 {
            0 => (self.x, self.y),
            90 => (-self.y, self.x),
            180 => (-self.x, -self.y),
            270 => (self.y, -self.x),
            c => panic!("invalid angle {:}", c),
        };
        self.x = nx;
        self.y = ny;
    }
}

impl Ship {
    fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            angle: 0,
            waypoint: Waypoint { x: 10, y: 1 },
        }
    }
    fn part1(&mut self, c: &Command) {
        // dbg!{&c,&self};
        match *c {
            Command::North(v) => self.y += v,
            Command::South(v) => self.y -= v,
            Command::West(v) => self.x -= v,
            Command::East(v) => self.x += v,
            Command::Left(v) => self.angle = (self.angle + 360 + v) % 360,
            Command::Right(v) => self.angle = (self.angle + 360 - v) % 360,
            Command::Forward(v) => match self.angle {
                0 => self.x += v as isize,
                180 => self.x -= v as isize,
                90 => self.y += v as isize,
                270 => self.y -= v as isize,
                v => panic!("invalid angle: {:}", v),
            },
        }
        // dbg!{&self};
    }
    fn part2(&mut self, c: &Command) {
        // dbg!{&c,&self};
        match *c {
            Command::North(v) => self.waypoint.y += v,
            Command::South(v) => self.waypoint.y -= v,
            Command::West(v) => self.waypoint.x -= v,
            Command::East(v) => self.waypoint.x += v,
            Command::Left(v) => self.waypoint.rotate(v),
            Command::Right(v) => self.waypoint.rotate(-v),
            Command::Forward(v) => {
                self.x += v * self.waypoint.x;
                self.y += v * self.waypoint.y;
            }
        }
        // dbg!{&self};
    }
    fn travelled(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[derive(Debug)]
enum Command {
    North(isize),
    South(isize),
    West(isize),
    East(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

fn main() {
    let args = aoc_utils::app("12").get_matches();
    let cmds: Vec<Command> = aoc_utils::file_iter_clap(&args)
        .map(|x| {
            let (direct, units) = x.split_at(1);
            // dbg!(&direct, &units);
            let units = units.parse().unwrap();
            match direct.chars().next().unwrap() {
                'N' => Command::North(units),
                'S' => Command::South(units),
                'W' => Command::West(units),
                'E' => Command::East(units),
                'L' => Command::Left(units),
                'R' => Command::Right(units),
                'F' => Command::Forward(units),
                v => panic!("unexpected input: {:}", v),
            }
        })
        .collect();
    let mut s = Ship::new();
    for c in &cmds {
        s.part1(c);
    }
    println!("Part 1: {:}", s.travelled());
    let mut s = Ship::new();
    for c in cmds {
        s.part2(&c);
    }
    println!("Part 2: {:}", s.travelled());
}
