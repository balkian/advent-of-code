use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    value: usize,
    next: Option<Box<Node>>,
}

impl IntoIterator for Node {
    type Item = usize;
    type IntoIter = NodeIterator;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator(Some(self))
    }
}

pub struct NodeIterator(Option<Node>);

impl Iterator for NodeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(this) = self.0.take() {
            let val = this.value;
            self.0 = this.next.map(|b| *b);
            Some(val)
        } else {
            None
        }
    }
}

impl Node {
    fn blink(&mut self) -> &mut Self {
        if self.value == 0 {
            self.value = 1;
            return self;
        }
        let n_digits = self.value.ilog10() + 1;
        if n_digits % 2 == 0 {
            let mult = 10usize.pow(n_digits / 2);
            let tail = self.next.replace(Box::new(Node {
                value: self.value % mult,
                next: None,
            }));
            self.value /= mult;
            let right = self.next.as_mut().unwrap();
            right.next = tail;
            right
        } else {
            self.value *= 2024;
            self
        }
    }
}

pub fn parse(i: &str) -> Vec<usize> {
    i.split_whitespace()
        .map(|n| n.parse::<usize>().expect("could not parse number"))
        .collect()
}

pub fn part1(i: &[usize]) -> usize {
    let mut root = i
        .iter()
        .rev()
        .fold(None, |acc, val| {
            Some(Node {
                value: *val,
                next: acc.map(Box::new),
            })
        })
        .expect("no nodes");
    for _i in 0..25 {
        let mut cur = &mut root;
        loop {
            cur = cur.blink();
            if let Some(nxt) = cur.next.as_mut() {
                cur = nxt.as_mut();
            } else {
                break;
            }
        }
    }
    root.into_iter().count()
}
pub fn part2(i: &[usize]) -> usize {
    let mut counter: HashMap<usize, usize> = i.iter().fold(Default::default(), |mut acc, val| {
        *acc.entry(*val).or_default() += 1;
        acc
    });
    for _i in 0..75 {
        let mut nc: HashMap<usize, usize> = Default::default();
        for (val, count) in counter {
            if val == 0 {
                *nc.entry(1).or_default() += count;
                continue;
            }
            let n_digits = val.ilog10() + 1;
            if n_digits % 2 == 0 {
                let mult = 10usize.pow(n_digits / 2);
                let left = val / mult;
                let right = val % mult;
                *nc.entry(left).or_default() += count;
                *nc.entry(right).or_default() += count;
            } else {
                *nc.entry(val * 2024).or_default() += count;
            }
        }
        counter = nc;
    }
    counter.values().sum()
}
