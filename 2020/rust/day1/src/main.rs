use aoc_utils;

// fn part1<'a,T>(it: T) -> Option<i32>
// where
//     T: IntoIterator<Item=&'a i32>,
// {

//     let mut nums = Vec::<i32>::new();
//     for num in it {
//         if nums.contains(num) {
//             println!("Found: {:} - {:}", num, num*(2020-num));
//             return Some(*num);
//         }
//         nums.push(2020-*num);
//     }
//     None
// }

#[derive(Debug)]
struct Mix {
    remaining: i32,
    others: Vec<i32>,
}

fn part2<'a, T>(it: T, cap: usize) -> Option<i32>
where
    T: IntoIterator<Item = &'a i32>,
{
    let mut opts: Vec<Mix> = Vec::new();

    for num in it {
        // dbg!{&opts};
        let mut new_opts: Vec<Mix> = Vec::new();
        for opt in opts.iter() {
            if opt.remaining >= *num {
                let mut others = opt.others.clone();
                others.push(*num);
                let new_opt = Mix {
                    remaining: opt.remaining - num,
                    others: others,
                };
                if new_opt.remaining == 0 && new_opt.others.len() == cap {
                    println!(
                        "Found {:?}. {:}",
                        new_opt.others,
                        new_opt.others.iter().fold(1, |a, b| a * b)
                    );
                }
                new_opts.push(new_opt);
            }
        }
        new_opts.push(Mix {
            remaining: 2020 - *num,
            others: vec![*num],
        });
        opts.extend(new_opts);
    }
    None
}

fn main() {
    let it: Vec<i32> = aoc_utils::file_iter();
    // dbg!(&it);
    // part1(&it);
    part2(&it, 2);
    part2(&it, 3);
}
