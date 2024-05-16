use glam::DVec3;
use std::fmt;

type Position = DVec3;
type Velocity = Position;

#[derive(Clone, Copy, PartialEq)]
struct Stone {
    pos: Position,
    vel: Velocity,
    a: f64,
    b: f64,
}

impl fmt::Debug for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}", self.pos, self.vel)
    }
}

impl Stone {
    fn from_str(input: &str) -> Self {
        let tokens: Vec<f64> = input
            .split(&[',', '@'])
            .map(|t| t.trim().parse::<f64>().expect("invalid number"))
            .collect();
        let pos = [tokens[0], tokens[1], tokens[2]];
        let vel = [tokens[3], tokens[4], tokens[5]];
        Self::new(pos.into(), vel.into())
    }
    fn new(pos: Position, vel: Velocity) -> Self {
        let a = vel[1] / vel[0];
        let b = pos[1] - pos[0] * (vel[1] / vel[0]);

        Self {
            vel,
            pos,
            a,
            b,
        }
    }

    fn collision_at_xy(&self, other: &Self) -> Option<[f64; 2]> {
        let (a, b, c, d) = { (self.a, other.a, self.b, other.b) };

        if a == b {
            if c == d {
                panic!("same line!");
            } else {
                return None;
            }
        }

        let x = (d - c) / (a - b);
        let y1 = a * x + c;
        let y2 = b * x + d;

        if self.is_past_at_xy([x, y1]) || other.is_past_at_xy([x, y2]) {
            //eprintln!("ïn the past");
            None
        } else {
            Some([x, y1])
        }
    }

    fn is_past_at_xy(&self, pos: [f64; 2]) -> bool {
        (((pos[0] - self.pos.x) < 0.0) ^ (self.vel.x < 0.0))
            || (((pos[1] - self.pos.y) < 0.0) ^ (self.vel.y < 0.0))
    }
}

#[derive(Debug, Clone)]
pub struct Hail {
    stones: Vec<Stone>,
}

pub fn parse(input: &str) -> Hail {
    Hail {
        stones: input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Stone::from_str(line.trim()))
            .collect(),
    }
}

fn solve(hail: &Hail, min: f64, max: f64) -> usize {
    let mut count = 0;
    for (ix, stone) in hail.stones.iter().enumerate() {
        for other in hail.stones.iter().take(ix) {
            if let Some([y, x]) = stone.collision_at_xy(other) {
                //dbg!(stone, other, [y, x]);
                if x >= min && x <= max && y >= min && y <= max {
                    //eprintln!("Valid");
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part1(input: &Hail) -> usize {
    if input.stones.len() > 10 {
        solve(input, 200000000000000f64, 400000000000000f64)
    } else {
        solve(input, 7f64, 72f64)
    }
}

pub fn part2(input: &Hail) -> usize {
    // It has to collide with every hailstone,
    // so we can pick one at random as our reference.
    let origin = input.stones[0];
    // Then we convert the coordinate system to use our origin
    // hailstone as origin.
    let rotated: Vec<Stone> = input
        .stones
        .iter()
        .skip(1)
        // We need at least two more points to define the trajectory
        .take(2)
        .map(|s| Stone::new(s.pos - origin.pos, s.vel - origin.vel))
        .collect();

    // We need to find out the points where the other two
    // hailstones are aligned with the center
    // At those points, the vectors p1 and p2 are aligned (zero doc product)
    let p1 = rotated[0].pos;
    let v1 = rotated[0].vel;
    let p2 = rotated[1].pos;
    let v2 = rotated[1].vel;
    let t1 = -p1.cross(p2).dot(v2) / (v1.cross(p2).dot(v2));
    let t2 = -p1.cross(p2).dot(v1) / (p1.cross(v2).dot(v1));
    let c1 = p1 + t1 * v1;
    let c2 = p2 + t2 * v2;

    let v = (c2 - c1) / (t2 - t1);
    let p = (c1 - t1 * v) + origin.pos;
    //dbg!(p);
    p.element_sum().ceil() as usize
}
