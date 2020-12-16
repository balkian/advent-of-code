use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RULE: Regex =
        Regex::new(r"(?P<field>[\w ]+): (?P<n0>\d+)-(?P<n1>\d+) or (?P<n2>\d+)-(?P<n3>\d+)")
            .unwrap();
}

type Range = (usize, usize, usize, usize);
type Ticket = Vec<usize>;
type TicketRef<'a> = &'a [usize];

fn read_ticket(line: &str) -> Vec<usize> {
    let mut values = Vec::new();
    for num in line.split(',') {
        values.push(num.parse().unwrap());
    }
    values
}

fn part1(ranges: &[Range], ticket: &[usize], notfound: &mut Vec<usize>) -> bool {
    let mut found = true;
    'outer: for number in ticket {
        for v in ranges {
            if within(number, v) {
                continue 'outer;
            }
        }
        found = false;
        notfound.push(*number);
    }
    found
}

fn within(number: &usize, range: &Range) -> bool {
    ((range.0)..=(range.1)).contains(number) || ((range.2)..=(range.3)).contains(number)
}

fn part2(valid: &[&Ticket], names: &[String], ranges: &[Range], mine: TicketRef) {
    let mut poss: Vec<Vec<usize>> = names.iter().map(|_x| (0..ranges.len()).collect()).collect();
    for (field_idx, range) in ranges.iter().enumerate() {
        for ticket in valid {
            for (value_idx, value) in ticket.iter().enumerate() {
                if !within(value, range) {
                    poss[field_idx].retain(|&x| x != value_idx);
                    continue;
                }
            }
        }
    }
    let mut locked: HashMap<String, usize> = HashMap::new();
    while locked.len() != poss.len() {
        for pos_idx in 0..poss.len() {
            if poss[pos_idx].len() == 1 {
                let found = poss[pos_idx][0];
                locked.insert(names[pos_idx].clone(), found);
                for other in poss.iter_mut() {
                    other.retain(|&x| x != found);
                }
            }
        }
    }
    // dbg!{&locked};

    let mut result = 1;
    for (name, idx) in locked {
        if name.contains("departure") {
            result *= mine[idx];
        }
    }
    println!("Part 2: {}", result);
}

fn main() {
    let args = aoc_utils::app(env!("CARGO_PKG_NAME")).get_matches();
    let mut file = aoc_utils::file_iter_clap(&args);

    let mut names: Vec<String> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();

    for line in &mut file {
        if line.is_empty() {
            break;
        }

        let reg = RULE.captures(&line).unwrap();

        names.push(reg.name("field").unwrap().as_str().to_string());
        ranges.push((
            reg.name("n0").unwrap().as_str().parse().unwrap(),
            reg.name("n1").unwrap().as_str().parse().unwrap(),
            reg.name("n2").unwrap().as_str().parse().unwrap(),
            reg.name("n3").unwrap().as_str().parse().unwrap(),
        ));
    }

    assert_eq!("your ticket:", file.next().unwrap());

    let mine = read_ticket(&file.next().unwrap());

    assert_eq!("nearby tickets:", file.nth(1).unwrap());

    let mut nearby = Vec::new();

    for ticket in file {
        nearby.push(read_ticket(&ticket));
    }

    let notfound = &mut Vec::new();
    let mut valid: Vec<&Vec<usize>> = nearby
        .iter()
        .filter(|ticket| part1(&ranges, ticket, notfound))
        .collect();
    valid.push(&mine);

    println!("Part 1: {}", notfound.iter().sum::<usize>());

    part2(&valid, &names, &ranges, &mine);
}
