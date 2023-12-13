type Map = Vec<Vec<bool>>;

pub fn parse(input: &str) -> Vec<Map> {
    let mut rows = vec![];
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        rows.push(
            lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        );
    }
    rows
}

pub fn indices(map: &Map, tolerance: usize) -> (Option<usize>, Option<usize>) {
    let mut cols = None;
    let mut rows = None;
    for col in 1..map[0].len() {
        if map
            .iter()
            .map(|row| {
                row[..col]
                    .iter()
                    .rev()
                    .zip(row[col..].iter())
                    .filter(|(l, r)| l != r)
                    .count()
            })
            .sum::<usize>()
            == tolerance
        {
            cols = Some(col);
            break;
        }
    }
    for row in 1..map.len() {
        if map[..row]
            .iter()
            .rev()
            .zip(map[row..].iter())
            .map(|(up, down)| down.iter().zip(up.iter()).filter(|(u, d)| u != d).count())
            .sum::<usize>()
            == tolerance
        {
            rows = Some(row);
            break;
        }
    }
    (cols, rows)
}

pub fn part1(input: &[Map]) -> usize {
    input
        .iter()
        .map(|map| indices(map, 0))
        .fold(0, |acc, (col, row)| {
            acc + col.unwrap_or(0) + row.unwrap_or(0) * 100
        })
}

pub fn part2(input: &[Map]) -> usize {
    input
        .iter()
        .map(|map| indices(map, 1))
        .fold(0, |acc, (col, row)| {
            acc + col.unwrap_or(0) + row.unwrap_or(0) * 100
        })
}
