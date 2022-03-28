type Algo = [bool; 512];
type Image = Vec<Vec<bool>>;

pub fn parse(input: &str) -> (Algo, Image) {
    let lines: Vec<Vec<bool>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    c => panic!("unknown literal {c}"),
                })
                .collect()
        })
        .collect();
    let algo: Algo = lines[0].iter().as_slice().try_into().unwrap();
    let image: Image = lines[1..]
        .iter()
        .map(|line| Vec::from_iter(line.clone()))
        .collect();
    (algo, image)
}

fn print(image: &Image) {
    println!("Printing image");

    for row in image {
        for &cell in row {
            let c = if cell { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn evolve(image: &Image, algo: &Algo, infinity: bool) -> (Image, bool) {
    let added = 6;
    let padding = added / 2;

    let new_infinity = if infinity { algo[511] } else { algo[0] };

    let mut result = vec![vec![new_infinity; image[0].len() + added]; image.len() + added];

    for (i, row) in result.iter_mut().enumerate() {
        let y = i as isize;
        for (j, cell) in row.iter_mut().enumerate() {
            let mut count = 0;
            let x = j as isize;

            for ny in (y - 1)..=(y + 1) {
                let dy = ((y + 1) - ny) as u32;

                for nx in (x - 1)..=(x + 1) {
                    if nx as usize >= padding
                        && ny as usize >= padding
                        && (ny as usize) - padding < image.len()
                        && (nx as usize) - padding < image[ny as usize - padding].len()
                    {
                        if !image[(ny as usize) - padding][(nx as usize) - padding] {
                            // print!(".");
                            continue;
                        }
                    } else if !infinity {
                        continue;
                    }
                    // print!("#");
                    let dx = ((x + 1) - nx) as u32;
                    count += 2usize.pow(3 * (dy as u32) + (dx as u32));
                }
                // println!();
            }
            *cell = algo[count];
            // println!("{count}");
            // println!();
        }
    }

    (result, new_infinity)
}

pub fn solve((algo, image): &(Algo, Image), times: usize, printing: bool) -> usize {
    let mut image = image.clone();
    let mut infinity = false;
    dbg!(algo[0]);
    for _ in 0..times {
        (image, infinity) = evolve(&image, algo, infinity);
        if printing {
            print(&image);
        }
    }
    image.iter().flatten().filter(|&x| *x).count()
}
pub fn part1(problem: &(Algo, Image)) -> usize {
    // print(&problem.1);
    solve(problem, 2, false)
}

pub fn part2(problem: &(Algo, Image)) -> usize {
    solve(problem, 50, false)
}
