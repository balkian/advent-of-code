pub fn parse(input: &str) -> &str {
    input.trim()
}

fn is_valid(pwd: &str) -> bool {
    pwd.as_bytes()
        .windows(3)
        .any(|byt| byt[0] + 1 == byt[1] && byt[1] + 1 == byt[2])
        && !&['i', 'o', 'l'].into_iter().any(|c| pwd.contains(c))
        && pwd
            .as_bytes()
            .windows(2)
            .scan(false, |stt, byt| {
                if byt[0] == byt[1] && !*stt {
                    *stt = true;
                    Some(1)
                } else {
                    *stt = false;
                    Some(0)
                }
            })
            .sum::<usize>()
            >= 2
}

fn rotate(s: &str) -> String {
    let mut n = String::new();
    let mut carry = true;
    for c in s.chars().rev() {
        let nc = if carry {
            if c == 'z' {
                97 as char
            } else {
                carry = false;
                ((c as u8) + 1) as char
            }
        } else {
            c
        };
        n.push(nc);
    }
    if carry {
        n.push('a');
    }
    n.chars().rev().collect()
}

// a-z = U+0061 - U+007A = 97-122 decimal
pub fn part1(input: &str) -> String {
    let mut input = rotate(input);
    while !is_valid(&input) {
        input = rotate(&input);
    }
    input
}

pub fn part2(input: &str) -> String {
    part1(&part1(input))
}
