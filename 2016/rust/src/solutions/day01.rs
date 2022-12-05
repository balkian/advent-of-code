use std::collections::HashSet;

pub type Dir = (usize, isize);

#[derive(Debug)]
struct Person {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl Person {
    fn new() -> Person {
        Person {
            pos: (0, 0),
            dir: (0, 1),
        }
    }

    fn walk(&mut self, &(steps, angle): &Dir) -> Vec<(isize, isize)> {
        match angle {
            90 => self.dir = (-self.dir.1, self.dir.0),
            -90 => self.dir = (self.dir.1, -self.dir.0),
            _ => panic!("unsupported angle"),
        }

        let mut pos = Vec::with_capacity(steps);
        for _i in 0..steps {
            self.pos.0 += self.dir.0;
            self.pos.1 += self.dir.1;
            pos.push(self.pos);
        }
        pos
    }
}

pub fn parse(input: &str) -> Vec<Dir> {
    input
        .split(',')
        .map(|i| {
            let i = i.trim();
            let num = i[1..].parse().unwrap();
            match i.as_bytes()[0] {
                b'R' => (num, -90),
                b'L' => (num, 90),
                _ => panic!("unknown instruction"),
            }
        })
        .collect()
}

pub fn part1(input: &[Dir]) -> usize {
    let mut person = Person::new();
    for i in input {
        person.walk(i);
    }
    (person.pos.0.abs() + person.pos.1.abs()) as usize
}

pub fn part2(input: &[Dir]) -> usize {
    let mut locs: HashSet<(isize, isize)> = HashSet::new();
    let mut person = Person::new();
    locs.insert(person.pos);
    for i in input {
        for pos in person.walk(i) {
            if locs.contains(&pos) {
                return (pos.0.abs() + pos.1.abs()) as usize;
            }
            locs.insert(pos);
        }
    }
    panic!("Did not find a position twice");
}
