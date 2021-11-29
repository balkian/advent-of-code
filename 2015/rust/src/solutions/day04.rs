use crate::aoc_test;
use md5;

pub fn parse(input: &str) -> &str {
    input
}

fn is_valid(input: &str, i: usize, difficulty: usize) -> bool {
    let st = input.to_string() + &i.to_string();
    let digest = md5::compute(st.as_bytes());
    let bytes = difficulty / 2;

    for i in 0..bytes {
        if digest[i] != 0 {
            return false
        }
    }
    if (difficulty % 2 == 1) && (digest[bytes] & 0xF0) != 0 {
        return false
    }
    true
}

pub fn part1(input: &str) -> usize {
    (0..).into_iter().find(|i| is_valid(input, *i, 5)).unwrap()
}

pub fn part2(input: &str) -> usize {
    (0..).into_iter().find(|i| is_valid(input, *i, 6)).unwrap()
}

#[test]
fn test_valid(){
    assert!(is_valid("abcdef", 609043, 5));

}

aoc_test!(part1, abc, "abcdef", 609043;
          part1, pqr, "pqrstuv", 1048970);
