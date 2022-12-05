use counter::Counter;
use regex::Regex;

#[derive(Debug)]
pub struct Room<'a> {
    id: usize,
    checksum: &'a str,
    name: &'a str,
    calculated_checksum: String,
}

pub fn parse(input: &str) -> Vec<Room<'_>> {
    let re = Regex::new(r"(?P<name>[a-zA-Z-]+)-(?P<id>\d+)\[(?P<checksum>\w+)\]")
        .expect("invalid regex");
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let name = cap.name("name").unwrap().as_str();
            let id: usize = cap.name("id").unwrap().as_str().parse().unwrap();
            let checksum = cap.name("checksum").unwrap().as_str();
            let calculated_checksum = name
                .replace('-', "")
                .chars()
                .collect::<Counter<_>>()
                .most_common_tiebreaker(|&a, &b| a.cmp(&b))
                .iter()
                .into_iter()
                .map(|(v, _)| v)
                .take(5)
                .collect();
            Room {
                id,
                name,
                checksum,
                calculated_checksum,
            }
        })
        .collect()
}

pub fn part1(input: &[Room]) -> usize {
    input
        .iter()
        .filter(|room| room.checksum == room.calculated_checksum)
        .fold(0, |acc, room| acc + room.id)
}

pub fn part2(input: &[Room]) -> usize {
    let modulo = (b'z' - b'a' + 1) as usize;
    let min = b'a' as usize;
    input
        .iter()
        .filter(|room| room.checksum == room.calculated_checksum)
        .find(|room| {
            room.name
                .as_bytes()
                .iter()
                .filter(|&c| *c != b'-')
                .map(|c| ((((*c as usize) + room.id - min) % modulo) + min) as u8 as char)
                .collect::<String>()
                == "northpoleobjectstorage"
        })
        .map(|room| room.id)
        .unwrap()
}
