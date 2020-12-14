fn main() {
    let args = aoc_utils::app("13").get_matches();
    let mut lines = aoc_utils::file_iter_clap(&args);
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, bus)| Some((idx, bus.parse().ok()?)))
        .collect();

    let next = buses
        .iter()
        .map(|(_, bus)| (bus - (earliest % bus), bus))
        .min()
        .unwrap();

    println!("Part 1: {:?}", next.0 * next.1);

    let mut period = buses[0].1;
    let mut epoch = period;

    for (delay, current) in buses[1..].iter() {
        for _ in 0..*current {
            if (epoch + delay) % current == 0 {
                break;
            }
            epoch += period;
        }
        period *= current;
    }
    println!("Part 2: {:?}", epoch);
}
