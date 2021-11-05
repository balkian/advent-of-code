use std::fs;

fn solve(input: &str) -> String {
    let result: i32 = input
        .split(&['\n', ',', ' '][..])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("valid number"))
        .sum();
    format!("{}", result)
}

fn solve2(input: &str) -> String {
    let mut cache: Vec<i32> = vec![0];

    let repeated = input
        .split(&['\n', ',', ' '][..])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("valid number"))
        .cycle()
        .scan(0i32, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .find_map(|x| {
            // dbg!{x};
            // dbg!{&cache};
            let found = cache.contains(&x);
            cache.push(x);
            found.then(|| x)
        })
        .unwrap();

    format!("{}", repeated)
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    println!("{}", solve(&data));
    println!("{}", solve2(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(solve("+1, +1, +1"), "3");
        assert_eq!(solve("+1, +1, -2"), "0");
        assert_eq!(solve("-1, -2, -3"), "-6");
    }

    #[test]
    fn ex2() {
        assert_eq!(solve2("+1, -1"), "0");
        assert_eq!(solve2("-6, +3, +8, +5, -6"), "5");
        assert_eq!(solve2("+7, +7, -2, -7, -4"), "14");
        assert_eq!(solve2("+3, +3, +4, -2, -4"), "10");
    }
}
