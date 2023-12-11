type TimeSeries = Vec<isize>;

pub fn parse(input: &str) -> Vec<TimeSeries> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace().map(|n| n.parse::<isize>().expect("could not convert number {n}")).collect()
        }).collect()
}

fn derive(series: &TimeSeries) -> TimeSeries {
    series.windows(2).map(|v| {
        v[1] - v[0]
     }).collect()
}

fn derive_all(series: &TimeSeries) -> Vec<TimeSeries> {
    let mut series = series.clone();
    let mut hist = vec![];
    while !series.is_empty() && series.iter().any(|&n| n!=0){
        let past = series;
        series = derive(&past);
        hist.push(past);
    }
    hist
}

fn reconstruct(series: &TimeSeries) -> isize {
    let hist = derive_all(series);
    hist.iter().fold(0, |acc, c| acc + c.last().expect("vec was empty"))
}

fn reconstruct_backwards(series: &TimeSeries) -> isize {
    let hist = derive_all(series);
    hist.iter().rev().fold(0, |acc, c| c
        .first()
        .expect("vec was empty") - acc)
}

pub fn part1(input: &[TimeSeries]) -> isize {
    input.iter().map(reconstruct).sum()
}

pub fn part2(input: &[TimeSeries]) -> isize {
    input.iter().map(reconstruct_backwards).sum()
}
