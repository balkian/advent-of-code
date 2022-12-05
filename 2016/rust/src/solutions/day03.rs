type Tri = [usize; 3];

pub fn parse(input: &str) -> Vec<Tri> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

pub fn part1(input: &[Tri]) -> usize {
    input
        .iter()
        .filter(|&line| {
            let mut line = *line;
            line.sort();
            line[0] + line[1] > line[2]
        })
        .count()
}

pub fn part2(input: &[Tri]) -> usize {
    // There's probably a more elegant way to do it with multiunzip or similar
    let mut triangles = vec![];
    let mut temp: [Tri; 3] = Default::default();

    for chunk in input.chunks(3) {
        for (idx, line) in chunk.iter().enumerate() {
            for (t, num) in temp.iter_mut().zip(line.iter()) {
                t[idx] = *num;
            }
        }
        triangles.extend(temp.into_iter());
        temp = Default::default();
    }
    part1(&triangles)
}
