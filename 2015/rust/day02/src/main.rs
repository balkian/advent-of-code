use std::env;

fn calculate(l: &usize, w: &usize, h: &usize) -> usize {
    let sides = [l * w, w * h, h * l];
    let min = sides.iter().min().unwrap().to_owned();
    2 * sides.iter().sum::<usize>() + min
}

fn ribbon(l: &usize, w: &usize, h: &usize) -> usize {
    let m3 = l * w * h;
    let perim: usize = [(l + w), (w + h), (h + l)].iter().min().unwrap() * 2;
    m3 + perim
}

fn main() {
    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let dimensions: Vec<(usize, usize, usize)> = std::fs::read_to_string(file)
        .expect("could not read the file")
        .lines()
        .into_iter()
        .map(|x| {
            let dims: Vec<usize> = x.split('x').map(|token| token.parse().unwrap()).collect();
            (dims[0], dims[1], dims[2])
        })
        .collect();
    let total: usize = dimensions.iter().map(|(w, l, h)| calculate(w, l, h)).sum();
    println!("Part 1: {}", total);

    let ribbon: usize = dimensions.iter().map(|(w, l, h)| ribbon(w, l, h)).sum();
    println!("Part 2: {}", ribbon);
}
