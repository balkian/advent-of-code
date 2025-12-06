#[derive(Debug, Clone)]
pub enum Operation {
    Sum,
    Multiply,
}

impl Operation {
    fn reduce(&self, nums: &[usize]) -> usize {
        match self {
            Operation::Sum => nums.iter().sum(),
            Operation::Multiply => nums.iter().product(),
        }
    }
}

fn convert_ceph<'a>(nums: impl IntoIterator<Item = &'a str>) -> Vec<usize> {
    let mut result = vec![];

    for num in nums.into_iter() {
        while num.len() > result.len() {
            result.push(0);
        }
        for (res, digit) in result.iter_mut().zip(num.chars()) {
            let Some(this) = digit.to_digit(10) else {
                continue;
            };
            *res = *res * 10 + this as usize;
        }
    }
    result
}

type Input<'a> = (Vec<Vec<&'a str>>, Vec<Operation>);

pub fn parse(i: &str) -> &str {
    i
}

pub fn parse_ceph<'a>(i: &'a str) -> Input<'a> {
    let mut separator = vec![];
    for line in i.lines() {
        let bytes = line.bytes();
        while bytes.len() > separator.len() {
            separator.push(true);
        }
        for (byte, sep) in bytes.into_iter().zip(separator.iter_mut()) {
            if !matches!(byte, b' ') {
                *sep = false;
            }
        }
    }
    let sizes: Vec<_> = separator
        .iter()
        .enumerate()
        .filter_map(|(ix, val)| val.then_some(ix))
        .scan(0, |last, b| {
            let size = b - *last;
            *last = b + 1;
            Some(size)
        })
        .collect();

    let split_ceph = move |line: &'a str| {
        let mut chunks = vec![];
        let mut line = line;
        for size in sizes.iter() {
            let (chunk, nxt) = line.split_at(*size);
            chunks.push(chunk);
            line = &nxt[1..];
        }
        chunks.push(line);
        chunks
    };

    let mut lines = i
        .lines()
        .rev()
        .filter(|line| !line.is_empty())
        .map(split_ceph);
    let operations: Vec<_> = lines
        .by_ref()
        .next()
        .expect("could not get operations")
        .into_iter()
        .map(|chunk| {
            let op = chunk.trim();
            match op {
                "*" => Operation::Multiply,
                "+" => Operation::Sum,
                _ => panic!("unknown operation {op}"),
            }
        })
        .collect();
    let mut cols = vec![];

    for row in lines.rev() {
        while cols.len() < row.len() {
            cols.push(vec![]);
        }
        for (cell, dest) in row.into_iter().zip(cols.iter_mut()) {
            dest.push(cell);
        }
    }
    (cols, operations)
}

pub fn part1(i: &str) -> usize {
    let (cols, ops) = parse_ceph(i);
    cols.iter()
        .zip(ops)
        .map(|(col, op)| {
            let col: Vec<_> = col
                .iter()
                .map(|cell| cell.trim().parse::<usize>().expect("could not parse int"))
                .collect();
            op.reduce(&col)
        })
        .sum()
}

pub fn part2(i: &str) -> usize {
    let (cols, ops) = parse_ceph(i);
    cols.iter()
        .zip(ops)
        .map(|(col, op)| op.reduce(&convert_ceph(col.iter().cloned())))
        .sum()
}
