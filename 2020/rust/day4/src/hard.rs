/// First implementation using a custom struct ([`Passport`]).
/// There are three solutions,
///   * the normal one ([`solve`])
///   * A parallelized version that uses a channel and a thread [`solve_par`]
///   * A parallelized version using rayon [`solve_par2`]
///
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::sync::mpsc::channel;
use std::thread;

use log::debug;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(byr:(?P<byr>\d+))|(iyr:(?P<iyr>\d+))|(eyr:(?P<eyr>\d+))|(hgt:(?P<hgt>\d+)(?P<units>in|cm)?)|(hcl:(?P<hcl>\S+))|(ecl:(?P<ecl>\S+))|(pid:(?P<pid>\S+))|(cid:(?P<cid>\S+))").unwrap();
    static ref COLOR: Regex = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();

    static ref NAMES: Vec<&'static str> = RE.capture_names().filter_map(|x| x).collect();
}

/// Basic struct to save and validate passport data from each line
#[derive(Default, Debug)]
pub struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<u32>,
    units: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
    lines: Vec<String>,
    valid: bool,
}

pub fn new() -> Passport {
    Passport {
        ..Default::default()
    }
}

impl Passport {
    fn add_line(&mut self, line: String) {
        for c in RE.captures_iter(&line) {
            for name in NAMES.iter() {
                let value = c.name(&name);
                match (*name, value) {
                    ("byr", Some(value)) => {
                        self.byr = Some(value.as_str().parse().unwrap());
                    }
                    ("iyr", Some(value)) => {
                        self.iyr = Some(value.as_str().parse().unwrap());
                    }
                    ("eyr", Some(value)) => {
                        self.eyr = Some(value.as_str().parse().unwrap());
                    }
                    ("hgt", Some(value)) => {
                        self.hgt = Some(value.as_str().parse().unwrap());
                    }
                    ("units", Some(value)) => {
                        self.units = Some(value.as_str().into());
                    }
                    ("hcl", Some(value)) => {
                        self.hcl = Some(value.as_str().into());
                    }
                    ("ecl", Some(value)) => {
                        self.ecl = Some(value.as_str().into());
                    }
                    ("pid", Some(value)) => {
                        self.pid = Some(value.as_str().into());
                    }
                    ("cid", Some(value)) => {
                        self.cid = Some(value.as_str().into());
                    }
                    _ => {
                        // dbg!(name, value);
                    }
                }
            }
        }
        self.lines.push(line);
    }

