use std::collections::HashMap;

type Button = char;

type Pos = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Accept,
}

const INVALID: char = '@';

impl Dir {
    fn as_char(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
            Dir::Accept => 'A',
        }
    }
    fn add(&self, other: Pos) -> Pos {
        match self {
            Dir::Up => (other.0 - 1, other.1),
            Dir::Down => (other.0 + 1, other.1),
            Dir::Right => (other.0, other.1 + 1),
            Dir::Left => (other.0, other.1 - 1),
            Dir::Accept => other,
        }
    }
}

const NUMBERPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [INVALID, '0', 'A'],
];

const DIRECTIONPAD: [[char; 3]; 2] = [[INVALID, '^', 'A'], ['<', 'v', '>']];

fn cost(b: Button) -> usize {
    match b {
        'A' => 1,
        '>' | '^' => 2,
        'v' => 3,
        '<' => 4,
        _ => panic!("unknown"),
    }
}

#[derive(Debug)]
struct Robot<const M: usize, const N: usize> {
    pad: &'static [[char; N]; M],
    initial_pos: Pos,
    coordinates: HashMap<Button, Pos>,
}

impl<const M: usize, const N: usize> Robot<M, N> {
    fn new(pad: &'static [[char; N]; M]) -> Self {
        let mut coordinates: HashMap<Button, Pos> = Default::default();
        for (m, row) in pad.iter().enumerate() {
            for (n, cell) in row.iter().enumerate() {
                coordinates.insert(*cell, (m, n));
            }
        }
        let initial_pos = *coordinates.get(&'A').expect("button <A> not found");
        Robot {
            pad,
            initial_pos,
            coordinates,
        }
    }
}

impl<const M: usize, const N: usize> Robot<M, N> {
    fn press(&self, button: Button, pos: &mut Pos) -> Vec<Vec<Button>> {
        // Identify where to move next
        let mut heads = vec![(*pos, vec![])];
        let target = *self
            .coordinates
            .get(&button)
            .expect("that button is not available");
        let mut paths = vec![];
        while let Some((pos, path)) = heads.pop() {
            if pos == target {
                paths.push(path);
                continue;
            }
            if self.pad[pos.0][pos.1] == INVALID {
                continue;
            }
            if target.1 > pos.1 {
                let mut pos = pos;
                let dir = Dir::Right;
                let mut newhead = path.clone();
                while target.1 > pos.1 {
                    newhead.push(dir.as_char());
                    pos = dir.add(pos);
                }
                heads.push((pos, newhead));
            }
            if target.1 < pos.1 {
                let mut pos = pos;
                let dir = Dir::Left;
                let mut newhead = path.clone();
                while target.1 < pos.1 {
                    newhead.push(dir.as_char());
                    pos = dir.add(pos);
                }
                heads.push((pos, newhead));
            }
            if target.0 < pos.0 {
                let mut pos = pos;
                let dir = Dir::Up;
                let mut newhead = path.clone();
                while target.0 < pos.0 {
                    newhead.push(dir.as_char());
                    pos = dir.add(pos);
                }
                heads.push((pos, newhead));
            }
            if target.0 > pos.0 {
                let mut pos = pos;
                let dir = Dir::Down;
                let mut newhead = path.clone();
                while target.0 > pos.0 {
                    newhead.push(dir.as_char());
                    pos = dir.add(pos);
                }
                heads.push((pos, newhead));
            }
        }
        *pos = target;
        for path in paths.iter_mut() {
            path.push(Dir::Accept.as_char());
        }
        //println!("Robot {} pressed {}", self.name, self.pad[self.pos.0][self.pos.1]);
        paths
    }
}

type Codes = Vec<Vec<Button>>;

pub fn parse(i: &str) -> Codes {
    i.lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn expand_history(history: &mut Vec<Vec<Button>>) {
    let first = Robot::new(&DIRECTIONPAD);
    let mut newalts = vec![];
    let mut mincost = usize::MAX;
    for alt in history.drain(..) {
        let mut pos = first.initial_pos;
        let mut sofar = vec![vec![]];
        for digit in alt {
            let mut newfar = vec![];
            let paths = first.press(digit, &mut pos);
            for s in sofar {
                for path in paths.iter() {
                    let mut np = s.clone();
                    np.extend(path.iter().cloned());
                    newfar.push(np);
                }
            }
            sofar = newfar;
        }
        for sf in sofar {
            let c = sf.iter().map(|d| cost(*d)).sum::<usize>();
            mincost = mincost.min(c);
            newalts.push((c, sf));
        }
    }

    *history = newalts
        .into_iter()
        .filter_map(|(c, p)| (c == mincost).then_some(p))
        .collect();
}

pub fn part1(i: &Codes) -> usize {
    let mut total_complexity = 0;
    let pad = Robot::new(&NUMBERPAD);
    for code in i {
        let pos = &mut pad.initial_pos.clone();

        let mut history = vec![code.clone()];
        let mut newalts = vec![];
        for alt in history.drain(..) {
            let mut sofar = vec![vec![]];
            for digit in alt {
                let mut newfar = vec![];
                let paths = pad.press(digit, pos);
                for s in sofar {
                    for path in paths.iter() {
                        let mut np = s.clone();
                        np.extend(path.iter().cloned());
                        newfar.push(np);
                    }
                }
                sofar = newfar;
            }
            newalts.extend(sofar);
        }
        history = newalts;

        expand_history(&mut history);
        expand_history(&mut history);

        let least = history
            .into_iter()
            .min_by_key(|h| h.len())
            .expect("at least one should be available");

        let code_str: String = code.iter().collect();
        let complexity = code_str[..3]
            .parse::<usize>()
            .expect("cannot read number from code")
            * least.len();
        total_complexity += complexity;
    }
    total_complexity
}

fn conquer<const M: usize, const N: usize>(
    code: &[Button],
    robot: &Robot<M, N>,
    level: usize,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if code.is_empty() {
        return 0;
    }
    if level == 0 {
        return code.len();
    }
    let key = (code.iter().collect(), level);
    if let Some(v) = memo.get(&key) {
        return *v;
    }
    let mut total = 0;
    for part in code.split_inclusive(|c| c == &'A') {
        let mut pos = robot.initial_pos;
        for b in part {
            let mut button_min = usize::MAX;
            for p in robot.press(*b, &mut pos) {
                button_min = button_min.min(conquer(&p, robot, level - 1, memo));
            }
            total += button_min;
        }
    }
    memo.insert(key, total);
    total
}

pub fn part2(i: &Codes) -> usize {
    let mut total_complexity = 0;
    let door = Robot::new(&NUMBERPAD);
    let first = Robot::new(&DIRECTIONPAD);
    let memo = &mut HashMap::new();
    for code in i {
        let mut pos = door.initial_pos;
        let mut history = vec![code.clone()];
        let mut newalts = vec![];
        for alt in history.drain(..) {
            let mut sofar = vec![vec![]];
            for digit in alt {
                let mut newfar = vec![];
                let paths = door.press(digit, &mut pos);
                for s in sofar {
                    for path in paths.iter() {
                        let mut np = s.clone();
                        np.extend(path.iter().cloned());
                        newfar.push(np);
                    }
                }
                sofar = newfar;
            }
            newalts.extend(sofar);
        }
        history = newalts;
        let mut min = usize::MAX;
        for h in &history {
            min = min.min(conquer(h, &first, 25, memo));
        }
        let code_str: String = code.iter().collect();

        let complexity = code_str[..3]
            .parse::<usize>()
            .expect("cannot read number from code")
            * min;

        total_complexity += complexity;
    }
    total_complexity
}
