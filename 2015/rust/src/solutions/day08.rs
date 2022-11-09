use crate::aoc_test;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(\\\\|\\x[0-9a-fA-F]{2}|\\"|.)"#).unwrap();
}

fn diff(line: &str) -> usize {
    let chars = RE
        .captures_iter(line)
        .map(|cap| cap.get(1).unwrap().as_str().to_string());
    line.len() - (chars.count() - 2)
}

fn diffinv(line: &str) -> usize {
    2 + line.chars().filter(|x| x == &'"' || x == &'\\').count()
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    input.lines().map(diff).sum()
}

pub fn part2(input: &str) -> usize {
    input.lines().map(diffinv).sum()
}

aoc_test! {
   part1, example, {parse(r#"""
"abc"
"aaa\"aaa"
"\x27"
"#)}, 12;
   part2, example_inv, {parse(r#"""
"abc"
"aaa\"aaa"
"\x27"
"#)}, 19;
}