    fn check(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn check2(&self) -> Result<(), String> {
        self.byr
            .filter(|&x| (1920..=2002).contains(&x))
            .ok_or(format!("invalid byr: {:?}", self.byr))?;

        self.iyr
            .filter(|&x| (2010..=2020).contains(&x))
            .ok_or(format!("invalid iyr: {:?}", self.iyr))?;

        self.eyr
            .filter(|&x| (2020..=2030).contains(&x))
            .ok_or(format!("invalid eyr: {:?}", self.eyr))?;

        let max: u32;
        let min: u32;

        match self.units.as_deref() {
            None => return Err("no units provided".into()),
            // Requires feature: destructuring assignments
            Some("cm") => (min, max) = (150, 193),
            Some("in") => (min, max) = (59, 76),
            Some(other) => return Err(format!("invalid units: {}", other)),
        }

        match &self.hgt {
            None => return Err("no hgt provided".into()),
            Some(hgt) => {
                if min > *hgt || *hgt > max {
                    return Err(format!("invalid hgt: {}", hgt));
                }
            }
        }

        match self.hcl.as_deref() {
            None => Err("no hcl".into()),
            Some(hcl) if COLOR.is_match(hcl) => Ok(()),
            Some(hcl) => Err(format!("invalid hcr: {}", hcl)),
        }?;

        match &self.ecl {
            None => return Err("no hcl".into()),
            // Requires feature or_patterns
            Some(color) => match color.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                ecl => return Err(format!("invalid ecl: {}", ecl)),
            },
        }

        match &self.pid {
            None => return Err("no pid provided".into()),
            Some(pid) if pid.len() != 9 => {
                return Err(format!("invalid pid: {}", pid));
            }
            Some(pid) => {
                let pid: i64 = pid
                    .parse()
                    .map_err(|x| format!("cannot parse pid: {}", x))?;

                if pid >= 10e10 as i64 {
                    let err = format!("invalid pid: {}", pid);
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    // not needed anymore
    #[allow(dead_code)]
    fn colons(&self) -> usize {
        self.lines
            .iter()
            .flat_map(|x| x.chars().filter(|x| *x == ':'))
            .count()
    }
    // not needed anymore
    #[allow(dead_code)]
    fn discrepancies(&self) -> bool {
        let c = self.colons();
        if self.valid {
            c < 7 || (c == 7 && self.cid.is_some())
        } else {
            c == 8 || (c == 7 && self.cid.is_none())
        }
    }
}

/// The first non-parallel solution
pub fn solve<T: IntoIterator<Item = String>>(it: T) {
    let count_1;
    let count_2;
    let mut passports: Vec<Passport> = Vec::new();

    let mut p = new();

    for line in it {
        if line.is_empty() {
            passports.push(p);
            p = new();
            continue;
        }
        p.add_line(line)
    }
    passports.push(p);

    let total = passports.len();

    count_1 = passports.iter().filter(|x| x.check()).count();

    let (valid, invalid): (Vec<Passport>, Vec<Passport>) =
        passports.into_par_iter().partition(|x| x.check2().is_ok());

    if true {
        for i in &invalid {
            debug!("Invalid: {:?}", i.lines);
        }
    }

    count_2 = valid.len();

    println!("Valid: {:} / {:}", count_1, total);
    println!("Valid2: {:} / {:}", count_2, total);
}

/// Solve part1 and part2 using parallel code and a channel
pub fn solve_par<T>(it: T)
where
    T: 'static + Send + IntoIterator<Item = String>,
{
    let count_1;
    let count_2;

    let rx = {
        let (tx, rx) = channel::<Vec<String>>();

        thread::spawn(move || {
            let mut it = it.into_iter();

            let mut batch: Vec<String> = Vec::new();

            loop {
                match it.next() {
                    None => {
                        if batch.is_empty() {
                            break;
                        }
                        tx.send(batch).unwrap();
                        batch = Vec::new();
                    }
                    Some(line) if line.is_empty() => {
                        tx.send(batch).unwrap();
                        batch = Vec::new();
                    }
                    Some(line) => {
                        batch.push(line);
                    }
                }
            }
        });
        rx
    };

    let passports: Vec<Passport> = rx
        .into_iter()
        .par_bridge()
        .map(|chunks| {
            let mut p = new();
            for c in chunks {
                p.add_line(c)
            }
            p.check();
            p
        })
        .collect();

    let total = passports.len();

    count_1 = passports.iter().filter(|x| x.check()).count();

    let (valid, invalid): (Vec<Passport>, Vec<Passport>) =
        passports.into_par_iter().partition(|x| x.check2().is_ok());

    if true {
        for i in &invalid {
            debug!("Invalid: {:?}", i.lines);
        }
    }

    count_2 = valid.len();

    println!("Valid: {:} / {:}", count_1, total);
    println!("Valid2: {:} / {:}", count_2, total);
}

/// Solve using `par_bridge`, from IterTools
/// I'm having trouble sending the iterator between threads, so
/// I've been unable to split this into two functions: read file
/// and process.
pub fn solve_par2() {
    let count_1;
    let count_2;

    let it = aoc_utils::file_iter();
    let mut finished = false;

    let pit = it.into_iter().batching(|it| {
        let mut batch: Vec<String> = Vec::new();
        loop {
            match it.next() {
                None if finished => return None,
                None => {
                    finished = true;
                    return Some(batch);
                }
                Some(x) => {
                    if x.is_empty() {
                        return Some(batch);
                    }
                    batch.push(x);
                }
            }
        }
    });

    let passports: Vec<Passport> = pit
        .par_bridge()
        .map(|chunks| {
            let mut p = new();
            for c in chunks {
                p.add_line(c)
            }
            p.check();
            p
        })
        .collect();

    let total = passports.len();

    count_1 = passports.iter().filter(|x| x.check()).count();

    let (valid, invalid): (Vec<Passport>, Vec<Passport>) =
        passports.into_par_iter().partition(|x| x.check2().is_ok());

    if true {
        for i in &invalid {
            debug!("Invalid: {:?}", i.lines);
        }
    }

    count_2 = valid.len();

    println!("Valid: {:} / {:}", count_1, total);
    println!("Valid2: {:} / {:}", count_2, total);
}
