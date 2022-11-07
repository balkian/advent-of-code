pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|st| {
            let vowels = ['a', 'e', 'i', 'o', 'u'];
            if st.chars().filter(|c| vowels.contains(c)).count() < 3 {
                return false;
            }
            let chars: Vec<char> = st.chars().collect();
            if !chars.windows(2).any(|a| a[0] == a[1]) {
                return false;
            }
            if ["ab", "cd", "pq", "xy"].iter().any(|c| st.contains(c)) {
                return false;
            }
            true
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|st| {
            let chars: Vec<char> = st.chars().collect();
            let pairs = chars
                .windows(2)
                .enumerate()
                .any(|(ix, a)| chars[ix + 2..].windows(2).any(|b| a == b));
            let single = chars.iter().zip(chars.iter().skip(2)).any(|(a, b)| a == b);
            pairs && single
        })
        .count()
}

crate::aoc_test!(
    part2, ex1, "qjhvhtzxzqqjkmpb", 1;
    part2, ex2, "xxyxx", 1;
    part2, ex3, "uurcxstgmygtbstg", 0;
    part2, ex4, "ieodomkazucvgmuy", 0;
);
