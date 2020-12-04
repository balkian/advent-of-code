#![feature(destructuring_assignment,or_patterns)]

use lazy_static::lazy_static;
use aoc_utils;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(byr:(?P<byr>\d+))|(iyr:(?P<iyr>\d+))|(eyr:(?P<eyr>\d+))|(hgt:(?P<hgt>\d+)(?P<units>in|cm)?)|(hcl:(?P<hcl>\S+))|(ecl:(?P<ecl>\S+))|(pid:(?P<pid>\S+))|(cid:(?P<cid>\S+))").unwrap();
    static ref COLOR: Regex = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();
}

#[derive(Default, Debug)]
struct Passport<'a> {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<u32>,
    units: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
    lines: Vec<&'a str>,
    valid: bool,
}

fn new<'a>() -> Passport<'a> {
    Passport {
        ..Default::default()
    }
}

impl<'a> Passport<'a> {
    fn add_line(&mut self, line: &'a str) {

        self.lines.push(line);
        let names: Vec<&str> = RE.capture_names().filter_map(|x| x).collect();
        // dbg!{&names};
        for c in RE.captures_iter(line) {
            for name in &names {
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
                        self.units = Some(value.as_str());
                    }
                    ("hcl", Some(value)) => {
                        self.hcl = Some(value.as_str());
                    }
                    ("ecl", Some(value)) => {
                        self.ecl = Some(value.as_str());
                    }
                    ("pid", Some(value)) => {
                        self.pid = Some(value.as_str());
                    }
                    ("cid", Some(value)) => {
                        self.cid = Some(value.as_str());
                    }
                    _ => {
                        // dbg!(name, value);
                    }
                }
            }
        }
    }

    fn check(&mut self) -> bool {
        self.valid = self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
        self.valid
    }

    fn check2(&self) -> Result<(), String> {
        self.byr
            .filter(|&x| x >= 1920 && x <= 2002)
            .ok_or(format!("invalid byr: {:?}", self.byr))?;

        self.iyr
            .filter(|&x| 2010 <= x && x <= 2020)
            .ok_or(format!("invalid iyr: {:?}", self.iyr))?;

        self.eyr
            .filter(|&x| 2020 <= x && x <= 2030)
            .ok_or(format!("invalid eyr: {:?}", self.eyr))?;


        let max: u32;
        let min: u32;

        match self.units {
            None => {
                return Err("no units provided".into())
            },
            // Requires feature: destructuring assignments
            Some("cm") => (min, max) = (150, 193),
            Some("in") => (min, max) = (59, 76),
            Some(other) => return Err(format!("invalid units: {}", other))
        }

        match self.hgt {
            None =>  return Err("no hgt provided".into()),
            Some(hgt) => {
                if min > hgt || hgt > max {
                    return Err(format!("invalid hgt: {}", hgt));
                }
            }
        }

        match self.hcl {
            None => Err("no hcl".into()),
            Some(hcl) if COLOR.is_match(hcl) => Ok(()),
            Some(hcl) => Err(format!("invalid hcr: {}", hcl)),
        }?;

        match self.ecl {
            None => return Err("no hcl".into()),
            // Requires feature or_patterns
            Some("amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" ) => {}
            Some(ecl) => return Err(format!("invalid ecl: {}", ecl)),
        }

        match self.pid {
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

fn solve<'a, T>(it: T)
where
    T: IntoIterator<Item = &'a String>,
{
    let count_1;
    let count_2;

    let mut p = new();

    let mut passports: Vec<Passport> = Vec::new();

    for line in it {
        if line.is_empty() {
            p.check();
            passports.push(p);
            p = new();
            continue;
        }
        p.add_line(line);
        // dbg!{&p};
    }

    p.check();
    passports.push(p);

    dbg! {&passports, passports.len()};

    count_1 = passports
        .iter()
        .filter(|x| {
            // dbg!(&x, x.colons(), x.discrepancies());
            x.valid
        })
        .count();

    count_2 = passports
        .iter()
        .filter_map(|x| {
            let t = x.check2();
            if let Err(e) = &t {
                eprintln!("Invalid passport ({:}): {:?}", e, x)
            }
            t.ok()
        })
        .count();

    println!("Valid: {:} / {:}", count_1, passports.len());
    println!("Valid2: {:} / {:}", count_2, passports.len());
}

fn main() {
    let it: Vec<String> = aoc_utils::file_iter();
    solve(&it);
}
