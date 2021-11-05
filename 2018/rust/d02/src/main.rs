use std::collections::HashMap;
use std::fs;

/// To make sure you didn't miss any, you scan the likely candidate boxes again,
/// counting the number that have an ID containing exactly two of any letter and
/// then separately counting those with exactly three of any letter. You can multiply
/// those two counts together to get a rudimentary checksum and compare it to what your device predicts.
fn main() {
    println!("Hello, world!");
    println!("Solution 1: {}", solve(&fs::read_to_string("input").unwrap()));
    println!("Solution 2: {}", solve2(&fs::read_to_string("input").unwrap()));
}


/// Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
/// The boxes will have IDs which differ by exactly one character at the same position in both strings.
fn solve2(input: &str) -> String {
    let words: Vec<&str> = input.lines().collect();
    for i in 0..words.len(){
        for j in i..words.len(){
            let dist = diff(words[i], words[j]);
            if dist.len() == 1 {
                return words[i].chars().enumerate().filter(|&(i, _)| i != dist[0]).map(|(_, v)| v).collect()
            }
        }
    }
    panic!("did not find a solution");
}

fn diff(s1: &str, s2: &str) -> Vec<usize> {
    s1.chars().zip(s2.chars()).enumerate().filter_map(|(ix, (c1, c2))| (if c1 != c2 {Some(ix)} else {None})).collect()
}

fn solve(input: &str) -> i32 {
    let words: Vec<&str> = input.lines().collect();

    let counts = words.iter().fold((0, 0), |(mut t2, mut  t3), word| {
        let mut counts = HashMap::new();
        word.chars().for_each(|c| {
            *counts.entry(c).or_insert(0) += 1;
        });
        if counts.iter().any(|(_, count)| *count == 2) {
            t2 += 1;
        }
        if counts.iter().any(|(_, count)| *count == 3) {
            t3 += 1;
        }
        (t2, t3)
    });
    counts.0 * counts.1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ex1(){
        assert_eq!(solve("
abcdef
bababc 
abbcde 
abcccd 
aabcdd 
abcdee 
ababab"), 12);

    }

    #[test]
    fn ex2(){
        assert_eq!(solve2("abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"), "fgij");

    }
}
