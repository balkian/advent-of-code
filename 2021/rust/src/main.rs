pub use std::env;

mod solutions;

fn main() {
    solutions::main();
}

#[macro_export]
macro_rules! aoc_main {
    ($($day:ident;)*) => {

        use clap::{arg, Command};
        $(mod $day;)*

        pub fn main() {
            let args = Command::new("aoc")
                .version("1.0")
                .about("AoC solver")
                .author("Fernando Sánchez")
                .arg(arg!([day] "Day to solve").default_value("all"))
                .arg(arg!(-i --input <VALUE> "Input file to solve").required(false))
                .get_matches();

            match args.value_of("day") {
                $( Some(a) if a == stringify!($day) => {
                    let fname = args.value_of("input").unwrap_or_else(|| stringify!($day.input));
                    println!(stringify!(* Running $day));
                    let input = &std::fs::read_to_string(fname).expect("could not read input file");
                    let input = &$day::parse(input);
                    println!("\tPart 1 {}", $day::part1(input));
                    println!("\tPart 2 {}", $day::part2(input));
                },)*
                Some(a) if a == "all" => {
                    $(println!(stringify!(* Running $day));

                        let fname = stringify!($day.input);
                        let input = &std::fs::read_to_string(fname).expect("could not read input file");
                        let input = &$day::parse(input);
                        println!("\tPart 1 {}", $day::part1(input));
                        println!("\tPart 2 {}", $day::part2(input));
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
        aoc_test!($test, $sample, $part, $expected);
        aoc_test!($($otest, $osample, $opart, $oexpected;)*);
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
