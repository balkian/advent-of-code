use std::fs;
use std::collections::LinkedList;

fn main() {
    let input = fs::read_to_string("input").expect("could not read file");
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

fn play1(nplayers: usize, last: usize)  -> usize{
    let mut marbles = LinkedList::from([0]);
    let mut scores = vec![0; nplayers];
    for i in 1..=last {
        if i % 23 == 0 {
            let player = i % scores.len();
            scores[player] += i;
            for _ in 0..6 {
                let removed = marbles.pop_back().unwrap();
                marbles.push_front(removed);
            }
            scores[player] += marbles.pop_back().unwrap();

        } else {
            for _ in 0..2 {
                let removed = marbles.pop_front().unwrap();
                marbles.push_back(removed);
            }
            marbles.push_front(i);
        }
    }
    scores.iter().max().unwrap().to_owned()
    
}

fn parse(input: &str) -> (usize, usize) {
    let tokens: Vec<&str> = input.split(' ').collect();
    let nplayers = tokens[0].parse().unwrap();
    let last = tokens[6].parse().unwrap();
    (nplayers, last)
}

fn solve1(input: &str) -> usize{
    let (nplayers, last) = parse(input);
    play1(nplayers, last)

}
fn solve2(input: &str) -> usize{
    let (nplayers, last) = parse(input);
    play1(nplayers, last * 100)
}

#[test]
fn test_example(){
    assert_eq!(solve1("10 players; last marble is worth 1618 points"), 8317);
    assert_eq!(solve1("13 players; last marble is worth 7999 points"), 146373);
    assert_eq!(solve1("17 players; last marble is worth 1104 points"), 2764);
    assert_eq!(solve1("21 players; last marble is worth 6111 points"), 54718);
    assert_eq!(solve1("30 players; last marble is worth 5807 points"), 37305);
}
