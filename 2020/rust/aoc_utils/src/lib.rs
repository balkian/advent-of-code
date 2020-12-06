use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use log::info;

pub fn file_iter_parsed<T>() -> impl Iterator<Item = T> + Send
where
    T: FromStr,
    // If this is missing, the unwrap/expect method after parse() will not compile
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    file_iter().map(|line| line.parse::<T>().unwrap())
}

pub fn file_iter() -> impl Iterator<Item = String> + Send {
    let args: Vec<String> = env::args().collect();
    let input = &String::from("input.txt");
    let fname = args.get(1).unwrap_or(input);
    info!("Opening file: {}", fname);
    let file = File::open(fname).unwrap();

    BufReader::new(file).lines().map(|line| line.unwrap())
}

///Iterate a file in blocks
struct BlockIter<I, M, R, FM, FR, FS>
where
    I: Iterator<Item = String> + Send,
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
    FS: Fn(&String, &Vec<M>) -> bool,
{
    inner: I,
    map: FM,
    reduce: FR,
    split: FS,
}

pub fn blocks_default<I>(it: I) -> impl Iterator<Item = String>
where
    I: Iterator<Item = String> + Send,
{
    blocks(it, |x| x, |x| x.join("\n"), |line, _| line.is_empty())
}

pub fn blocks<I, M, R, FM, FR, FS>(
    inner: I,
    map: FM,
    reduce: FR,
    split: FS,
) -> impl Iterator<Item = R>
where
    I: Iterator<Item = String> + Send,
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
    FS: Fn(&String, &Vec<M>) -> bool,
{
    BlockIter {
        inner,
        map,
        reduce,
        split,
    }
}

pub fn default_split<T>(line: &String, _sofar: &Vec<T>) -> bool {
    line.is_empty()
}

impl<I, M, R, FM, FR, FS> Iterator for BlockIter<I, M, R, FM, FR, FS>
where
    I: Iterator<Item = String> + Send,
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
    FS: Fn(&String, &Vec<M>) -> bool,
{
    type Item = R;

    // The method that generates each item
    fn next(&mut self) -> Option<Self::Item> {
        let mut sofar: Vec<M> = Vec::new();

        loop {
            match self.inner.next() {
                Some(s) => {
                    if (self.split)(&s, &sofar) {
                        return Some((self.reduce)(sofar));
                    }
                    sofar.push((self.map)(s));
                    continue;
                }
                None if sofar.is_empty() => {
                    return None;
                }
                None => {
                    return Some((self.reduce)(sofar));
                }
            }
        }
    }
}

///Iterate the input file, process each line with `map`, group lines using `default_split`
///and aggregate results using `reduce`.
pub fn file_iter_blocks<M, R, FM, FR>(map: FM, reduce: FR) -> impl Iterator<Item = R>
where
    FM: Fn(String) -> M,
    FR: Fn(Vec<M>) -> R,
{
    blocks(file_iter(), map, reduce, default_split)
}
