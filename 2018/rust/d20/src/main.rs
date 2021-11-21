use std::iter::{Peekable, Iterator};

use petgraph::graphmap::UnGraphMap;
use petgraph::algo::k_shortest_path;

fn main() {
    // dbg!(Path::parse("^ENWWW(NEEE|SSE(EE|N))$"));
    let input = &std::fs::read_to_string("input").expect("could not read file");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}


fn solve1(input: &str) -> usize {
    let g = get_graph(&get_paths(input));
    let distances = k_shortest_path(&g, (0,0), None, 1, |_| 1);
    distances.values().max().cloned().unwrap_or(0)
}

fn solve2(input: &str) -> usize {
    let g = get_graph(&get_paths(input));
    let distances = k_shortest_path(&g, (0,0), None, 1, |_| 1);
    distances.values().filter(|&&x| x >= 1000).count()
}


fn get_paths(input: &str) -> Vec<String> {
    let peek = &mut input.chars().peekable();
    possible_it(peek).unwrap()
}

fn get_graph(paths: &[String]) -> UnGraphMap<(isize, isize), ()> {
    let mut g = UnGraphMap::<(isize, isize), ()>::new();

    let start = g.add_node((0,0));

    for path in paths.iter() {
        let mut pos = start;
        for s in path.chars() {
            let new_pos = match s {
                'N' => g.add_node((pos.0-1, pos.1)),
                'S' => g.add_node((pos.0+1, pos.1)),
                'W' => g.add_node((pos.0, pos.1-1)),
                'E' => g.add_node((pos.0, pos.1+1)),
                _ => panic!("unknown char"),
            };
            g.add_edge(pos, new_pos, ());
            pos = new_pos;
        }
    }
    g
}

fn print_g(g: &UnGraphMap<(isize, isize), ()>) {
    let min_x = g.nodes().map(|n| n.1).min().unwrap();
    let min_y = g.nodes().map(|n| n.0).min().unwrap();
    let max_x = g.nodes().map(|n| n.1).max().unwrap();
    let max_y = g.nodes().map(|n| n.0).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if g.contains_node((y,x)) {
                if (y,x) == (0,0) {'X'} else {'.'}
            } else {
                '#'
            };
            print!("{}", c);
            let c = if g.contains_edge((y, x), (y, x+1)){
                '|'
            } else {
                '#'
            };
            print!("{}", c);
        }
        println!();
        for x in min_x..=max_x {
            let c = if g.contains_edge((y,x), (y+1, x)) {
                "-#"
            } else {
                "##"
            };
            print!("{}", c);
        }
        println!();
    }
}


fn possible_it(it: &mut Peekable<impl Iterator<Item=char>>) -> Option<Vec<String>> {
    let mut opts = vec!();
    match it.peek() {
        Some('^') => {
            it.next();
            return possible_it(it)
        },
        Some('$') => {
            return None
        }
        None | Some(')') | Some('|') => {
            return None
        },
        Some('(') => {
            it.next();
            loop {
                if let Some(a) = possible_it(it) {
                    for p in a {
                        opts.push(p);
                    }
                }else {
                    println!("adding empty");
                    opts.push("".to_string());
                }
                match it.peek() {
                    Some(')') => break,
                    Some('|') => {it.next(); continue},
                    _ => panic!("this char should not happen"),
                }
            }
        }
        Some(a @ ('N' | 'S' | 'E' | 'W')) => {
            opts.push(a.to_string());
        }
        c => panic!("unknown character {:?}", c),
    }
    it.next();
    if let Some(next) = possible_it(it){
        let mut res = vec!();
        for opt in opts {
            for n in next.iter() {
                res.push(opt.clone() + n);
            }
        }
        Some(res)
    } else {
        Some(opts)
    }

}

#[test]
fn test_example(){
    macro_rules! assert_length {
        ($path:expr, $length:tt) => {
            assert_eq!(solve1($path), $length);
        }
    }
    assert_length!("^WNE$", 3);
    assert_length!("^ENWWW(NEEE|SSE(EE|N))$", 10);
    assert_length!("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23);
    assert_length!("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$", 31);
    assert_length!("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18);
}
