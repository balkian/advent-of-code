use regex::Regex;

pub fn parse(input: &str) -> Vec<Part> {
    let re = Regex::new(r"\((?P<count>\d+)x(?P<times>\d+)\)").unwrap();
    let mut input = input.trim();
    let mut out = vec![];
    while !input.is_empty() {
        if let Some(mat) = re.captures_iter(input).next() {
            let cap = mat.get(0).unwrap();
            out.extend(input.chars().take(cap.start()).map(|c| Part::Char(c)));
            let count = mat
                .name("count")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let times = mat
                .name("times")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            input = input.split_at(cap.end()).1;
            out.push(Part::Repeat(count, times, cap.range().len()));
        } else {
            out.extend(input.chars().map(|c| Part::Char(c)));
            break;
        }
    }
    out
}

pub enum Part {
    Char(char),
    Repeat(usize, usize, usize),
}

pub fn solve(mut input: &[Part], max_level: usize, mut max: usize) -> (&[Part], usize) {
    let mut total = 0;
    while let Some((i, next)) = input.split_first() {
        input = next;
        match *i {
            Part::Char(_) => {
                max -= 1;
                total += 1;
            }
            Part::Repeat(_, _, size) if max_level == 0 => {
                max -= size;
                total += size;
            }
            Part::Repeat(count, times, size) => {
                max -= size + count;
                let (new_input, delta) = solve(input, max_level - 1, count);
                input = new_input;
                total += delta * times;
            }
        }
        if max == 0 {
            break;
        }
    }
    (input, total)
}

pub fn part1(input: &[Part]) -> usize {
    solve(input, 1, usize::MAX).1
}

pub fn part2(input: &[Part]) -> usize {
    solve(input, usize::MAX, usize::MAX).1
}
