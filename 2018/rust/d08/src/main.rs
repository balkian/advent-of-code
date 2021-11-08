use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn parse(input: &[usize]) -> (Node, &[usize]) {
    let nchild = input[0];
    let nmeta = input[1];
    let mut input = &input[2..];
    let mut children = vec![];
    for _ in 0..nchild {
        let (child, next) = parse(input);
        input = next;
        children.push(child);
    }
    let mut metadata = vec![];
    for _ in 0..nmeta {
        metadata.push(input[0]);
        input = &input[1..];
    }
    (Node { children, metadata }, input)
}

impl Node {
    fn from_string(input: &str) -> Self {
        let numbers: Vec<usize> = input
            .trim()
            .split(' ')
            .map(|x| {
                x.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Invalid string: '{}'", x))
            })
            .collect();
        parse(&numbers).0
    }
}

fn solve1(input: &str) -> usize {
    let node = Node::from_string(input);
    fn sumall(n: &Node) -> usize {
        n.metadata.iter().sum::<usize>() + n.children.iter().map(sumall).sum::<usize>()
    }
    sumall(&node)
}

fn solve2(input: &str) -> usize {
    let node = Node::from_string(input);
    fn sumall(n: &Node) -> usize {
        if n.children.is_empty() {
            n.metadata.iter().sum::<usize>()
        } else {
            let child_values: Vec<usize> = n.children.iter().map(sumall).collect();
            n.metadata
                .iter()
                .map(|i| child_values.get(*i - 1).unwrap_or(&0))
                .sum::<usize>()
        }
    }
    sumall(&node)
}

#[test]
fn test_example() {
    assert_eq!(solve1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
    assert_eq!(solve2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);
}
