use regex::Regex;

pub fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r"row (?P<row>\d+), column (?P<column>\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let row: usize = cap.name("row").unwrap().as_str().parse().unwrap();
    let column: usize = cap.name("column").unwrap().as_str().parse().unwrap();
    (row, column)
}
pub fn part1(&(target_row, target_column): &(usize, usize)) -> usize {
    let mut value = 20151125;
    let mul = 252533;
    let modulus = 33554393;
    for max_row in 2.. {
        let mut row = max_row;
        for col in 1..=max_row {
            value = (value * mul) % modulus;
            if row == target_row && col == target_column {
                return value;
            }
            row -= 1;
        }
    }
    unreachable!();
}
pub fn part2(&_: &(usize, usize)) -> &str {
    "No second part"
}
