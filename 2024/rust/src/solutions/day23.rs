use std::collections::{HashSet, HashMap};

pub fn parse(i: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = Default::default();
    i.lines().filter(|line| !line.is_empty())
        .for_each(|line| {
            let (a, b) = line.split_once("-").expect("no edge on this line");
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        });
    graph
}

pub fn part1(g: &HashMap<&str, HashSet<&str>>) -> usize {
    //dbg!(&g);
    let candidates: HashSet<_> = g.keys().filter(|k| k.starts_with('t')).collect();
    let mut groups = vec![];

    for candidate in candidates {
        let first_hop = g.get(candidate).unwrap();
        for first in first_hop {
            if first.starts_with("t") && first <= candidate {
                continue
            }
            for second in first_hop {
                if first >= second || (second.starts_with("t") && second <= candidate) {
                    continue;
                }
                if g.get(first).unwrap().contains(second) {
                    groups.push((candidate, first, second));
                }
            }
        }
    }
    groups.len()
}

pub fn part2(g: &HashMap<&str, HashSet<&str>>) -> String {
    //dbg!(&g);
    let mut groups: Vec<_> = g.keys().map(|n| vec![*n]).collect();

    let mut candidates: Vec<&str> = g.keys().copied().collect();
    candidates.sort();

    for node in candidates {
        let Some(edges) = g.get(node) else {
            continue
        };
        let mut found = vec![];
        'groups:
        for group in groups.iter_mut() {
            if node <= group[group.len()-1] {
                continue;
            }
            for other in group.iter() {
                if !edges.contains(other) {
                    continue 'groups;
                }
            }
            let mut this = group.clone();
            this.push(node);
            found.push(this);
        }
        groups.extend(found);
    }
    let maximal = groups.into_iter().max_by_key(|g| g.len()).expect("no max found");
    maximal.join(",")
}
