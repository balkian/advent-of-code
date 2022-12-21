#[derive(Debug, Clone)]
pub struct Number {
    value: isize,
    next: usize,
    prev: usize,
}

#[derive(Clone, Debug)]
pub struct Message(Vec<Number>);

impl Message {
    fn get_pos(&self, value: isize) -> Option<usize> {
        for (idx, i) in self.0.iter().enumerate() {
            if i.value == value {
                return Some(idx);
            }
        }
        None
    }

    fn get_value_at_delta(&self, value: isize, delta: usize) -> isize {
        let mut pos = self.get_pos(value).unwrap();
        for _i in 0..(delta % self.0.len()) {
            pos = self.0[pos].next;
        }
        self.0[pos].value
    }

    fn shift(&mut self, pos: usize, delta: isize) {
        let delta = delta.rem_euclid((self.0.len() - 1) as isize) as usize;
        let mut prev = self.0[pos].prev;
        let mut next = self.0[pos].next;
        self.0[next].prev = prev;
        self.0[prev].next = next;
        for _i in 0..delta as usize {
            prev = next;
            next = self.0[next].next;
        }
        // println!(
        //     "{} moves between {} and {}",
        //     self.0[pos].value, self.0[prev].value, self.0[next].value
        // );
        self.0[prev].next = pos;
        self.0[next].prev = pos;
        self.0[pos].next = next;
        self.0[pos].prev = prev;
    }

    fn decode(&self) -> Self {
        let mut out = self.clone();

        for (idx, val) in self.0.iter().enumerate() {
            out.shift(idx, val.value)
        }
        out
    }
}

pub fn parse(input: &str) -> Message {
    let num_lines = input.lines().filter(|line| !line.is_empty()).count();
    Message(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(idx, line)| Number {
                value: line.trim().parse::<isize>().unwrap(),
                prev: (idx + num_lines - 1) % num_lines,
                next: (idx + 1) % num_lines,
            })
            .collect(),
    )
}

pub fn part1(input: &Message) -> isize {
    let decoded = input.decode();
    decoded.get_value_at_delta(0, 1000)
        + decoded.get_value_at_delta(0, 2000)
        + decoded.get_value_at_delta(0, 3000)
}

pub fn part2(input: &Message) -> isize {
    let mut decoded = input.clone();
    let dec_key: isize = 811589153;
    decoded.0.iter_mut().for_each(|n| n.value *= dec_key);
    for _i in 0..9 {
        decoded = decoded.decode();
    }
    part1(&decoded)
}
