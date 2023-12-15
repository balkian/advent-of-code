pub use clap::{arg, value_parser, Arg, ArgAction, Command};
use reqwest::blocking::Client;
pub use std::env;
use std::fs::{read_to_string, File};

use std::io;
use std::path::PathBuf;
pub extern crate clap;

pub fn section<T>(timeit: bool, title: &str, code: impl FnOnce() -> T) -> T {
    print!("{} ", title);
    let now = std::time::Instant::now();
    let res = code();
    if timeit {
        let mut elapsed = now.elapsed().as_nanos() as f64;
        let mut unit = "s";
        for i in ["ns", "µs", "ms"] {
            if elapsed < 1000.0 {
                unit = i;
                break;
            }
            elapsed /= 1000f64;
        }

        print!(" Took: {:.0} {}", elapsed, unit);
    }
    println!();
    res
}

pub fn download_day(day: &str, year: Option<usize>, fpath: &PathBuf) {
    if fpath.exists() {
        return;
    }
    eprintln!(
        "Input file not found. Downloading into: {}",
        fpath.to_str().unwrap()
    );
    let year = year.expect("specify the year so the input file can be downloaded");
    let day: usize = day[3..].parse().unwrap();
    let mut session = None;

    let mut current_dir: Option<PathBuf> = env::current_dir().ok();
    while let Some(dir) = current_dir {
        if let Ok(sess) = read_to_string(dir.join(".aoc-session")) {
            session = Some(sess.trim().to_string());
            break;
        }
        current_dir = dir.parent().map(|d| d.to_path_buf());
    }
    let session = session.expect("could not read a session file (.aoc-session)");
    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let resp = client
        .get(url)
        .header("Cookie", format!("session={session}"))
        .send()
        .expect("request failed");
    let body = resp.text().expect("body invalid");
    let mut out = File::create(fpath).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}

#[macro_export]
macro_rules! solve_1 {
    ($args:ident, $day:ident, $year:ident, $input:expr, $timeit:tt ) => {
        let st = stringify!($day.input);
        let mut inputs = std::path::Path::new("../inputs");
        if !inputs.exists() {
            eprintln!("Folder ../inputs does not exist");
            inputs = std::path::Path::new("inputs");
        }
        let i_f = inputs.join(st);
        $crate::download_day(stringify!($day), $year, &i_f);

        let def_file = i_f.to_str().unwrap();

        let fname = $input.unwrap_or(def_file);
        println!(
            "* Running {} {}",
            stringify!($day),
            $year.map(|y| y.to_string()).unwrap_or(String::new())
        );

        let input = &std::fs::read_to_string(fname).expect("could not read input file");
        let input = &$crate::section($timeit, "\tParsing...", || $day::parse(input));
        //let input = &$day::parse(input);

        let parts = match $args
            .value_of("part")
            .expect("the part argument should have a default value")
        {
            "1" | "a" => (true, false),
            "2" | "b" => (false, true),
            "all" => (true, true),
            _ => panic!("Unknown parameter"),
        };
        if parts.0 {
            $crate::section($timeit, "\tPart 1:", || {
                let res = $day::part1(input);
                print!("{res:<15}");
                res
            });
        }
        if parts.1 {
            $crate::section($timeit, "\tPart 2:", || {
                let res = $day::part2(input);
                print!("{res:<15}");
                res
            });
        }
    };
}

#[macro_export]
macro_rules! aoc_main {
    ($($day:ident;)*) => {

        $(mod $day;)*

        const PKG_NAME: &str = env!("CARGO_PKG_NAME");

        pub fn main() {

            let args = $crate::Command::new("aoc")
                .version("1.0")
                .about("AoC solver")
                .author("Fernando Sánchez")
                .arg($crate::arg!([day] "Day to solve").default_value("all"))
                .arg($crate::arg!([part] "Part to solve (1, 2 or all)").default_value("all"))
                .arg($crate::arg!(-i --input <VALUE> "Input file to solve").required(false))
                .arg(
                    $crate::arg!(-y --year <YEAR>)
                        .required(false)
                        .help("Year of the event you're solving")
                        .default_value(&PKG_NAME[PKG_NAME.len()-4..])
                        .value_parser($crate::value_parser!(usize)),
                )
                .arg(
                    $crate::Arg::new("notimes")
                        .long("no-times")
                        .short('T')
                        .action($crate::ArgAction::SetTrue)
                        .help("Do not show timing information."))
                .get_matches();

            let input = args.value_of("input");
            let timeit = !args.get_flag("notimes");
            let year = args.get_one("year").copied();

            match args.value_of("day") {
                $( Some(a) if a == stringify!($day) => {
                    $crate::solve_1!(args, $day, year, input, timeit);
                },)*
                Some(a) if a == "all" => {
                    $(
                    $crate::solve_1!(args, $day, year, None, timeit);
                    )*
                },
                _ => println!("Solution not implemented"),
            }
        }
    }
}

#[macro_export]
macro_rules! aoc_sample {
    ($test:ident, $sample:literal, $part:ident, $expected:expr $(;)?) => {
        #[test]
        fn $test () {
            let input = parse(include_str!($sample));
            assert_eq!($part(&input), $expected);
        }
    };

    ($test:ident, $sample:literal, $part:ident, $expected:expr $(; $otest:ident, $osample:literal, $opart:ident, $oexpected:expr)* $(;)?) => {
        $crate::aoc_test!($test, $sample, $part, $expected);
        $crate::aoc_test!($($otest, $osample, $opart, $oexpected;)*);
    };
}

/// Drop-in replacement for dbg that only prints in debug mode (not in release)
#[macro_export]
macro_rules! dbg {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}
