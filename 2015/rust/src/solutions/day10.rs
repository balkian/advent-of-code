pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    part(input, 40)
}
pub fn part2(input: &str) -> usize {
    part(input, 50)
}

fn part(input: &str, times: usize) -> usize {
    let mut input = input.to_string();
    for _ in 0..times {
        let mut new_input = String::new();
        let mut it = input.chars();
        let mut last = it.next().unwrap();
        let mut last_ix = 0;
        let mut len = 1;
        for i in it {
            if i != last {
                new_input += &format!("{}{}", len - last_ix, last);
                last_ix = len;
                last = i;
            }
            len += 1;
        }
        new_input += &format!("{}{}", len - last_ix, last);
        input = new_input;
    }
    input.len()
}
