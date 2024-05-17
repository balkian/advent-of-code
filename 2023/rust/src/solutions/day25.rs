use rand::prelude::*;
use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

pub fn parse(input: &str) -> Graph {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .fold(Default::default(), |mut acc, line| {
            let (key, vals) = line.trim().split_once(": ").unwrap();
            let toks: Vec<_> = vals.split_whitespace().collect();
            for tok in &toks {
                acc.entry(tok).or_default().insert(key, 1);
                acc.entry(key).or_default().insert(tok, 1);
            }
            acc
        })
}

pub fn part1(input: &Graph) -> usize {
    let node_counts: &mut HashMap<&str, usize> = &mut Default::default();
    loop {
        let g = &mut input.clone();
        node_counts.clear();
        for n in g.keys() {
            node_counts.insert(n, 1);
        }
        while g.len() > 2 {
            //dbg!(g.len());
            contract(g, node_counts);
            debug_assert_eq!(g.len(), node_counts.len());
        }
        let cut = *g.values().next().unwrap().values().next().unwrap();
        //dbg!(cut);
        if cut == 3 {
            //dbg!(&g);
            //dbg!(&node_counts);
            return node_counts.values().fold(1, |acc, &c| acc * c);
        }
    }
}

fn contract<'a>(graph: &mut Graph<'a>, node_counts: &mut HashMap<&'a str, usize>) {
    #[cfg(debug_assertions)]
    let active_nodes = graph.len();
    #[cfg(debug_assertions)]
    let total_nodes = node_counts.values().sum::<usize>();
    let rng = &mut rand::thread_rng();
    let n2: &str = *graph.keys().choose(rng).unwrap();

    // Remove every edge (n2, n3)
    let to_merge = graph.remove(n2).unwrap();

    let n1: &str = *to_merge.keys().choose(rng).unwrap();

    // Remove edge (n2, n1)

    for (n3, c) in to_merge {
        // Remove every edge (n3, n2)
        graph.entry(n3).or_default().remove(n2);
        // update weights
        if n3 != n1 {
            *graph.entry(n1).or_default().entry(n3).or_default() += c;
            *graph.entry(n3).or_default().entry(n1).or_default() += c;
        }
    }

    let cn2 = node_counts.remove(n2).unwrap();
    *node_counts.entry(n1).or_default() += cn2;
    #[cfg(debug_assertions)]
    {
        //dbg!(n1, n2, &graph);
        //dbg!(cn2);
        //dbg!(&node_counts);
        assert!(n1 != n2);
        assert!(!graph.contains_key(n2));
        assert_eq!(active_nodes - 1, graph.len());
        assert_eq!(active_nodes - 1, node_counts.len());
        assert_eq!(total_nodes, node_counts.values().sum::<usize>());
        assert!(graph
            .values()
            .flatten()
            .position(|(&k, _)| k == n2)
            .is_none());
    }
}

pub fn part2(_input: &Graph) -> &'static str {
    "Done!"
}
