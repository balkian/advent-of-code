fn main() {
    let args = aoc_utils::app("10").get_matches();
    let mut numbers = aoc_utils::file_iter_clap(&args)
        .map(|x| x.parse().expect("could not parse number"))
        .collect::<Vec<usize>>();
    numbers.push(0);
    numbers.sort_unstable();
    let max = numbers.last().unwrap() + 3;
    numbers.push(max);
    // dbg!(&numbers);

    let diffs = numbers
        .iter()
        .fold((0, 0, &0), |(ones, threes, x), y| match y - x {
            1 => (ones + 1, threes, y),
            3 => (ones, threes + 1, y),
            _ => (ones, threes, y),
        });
    println!("Part 1: {:}", diffs.0 * diffs.1);

    // We start iterating from the end so there are fewer options
    let mut numbers = numbers.into_iter().rev();
    let mut opts: Vec<(usize, usize)> = vec![(numbers.next().unwrap(), 1)];

    for smaller in numbers {
        let mut to_this = 0;
        // dbg!{&smaller};
        for idx in (0..opts.len()).rev() {
            let opt = opts[idx].0;
            let count = opts[idx].1;
            match (opt as isize - smaller as isize).abs() {
                0..=3 => {
                    to_this += count;
                }
                _ => {
                    opts.remove(idx);
                }
            }
        }
        opts.push((smaller, to_this));
        // dbg!(&opts);
    }
    for i in (0..opts.len()).rev() {
        if opts[i].0 != 0 {
            opts.remove(i);
        }
    }
    println!("Part 2: {:?}", opts[0].1)
}
