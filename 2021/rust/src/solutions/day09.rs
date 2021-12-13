pub fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    lowest(input)
        .iter()
        .map(|(y, x)| input[*y][*x] + 1)
        .sum::<usize>()
}

fn lowest(input: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for j in 0..input.len() {
        for i in 0..input[j].len() {
            if i > 0 && input[j][i - 1] <= input[j][i] {
                continue;
            }
            if i < input[j].len() - 1 && input[j][i + 1] <= input[j][i] {
                continue;
            }
            if j > 0 && input[j - 1][i] <= input[j][i] {
                continue;
            }
            if j < input.len() - 1 && input[j + 1][i] <= input[j][i] {
                continue;
            }
            points.push((j, i));
        }
    }
    points
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    let mut basins = vec![];
    for point in lowest(input) {
        let mut basin = vec![];
        let mut candidates = vec![(0, point)];
        while let Some((prev, (j, i))) = candidates.pop() {
            let this = input[j][i];
            if basin.contains(&(j, i)) || this == 9 || prev > this {
                continue;
            }
            basin.push((j, i));
            if i > 0 {
                candidates.push((this, (j, i - 1)));
            }
            if i < input[j].len() - 1 {
                candidates.push((this, (j, i + 1)));
            }
            if j > 0 {
                candidates.push((this, (j - 1, i)));
            }
            if j < input.len() - 1 {
                candidates.push((this, (j + 1, i)));
            }
        }
        basins.push(basin);
    }
    let mut sizes: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product::<usize>()
}

#[test]
fn test_example() {
    let input = &parse(
        "2199943210
3987894921
9856789892
8767896789
9899965678",
    );
    assert_eq!(part1(input), 15);
    assert_eq!(part2(input), 1134);
}
