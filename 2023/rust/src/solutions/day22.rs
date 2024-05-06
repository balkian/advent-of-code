use std::collections::{BTreeSet, BTreeMap};

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Copy,Clone,Debug,Hash,PartialOrd,Ord,PartialEq,Eq)]
pub struct Brick {
    c1: Point,
    c2: Point,
}

impl Brick {
    fn len(&self) -> usize {
        self.c1.x.abs_diff(self.c2.x) +
        self.c1.y.abs_diff(self.c2.y) +
        self.c1.z.abs_diff(self.c2.z) + 1
    }

    fn z(&self) -> usize {
        self.c1.z.min(self.c2.z)
    }

    fn shadow(&self) -> Vec<Point> {
        let (y0, y1) = if self.c1.y <= self.c1.y {
            (self.c1.y, self.c1.y)
        } else {
            (self.c2.y, self.c2.y)
        };
        let (x0, x1) = if self.c1.x <= self.c1.x {
            (self.c1.x, self.c1.x)
        } else {
            (self.c2.x, self.c2.x)
        };
        let mut points = Vec::with_capacity(y1-y0+x1-x0+1);
        for y in y0..=y1 {
            for x in x0..=x1 {
                points.push(Point{x, y, z: 0});
            }
        }
        points
    }
}

pub fn parse(input: &str) -> Vec<Brick> {
    input.trim().lines().filter(|line| !line.is_empty())
        .map(|line| {
            let mut points = line.split("~").map(|coords| {
                let nums: Vec<usize> = coords.split(",").map(|number| number.parse::<usize>().unwrap_or_else(|_| panic!("invalid number {number}"))).collect();
                Point{x: nums[0], y: nums[1], z: nums[2]}
            });
            Brick{c1: points.next().unwrap(), c2: points.next().unwrap()}
        }).collect()
}

#[allow(dead_code)]
fn check_differences(input: &[Brick]) {
    dbg!(&input);
    for brick in input {
        let mut diff = 0;
        if brick.c1.x != brick.c2.x { 
            diff += 1;
        }
        if brick.c1.y != brick.c2.y { 
            diff += 1;
        }
        if brick.c1.z != brick.c2.z { 
            diff += 1;
        }
        if diff > 1 {
            println!("Brick with diff {diff}: {brick:?}");
        }
    }
}

pub fn part1(input: &[Brick]) -> usize {
    dbg!(input.len());
    let mut input = Vec::from(input);
    dbg!(input.iter().map(|brick| brick.len()).sum::<usize>());
    let mut shadows: BTreeMap<Point, BTreeSet<usize>> = BTreeMap::new();
    for (ix, brick) in input.iter().enumerate() {
        for p in brick.shadow() {
            shadows.entry(p).or_default().insert(ix);
        }
    }
    //shadows.retain(|k, v| v.len() > 1);
    let mut collisions: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut free: BTreeSet<Brick> = BTreeSet::from_iter(input.iter().cloned());
    for v in shadows.values() {
        for o1 in v {
            for o2 in v {
                if o1 != o2 {
                    collisions.entry(*o1).or_default().insert(*o2);
                    collisions.entry(*o2).or_default().insert(*o1);
                }
            }
        }
    }
    dbg!(collisions.values().map(|vs| dbg!(vs.len())).max());
    dbg!(collisions.len(), input.len());
    todo!();
}
pub fn part2(input: &[Brick]) -> usize {
    todo!();
}
