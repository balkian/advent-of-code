use crate::dbg;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use sscanf::scanf;

const N_COMMON: usize = 12;

/// Coordinates of a given point
#[allow(clippy::upper_case_acronyms)]
type XYZ = [i32; 3];

type Transformation = ([usize; 3], [isize; 3]);

/// Generate all possible XYZ transformation
const ALL_TRANSFORMS: [([usize; 3], [isize; 3]); 24] = {
    let mut res = [([0usize; 3], [0isize; 3]); 24];
    let mut position = 0;
    while position < 24 {
        let perm = position % 6;
        let signs = position / 6;
        let mut positions = [0, 0, 0];
        let idx_first = perm % 3;
        positions[idx_first] = 1;
        positions[((idx_first + 1) + perm / 3) % 3] = 2;

        let mut signs = [
            if (signs % 2) == 0 { 1 } else { -1 },
            if ((signs / 2) % 2) == 0 { 1 } else { -1 },
            1,
        ];
        if (positions[0] + 1) % 3 == positions[1] {
            signs[2] = signs[0] * signs[1];
        } else {
            signs[2] = -signs[0] * signs[1];
        }
        res[position] = (positions, signs);
        position += 1;
    }
    res
};

/// Calculate vector distance between two positions
fn distance(this: &XYZ, other: &XYZ) -> XYZ {
    this.iter()
        .zip(other.iter())
        .map(|(p2, p1)| p2 - p1)
        .collect::<Vec<i32>>()
        .as_slice()
        .try_into()
        .unwrap()
}

fn transform(point: &XYZ, (indices, signs): &Transformation, mean: XYZ, target_mean: XYZ) -> XYZ {
    [
        target_mean[0] + ((point[indices[0]] - mean[indices[0]]) * (signs[0] as i32)),
        target_mean[1] + ((point[indices[1]] - mean[indices[1]]) * (signs[1] as i32)),
        target_mean[2] + ((point[indices[2]] - mean[indices[2]]) * (signs[2] as i32)),
    ]
}

#[derive(Debug, Default, Clone)]
pub struct Cluster {
    name: String,
    points: Vec<XYZ>,
    dists: HashMap<u64, Vec<(XYZ, XYZ)>>,
    center: XYZ,
}

impl Cluster {
    fn new(name: String, points: Vec<XYZ>) -> Cluster {
        let mut dists: HashMap<u64, Vec<(XYZ, XYZ)>> = HashMap::new();
        for (a, b) in points.iter().tuple_combinations() {
            let dist = ((((a[0] - b[0]) as i64).pow(2))
                + (((a[1] - b[1]) as i64).pow(2))
                + (((a[2] - b[2]) as i64).pow(2))) as u64;
            dists.entry(dist).or_default().push((*a, *b));
        }
        Cluster {
            name,
            points,
            dists,
            center: [0; 3],
        }
    }

    fn compare(self: &Cluster, other: &Cluster) -> Vec<u64> {
        self.dists
            .keys()
            .filter(|d1| other.dists.contains_key(d1))
            .copied()
            .collect()
    }

    fn get_clique(self: &Cluster, dists: &[u64], size: usize) -> HashMap<Vec<u64>, XYZ> {
        if (size * (size - 1)) / 2 > dists.len() {
            panic!(
                "not enough distances ({}) to get a clique of size {size}",
                dists.len()
            );
        }

        let mut alldists =
            dists
                .iter()
                .fold(HashMap::<XYZ, HashMap<XYZ, u64>>::new(), |mut v, d| {
                    let ps = self.dists.get(d).unwrap();
                    for (p1, p2) in ps {
                        v.entry(*p1).or_default().insert(*p2, *d);
                        v.entry(*p2).or_default().insert(*p1, *d);
                    }
                    v
                });

        alldists.retain(|_k, v| v.len() >= size - 1);

        let allkeys: Vec<XYZ> = alldists.keys().copied().collect();

        let mut table: HashMap<Vec<u64>, XYZ> = Default::default();
        for (k, v) in alldists.into_iter() {
            let mut array: Vec<u64> = v
                .into_iter()
                .filter(|(p2, _)| allkeys.contains(p2))
                .map(|(_, d)| d)
                .collect();
            array.sort_unstable();
            table.insert(array, k);
        }
        debug_assert!(table.len() == size);
        table
    }

    /// Use the information about matching distances to move this cluster's
    /// points to match those of the other cluster
    fn adjust_to(
        self: &mut Cluster,
        other: &Cluster,
        dists: &[u64],
        size: usize,
    ) -> &'static Transformation {
        // println!("Adjusting {} to {}", &self.name, &other.name);

        let mine = self.get_clique(dists, size);
        let theirs = other.get_clique(dists, size);

        debug_assert!(theirs.len() == mine.len());

        for (m, v) in mine.iter() {
            if !theirs.contains_key(m) {
                dbg!(&theirs);
                dbg!(&mine);
            }
            debug_assert!(theirs.contains_key(m));
            debug_assert!(self.points.contains(v));
        }

        for (m, v) in theirs.iter() {
            debug_assert!(mine.contains_key(m));
            debug_assert!(&other.points.contains(v));
        }

        let (d_reference, reference) = mine.iter().next().unwrap();

        let target = theirs
            .get(d_reference)
            .expect("No match in the other clique");

