/// What a mess! Next time I should use a proper graph/tree library
use std::collections::{HashMap, VecDeque};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct TreeDef<'a> {
    out: HashMap<&'a str, Node>,
    stack: VecDeque<(&'a str, &'a str, &'a str, &'a str)>,
}

impl<'a> TreeDef<'a> {
    fn get_root(&self) -> Node {
        let mut out = self.out.clone();
        let mut stack = self.stack.clone();
        while let Some((name, op1, op, op2)) = stack.pop_front() {
            if out.contains_key(op1) && out.contains_key(op2) {
                if let (Some(op1), Some(op2)) = (out.get(&op1), out.get(&op2)) {
                    let newnode = match op {
                        "*" => op1 * op2,
                        "-" => op1 - op2,
                        "+" => op1 + op2,
                        "/" => op1 / op2,
                        "=" => Node::Op(Op::Eq, Box::new(op1.clone()), Box::new(op2.clone())),
                        t => panic!("unknown operation {t}"),
                    };
                    out.insert(name, newnode);
                }
            } else {
                stack.push_back((name, op1, op, op2));
            }
        }

        out.remove("root").unwrap()
    }
}

type NodeRef = Box<Node>;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Op {
    fn apply(&self, a: &i128, b: &i128) -> Option<i128> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => {
                if a % b == 0 {
                    Some(a / b)
                } else {
                    None
                }
            }
            Op::Eq => panic!("this should never be applied"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Op(Op, NodeRef, NodeRef),
    Num(i128),
    Human,
}

impl Node {
    fn simplify(&self) -> Option<Self> {
        let res = match self {
            Node::Op(op, a, b) => match (a.simplify(), b.simplify()) {
                (Some(Node::Num(a)), Some(Node::Num(b))) => Node::Num(op.apply(&a, &b)?),
                (Some(a), Some(b)) => Node::Op(*op, Box::new(a), Box::new(b)),
                (Some(a), None) => Node::Op(*op, Box::new(a), b.clone()),
                (None, Some(b)) => Node::Op(*op, a.clone(), Box::new(b)),
                _ => return None,
            },
            _ => self.clone(),
        };
        Some(res)
    }

    fn balance(&self) -> Option<Self> {
        match self {
            Node::Op(Op::Eq, a, b) => match (a.as_ref(), b.as_ref()) {
                (a @ &Node::Num(i), Node::Op(op, b, c))
                | (Node::Op(op, b, c), a @ &Node::Num(i)) => {
                    let (left, right) = match (op, b.value(), c.value()) {
                        //     (&Node::Num(b), c) | (c, &Node::Num(b)) => {
                        (Op::Add, Some(_), None) => (c.clone(), a - b),
                        (Op::Add, None, Some(_)) => (b.clone(), a - c),
                        (Op::Sub, Some(_), None) => (c.clone(), &**b - a),
                        (Op::Sub, None, Some(_)) => (b.clone(), a + c),
                        (Op::Mul, None, Some(j)) if i % j == 0 => (b.clone(), a / c),
                        (Op::Mul, Some(j), None) if i % j == 0 => (c.clone(), a / b),
                        (Op::Div, Some(j), None) if j % i == 0_ => (c.clone(), &(**b).clone() / a),
                        (Op::Div, None, Some(_)) => (b.clone(), a * c),
                        _ => todo!(),
                    };
                    return Some(Node::Op(
                        Op::Eq,
                        Box::new(left.simplify().unwrap()),
                        Box::new(right.simplify().unwrap()),
                    ));
                }
                _ => {}
            },
            _ => {}
        }
        None
    }

    fn value(&self) -> Option<i128> {
        match self {
            Node::Num(i) => Some(*i),
            _ => None,
        }
    }
}

macro_rules! impl_op {
    ($key:ident, $func:ident) => {
        impl $key for &Node {
            type Output = Node;

            fn $func(self, other: Self) -> Self::Output {
                Node::Op(Op::$key, Box::new(self.clone()), Box::new(other.clone()))
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Mul, mul);
impl_op!(Sub, sub);
impl_op!(Div, div);

pub fn parse(input: &str) -> TreeDef {
    let mut out: HashMap<&str, Node> = Default::default();
    let mut stack: VecDeque<(&str, &str, &str, &str)> = Default::default();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (l, r) = line.trim().split_once(": ").unwrap();
        if let Ok(num) = r.trim().parse::<i128>() {
            out.insert(l, Node::Num(num));
        } else {
            let toks: Vec<_> = r.trim().split_whitespace().collect();
            stack.push_front((l, toks[0], toks[1], toks[2]));
        }
    }
    TreeDef { out, stack }
}

pub fn part1(input: &TreeDef) -> i128 {
    let root = input.get_root().simplify().unwrap();
    root.value().unwrap()
}

pub fn part2(input: &TreeDef) -> i128 {
    let mut input = input.clone();
    input.out.insert("humn", Node::Human);
    input.stack.iter_mut().for_each(|(name, _op1, op, _op2)| {
        if *name == "root" {
            *op = "="
        }
    });
    let mut root = input.get_root().simplify().unwrap();
    while let Some(new_root) = root.balance() {
        root = new_root;
    }
    match root {
        Node::Op(Op::Eq, a, b) => match (*a, *b) {
            (num, Node::Human) | (Node::Human, num) => {
                return num.value().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
    panic!("no human on either side");
}
