use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::str::FromStr;

use log::{info};

pub fn file_iter<'a,T>() -> impl Iterator<Item=T> + 'a + Send
where
    T: FromStr,
    // If this is missing, the unwrap/expect method after parse() will not compile
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    file_iter_plain().map(|line| line.parse::<T>().unwrap())
}

pub fn file_iter_plain<'a>() -> impl Iterator<Item=String> + 'a + Send {
    let args: Vec<String> = env::args().collect();
    let input = &String::from("input.txt");
    let fname = args.get(1).unwrap_or(input);
    info!("Opening file: {}", fname);
    let file = File::open(fname).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())

}
