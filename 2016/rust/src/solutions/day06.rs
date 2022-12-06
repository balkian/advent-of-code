use counter::Counter;

pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.trim().into()).collect()
}

pub fn solve(input: &[String], max: bool) -> String {
    let counters = input.iter().fold(
        vec![Counter::<char>::new(); input[0].len()],
        |mut acc, word| {
            acc.iter_mut().zip(word.chars()).for_each(|(count, w)| {
                count.update(std::iter::once(w));
            });
            acc
        },
    );
    counters
        .into_iter()
        .map(|c| {
            if max {
                c.k_most_common_ordered(1)[0].0
            } else {
                c.most_common_ordered().last().unwrap().0
            }
        })
        .collect()
}

pub fn part1(input: &[String]) -> String {
    solve(input, true)
}

pub fn part2(input: &[String]) -> String {
    solve(input, false)
}
