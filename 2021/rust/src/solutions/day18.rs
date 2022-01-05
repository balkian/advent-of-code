//! I have split this problem into two parts:
//!   * Parsing the input into a binary tree
//!   * Implementing binary tree "snailfish number" addition
//!  
//! The trickiest part was to code a Tree that would allow "stateful" traversal
//! i.e., traversing the graph while keeping a reference to visited nodes.
//! That was needed in order to explode/split some numbers.
//!
//! Note that this was just an excuse to learn more about Rc and RefCell :)
//! There are much simpler alternatives than rolling your own Tree.
//! One example is using string manipulation. The "snailfish" numbers
//! and the operations are so simple that a tree is not necessary.
//! But even if you want to use a tree, there are many crates around.
//!
//! The code is rough around the edges, and I may revisit it to improve it.
//! For instance, there's a lot of code repetition. Some of it might be avoided
//! by converting the [`TreeRef`] type into a struct with an inner Rc, and
//! implementing [`Deref`] and [`Clone`].

use std::cell::RefCell;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;
use std::rc::Rc;

pub fn parse(input: &str) -> Vec<Tree> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Tree::parse)
        .collect()
}

pub fn part1(input: &[Tree]) -> usize {
    let result = input.iter().sum::<Tree>();
    result.magnitude()
}

pub fn part2(input: &[Tree]) -> usize {
    input
        .iter()
        .flat_map(|x| input.iter().filter(move |y| y != &x).map(move |y| (x, y)))
        .map(|(x, y)| (x + y).magnitude())
        .max()
        .unwrap()
}

#[derive(PartialEq)]
pub struct Tree {
    root: Option<TreeRef>,
}

/// Indirection over tree elements that provides interior mutability
/// (necessary for our traversal algorithm).
pub type TreeRef = Rc<RefCell<TreeNode>>;

#[derive(PartialEq)]
pub enum TreeNode {
    Leaf(usize),
    Branch { left: TreeRef, right: TreeRef },
}

use TreeNode::*;

/// Deep-clone a Tree.
/// The root of a tree is an Rc element, whose clone behavior is to increment the Rc, instead
/// of copying the value.
/// In this problem we only really need Rcs to traverse the graph, we don't want cloned trees
/// to share inner value.
impl Clone for Tree {
    fn clone(&self) -> Self {
        Tree {
            root: self
                .root
                .as_ref()
                .map(|r| Rc::new(RefCell::new((*r).borrow().clone()))),
        }
    }
}

/// Deep-clone a TreeNode.
/// Branches include Rc elements, and their clone behavior is to increment the Rc, instead
/// See [`Tree::Clone`].
impl Clone for TreeNode {
    fn clone(&self) -> Self {
        match self {
            Leaf(u) => Leaf(*u),
            Branch { left, right } => Branch {
                left: Rc::new(RefCell::new(TreeNode::clone(&(*left).borrow()))),
                right: Rc::new(RefCell::new(TreeNode::clone(&(*right).borrow()))),
            },
        }
    }
}

impl TreeNode {
    fn magnitude(&self) -> usize {
        match self {
            Leaf(i) => *i,
            Branch { left: a, right: b } => 3 * a.borrow().magnitude() + 2 * b.borrow().magnitude(),
        }
    }
}

impl<'a, 'b> Add<&'b Tree> for &'a Tree {
    type Output = Tree;
    fn add(self, other: &'b Tree) -> Self::Output {
        if let Some(myroot) = &self.root {
            if let Some(thatroot) = &other.root {
                let mut out = Tree {
                    root: Some(Rc::new(RefCell::new(Branch {
                        left: Rc::new(RefCell::new(TreeNode::clone(&(*myroot).borrow()))),
                        right: Rc::new(RefCell::new(TreeNode::clone(&(*thatroot).borrow()))),
                    }))),
                };
                out.simplify();
                out
            } else {
                self.clone()
            }
        } else {
            other.clone()
        }
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn from_root(root: TreeNode) -> Self {
        Tree {
            root: Some(Rc::new(RefCell::new(root))),
        }
    }

    fn parse(line: &str) -> Self {
        let mut number = Leaf(0);
        let mut heap: Vec<TreeNode> = vec![];
        for k in line.trim().chars() {
            match k {
                '[' => {
                    let old_number = std::mem::replace(
                        &mut number,
                        Branch {
                            left: Rc::new(RefCell::new(Leaf(0))),
                            right: Rc::new(RefCell::new(Leaf(0))),
                        },
                    );
                    heap.push(old_number);
                }
                ']' => {
                    let old_number = std::mem::replace(&mut number, heap.pop().unwrap());
                    if let Branch { ref mut right, .. } = number {
                        *right.borrow_mut() = old_number;
                    }
                }
                ',' => {
                    let old_number = std::mem::replace(&mut number, heap.pop().unwrap());
                    if let Branch { ref mut left, .. } = number {
                        *left.borrow_mut() = old_number;
                    }
                }
                _ => {
                    let num = k.to_digit(10).unwrap() as usize;
                    let old_number = std::mem::replace(&mut number, Leaf(num));
                    heap.push(old_number);
                }
            }
        }
        Tree::from_root(number)
    }
    fn magnitude(&self) -> usize {
        match &self.root {
            None => 0,
            Some(x) => x.borrow().magnitude(),
        }
    }

    fn split(&mut self) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];

