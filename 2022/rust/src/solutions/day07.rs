/// TOTALLY SLOPPY SOLUTION. I realized as some point I had chosen the wrong data structure (enums), but
/// I just went with it.
///
/// There were some tricky parts around using the borrow checker. My original solution used a self-referential
/// data structure, which I knew was a bad idea to begin with. Then I chose to use node numbers (inodes) and a separate
/// structure to contain node info (a hashmap, FS). That led to a very odd API.
///
/// TODO: Revisit this problem in the future and clean it up.

/// Using nom was a good decision. I could've just parsed it manually, but I took this as a chance to learn nom
use nom::branch::alt;
use nom::{
    bytes::complete::{is_not, tag, take_till, take_while_m_n},
    character::complete::{alpha1, char, digit1, multispace0, multispace1, newline, space1},
    character::{is_newline, is_space},
    combinator::{map, map_res, not},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use itertools::Itertools;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::iter;

/// I've chosen a totally wrong data structure (enums)
/// At some point
#[derive(Clone, Debug)]
pub enum Node<'a> {
    File(&'a str, usize),
    Dir(&'a str, Option<usize>, Vec<String>),
}

impl<'a> Node<'a> {
    fn name(&self) -> &'a str {
        match self {
            Node::File(name, _) => name,
            Node::Dir(name, _, _) => name,
        }
    }
}

#[derive(Debug)]
enum Cmd<'a> {
    Cd(&'a str),
    Ls(Vec<Node<'a>>),
}

fn dirname(input: &str) -> IResult<&str, &str> {
    terminated(take_till(|c| c == '\n'), newline)(input)
}

fn lsdir(input: &str) -> IResult<&str, Node> {
    let (input, dirname) = preceded(tag("dir "), dirname)(input)?;
    let dir = Node::Dir(dirname, None, vec![]);
    Ok((input, dir))
}

fn lsfile(input: &str) -> IResult<&str, Node> {
    let (input, (size, name)) = tuple((terminated(digit1, multispace1), dirname))(input)?;
    let file = Node::File(name, size.parse().unwrap());
    Ok((input, file))
}

fn command(input: &str) -> IResult<&str, Cmd> {
    let ls = map(
        preceded(tuple((tag("ls"), newline)), many0(alt((lsdir, lsfile)))),
        Cmd::Ls,
    );
    let cd = map(preceded(tag("cd "), dirname), Cmd::Cd);
    preceded(tag("$ "), alt((cd, ls)))(input)
}

type FS<'a> = HashMap<String, Node<'a>>;

#[allow(dead_code)]
fn print_hierarchy(s: &FS) {
    println!();
    let mut stack = vec![(0, "/")];
    while let Some((level, next)) = stack.pop() {
        let node = s.get(next).unwrap();
        print!("{:level$}", "", level = level * 5);
        match node {
            Node::File(name, size) => println!("{name} [{size}]"),
            Node::Dir(name, size, others) => {
                println!("{name} [{size:?}]");
                for other in others {
                    stack.push((level + 1, other));
                }
            }
        }
    }
}

fn calculate_size(inodes: &mut FS, name: &str) -> usize {
    let mut node = inodes.get_mut(name).expect("unknown inode").clone();
    match node {
        Node::File(_, size) => size,
        Node::Dir(_, Some(size), _) => size,
        Node::Dir(dirname, ref mut a, ref v) => {
            let v = v.clone();
            let size = v.iter().map(|s| calculate_size(inodes, s)).sum::<usize>();
            *a = Some(size);
            inodes.insert(name.to_string(), node);
            size
        }
    }
}

pub fn parse(input: &str) -> FS {
    let (input, cmds) = terminated(many1(command), multispace0)(input).unwrap();
    assert!(input.is_empty());
    let mut current = ("", None, vec![]);
    let mut inodes: HashMap<String, Node> = HashMap::new();
    let mut path = String::from("/");

    for cmd in cmds {
        dbg!(&cmd);
        match cmd {
            Cmd::Cd(dir) => {
                let last_path = path.clone();
                match dir {
                    "/" => path = "/".to_string(),
                    ".." => {
                        path = path.rsplit_once('/').unwrap().0.to_string();
                    }
                    name => {
                        path = path.clone() + "/" + name;
                        if !current.2.contains(&path) {
                            current.2.push(path.clone());
                        }
                    }
                };
                inodes.insert(last_path, Node::Dir(current.0, current.1, current.2));
                current = match inodes.remove(&path).expect("file not already in the list") {
                    Node::Dir(name, size, vs) => (name, size, vs),
                    _ => panic!("CD'ing into a file"),
                }
            }
            Cmd::Ls(nodes) => {
                for node in nodes {
                    let name = path.clone() + "/" + node.name();
                    current.2.push(name.clone());
                    inodes.insert(name, node);
                }
            }
        }
    }
    inodes.insert(path, Node::Dir(current.0, current.1, current.2));
    calculate_size(&mut inodes, "/");
    // print_hierarchy(&fs);
    inodes
}

pub fn part1(input: &FS) -> usize {
    let fs = input.clone();
    fs.values().fold(0, |acc, f| {
        acc + match f {
            Node::Dir(_, Some(size), _) if *size <= 100000 => *size,
            Node::Dir(name, None, _) => {
                dbg!(&f);
                panic!("unknown size for dir {name}")
            }
            _ => 0,
        }
    })
}

pub fn part2(input: &FS) -> usize {
    let total = 70000000;
    let minimum = 30000000;
    let occupied = match input.get("/").unwrap() {
        Node::Dir(_, Some(size), _) => size,
        _ => panic!(),
    };
    let target = occupied - (total - minimum);
    input
        .values()
        .filter_map(|d| match d {
            Node::Dir(_, Some(size), _) if *size >= target => Some(*size),
            _ => None,
        })
        .min()
        .unwrap()
}
