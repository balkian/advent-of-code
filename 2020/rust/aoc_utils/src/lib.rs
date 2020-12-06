use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::str::FromStr;

use log::{info};

pub fn file_iter<T>() -> impl Iterator<Item=T> + Send
where
    T: FromStr,
    // If this is missing, the unwrap/expect method after parse() will not compile
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    file_iter_plain().map(|line| line.parse::<T>().unwrap())
}

pub fn file_iter_plain() -> impl Iterator<Item=String> + Send {
    let args: Vec<String> = env::args().collect();
    let input = &String::from("input.txt");
    let fname = args.get(1).unwrap_or(input);
    info!("Opening file: {}", fname);
    let file = File::open(fname).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())

}

struct BlockIter<I,M,R,FM,FR>
where
    I: Iterator<Item=String> + Send,
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
{
    inner:  I,
    map: FM,
    reduce: FR,
}

pub fn blocks<I>(it: I) -> impl Iterator<Item=String>
where
    I: Iterator<Item=String> + Send,
{
    blocks_with_f(it, |x| x, |x| x.join("\n"))
}

pub fn blocks_with_f<I,M,R,FM,FR>(inner: I,
                               map: FM,
                               reduce: FR) -> impl Iterator<Item=R>
where
    I: Iterator<Item=String> + Send,
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
{
    BlockIter{
        inner,
        map,
        reduce,
    }
}

impl<I,M,R,FM,FR> Iterator for BlockIter<I,M,R,FM,FR>
where
    I: Iterator<Item=String> + Send,
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
{
    type Item = R;

    // The method that generates each item
    fn next(&mut self) -> Option<Self::Item> {
        let mut sofar: Vec<M> = Vec::new();

        loop {
            match self.inner.next() {
                Some(s) => {
                    if s.is_empty() {
                        return Some((self.reduce)(sofar));
                    }
                    sofar.push((self.map)(s));
                    continue
                },
                None if sofar.is_empty() => {
                    return None;
                },
                None => {
                    return Some((self.reduce)(sofar));
                }
            }
        }

    }
}


pub fn file_iter_blocks<M,R,FM,FR>(map: FM, reduce: FR) -> impl Iterator<Item=R>
    where
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
{
    blocks_with_f(file_iter_plain(), map, reduce)
}
