use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use counter::Counter;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\] (.*)").unwrap();
    static ref GU: Regex = Regex::new(r"Guard \#(\d+) begins shift").unwrap();
}


#[derive(Debug,PartialEq)]
enum Action {
    Start(ID),
    Sleep,
    WakeUp,

}

#[derive(Debug)]
struct Time {
    year: usize,
    month: usize,
    day: usize,

    hour: usize,
    minute: Minute,
}

type Minute = usize;
type ID = usize;

#[derive(Debug)]
pub struct Record {
    guard: ID,

    time: Time,

    action: Action,

}

fn parse_line(text: &str) -> (Time, Action) {
    let cap = RE.captures(text).unwrap();
    // dbg!{&cap};
    let time = Time{
        year: cap[1].parse().unwrap(),
        month: cap[2].parse().unwrap(),
        day: cap[3].parse().unwrap(),
        hour: cap[4].parse().unwrap(),
        minute: cap[5].parse().unwrap()
    };
    let action = if let Some(gc) = GU.captures(&cap[6]){
        Action::Start(gc[1].parse().unwrap())
    } else if cap[6].contains("wakes") {
        Action::WakeUp
    } else {
        Action::Sleep
    };
    (time, action)
}

fn parse(input: &str) -> Vec<Record> {
    // (t1, a1), (t2, a2)| {
    let mut res: Vec<(Time, Action)> = input
        .lines()
        .map(parse_line).collect();

    res.sort_by_key(|(t, _)| (t.year, t.month, t.day, t.hour, t.minute));
    let records: Vec<Record> = res.into_iter().scan(None, |acc, (t, a)| {
        let guard = if let Action::Start(guard) = a {
            *acc = Some(guard);
            guard
        } else {
            acc.unwrap()
        };
        Some(Record{time: t, action: a, guard: (guard as ID)})
        }).collect();
    records
}

fn get_minutes(input: &str) -> Vec<(ID, Minute)> {
    let records = parse(input);
    let mut asleep: Vec<(ID, Minute)> = vec!();
    let mut naptime: usize = 0;
    let mut naphour: usize = 0;
    let mut napyear: usize = 0;
    let mut turn: ID = 0;
    // dbg!(&records);
    for record in records {
        match record.action {
            Action::Start(who) => {
                turn = who;
            },
            Action::Sleep =>  {
                assert_eq!(record.guard, turn);
                naptime = record.time.minute;
                naphour = record.time.hour;
                napyear = record.time.year;
            },
            Action::WakeUp => {
                assert_eq!(record.guard, turn);
                assert_eq!(naphour, record.time.hour);
                assert_eq!(napyear, record.time.year);
                for time in naptime..record.time.minute {
                    asleep.push((record.guard, time));
                }
            }
        }
    }
    asleep
}

fn solve1(input: &str) -> usize {
    let asleep = get_minutes(input);
    let sleepcount: Counter<_> = asleep.iter().map(|(agent, _)| agent).collect();
    let sleeper = sleepcount.most_common()[0].0;
    let minutecounts: Counter<_> = asleep.iter()
        .filter(|(agent, _)| agent == sleeper)
        .map(|(_, minute)| minute).collect();
    let when = minutecounts.most_common()[0].0;
    when * sleeper
}

fn solve2(input: &str) -> usize {
    let asleep = get_minutes(input);
    let sleepcount: Counter<_> = asleep.iter().collect();
    let (who, when) = sleepcount.most_common()[0].0;
    when * who
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();
    println!("Part 1: {}", solve1(input));
    println!("Part 2: {}", solve2(input));
    // println!("Part 2: {}", solve2(&input));
}

#[test]
fn test_example(){
    assert_eq!(solve1(include_str!("../example")), 240);
    assert_eq!(solve2(include_str!("../example")), 4455);
}

#[test]
fn test_parse(){
    let (time, action) = parse_line("[1518-03-14 00:43] falls asleep");
    assert_eq!(time.year, 1518);
    assert_eq!(time.month, 3);
    assert_eq!(time.day, 14);
    assert_eq!(time.hour, 0);
    assert_eq!(time.minute, 43);

    assert_eq!(action, Action::Sleep);
}
