use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub use clap;

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
    let matches = app("unspecified").get_matches();
    let fname = matches.value_of("input").expect("no input specified");
    info!("Opening file: {}", fname);
    file_iter_from(fname)

}

pub fn file_iter_from(fname: &str) -> impl Iterator<Item = String> + Send {
    let file = File::open(fname).unwrap();
    BufReader::new(file).lines().map(|line| line.unwrap())
}

///Get the input file from clap's matches and return an iterator over it
pub fn file_iter_clap(matches: &clap::ArgMatches) -> impl Iterator<Item = String> + Send {
    let fname = matches.value_of("input").expect("no input file provided");
    file_iter_from(fname)
}

///Iterate a file in blocks
struct BlockIter<I, M, R, FM, FR, FS>
where
    I: Iterator<Item = String> + Send,
    FM: Send+Fn(String) -> M,
    FR: Send+Fn(Vec<M>) -> R,
    FS: Send+Fn(&str, &[M]) -> bool,
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
) -> impl Iterator<Item = R>+Send
where
    I: Iterator<Item = String> + Send,
    FM: Send+Fn(String) -> M,
    FR: Send+Fn(Vec<M>) -> R,
    FS: Send+Fn(&str, &[M]) -> bool,
{
    BlockIter {
        inner,
        map,
        reduce,
        split,
    }
}

pub fn default_split<T>(line: &str, _sofar: &[T]) -> bool {
    line.is_empty()
}

impl<I, M, R, FM, FR, FS> Iterator for BlockIter<I, M, R, FM, FR, FS>
where
    I: Iterator<Item = String> + Send,
    FM: Send+Fn(String) -> M,
    FR: Send+Fn(Vec<M>) -> R,
    FS: Send+Fn(&str, &[M]) -> bool,
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
    FM: Send+Fn(String) -> M,
    FR: Send+Fn(Vec<M>) -> R,
{
    blocks(file_iter(), map, reduce, default_split)
}

pub fn app<'a>(day: &str) -> clap::App<'a,'a> {
    clap::App::new(format!("Advent of Code 2020. Day {:}", day))
        .version("1.0")
        .author("Fernando Sánchez <aoc@sinpapel.es>")
        .about("Tries to solve the riddle for AoC 2020")
        .arg(clap::Arg::with_name("input")
             .value_name("FILE")
             .help("Sets a custom input file")
             .default_value("input.txt"))
}
