pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> String {
    let mut passwd = String::new();

    for i in 0.. {
        let msg = format!("{}{}", input, i);
        let digest = md5::compute(msg).0;
        if (digest[0] | digest[1] | (digest[2] >> 4)) == 0 {
            let c = format!("{:x?}", digest[2]);
            debug_assert!(c.len() == 1);
            passwd.push_str(&c);
            if passwd.len() == 8 {
                break;
            }
        }
    }
    passwd
}

pub fn part2(input: &str) -> String {
    let mut passwd: [u8; 8] = [b'X'; 8];

    for i in 0.. {
        let msg = format!("{}{}", input, i);
        let digest = md5::compute(msg).0;
        if (digest[0] | digest[1] | (digest[2] >> 4)) == 0 {
            if let Ok(position) = format!("{:x?}", digest[2]).parse::<usize>() {
                if position > 7 || passwd[position] != b'X' {
                    continue;
                }
                let c = format!("{:x?}", digest[3] >> 4);
                passwd[position] = c.as_bytes()[0];
                if !passwd.contains(&b'X') {
                    break;
                }
            }
        }
    }
    String::from_utf8(passwd.to_vec()).unwrap()
}
