use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub fn file_iter<T>() -> Vec<T>
where
    T: FromStr,
    // If this is missing, the unwrap/expect method after parse() will not compile
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let args: Vec<String> = env::args().collect();
    let input = &String::from("input.txt");
    let fname = args.get(1).unwrap_or(input);
    let file = File::open(fname).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<T>>()
}
