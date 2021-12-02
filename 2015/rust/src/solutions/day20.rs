pub fn parse(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn calculate(c: usize) -> usize {
    let mut gifts = 0;
    let lim = 1 + ((c as f32).sqrt() as usize);
    for i in 1..=lim {
        if c % i == 0 {
            gifts += c / i;
            gifts += i;
        }
    }
    gifts * 10
}

fn calculate2(c: usize) -> usize {
    let mut gifts = 0;
    let lim = 1 + ((c as f32).sqrt() as usize);
    for i in 1..=lim {
        if c % i == 0 {
            let times = c / i;
            if times <= 50 {
                gifts += i;
            }
            if i <= 50 {
                gifts += times;
            }
        }
    }
    gifts * 11
}

pub fn part1(target: &usize) -> usize {
    // a number x that is divisible by every possible factor will have (log(x)*x * 10) gifts
    let guess = target / (14 * 10);
    for pointer in guess.. {
        let gifts = calculate(pointer);
        if gifts > *target {
            return pointer;
        }
    }
    unreachable!()
}

pub fn part2(target: &usize) -> usize {
    // Same logic as before, but now only log(50) ~= 3.91
    let guess = target / (4 * 11);
    for pointer in guess.. {
        let gifts = calculate2(pointer);
        if gifts > *target {
            return pointer;
        }
    }
    unreachable!()
}
