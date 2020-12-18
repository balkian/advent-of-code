#[derive(Debug, Clone)]
enum Node {
    Op(char),
    Value(isize),
    Tree(Box<Tree>),
}

#[derive(Debug, Clone)]
struct Tree {
    children: Vec<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree { children: vec![] }
    }
    fn binarize(&mut self, precedence: &[char]) {
        for child in self.children.iter_mut() {
            if let Node::Tree(t) = child {
                t.binarize(precedence);
            }
        }
        match self.children.len() {
            x if x == 3 => {}
            x if x > 3 => {
                'outer: for op in precedence {
                    loop {
                        if self.children.len() == 3 {
                            break 'outer;
                        }
                        let idx = self.children.iter().position(|x| {
                            if let Node::Op(this) = x {
                                this == op
                            } else {
                                false
                            }
                        });
                        match idx {
                            Some(pos) => {
                                let children = self.children.drain(pos - 1..=pos + 1).collect();
                                self.children.insert(
                                    pos - 1,
                                    Node::Tree(Box::new(Tree { children })),
                                );
                                continue;
                            }
                            None => break,
                        }
                    }
                }
                while self.children.len() > 3 {
                    let children = self.children.drain(0..=2).collect();
                    self.children
                        .insert(0, Node::Tree(Box::new(Tree { children })));
                }
            }
            c => {
                panic!("invalid children: {}", c)
            }
        }
    }

    fn calculate(&self) -> isize {
        if self.children.is_empty() {
            return 0;
        }
        match self.children.len() {
            0 => 0,
            1 => match &self.children[0] {
                Node::Value(x) => *x,
                o => panic!("invalid single operand: {:?}", o),
            },
            3 => {
                let x = match &self.children[0] {
                    Node::Value(x) => *x,
                    Node::Tree(t) => t.calculate(),
                    _ => panic!("invalid first operand"),
                };
                let y = match &self.children[2] {
                    Node::Value(x) => *x,
                    Node::Tree(t) => t.calculate(),
                    _ => panic!("invalid second operand"),
                };
                match &self.children[1] {
                    Node::Op('*') => x * y,
                    Node::Op('+') => x + y,
                    _ => panic!("invalid operation"),
                }
            }
            _ => panic!("invalid children"),
        }
    }
}

fn parse(input: &str) -> Tree {
    let mut buffer = String::new();
    let mut stack: Vec<Tree> = Vec::new();
    stack.push(Tree::new());

    for c in input.chars().chain(std::iter::once('\n')) {
        match c {
            '0'..='9' => {
                buffer.push(c);
            }
            ' ' => {}
            '(' | ')' | '+' | '*' | '\n' => {
                let last = stack.last_mut().unwrap();
                if !buffer.is_empty() {
                    let value: isize = buffer.parse().expect("could not parse as isize");
                    buffer = String::new();
                    last.children.push(Node::Value(value));
                }
                match c {
                    '(' => {
                        stack.push(Tree::new());
                    }
                    ')' => {
                        let child = stack
                            .pop()
                            .expect("at least one element has been added to the level");
                        let l = stack.len();
                        stack[l - 1].children.push(Node::Tree(Box::new(child)));
                    }
                    '*' | '+' => {
                        last.children.push(Node::Op(c));
                    }
                    '\n' => {}
                    _ => {
                        panic!("invalid Op")
                    }
                }
            }
            _ => {
                panic! {"invalid"}
            }
        }
    }
    stack.pop().unwrap()
}

fn calculate(input: &str) -> isize {
    let mut tree = parse(input);
    tree.binarize(&[]);
    tree.calculate()
}

fn calculate2(input: &str) -> isize {
    let mut tree = parse(input);
    tree.binarize(&['+', '*']);
    tree.calculate()
}

fn main() {
    let args = aoc_utils::app(env!("CARGO_PKG_NAME")).get_matches();
    let mut res1 = 0;
    let mut res2 = 0;
    for line in aoc_utils::file_iter_clap(&args) {
        res1 += calculate(&line);
        res2 += calculate2(&line);
    }
    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut expr = parse("1+2");
        dbg!(&expr);
        assert_eq!(expr.children.len(), 3);

        expr = parse("1 + 2 + 3");
        dbg!(&expr);
        assert_eq!(expr.children.len(), 5);

        expr = parse("(1 + 2) + 3");
        dbg!(&expr);
        assert_eq!(expr.children.len(), 3);
    }

    #[test]
    fn test_part1() {
        let examples = &[
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];
        for (input, expected) in examples {
            assert_eq!(calculate(input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let examples = &[
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];
        for (input, expected) in examples {
            assert_eq!(calculate2(input), *expected);
        }
    }
}
