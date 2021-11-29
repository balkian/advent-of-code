pub use std::env;

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
macro_rules! aoc_test {
    ($part:ident, $name:ident, $input:expr, $expected:expr $(;)?) => {
        #[test]
        fn $name () {
            assert_eq!($part($input), $expected);
        }
    };
    ($part:ident, $name:ident, $input:expr, $expected:expr $(; $opart:ident, $oname:ident, $oinput:tt, $oexpected:expr)* $(;)?) => {
        aoc_test!($part, $name, $input, $expected);
        aoc_test!($($opart, $oname, $oinput, $oexpected;)*);
    };
}

mod solutions;

fn main() {
    solutions::main();
}
