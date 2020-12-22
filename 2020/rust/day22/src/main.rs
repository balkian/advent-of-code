use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

type Deck = VecDeque<usize>;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    // pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    let input = &read_to_string(args.get(1).unwrap_or(&"input.txt".into())).unwrap();
    part1(input);
    part2(input);
}

fn part1(input: &str) -> usize {
    let mut decks = parse(input);

    let score = loop {
        if let Some(res) = round(&mut decks) {
            break res;
        }
    };
    println!("Part 1: {}", score);
    score
}

fn part2(input: &str) -> usize {
    let mut decks = parse(input);

    let winner = game2(&mut decks, 1);
    let score = score(&decks[winner]);
    println!("Part 2: {}", score);
    score
}

#[cfg(not(feature="step_by_step"))]
fn pause() {
}

#[cfg(not(feature="step_by_step"))]
macro_rules! debug {
    ($($arg: tt)*) => {}
}


#[cfg(feature="step_by_step")]
fn pause() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
}

#[cfg(feature="step_by_step")]
macro_rules! debug {
    ($($arg: tt)*) => { println!($($arg)*) }
}


fn game2(decks: &mut Vec<Deck>, level: usize) -> usize {
    debug!("Game {}", level);
    let mut memory = HashSet::new();
    loop {
        debug!("P1: {:?}", &decks[0]);
        debug!("P2: {:?}", &decks[1]);
        let key = calculate_hash(&decks);
        pause();

        if memory.contains(&key) {
            return 0;
        }
        memory.insert(key);

        if decks[0].is_empty() {
            return 1;
        } else if decks[1].is_empty() {
            return 0;
        }

        round2(decks, level)
    }
}

fn round2(decks: &mut Vec<Deck>, level: usize) {
    let winner;

    let mut drawn: Vec<usize> = decks
        .iter_mut()
        .map(|deck| deck.pop_front().unwrap())
        .collect();
    if decks
        .iter()
        .zip(drawn.iter())
        .all(|(deck, &card)| deck.len() >= card)
    {
        let mut new_decks = decks
            .iter_mut()
            .zip(drawn.iter())
            .map(|(deck, &card)| deck.iter().take(card).copied().collect::<VecDeque<usize>>())
            .collect();
        winner = game2(&mut new_decks, level + 1);
    } else {
        winner = drawn
            .iter()
            .enumerate()
            .fold((0, 0), |(current, max), (idx, value)| {
                if *value > max {
                    (idx, *value)
                } else {
                    (current, max)
                }
            })
            .0;
    }

    decks[winner].push_back(drawn.remove(winner));
    decks[winner].push_back(drawn.remove(0));
}

fn score(deck: &Deck) -> usize {
    return deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, value)| acc + (idx + 1) * value);
}

fn round(decks: &mut Vec<Deck>) -> Option<usize> {
    let playing: Vec<&Deck> = decks.iter().filter(|x| !x.is_empty()).collect();
    if playing.len() == 1 {
        return Some(score(playing[0]));
    }

    let mut cards: Vec<usize> = decks
        .iter_mut()
        .map(|deck| deck.pop_front().unwrap())
        .collect();

    let winner: usize = cards
        .iter()
        .enumerate()
        .fold((0, 0), |(current, max), (idx, value)| {
            if *value > max {
                (idx, *value)
            } else {
                (current, max)
            }
        })
        .0;

    decks[winner].push_back(cards.remove(winner));
    decks[winner].push_back(cards.remove(0));
    None
}

fn parse(input: &str) -> Vec<Deck> {
    let mut decks = vec![VecDeque::new(), VecDeque::new()];
    let it = &mut input.lines();

    for deck in &mut decks {
        it.next();
        deck.extend(
            it.take_while(|line| !line.is_empty())
                .map(|x| x.parse::<usize>().unwrap()),
        );
    }
    decks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 306);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 291);
    }
}
