/// This implementation is closer to the Python one: we use hashmaps instead
/// of creating a custom struct.
///
use std::collections::HashMap;
// use log::{debug};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

struct PassIter<'a> {
    inner: &'a mut (dyn Iterator<Item = String> + 'a + Send),
}

type Passport = HashMap<String, String>;

fn new<'a>(it: &'a mut (dyn Iterator<Item = String> + 'a + Send)) -> PassIter {
    PassIter { inner: it }
}
lazy_static! {
    static ref COLOR: Regex = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();
}

impl<'a> Iterator for PassIter<'a> {
    type Item = Passport;

    // The method that generates each item
    fn next(&mut self) -> Option<Self::Item> {
        let mut sofar: Passport = HashMap::new();

        loop {
            match self.inner.next() {
                Some(s) => {
                    if s.is_empty() {
                        return Some(sofar);
                    }
                    s.split(' ').for_each(|tok| {
                        let sp = tok.split(':').collect::<Vec<&str>>();
                        sofar.insert(sp[0].into(), sp[1].into());
                    });
                    continue;
                }
                None if sofar.is_empty() => {
                    return None;
                }
                None => {
                    return Some(sofar);
                }
            }
        }
    }
}

fn check_part1(x: &Passport) -> bool {
    let count = x.keys().count();
    count > 7 || (count == 7 && !x.contains_key("cid"))
}

fn check_part2(x: &Passport) -> Option<&Passport> {
    let byr = x.get("byr")?.parse::<u32>().ok()?;
    if !(1920..=2020).contains(&byr) {
        return None;
    }

    let iyr = x.get("iyr")?.parse::<u32>().ok()?;
    if !(2010..=2020).contains(&iyr) {
        return None;
    }

    let eyr = x.get("eyr")?.parse::<u32>().ok()?;
    if !(2020..=2030).contains(&eyr) {
        return None;
    }

    let hgt = x.get("hgt")?;
    if hgt.len() < 3 {
        return None;
    }
    let (min, max) = match &hgt[hgt.len() - 2..] {
        "cm" => (150, 193),
        "in" => (59, 76),
        x => {
            dbg! {x};
            return None;
        }
    };

    let value = hgt[..hgt.len() - 2].parse::<u32>().ok()?;

    if value < min || value > max {
        return None;
    }

    match x.get("ecl")?.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
        _ => return None,
    };

    match x.get("hcl") {
        None => return None,
        Some(hcl) if COLOR.is_match(hcl) => {}
        Some(_) => return None,
    };

    match x.get("pid") {
        None => return None,
        Some(pid) if pid.len() != 9 => {
            return None;
        }
        Some(pid) => {
            let pid: i64 = pid.parse().ok()?;

            if pid >= 10e10 as i64 {
                return None;
            }
        }
    };
    Some(x)
}

pub fn solve_hashmap<T: Iterator<Item = String> + Send>(mut it: T) {
    let valid: (u64, u64) = new(&mut it)
        .filter(|x| check_part1(&x))
        .fold((0, 0), |mut c, x| {
            if check_part2(&x).is_some() {
                c.1 += 1;
            }
            c.0 += 1;
            c
        });
    println!("Valid in part 1: {:}", valid.0);
    println!("Valid in part 2: {:}", valid.1);
}

pub fn solve_hashmap_par<T: Iterator<Item = String> + Send>(mut it: T) {
    let valid: (u64, u64) = new(&mut it)
        .par_bridge()
        .filter(|x| check_part1(&x))
        .fold(
            || (0, 0),
            |mut c, x| {
                if check_part2(&x).is_some() {
                    c.1 += 1;
                }
                c.0 += 1;
                c
            },
        )
        .reduce(|| (0, 0), |sum, i| (sum.0 + i.0, sum.1 + i.1));
    println!("Valid in part 1: {:}", valid.0);
    println!("Valid in part 2: {:}", valid.1);
}

///Solve using the [`file_tier_blocks`] function.
pub fn solve_hashmap2(it: impl Iterator<Item = String> + Send) {
    let valid: (usize, usize) = aoc_utils::blocks(
        it,
        |line| {
            line.split(' ')
                .map(|tok| {
                    let res = tok.split(':').collect::<Vec<&str>>();
                    (res[0].to_string(), res[1].to_string())
                })
                .collect::<Vec<(String, String)>>()
        },
        |block| {
            let mut p: Passport = HashMap::new();

            for (k, v) in block.into_iter().flatten() {
                p.insert(k, v);
            }
            let mut c1 = 0;
            let mut c2 = 0;
            if check_part1(&p) {
                c1 = 1;
            }
            if check_part2(&p).is_some() {
                c2 = 1;
            }
            (c1, c2)
        },
        aoc_utils::default_split,
    )
    .fold((0, 0), |c, x| (c.0 + x.0, c.1 + x.1));

    println!("Valid in part 1: {:}", valid.0);
    println!("Valid in part 2: {:}", valid.1);
}

///Solve using the [`file_tier_blocks`] function.
pub fn solve_hashmap2_par(it: impl Iterator<Item = String> + Send) {
    let valid: (usize, usize) = aoc_utils::blocks(
        it,
        |line| {
            line.split(' ')
                .map(|tok| {
                    let res = tok.split(':').collect::<Vec<&str>>();
                    (res[0].to_string(), res[1].to_string())
                })
                .collect::<Vec<(String, String)>>()
        },
        |block| block,
        aoc_utils::default_split,
    )
    .par_bridge()
    .map(|block| {
        let mut p: Passport = HashMap::new();

        for (k, v) in block.into_iter().flatten() {
            p.insert(k, v);
        }
        let mut c1 = 0;
        let mut c2 = 0;
        if check_part1(&p) {
            c1 = 1;
        }
        if check_part2(&p).is_some() {
            c2 = 1;
        }
        (c1, c2)
    })
    .reduce(|| (0, 0), |sum, i| (sum.0 + i.0, sum.1 + i.1));

    println!("Valid in part 1: {:}", valid.0);
    println!("Valid in part 2: {:}", valid.1);
}
