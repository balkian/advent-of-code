pub fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

fn complete(input: &str) -> (usize, usize) {
    let mut opening = vec![];
    for c in input.chars() {
        match (c, opening.last()) {
            (')', Some('(')) | (']', Some('[')) | ('}', Some('{')) | ('>', Some('<')) => {
                opening.pop();
            }
            ('(' | '[' | '{' | '<', _) => opening.push(c),
            (')', _) => return (3, 0),
            (']', _) => return (57, 0),
            ('}', _) => return (1197, 0),
            ('>', _) => return (25137, 0),
            _ => panic!("unknown situation"),
        }
    }
    let mut score = 0;
    for i in opening.into_iter().rev() {
        score *= 5;
        score += match i {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        }
    }
    (0, score)
}
pub fn part1(input: &[&str]) -> usize {
    input.iter().map(|line| complete(line).0).sum::<usize>()
}
pub fn part2(input: &[&str]) -> usize {
    let mut scores: Vec<usize> = input
        .iter()
        .map(|line| complete(line).1)
        .filter(|score| *score != 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[test]
fn test_example() {
    let input = parse(
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
    );
    assert_eq!(part1(&input), 26397);
    assert_eq!(part2(&input), 288957);
}
