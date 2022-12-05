pub use std::env;
pub extern crate clap;

pub mod build;

#[macro_export]
macro_rules! timed {
    ($timeit:tt, $title:literal, $($code:tt)+) => {

        print!("{}: ", $title);
        let now = std::time::Instant::now();
        // we sleep for 2 seconds
        let res = {$($code)+};


        println!("{}", res);
        if $timeit {
            let mut elapsed = now.elapsed().as_nanos() as f64;
            let mut exp = 0;
            let mut unit = "s";
            for i in ["ns", "µs", "ms"] {
                if elapsed < 1000.0 {
                    unit = i;
                    break;
                }
                elapsed /= 1000f64;
            }

            println!("\t# Took: {:.0} {}", elapsed, unit);
        }

    };
}

#[macro_export]
macro_rules! solve_1 {
    ($args:ident, $day:ident, $input:expr, $timeit:tt ) => {
        let st = stringify!($day.input);
        let i_f = std::path::Path::new("../inputs").join(st);
        let def_file = i_f.to_str().unwrap();
        let fname = $input.unwrap_or(def_file);
        println!(stringify!(* Running $day));
        let input = &std::fs::read_to_string(fname).expect("could not read input file");
        let input = &$day::parse(input);

        match $args
            .value_of("part")
            .expect("the part argument should have a default value")
        {
            "1" | "a" => {
                $crate::timed!($timeit, "\tPart 1", $day::part1(input));
            }
            "2" | "b" => {
                $crate::timed!($timeit, "\tPart 2", $day::part2(input));
            }
            "all" => {
                $crate::timed!($timeit, "\tPart 1", $day::part1(input));
                $crate::timed!($timeit, "\tPart 2", $day::part2(input));
            }
            _ => panic!("Unknown parameter"),
        }
    };
}

#[macro_export]
macro_rules! aoc_main {
    ($($day:ident;)*) => {

        use $crate::clap::{arg, Command, Arg, ArgAction};
        $(mod $day;)*

        pub fn main() {
            let args = Command::new("aoc")
                .version("1.0")
                .about("AoC solver")
                .author("Fernando Sánchez")
                .arg(arg!([day] "Day to solve").default_value("all"))
                .arg(arg!([part] "Part to solve (1, 2 or all)").default_value("all"))
                .arg(arg!(-i --input <VALUE> "Input file to solve").required(false))
                .arg(
                    Arg::new("timed")
                        .long("timed")
                        .short('t')
                        .action(ArgAction::SetTrue)
                        .help("Time solutions"))
                .get_matches();

            let input = args.value_of("input");
            let timeit = args.get_flag("timed");

            match args.value_of("day") {
                $( Some(a) if a == stringify!($day) => {
                    $crate::solve_1!(args, $day, input, timeit);
                },)*
                Some(a) if a == "all" => {
                    $(
                    $crate::solve_1!(args, $day, None, timeit);
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
