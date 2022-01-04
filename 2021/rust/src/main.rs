pub use std::env;

mod solutions;

fn main() {
    solutions::main();
}

#[macro_export]
macro_rules! aoc_main {
    ($($day:ident;)*) => {
        $(mod $day;)*

        pub fn main() {
            match crate::env::args().nth(1) {
                $( Some(a) if a == stringify!($day) => {
                    println!(stringify!(* Running $day));
                    let fname = stringify!($day.input);
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
