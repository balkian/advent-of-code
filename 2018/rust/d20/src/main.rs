use petgraph::algo::k_shortest_path;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;
use std::iter::{Iterator, Peekable};

fn main() {
    let input = &std::fs::read_to_string("input").expect("could not read file");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

fn solve1(input: &str) -> usize {
    let g = get_graph(input);
    let distances = k_shortest_path(&g, (0, 0), None, 1, |_| 1);
    distances.values().max().cloned().unwrap_or(0)
}

fn solve2(input: &str) -> usize {
    let g = get_graph(input);
    let distances = k_shortest_path(&g, (0, 0), None, 1, |_| 1);
    distances.values().filter(|&&x| x >= 1000).count()
}

fn get_graph(input: &str) -> RoomGraph {
    let peek = &mut input.chars().peekable();
    let mut g = RoomGraph::new();
    walk_graph(&mut g, peek, &HashSet::from([(0, 0)]));
    if cfg!(debug_assertions) {
        print_g(&g);
    }
    g
}

fn print_g(g: &RoomGraph) {
    let min_x = g.nodes().map(|n| n.1).min().unwrap();
    let min_y = g.nodes().map(|n| n.0).min().unwrap();
    let max_x = g.nodes().map(|n| n.1).max().unwrap();
    let max_y = g.nodes().map(|n| n.0).max().unwrap();
    println!();
    let cell = |(y, x)| -> char {
        if !g.contains_node((y, x)) {
            '#'
        } else if (y, x) == (0, 0) {
            'X'
        } else {
            ' '
        }
    };
    let edge = |(y, x), (dy, dx)| -> char {
        if !g.contains_edge((y, x), (y + dy, x + dx)) {
            '#'
        } else {
            ' '
        }
    };
    for y in min_y - 1..=max_y {
        print!("#");
        for x in min_x..=max_x {
            print!("{}{}", cell((y, x)), edge((y, x), (0, 1)));
        }
        println!();
        if y < min_y {
            continue;
        }
        print!("#");
        for x in min_x..=max_x {
            print!("{}#", edge((y, x), (1, 0)));
        }
        println!();
    }
    println!();
}

type RoomGraph = UnGraphMap<(isize, isize), ()>;

fn walk_graph(
    g: &mut RoomGraph,
    it: &mut Peekable<impl Iterator<Item = char>>,
    branches: &HashSet<(isize, isize)>,
) -> Option<Vec<(isize, isize)>> {
    match it.peek() {
        Some('^') => {
            it.next();
            walk_graph(g, it, branches)
        }
        Some('$') => None,
        None | Some(')') | Some('|') => None,
        Some('(') => {
            it.next();
            let mut new_branches = HashSet::with_capacity(branches.len());
            loop {
                if let Some(new_b) = walk_graph(g, it, branches) {
                    new_branches.extend(new_b.into_iter());
                } else {
                    new_branches.extend(branches.iter().cloned())
                }
                match it.next() {
                    Some(')') => break,
                    Some('|') => continue,
                    _ => panic!("this char should not happen"),
                }
            }
            walk_graph(g, it, &new_branches)
        }
        Some('N' | 'S' | 'E' | 'W') => {
            let a = it.next().unwrap();
            let mut new_branches = HashSet::with_capacity(branches.len());
            for pos in branches {
                let new_pos = match a {
                    'N' => g.add_node((pos.0 - 1, pos.1)),
                    'S' => g.add_node((pos.0 + 1, pos.1)),
                    'W' => g.add_node((pos.0, pos.1 - 1)),
                    'E' => g.add_node((pos.0, pos.1 + 1)),
                    _ => panic!("unknown char"),
                };
                g.add_edge(*pos, new_pos, ());
                new_branches.insert(new_pos);
            }
            walk_graph(g, it, &new_branches)
        }
        c => panic!("unknown character {:?}", c),
    }
}

#[test]
fn test_example() {
    macro_rules! assert_length {
        ($path:expr, $length:tt) => {
            assert_eq!(solve1($path), $length);
        };
    }
    assert_length!("^WNE$", 3);
    assert_length!("^ENWWW(NEEE|SSE(EE|N))$", 10);
    assert_length!("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23);
    assert_length!(
        "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
        31
    );
    assert_length!("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18);
}
