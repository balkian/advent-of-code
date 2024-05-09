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
    min: Point,
    max: Point,
}

impl Brick {
    fn new(c1: Point, c2: Point) -> Self {
        let y0 = c1.y.min(c2.y);
        let y1 = c1.y.max(c2.y);
        let x0 = c1.x.min(c2.x);
        let x1 = c1.x.max(c2.x);
        let z0 = c1.z.min(c2.z);
        let z1 = c1.z.max(c2.z);
        Brick{
            c1,
            c2,
            min: Point{x: x0, y: y0, z: z0},
            max: Point{x: x1, y: y1, z: z1}
        }
    }

    fn fall(&mut self, height: usize) {
        self.c1.z -= height;
        self.c2.z -= height;
        self.min.z -= height;
        self.max.z -= height;
    }

    fn z(&self) -> usize {
        self.min.z
    }

    fn bottom_at(&self, xt: usize, yt: usize) -> usize {
        for z in (self.min.z..=self.max.z) {
            for y in self.min.y..=self.max.y {
                for x in self.min.x..=self.max.x {
                    if(x == xt && y == yt) {
                        return z;
                    }
                }
            }
        }
        0
    }

    fn top_at(&self, xt: usize, yt: usize) -> usize {
        for z in (self.min.z..=self.max.z).rev() {
            for y in self.min.y..=self.max.y {
                for x in self.min.x..=self.max.x {
                    if(x == xt && y == yt) {
                        return z;
                    }
                }
            }
        }
        0
    }

    fn shadow(&self) -> Vec<Point> {
        let mut points = Vec::with_capacity(self.max.y-self.min.y+self.max.x-self.min.x+1);
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                points.push(Point{x, y, z: 0});
            }
        }
        points
    }
}

#[derive(Debug,Clone)]
pub struct Problem {
    bricks: Vec<Brick>,
    collisions: BTreeMap<usize, BTreeSet<usize>>,
    shadows: BTreeMap<Point, BTreeSet<usize>>,
    ignored: BTreeSet<usize>,
}

impl Problem {
    fn new(bricks: &[Brick]) -> Self {
        let mut collisions: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        let shadows = BTreeMap::new();
        let ignored = BTreeSet::new();
        let mut s = Self{bricks: bricks.into(), collisions, shadows, ignored};
        s.recalculate();
        s
    }

    fn recalculate(&mut self) {
        self.collisions.clear();
        self.shadows.clear();
        for (ix, brick) in self.bricks.iter().enumerate() {
            for p in brick.shadow() {
                self.shadows.entry(p).or_default().insert(ix);
            }
        }
        self.shadows.retain(|_, v| v.len() > 1);
        for v in self.shadows.values() {
            for o1 in v {
                for o2 in v {
                    if o1 != o2 {
                        self.collisions.entry(*o1).or_default().insert(*o2);
                        self.collisions.entry(*o2).or_default().insert(*o1);
                    }
                }
            }
        }
        //dbg!(self.bricks.iter().map(|b| b.len()).max());
        //dbg!(self.collisions.values().map(|vs| vs.len()).max());
        //dbg!(self.collisions.len(), self.bricks.len());
    }

    fn settle(&mut self, early_stop: bool) -> usize {
        let mut changed: BTreeSet<usize> = BTreeSet::new();
       // run until no moves are made:
        loop {
            let mut done = true;
            // For every Brick
            for i in 0..self.bricks.len() {
                if self.ignored.contains(&i) {
                    continue;
                }
                if self.settle_brick(i) > 0 {
                    done = false;
                    changed.insert(i);
                    if early_stop {
                        return 1;
                    }
                }
            }
            if done {
                break;
            }
        }
        changed.len()
    }

    fn settle_brick(&mut self, pos: usize) -> usize {
        let h = self.brick_height(pos);
        self.bricks[pos].fall(h);
        h
    }

    fn brick_height(&self, pos: usize) -> usize {
        // For every other shadow position of this brick
        let mut brick = self.bricks[pos];
        let mut max_drop = brick.z() - 1;
        let shadow = brick.shadow();
        for point in shadow {
            let z = brick.bottom_at(point.x, point.y);
            let mut new_z = 1;
            if let Some(others) = self.shadows.get(&point) {
                for other in others.iter() {
                    if self.ignored.contains(other) {
                        continue;
                    }
                    let other = self.bricks[*other];
                    if other == brick {
                        continue;
                    }
                    let oz = other.top_at(point.x, point.y);
                    if oz == z {
                        panic!("invalid height, there is an overlap!!!!");
                    }
                    if oz > z || oz < new_z {
                        continue;
                    }
                    new_z = oz + 1;
                }
            };
            max_drop = max_drop.min(z - new_z);
        } 
        max_drop
    }

    fn is_settled(&self) -> bool {
        for i in 0..self.bricks.len() {
            if self.ignored.contains(&i) {
                continue;
            }
            if self.brick_height(i) > 0 {
                return false;
            }
        }
        true
    }

    fn disintegrate(&mut self, pos: usize) {
        self.ignored.insert(pos);
        //let last = &(self.bricks.len() - 1);
        //self.bricks.swap_remove(pos);
        //collisions: BTreeMap<usize, BTreeSet<usize>>,
        //shadows: BTreeMap<Point, BTreeSet<usize>>,
        //self.collisions.remove(&pos);
        //for (k, v) in self.collisions.iter_mut() {
        //    v.remove(&pos);
        //    if k != last {
        //        if v.remove(last) {
        //            v.insert(pos);
        //            
        //        };
        //    }
        //}
        //for v in self.shadows.values_mut() {
        //    v.remove(&pos);
        //    if v.remove(last) {
        //        v.insert(pos);
        //    };
        //}
        //if let Some(mut c) = self.collisions.remove(last) {
        //    c.remove(&pos);
        //    self.collisions.insert(pos, c);
        //}
    }
}

pub fn parse(input: &str) -> Problem {
    let mut bricks: Vec<Brick> = input.trim().lines().filter(|line| !line.is_empty())
        .map(|line| {
            let mut points = line.split("~").map(|coords| {
                let nums: Vec<usize> = coords.split(",").map(|number| number.parse::<usize>().unwrap_or_else(|_| panic!("invalid number {number}"))).collect();
                Point{x: nums[0], y: nums[1], z: nums[2]}
            });
            Brick::new(points.next().unwrap(), points.next().unwrap())
        }).collect();
    let mut p = Problem::new(&bricks);
    p.settle(false);
    p
}


pub fn part1(problem: &Problem) -> usize {
    //dbg!(problem.bricks.len());
    let mut safetodisintegrate = 0;
    for i in 0..problem.bricks.len() {
        //let b = problem.bricks[i];
        let mut alt = problem.clone();
        alt.disintegrate(i);
        if alt.is_settled() {
            //println!("Can disintegrate: #{i} {b:?} ");
            safetodisintegrate += 1;
        }
    }
    safetodisintegrate
}

pub fn part2(problem: &Problem) -> usize {
    let problem = problem.clone();
    let mut fallen = 0;
    for i in 0..problem.bricks.len() {
        let mut alt = problem.clone();
        alt.disintegrate(i);
        fallen += alt.settle(false);
    }
    fallen
}
