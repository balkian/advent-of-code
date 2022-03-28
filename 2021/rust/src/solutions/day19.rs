use itertools::Itertools;
use std::collections::HashMap;

use sscanf::scanf;
type XYZ = (i32, i32, i32);

#[derive(Debug, Default, Clone)]
pub struct Cluster {
    name: String,
    points: Vec<XYZ>,
    dists: HashMap<u64, Vec<(XYZ, XYZ)>>,
}

impl Cluster {
    fn new(name: String, points: Vec<XYZ>) -> Cluster {
        let mut dists: HashMap<u64, Vec<(XYZ, XYZ)>> = HashMap::new();
        for (a,b) in points
            .iter()
            .tuple_combinations() {
                let dist = ((((a.0 - b.0) as i64).pow(2))
                     + (((a.1 - b.1) as i64).pow(2))
                     + (((a.2 - b.2) as i64).pow(2))) as u64;
                if dists.contains_key(&dist) {
                    println!("Distance already present: {dist}");
                }
                dists.entry(dist).or_default().push((a.clone(), b.clone()));
            
            }
        Cluster {
            name,
            points,
            dists,
        }
    }

    fn compare(self: &Cluster, other: &Cluster) -> Vec<u64> {
        self.dists
            .keys()
            .filter(|d1| other.dists.contains_key(d1))
            .copied()
            .collect()
    }

    fn get_clique(self: &Cluster, dists: &[u64]) -> Vec<XYZ> {
        let allpoints = dists.iter().fold(HashMap::<XYZ, u64>::new(), |mut v, d| {
            let ps = self.dists.get(d).unwrap();
            for (p1, p2) in ps {
                *v.entry(*p1).or_default() += 1;
                *v.entry(*p2).or_default() += 1;
            }
            v
        });
        println!("{allpoints:?}");
        unreachable!();
        //allpoints

    }

    /// Use the information about matching distances to move this cluster's
    /// points to match those of the other cluster
    fn adjust_to(self: &mut Cluster, other: &Cluster, dists: &[u64]) {
        println!("Adjusting {} to {}", &self.name, &other.name);

        let theirs: Vec<XYZ> = self.get_clique(dists);        //let theirs: Vec<XYZ>
        let mine: Vec<XYZ> = other.get_clique(dists);         //let theirs: Vec<XYZ>
        unimplemented!();
        // for each config of mine (rotating my points)
        // check if the configuration matches their points
        // if it does, apply the config to all my points

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
            points.as_mut().unwrap().push(coords);
        }
    }
    if let Some(points) = points {
        clusters.push(Cluster::new(name.take().unwrap(), points));
    }

    clusters
}

pub fn part1(clusters: &[Cluster]) -> isize {
    // println!("{clusters:?}");
    let mut clusters: Vec<Cluster> = clusters.iter().cloned().collect();
    let pairs: Vec<_> = clusters
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((idx1, c1), (idx2, c2))| (idx1, idx2, c1.compare(c2)))
        .filter(|(_,_,common)| common.len() >= (11*6))
        .collect();
    let mut current: Cluster = Default::default();
    for (c1, c2, common) in pairs {
            println!("{:?} - {}", (&clusters[c1].name, &clusters[c2].name), common.len());
            current = std::mem::replace(&mut clusters[c2], current);
            current.adjust_to(&mut clusters[c1], &common);
            current = std::mem::replace(&mut clusters[c2], current);
    }
    unimplemented!();
}

pub fn part2(clusters: &[Cluster]) -> isize {
    unimplemented!();
}
