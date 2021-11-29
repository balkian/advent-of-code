use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use petgraph::graphmap::UnGraphMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
}

type Graph<'a> = UnGraphMap::<&'a str, usize>;

pub fn parse(input: &str) -> Graph {
    let mut g: Graph = UnGraphMap::new();
    for line in input.lines() {
        let res = RE.captures(line).unwrap();
        g.add_edge(res.get(1).unwrap().as_str(), 
                   res.get(2).unwrap().as_str(),
                   res.get(3).unwrap().as_str().parse::<usize>().unwrap());
    }
    g
}


fn walk<'a, 'b, F>(g: &'a Graph, cur: Option<&'b str>, dist: usize,  path: Vec<&'a str>, sort: F) -> Option<(usize, Vec<&'a str>)>
where F: Copy + Fn(&usize, &usize) -> Ordering  
{
    if path.len() == g.node_count() {
        return Some((dist, path))
    }
    let mut opts = vec!();
    for n in g.nodes() {
        if path.contains(&n) {
            continue
        }
        let ndist = match cur {
            None => dist,
            Some(a) if g.contains_edge(a, n) => dist + g.edge_weight(a, n).unwrap(),
            _ => continue
        };

        let mut nvis = path.clone();
        nvis.push(n);
        if let Some(res) = walk(g, Some(n), ndist, nvis, sort) {
            opts.push(res);
        }
    }
    if opts.is_empty() {
        return None
    }
    opts.sort_by(|(dist1, _), (dist2, _)| sort(dist1, dist2));
    Some(opts.remove(0))

}

pub fn part1(input: &Graph) -> usize {
    walk(input, None, 0, vec!(), |a,b| a.cmp(b) ).unwrap().0
}

pub fn part2(input: &Graph) -> usize {
    walk(input, None, 0, vec!(), |a,b| b.cmp(a) ).unwrap().0
}
