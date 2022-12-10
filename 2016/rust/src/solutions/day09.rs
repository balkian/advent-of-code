use regex::Regex;

pub fn parse(mut input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    let re = Regex::new(r"\((?P<count>\d+)x(?P<times>\d+)\)").unwrap();
    let mut input = input;
    let mut out = String::new();
    while !input.is_empty() {
        if let Some(mat) = re.captures_iter(input).next() {
            let cap = mat.get(0).unwrap();
            out += input.split_at(cap.start()).0;
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
            dbg!(&count, times, &input);
            for i in 0..times {
                out += &input[0..count];
            }
            input = input.split_at(count).1;
        } else {
            out += input;
            break;
        }
    }
    out.len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}
