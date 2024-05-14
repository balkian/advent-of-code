use std::collections::{BTreeMap, BTreeSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Brick {
    points: BTreeMap<(usize, usize), Vec<usize>>,
    bottom: usize,
    top: usize,
}

impl Brick {
    fn new(c1: Point, c2: Point) -> Self {
        let y0 = c1.y.min(c2.y);
        let y1 = c1.y.max(c2.y);
        let x0 = c1.x.min(c2.x);
        let x1 = c1.x.max(c2.x);
        let z0 = c1.z.min(c2.z);
        let z1 = c1.z.max(c2.z);

        let mut points: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                for z in z0..=z1 {
                    points.entry((x, y)).or_default().push(z);
                }
            }
        }
        Brick {
            points,
            bottom: z0,
            top: z1,
        }
    }

    fn fall(&mut self, height: usize) {
        self.bottom -= height;
        self.top -= height;
        for v in self.points.values_mut() {
            for z in v.iter_mut() {
                *z -= height;
            }
        }
    }

    fn z(&self) -> usize {
        self.bottom
    }

    fn distance(&self, other: &Self) -> Option<usize> {
        self.points
            .iter()
            .filter_map(|(xy, zs)| {
                let myz = &zs[0];
                other
                    .points
                    .get(xy)
                    .and_then(|ozs| ozs.iter().filter(|&oz| oz < myz).last().map(|oz| myz - oz))
            })
            .min()
            .map(|m| m - 1)
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    bricks: Vec<Brick>,
    dependencies: BTreeMap<usize, BTreeSet<usize>>,
}

impl Problem {
    fn new(bricks: &[Brick]) -> Self {
        let mut bricks: Vec<_> = bricks.into();
        bricks.sort_by(|a, b| a.top.cmp(&b.top));
        let bricks = bricks.into_iter().fold(vec![], |mut acc, b| {
            let b = make_fall(&b, &acc);
            let pos = acc
                .binary_search_by_key(&b.top, |b| b.top)
                .unwrap_or_else(|e| e);
            acc.insert(pos, b);
            acc
        });
        let mut dependencies: BTreeMap<usize, BTreeSet<usize>> = Default::default();
        for (ix, bi) in bricks.iter().enumerate() {
            for (jx, bj) in bricks.iter().enumerate() {
                if ix == jx {
                    continue;
                }
                if bi.distance(bj) == Some(0) {
                    dependencies.entry(ix).or_default().insert(jx);
                }
            }
        }
        Self {
            bricks,
            dependencies,
        }
    }
}

fn make_fall(brick: &Brick, fallen: &[Brick]) -> Brick {
    let mut max_drop: Option<usize> = None;

    for other in fallen.iter().rev() {
        if let Some(thisdrop) = brick.distance(other) {
            if let Some(m) = max_drop {
                max_drop = Some(m.min(thisdrop));
            } else {
                max_drop = Some(thisdrop);
            }
            break;
        }
    }
    let mut result = brick.clone();
    result.fall(max_drop.unwrap_or_else(|| brick.z() - 1));
    result
}

pub fn parse(input: &str) -> Problem {
    let bricks: Vec<Brick> = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut points = line.split('~').map(|coords| {
                let nums: Vec<usize> = coords
                    .split(',')
                    .map(|number| {
                        number
                            .parse::<usize>()
                            .unwrap_or_else(|_| panic!("invalid number {number}"))
                    })
                    .collect();
                Point {
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                }
            });
            Brick::new(points.next().unwrap(), points.next().unwrap())
        })
        .collect();
    Problem::new(&bricks)
}

pub fn part1(problem: &Problem) -> usize {
    let mut safetodisintegrate = 0;
    for i in 0..problem.bricks.len() {
        if (i + 1..problem.bricks.len()).all(|j| {
            if let Some(deps) = problem.dependencies.get(&j) {
                !(deps.contains(&i) && deps.len() == 1)
            } else {
                true
            }
        }) {
            safetodisintegrate += 1;
        }
    }
    safetodisintegrate
}

pub fn part2(problem: &Problem) -> usize {
    let mut fallen = 0;
    let dependencies = &problem.dependencies;
    for i in 0..problem.bricks.len() {
        let mut falling: BTreeSet<usize> = Default::default();
        falling.insert(i);
        for j in i + 1..problem.bricks.len() {
            if let Some(deps) = dependencies.get(&j) {
                if !deps.is_subset(&falling) {
                    continue;
                }
                falling.insert(j);
            };
        }
        fallen += falling.len() - 1;
    }
    fallen
}
