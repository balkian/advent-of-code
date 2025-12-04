pub fn parse(input: &str) -> Vec<Vec<bool>> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim().chars().map(|c| c == '@').collect()
        }).collect()
}

pub fn neighbors((x, y): (usize, usize), (lenx, leny): (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
    let xrange = x.saturating_sub(1)..(x+2).min(lenx);
    let yrange = y.saturating_sub(1)..(y+2).min(leny);
    xrange.flat_map(move |nx| {
        yrange.clone().map(move |ny| (nx, ny))
    }).filter(move |(nx, ny)| *nx != x || *ny != y)
}

pub fn part1(map: &[Vec<bool>]) -> usize {

    let mut total = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }
            let filled = neighbors((i, j), (map.len(), row.len())).filter(|(nx, ny)| map[*nx][*ny]).count();
            if filled < 4 {
                //eprintln!("({i}, {j}) {filled}");
                total += 1;
            }
        }
    }
    total
}

pub fn part2(map: &[Vec<bool>]) -> usize {
    let mut total = 0;
    let mut map: Vec<Vec<bool>> = map.to_vec();
    let mut nmap: Vec<Vec<bool>>;
    loop {
        let mut removed = 0;
        nmap = map.clone();
        for (i, row) in map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if !cell {
                    continue;
                }
                let filled = neighbors((i, j), (map.len(), row.len())).filter(|(nx, ny)| map[*nx][*ny]).count();
                if filled < 4 {
                    //eprintln!("({i}, {j}) {filled}");
                    removed += 1;
                    nmap[i][j] = false;
                }
            }
        }
        if removed == 0 {
            break;
        }
        total += removed;
        map = nmap;
    }
    total
}
