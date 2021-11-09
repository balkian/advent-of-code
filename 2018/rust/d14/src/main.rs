// use std::time::Duration;
// use std::thread::sleep;

fn main() {
    println!("Solution 1: {}", solve1(430971));
    println!("Solution 2: {}", solve2("430971"));
}

fn solve1(idx: usize) -> usize {
    let scores = play(|scores| scores.len() >= 10 + idx);
    scores
        .iter()
        .skip(idx)
        .take(10)
        .fold(0, |acc, digit| acc * 10 + digit)
}

fn play(stop: impl Fn(&Vec<usize>) -> bool) -> Vec<usize> {
    let mut players = [0, 1];
    let mut scores = vec![3, 7];
    loop {
        // dbg!(&players);
        let newscore = scores[players[0]] + scores[players[1]];
        // dbg!(&newscore);
        if newscore >= 10 {
            scores.push(newscore / 10);
            if stop(&scores) {
                return scores;
            }
        }
        scores.push(newscore % 10);
        if stop(&scores) {
            return scores;
        }

        players
            .iter_mut()
            .for_each(|val| *val = (*val + 1 + scores[*val]) % scores.len());
    }
}

fn solve2(idx: &str) -> usize {
    let mask: Vec<usize> = idx
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let scores = play(|scores| {
        if scores.len() < mask.len() {
            return false;
        }
        // dbg!(&scores);
        // sleep(Duration::from_millis(1000));
        scores
            .iter()
            .rev()
            .zip(mask.iter().rev())
            .take(mask.len())
            .all(|(a, b)| a == b)
    });
    scores.len() - mask.len()
}

#[test]
fn test_example() {
    assert_eq!(solve1(9), 5158916779);
    assert_eq!(solve1(5), 0124515891);
    assert_eq!(solve1(18), 9251071085);
    assert_eq!(solve1(2018), 5941429882);
}

#[test]
fn test_example2() {
    assert_eq!(solve2("51589"), 9);
    assert_eq!(solve2("01245"), 5);
    assert_eq!(solve2("92510"), 18);
    assert_eq!(solve2("59414"), 2018);
}