        let mut to_replace = None;

        stack.push(self.root.to_owned().unwrap());

        while let Some(curr) = stack.pop() {
            match *curr.borrow() {
                Leaf(c) => {
                    if c > 9 {
                        to_replace = Some(Branch {
                            left: Rc::new(RefCell::new(Leaf(c / 2))),
                            right: Rc::new(RefCell::new(Leaf((c + 1) / 2))),
                        });
                    }
                }
                Branch {
                    ref left,
                    ref right,
                } => {
                    stack.push(right.to_owned());
                    stack.push(left.to_owned());
                }
            }
            if let Some(r) = to_replace {
                *curr.borrow_mut() = r;
                return true;
            }
        }
        false
    }
    fn explode(&mut self) -> bool {
        let mut visited = vec![];
        let mut stack: Vec<(usize, Rc<RefCell<TreeNode>>)> = vec![];
        let mut replace_right = None;
        let mut replace_this = false;

        stack.push((0, self.root.to_owned().unwrap()));

        while let Some((level, curr)) = stack.pop() {
            match &mut *curr.borrow_mut() {
                Leaf(ref mut c) => {
                    if let Some(ref b) = replace_right {
                        *c += b;
                        return true;
                    }
                    visited.push(curr.clone());
                }
                Branch {
                    ref left,
                    ref right,
                } => match (level, &*left.borrow(), &*right.borrow()) {
                    (lev, Leaf(a), Leaf(b)) if replace_right.is_none() && lev >= 4 => {
                        if !visited.is_empty() {
                            if let Leaf(ref mut c) = *(visited[visited.len() - 1]).borrow_mut() {
                                *c += a
                            } else {
                                panic!("element in the visited list is not a leaf");
                            }
                        }
                        replace_right = Some(*b);
                        replace_this = true;
                    }
                    _ => {
                        stack.push((level + 1, right.to_owned()));
                        stack.push((level + 1, left.to_owned()));
                    }
                },
            }
            if replace_this {
                replace_this = false;
                *curr.borrow_mut() = Leaf(0);
            }
        }
        replace_right.is_some()
    }

    fn simplify(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }
}

impl<'a> Sum<&'a Self> for Tree {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        if let Some(first) = iter.next() {
            iter.fold(first.clone(), |a, b| &a + b)
        } else {
            Tree::new()
        }
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.root.as_ref().unwrap().borrow()).fmt(f)
    }
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Leaf(i) => write!(f, "{}", i)?,
            Branch { left, right } => {
                write!(f, "[{:?},{:?}]", left.borrow(), right.borrow())?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_reduce1() {
    for (input, expected) in [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
        (
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ),
        (
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        ),
    ] {
        let mut input = Tree::parse(input);
        let expected = Tree::parse(expected);
        input.simplify();
        assert_eq!(input, expected);
    }
}

#[test]
fn test_sum() {
    let p1 = Tree::parse("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]");
    let p2 = Tree::parse("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
    let res = Tree::parse("[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]");
    assert_eq!((&p1 + &p2), res);
    assert_eq!((&p1 + &p2).magnitude(), 3993);
}

use crate::aoc_sample;
aoc_sample!(day18sample1part1, "../../day18.sample1", part1, 4140);
aoc_sample!(day18sample4part2, "../../day18.sample1", part2, 3993);
