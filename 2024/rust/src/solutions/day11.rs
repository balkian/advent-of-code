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
    fn blink(&mut self) -> &mut Self  {
        if self.value == 0 {
            self.value = 1;
            return self;
        }
        let n_digits = self.value.ilog10() + 1;
        if n_digits % 2 == 0 {
            let mult = 10usize.pow(n_digits / 2);
            let tail = self.next.replace(Box::new(Node{value: self.value % mult, next: None}));
            self.value = self.value / mult;
            let mut right = self.next.as_mut().unwrap();
            right.next = tail;
            right
        } else {
            self.value = self.value * 2024;
            return self;
        }
    }

}

pub fn parse(i: &str) -> Node {
    let mut nodes = i.split_whitespace().map(|n| Node{value: n.parse::<usize>().expect("could not parse number"), next: None}).rev();
    let last = nodes.next().expect("no last node");
    nodes.fold(last, |acc, mut node| {node.next = Some(Box::new(acc)); node})
}

pub fn part1(i: &Node) -> usize {
    let mut root = i.clone();
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
pub fn part2(i: &Node) -> usize {
    todo!();
}
