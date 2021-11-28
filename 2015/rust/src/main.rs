use std::env;

macro_rules! aoc_main {

    ($($day:ident),*) => {
        $(mod $day;)*

        fn main() {
            match env::args().nth(1) {
                $( Some(a) if a == stringify!($day) => {
                    println!(stringify!(Running $day));

                    let fname = stringify!($day.input);
                    let input = &std::fs::read_to_string(fname).expect("could not read input file");
                    let input = &$day::parse(input);
                    println!("Part 1 {}", $day::part1(input));
                    println!("Part 2 {}", $day::part2(input));
                },)*
                _ => panic!("not implemented"),
            }
        }
    }
}

#[macro_export]
macro_rules! aoc_test {
    ($part:ident, $name:ident, $input:tt, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($part($input), $expected);
        }
    };
}

aoc_main!(day04, day05);