        let calculate = || {
            'trans: for t in ALL_TRANSFORMS.iter() {
                let transformed = transform(reference, t, Default::default(), Default::default());
                let offset = distance(target, &transformed);

                for (dist, point) in mine.iter().skip(1) {
                    let transformed = transform(point, t, Default::default(), Default::default());

                    let target = theirs.get(dist).expect("No match in the other clique");

                    let new_offset = distance(target, &transformed);

                    debug_assert!(
                        distance(target, &transform(point, t, [0; 3], new_offset)) == [0; 3]
                    );

                    if new_offset != offset {
                        continue 'trans;
                    }
                }
                return (t, offset);
            }
            panic!("no valid solution found");
        };

        let (transformation, offset) = calculate();
        self.points = self
            .points
            .iter()
            .map(|my| transform(my, transformation, [0; 3], offset))
            .collect();

        for (_, p2) in theirs.iter() {
            debug_assert!(self.points.contains(p2));
        }

        for (_k, vs) in self.dists.iter_mut() {
            for (v1, v2) in vs.iter_mut() {
                *v1 = transform(v1, transformation, [0; 3], offset);
                *v2 = transform(v2, transformation, [0; 3], offset);
            }
        }

        self.center = transform(&[0i32; 3], transformation, [0; 3], offset);
        transformation
    }
}

pub fn parse(input: &str) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = vec![];
    let mut points: Option<Vec<XYZ>> = None;
    let mut name: Option<String> = None;
    for line in input.lines() {
        if line.starts_with("---") {
            let x: &[_] = &['-', ' '];

            name = Some(line.trim_matches(x).to_owned());
            points = Some(vec![]);
        } else if line.is_empty() {
            clusters.push(Cluster::new(name.take().unwrap(), points.take().unwrap()));
        } else {
            let coords = scanf!(line.trim(), "{},{},{}", i32, i32, i32).unwrap();
            points
                .as_mut()
                .unwrap()
                .push([coords.0, coords.1, coords.2]);
        }
    }
    if let Some(points) = points {
        clusters.push(Cluster::new(name.take().unwrap(), points));
    }

    clusters
}

fn merge(clusters: &mut Vec<Cluster>) {
    let configs: HashSet<Transformation> = HashSet::from_iter(ALL_TRANSFORMS);
    debug_assert!(configs.len() == 24);

    let mut pairs: HashMap<usize, HashMap<usize, Vec<u64>>> = Default::default();

    for ((idx1, c1), (idx2, c2)) in clusters.iter().enumerate().tuple_combinations() {
        let common = c1.compare(c2);
        if common.len() < (N_COMMON) * (N_COMMON - 1) / 2 {
            continue;
        }
        pairs.entry(idx1).or_default().insert(idx2, common.clone());
        pairs.entry(idx2).or_default().insert(idx1, common);
    }

    let mut queue = vec![];
    let mut missing = vec![false; clusters.len()];
    for (&c1, map) in pairs.iter() {
        missing[c1] = true;
        for &c2 in map.keys() {
            missing[c2] = true;
        }
    }

    let mut current: Cluster = Default::default();
    let mut adjusted: HashMap<String, usize> = Default::default();
    while let Some(first) = missing
        .iter()
        .enumerate()
        .filter_map(|(idx, &c)| if c { Some(idx) } else { None })
        .next()
    {
        let c1 = queue
            .pop()
            .or_else(|| {
                missing[first] = false;
                Some(first)
            })
            .unwrap();
        if let Some(mapping) = pairs.get(&c1) {
            for (c2, common) in mapping.iter() {
                if !missing[*c2] {
                    continue;
                }
                current = std::mem::replace(&mut clusters[*c2], current);
                *adjusted.entry(current.name.clone()).or_default() += 1;
                current.adjust_to(&clusters[c1], common, N_COMMON);
                current = std::mem::replace(&mut clusters[*c2], current);
                missing[*c2] = false;
                queue.push(*c2);
            }
        }
    }
}

pub fn part1(clusters: &[Cluster]) -> usize {
    let mut clusters: Vec<Cluster> = clusters.to_vec();
    let unique: HashSet<XYZ> = clusters
        .iter()
        .flat_map(|c| c.points.iter())
        .copied()
        .collect();

    let n_before = unique.len();
    merge(&mut clusters);
    let unique: HashSet<XYZ> = clusters
        .iter()
        .flat_map(|c| c.points.iter())
        .copied()
        .collect();
    dbg!(n_before);
    dbg!(unique.len());
    unique.len()
}

pub fn part2(clusters: &[Cluster]) -> i32 {
    let mut clusters: Vec<Cluster> = clusters.to_vec();
    merge(&mut clusters);
    let mut max_dist = 0;
    for c1 in clusters.iter() {
        for c2 in clusters.iter() {
            if c1.name == c2.name {
                continue;
            }
            let dist = distance(&c1.center, &c2.center)
                .iter()
                .map(|c| c.abs())
                .sum::<i32>();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust() {
        let c1 = Cluster::new(String::from("c1"), vec![[-1, -2, -3], [1, 2, 3]]);

        for t in ALL_TRANSFORMS.iter() {
            let mut c2 = Cluster::new(
                String::from("c2"),
                c1.points
                    .iter()
                    .map(|p| transform(p, t, [0, 0, 0], [100, 200, 300]))
                    .collect(),
            );
            let dists = c1.compare(&c2);
            assert!(t == c2.adjust_to(&c1, &dists, 2));
        }
    }
}
